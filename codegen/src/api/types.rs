use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, btree_map},
    sync::Arc,
};

use aide::openapi;
use anyhow::{Context as _, bail, ensure};
use indexmap::IndexMap;
use schemars::schema::{
    InstanceType, ObjectValidation, Schema, SchemaObject, SingleOrVec, SubschemaValidation,
};
use serde::{Deserialize, Serialize};

use crate::cli_v1::IncludeMode;

use heck::ToUpperCamelCase as _;

use super::{
    get_schema_name,
    resources::{self, Resource, Resources},
};

/// Named types referenced by API operations.
///
/// Intermediate representation of (some) `components` from the spec.
pub(crate) type Types = BTreeMap<String, Type>;

pub(crate) fn from_referenced_components(
    res: &Resources,
    schemas: &mut IndexMap<String, openapi::SchemaObject>,
    webhooks: &[String],
    include_mode: IncludeMode,
) -> Types {
    let mut referenced_components: Vec<&str> = match include_mode {
        IncludeMode::OnlyPublic | IncludeMode::PublicAndInternal | IncludeMode::OnlyInternal => {
            webhooks.iter().map(|s| &**s).collect()
        }
        IncludeMode::OnlySpecified => vec![],
    };
    referenced_components.extend(resources::referenced_components(res));

    let mut types = BTreeMap::new();
    let mut add_type = |schema_name: &str, extra_components: &mut BTreeSet<_>| {
        let Some(s) = schemas.swap_remove(schema_name) else {
            tracing::warn!(schema_name, "schema not found");
            return;
        };

        let obj = match s.json_schema {
            Schema::Bool(_) => {
                tracing::warn!(schema_name, "found $ref'erenced bool schema, wat?!");
                return;
            }
            Schema::Object(o) => o,
        };

        match Type::from_schema(schema_name.to_owned(), obj) {
            Ok(ty) => {
                extra_components.extend(
                    ty.referenced_components()
                        .into_iter()
                        .filter(|&c| c != schema_name && !types.contains_key(c))
                        .map(ToOwned::to_owned),
                );
                types.insert(schema_name.to_owned(), ty);
            }
            Err(e) => {
                tracing::warn!(schema_name, "unsupported schema: {e:#}");
            }
        }
    };

    let mut extra_components: BTreeSet<_> = referenced_components
        .into_iter()
        .map(ToOwned::to_owned)
        .collect();
    while let Some(c) = extra_components.pop_first() {
        add_type(&c, &mut extra_components);
    }

    // Resolve SchemaRef inner types for string alias detection
    // This allows to_XXX_typename() methods to check if a reference is to a string alias
    resolve_schema_refs(&mut types);

    types
}

/// Resolve SchemaRef inner types by looking up the referenced type.
/// This allows type conversion methods to detect string aliases and return the appropriate type.
fn resolve_schema_refs(types: &mut Types) {
    let string_alias_names = collect_string_alias_names(types);

    // Update SchemaRef inner fields for string alias references
    for ty in types.values_mut() {
        resolve_schema_refs_in_type(ty, &string_alias_names);
    }
}

/// Collect the names of all string alias types.
pub(crate) fn collect_string_alias_names(types: &Types) -> BTreeSet<String> {
    types
        .iter()
        .filter(|(_, ty)| matches!(ty.data, TypeData::StringAlias))
        .map(|(name, _)| name.clone())
        .collect()
}

/// Resolve a SchemaRef field type if it references a string alias.
/// This is public so it can be used by the resources module.
pub(crate) fn resolve_schema_ref_in_field_type_public(
    field_type: &mut FieldType,
    string_alias_names: &BTreeSet<String>,
) {
    resolve_schema_ref_in_field_type(field_type, string_alias_names);
}

fn resolve_schema_refs_in_type(ty: &mut Type, string_alias_names: &BTreeSet<String>) {
    match &mut ty.data {
        TypeData::Struct { fields } => {
            for field in fields {
                resolve_schema_ref_in_field_type(&mut field.r#type, string_alias_names);
            }
        }
        TypeData::StructEnum { fields, repr, .. } => {
            for field in fields {
                resolve_schema_ref_in_field_type(&mut field.r#type, string_alias_names);
            }
            match repr {
                StructEnumRepr::AdjacentlyTagged { variants, .. }
                | StructEnumRepr::InternallyTagged { variants } => {
                    for variant in variants {
                        if let EnumVariantType::Struct { fields } = &mut variant.content {
                            for field in fields {
                                resolve_schema_ref_in_field_type(
                                    &mut field.r#type,
                                    string_alias_names,
                                );
                            }
                        }
                    }
                }
            }
        }
        TypeData::StringEnum { .. } | TypeData::IntegerEnum { .. } | TypeData::StringAlias => {}
    }
}

fn resolve_schema_ref_in_field_type(
    field_type: &mut FieldType,
    string_alias_names: &BTreeSet<String>,
) {
    match field_type {
        FieldType::SchemaRef { name, inner }
            if string_alias_names.contains(name) && inner.is_none() =>
        {
            *inner = Some(Type {
                name: name.clone(),
                description: None,
                deprecated: false,
                data: TypeData::StringAlias,
            });
        }
        FieldType::List { inner } | FieldType::Set { inner } => {
            resolve_schema_ref_in_field_type(Arc::make_mut(inner), string_alias_names);
        }
        FieldType::Map { value_ty } => {
            resolve_schema_ref_in_field_type(Arc::make_mut(value_ty), string_alias_names);
        }
        _ => {}
    }
}

/// Promote every inline `FieldType::StringEnum` into a named top-level
/// `TypeData::StringEnum` and rewrite the field to `FieldType::SchemaRef`.
///
/// Name selection:
/// - If an existing top-level string enum has exactly the same values, reuse it.
/// - Else, use the schema `title` if present.
/// - Else, derive from context: `{parent}_{field}` (or `..._item` / `..._value`
///   when promoted through a List/Set/Map), converted to UpperCamelCase.
///
/// After this pass, `FieldType::StringEnum` must not appear anywhere; the render
/// methods treat it as `unreachable!`.
pub(crate) fn promote_inline_enums(
    types: &mut Types,
    resources: &mut Resources,
) -> anyhow::Result<()> {
    // Snapshot existing top-level string enums keyed by their value set, so we
    // can reuse them instead of generating duplicates (e.g. the subscription
    // `statuses` filter reuses the existing `SubscriptionStatusEnum`).
    let existing_by_values: BTreeMap<Vec<String>, String> = types
        .iter()
        .filter_map(|(name, ty)| match &ty.data {
            TypeData::StringEnum { values } => Some((values.clone(), name.clone())),
            _ => None,
        })
        .collect();

    let mut new_types: BTreeMap<String, Type> = BTreeMap::new();

    for (type_name, ty) in types.iter_mut() {
        match &mut ty.data {
            TypeData::Struct { fields } => {
                for field in fields {
                    let base = format!("{}_{}", type_name, field.name);
                    promote_field_type(
                        &mut field.r#type,
                        &base,
                        &existing_by_values,
                        &mut new_types,
                    )?;
                }
            }
            TypeData::StructEnum { fields, repr, .. } => {
                for field in fields {
                    let base = format!("{}_{}", type_name, field.name);
                    promote_field_type(
                        &mut field.r#type,
                        &base,
                        &existing_by_values,
                        &mut new_types,
                    )?;
                }
                let variants = match repr {
                    StructEnumRepr::AdjacentlyTagged { variants, .. }
                    | StructEnumRepr::InternallyTagged { variants } => variants,
                };
                for variant in variants {
                    if let EnumVariantType::Struct { fields } = &mut variant.content {
                        for field in fields {
                            let base = format!("{}_{}_{}", type_name, variant.name, field.name);
                            promote_field_type(
                                &mut field.r#type,
                                &base,
                                &existing_by_values,
                                &mut new_types,
                            )?;
                        }
                    }
                }
            }
            TypeData::StringEnum { .. } | TypeData::IntegerEnum { .. } | TypeData::StringAlias => {}
        }
    }

    for resource in resources.values_mut() {
        promote_inline_enums_in_resource(resource, &existing_by_values, &mut new_types)?;
    }

    for (name, ty) in new_types {
        match types.entry(name) {
            btree_map::Entry::Vacant(e) => {
                e.insert(ty);
            }
            btree_map::Entry::Occupied(e) => {
                if *e.get() != ty {
                    bail!(
                        "promoted inline enum `{}` collides with an existing type of different shape",
                        e.key()
                    );
                }
            }
        }
    }

    Ok(())
}

fn promote_inline_enums_in_resource(
    resource: &mut Resource,
    existing_by_values: &BTreeMap<Vec<String>, String>,
    new_types: &mut BTreeMap<String, Type>,
) -> anyhow::Result<()> {
    for sub in resource.subresources.values_mut() {
        promote_inline_enums_in_resource(sub, existing_by_values, new_types)?;
    }
    for op in &mut resource.operations {
        let op_id = op.id.clone();
        for param in &mut op.query_params {
            let base = format!("{}_{}", op_id, param.name);
            promote_field_type(&mut param.r#type, &base, existing_by_values, new_types)?;
        }
    }
    Ok(())
}

fn promote_field_type(
    ft: &mut FieldType,
    base_name: &str,
    existing_by_values: &BTreeMap<Vec<String>, String>,
    new_types: &mut BTreeMap<String, Type>,
) -> anyhow::Result<()> {
    match ft {
        FieldType::StringEnum { values, title } => {
            let values = std::mem::take(values);
            if let Some(existing_name) = existing_by_values.get(&values) {
                *ft = FieldType::SchemaRef {
                    name: existing_name.clone(),
                    inner: None,
                };
                return Ok(());
            }
            let name = match title.take() {
                Some(t) => t.to_upper_camel_case(),
                None => base_name.to_upper_camel_case(),
            };
            let promoted = Type {
                name: name.clone(),
                description: None,
                deprecated: false,
                data: TypeData::StringEnum {
                    values: values.clone(),
                },
            };
            match new_types.entry(name.clone()) {
                btree_map::Entry::Vacant(e) => {
                    e.insert(promoted);
                }
                btree_map::Entry::Occupied(e) => {
                    if *e.get() != promoted {
                        bail!(
                            "promoted inline enum `{name}` has conflicting value sets at different call sites"
                        );
                    }
                }
            }
            *ft = FieldType::SchemaRef { name, inner: None };
        }
        FieldType::List { inner } | FieldType::Set { inner } => {
            promote_field_type(
                Arc::make_mut(inner),
                &format!("{base_name}_item"),
                existing_by_values,
                new_types,
            )?;
        }
        FieldType::Map { value_ty } => {
            promote_field_type(
                Arc::make_mut(value_ty),
                &format!("{base_name}_value"),
                existing_by_values,
                new_types,
            )?;
        }
        FieldType::Bool
        | FieldType::Int16
        | FieldType::UInt16
        | FieldType::Int32
        | FieldType::Int64
        | FieldType::UInt64
        | FieldType::Float
        | FieldType::Double
        | FieldType::String
        | FieldType::Decimal
        | FieldType::DateTime
        | FieldType::Uri
        | FieldType::JsonObject
        | FieldType::SchemaRef { .. }
        | FieldType::StringConst { .. } => {}
    }
    Ok(())
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub(crate) struct Type {
    pub(crate) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    deprecated: bool,
    #[serde(flatten)]
    pub data: TypeData,
}

impl Type {
    pub(crate) fn from_schema(name: String, s: SchemaObject) -> anyhow::Result<Self> {
        let metadata = s.metadata.clone().unwrap_or_default();

        // First check for subschemas (allOf, oneOf) which take precedence over instance_type
        if let Some(ref subschemas) = s.subschemas {
            // Handle allOf - merge all schemas into one struct
            if let Some(ref all_of) = subschemas.all_of {
                let data = TypeData::from_all_of(all_of)?;
                return Ok(Self {
                    name,
                    description: metadata.description,
                    deprecated: metadata.deprecated,
                    data,
                });
            }

            // Handle oneOf with discriminator - create a struct enum
            if let Some(ref one_of) = subschemas.one_of {
                // Check if this is a discriminated union (has discriminator in extensions)
                if let Some(discriminator) = s.extensions.get("discriminator") {
                    let data = TypeData::from_discriminated_oneof(one_of, discriminator)?;
                    return Ok(Self {
                        name,
                        description: metadata.description,
                        deprecated: metadata.deprecated,
                        data,
                    });
                }
            }
        }

        // Handle OpenAPI 3.1 nullable type arrays like ["object", "null"]
        let effective_type = match &s.instance_type {
            Some(SingleOrVec::Vec(types)) => extract_non_null_type(types)?,
            Some(SingleOrVec::Single(t)) => Some(*t.clone()),
            None => None,
        };

        let data = match effective_type {
            Some(InstanceType::Object) => {
                let obj = s.object.unwrap_or_default();
                TypeData::from_object_schema(*obj, s.subschemas)?
            }
            Some(InstanceType::Integer) => {
                let enum_varnames = s
                    .extensions
                    .get("x-enum-varnames")
                    .context("unsupported: integer type without enum varnames")?
                    .as_array()
                    .context("unsupported: integer type enum varnames should be a list")?;
                let values = s
                    .enum_values
                    .context("unsupported: integer type without enum values")?;
                if enum_varnames.len() != values.len() {
                    bail!(
                        "enum varnames length ({}) does not match values length ({})",
                        enum_varnames.len(),
                        values.len()
                    );
                }
                TypeData::from_integer_enum(values, enum_varnames)?
            }
            Some(InstanceType::String) => {
                // String types can be either enums (with enum_values) or simple string types (IDs, etc.)
                if let Some(values) = s.enum_values {
                    TypeData::from_string_enum(values)?
                } else {
                    // Simple string type (like ID types) - these are just String aliases
                    // We'll render them as type aliases: pub type CustomerId = String;
                    TypeData::StringAlias
                }
            }
            Some(it) => bail!("unsupported type {it:?}"),
            None => {
                // No type - check if there's a subschema we didn't handle
                if s.subschemas.is_some() {
                    bail!("unsupported subschema combination");
                }
                bail!("unsupported: no type");
            }
        };

        Ok(Self {
            name,
            description: metadata.description,
            deprecated: metadata.deprecated,
            data,
        })
    }

    pub(crate) fn referenced_components(&self) -> BTreeSet<&str> {
        match &self.data {
            TypeData::Struct { fields } => fields_referenced_schemas(fields),
            TypeData::StringEnum { .. } => BTreeSet::new(),
            TypeData::IntegerEnum { .. } => BTreeSet::new(),
            TypeData::StringAlias => BTreeSet::new(),
            TypeData::StructEnum { repr, fields, .. } => {
                let mut res = repr.referenced_components();
                res.append(&mut fields_referenced_schemas(fields));
                res
            }
        }
    }
}

fn fields_referenced_schemas(fields: &[Field]) -> BTreeSet<&str> {
    fields
        .iter()
        .filter_map(|f| f.r#type.referenced_schema())
        .collect()
}

/// Extract the non-null type from an OpenAPI 3.1 type array like ["string", "null"].
/// Returns None if the array only contains null or is empty.
fn extract_non_null_type(types: &[InstanceType]) -> anyhow::Result<Option<InstanceType>> {
    let non_null_types: Vec<_> = types.iter().filter(|t| **t != InstanceType::Null).collect();

    match non_null_types.len() {
        0 => Ok(None),
        1 => Ok(Some(*non_null_types[0])),
        _ => bail!("unsupported: multiple non-null types in type array: {types:?}"),
    }
}

/// Extract the non-null schema from an OpenAPI 3.1 oneOf nullable pattern.
/// Pattern: oneOf: [{type: null}, {actual schema}]
/// Returns Some((inner_schema, true)) if it's a nullable oneOf, None otherwise.
fn extract_nullable_oneof(one_of: &[Schema]) -> anyhow::Result<Option<(Schema, bool)>> {
    if one_of.len() != 2 {
        // Not a simple nullable pattern, let the caller handle it
        return Ok(None);
    }

    let mut null_count = 0;
    let mut non_null_schema = None;

    for schema in one_of {
        match schema {
            Schema::Object(obj) => {
                // Check if this is a null type
                let is_null = match &obj.instance_type {
                    Some(SingleOrVec::Single(t)) => **t == InstanceType::Null,
                    Some(SingleOrVec::Vec(types)) => {
                        types.len() == 1 && types[0] == InstanceType::Null
                    }
                    None => false,
                };

                if is_null {
                    null_count += 1;
                } else {
                    non_null_schema = Some(schema.clone());
                }
            }
            Schema::Bool(_) => {
                // Not a nullable pattern we recognize
                return Ok(None);
            }
        }
    }

    #[allow(clippy::unnecessary_unwrap)]
    if null_count == 1 && non_null_schema.is_some() {
        Ok(Some((non_null_schema.unwrap(), true)))
    } else {
        Ok(None)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum TypeData {
    Struct {
        fields: Vec<Field>,
    },
    StringEnum {
        values: Vec<String>,
    },
    IntegerEnum {
        variants: Vec<(String, i64)>,
    },
    /// A type alias to String (for ID types like CustomerId, InvoiceId, etc.)
    StringAlias,
    StructEnum {
        /// Name of the field that identifies the variant.
        discriminator_field: String,

        /// JSON representation of the enum variants.
        #[serde(flatten)]
        repr: StructEnumRepr,

        /// Variant-independent fields.
        fields: Vec<Field>,
    },
}

impl TypeData {
    pub(super) fn from_object_schema(
        obj: ObjectValidation,
        subschemas: Option<Box<SubschemaValidation>>,
    ) -> anyhow::Result<Self> {
        ensure!(
            obj.additional_properties.is_none(),
            "additionalProperties not yet supported"
        );
        ensure!(obj.max_properties.is_none(), "unsupported: maxProperties");
        ensure!(obj.min_properties.is_none(), "unsupported: minProperties");
        ensure!(
            obj.pattern_properties.is_empty(),
            "unsupported: patternProperties"
        );
        // Note: property_names is a JSON Schema validation constraint that limits
        // property names to a certain format. For code generation, we can safely
        // ignore it since all JSON object keys are strings anyway.
        // ensure!(obj.property_names.is_none(), "unsupported: propertyNames");

        let fields: Vec<_> = obj
            .properties
            .into_iter()
            .map(|(name, schema)| {
                Field::from_schema(name.clone(), schema, obj.required.contains(&name))
                    .with_context(|| format!("unsupported field `{name}`"))
            })
            .collect::<anyhow::Result<_>>()?;

        if let Some(sub) = subschemas {
            ensure!(sub.all_of.is_none(), "unsupported: allOf subschema");
            ensure!(sub.any_of.is_none(), "unsupported: anyOf subschema");
            ensure!(sub.not.is_none(), "unsupported: not subschema");
            ensure!(sub.if_schema.is_none(), "unsupported: if subschema");
            ensure!(sub.then_schema.is_none(), "unsupported: then subschema");
            ensure!(sub.else_schema.is_none(), "unsupported: else subschema");

            if let Some(one_of) = sub.one_of {
                return Self::inline_struct_enum(&one_of, &fields);
            }
        }

        Ok(Self::Struct { fields })
    }

    /// Parse an allOf schema - merges all schemas into a single struct
    fn from_all_of(all_of: &[Schema]) -> anyhow::Result<Self> {
        let mut all_fields = Vec::new();

        for schema in all_of {
            match schema {
                Schema::Object(obj) => {
                    // If it's a $ref, we'll get the fields from the referenced schema later
                    if let Some(ref_name) = &obj.reference {
                        // For now, we add a field that references this schema
                        // The actual merging would require resolving the $ref
                        let schema_name =
                            get_schema_name(Some(ref_name)).context("invalid $ref in allOf")?;
                        // We'll flatten this reference - add a special marker field
                        all_fields.push(Field {
                            name: format!("__flatten_{}", schema_name.to_lowercase()),
                            r#type: FieldType::SchemaRef {
                                name: schema_name,
                                inner: None,
                            },
                            default: None,
                            description: None,
                            required: true,
                            nullable: false,
                            deprecated: false,
                            example: None,
                            flatten: true,
                        });
                    } else if let Some(ref obj_validation) = obj.object {
                        // Inline object schema - extract fields directly
                        for (name, prop_schema) in &obj_validation.properties {
                            let field = Field::from_schema(
                                name.clone(),
                                prop_schema.clone(),
                                obj_validation.required.contains(name),
                            )
                            .with_context(|| format!("unsupported field `{name}` in allOf"))?;
                            all_fields.push(field);
                        }
                    }
                }
                Schema::Bool(_) => bail!("unsupported bool schema in allOf"),
            }
        }

        Ok(Self::Struct { fields: all_fields })
    }

    /// Parse a oneOf schema with a discriminator - creates a struct enum
    fn from_discriminated_oneof(
        one_of: &[Schema],
        discriminator: &serde_json::Value,
    ) -> anyhow::Result<Self> {
        let discriminator_obj = discriminator
            .as_object()
            .context("discriminator should be an object")?;

        let property_name = discriminator_obj
            .get("propertyName")
            .and_then(|v| v.as_str())
            .context("discriminator.propertyName is required")?;

        let mapping = discriminator_obj.get("mapping").and_then(|v| v.as_object());

        let mut variants = Vec::new();

        for schema in one_of {
            match schema {
                Schema::Object(obj) => {
                    if let Some(ref_str) = &obj.reference {
                        let schema_name =
                            get_schema_name(Some(ref_str)).context("invalid $ref in oneOf")?;

                        // Find the discriminator value from the mapping
                        let discriminator_value = if let Some(map) = mapping {
                            map.iter()
                                .find(|(_, v)| v.as_str() == Some(ref_str))
                                .map(|(k, _)| k.clone())
                                .unwrap_or_else(|| schema_name.clone())
                        } else {
                            schema_name.clone()
                        };

                        variants.push(SimpleVariant {
                            name: discriminator_value,
                            content: EnumVariantType::Ref {
                                schema_ref: Some(schema_name),
                                inner: None,
                            },
                        });
                    } else if let Some(ref obj_validation) = obj.object {
                        // Inline schema - try to extract the discriminator value from properties
                        let discriminator_value = obj_validation
                            .properties
                            .get(property_name)
                            .and_then(|s| {
                                if let Schema::Object(disc_obj) = s {
                                    disc_obj
                                        .enum_values
                                        .as_ref()
                                        .and_then(|vals| vals.first())
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string())
                                } else {
                                    None
                                }
                            })
                            .context("inline schema in oneOf must have discriminator enum value")?;

                        // Parse fields from the inline schema (excluding the discriminator)
                        let fields: Vec<_> = obj_validation
                            .properties
                            .iter()
                            .filter(|(name, _)| *name != property_name)
                            .map(|(name, prop_schema)| {
                                Field::from_schema(
                                    name.clone(),
                                    prop_schema.clone(),
                                    obj_validation.required.contains(name),
                                )
                                .with_context(|| {
                                    format!("unsupported field `{name}` in inline oneOf schema")
                                })
                            })
                            .collect::<anyhow::Result<_>>()?;

                        variants.push(SimpleVariant {
                            name: discriminator_value,
                            content: EnumVariantType::Struct { fields },
                        });
                    } else {
                        bail!("oneOf variant must have either $ref or object properties");
                    }
                }
                Schema::Bool(_) => bail!("unsupported bool schema in oneOf"),
            }
        }

        Ok(Self::StructEnum {
            discriminator_field: property_name.to_string(),
            repr: StructEnumRepr::InternallyTagged { variants },
            fields: vec![],
        })
    }

    fn from_string_enum(values: Vec<serde_json::Value>) -> anyhow::Result<TypeData> {
        Ok(Self::StringEnum {
            values: values
                .into_iter()
                .enumerate()
                .map(|(i, v)| match v {
                    serde_json::Value::String(s) => Ok(s),
                    _ => bail!("enum value {} is not a string", i + 1),
                })
                .collect::<anyhow::Result<_>>()?,
        })
    }

    fn from_integer_enum(
        values: Vec<serde_json::Value>,
        enum_varnames: &[serde_json::Value],
    ) -> anyhow::Result<TypeData> {
        Ok(Self::IntegerEnum {
            variants: values
                .into_iter()
                .enumerate()
                .map(|(i, v)| match v {
                    serde_json::Value::Number(s) => {
                        let num = s
                            .as_i64()
                            .with_context(|| format!("enum value {s} is not an integer"))?;
                        Ok((
                            enum_varnames[i]
                                .as_str()
                                .with_context(|| {
                                    format!("enum varname {} is not a string", enum_varnames[i])
                                })?
                                .to_string(),
                            num,
                        ))
                    }
                    _ => bail!("enum value {} is not a number", i + 1),
                })
                .collect::<anyhow::Result<_>>()?,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "repr", rename_all = "snake_case")]
pub(crate) enum StructEnumRepr {
    // add more variants here to support other enum representations
    AdjacentlyTagged {
        /// Name of the field that contains the variant-specific fields.
        content_field: String,

        /// Enum variants.
        ///
        /// Every variant has a discriminator value that's stored in the discriminator field to
        /// identify the variant.
        variants: Vec<SimpleVariant>,
    },
    /// Internally tagged enum - the discriminator is a field inside each variant
    /// Used for OpenAPI discriminated oneOf patterns like Fee with fee_type discriminator
    InternallyTagged {
        /// Enum variants.
        variants: Vec<SimpleVariant>,
    },
}

impl StructEnumRepr {
    fn referenced_components(&self) -> BTreeSet<&str> {
        let variants = match self {
            StructEnumRepr::AdjacentlyTagged { variants, .. } => variants,
            StructEnumRepr::InternallyTagged { variants } => variants,
        };
        variants
            .iter()
            .filter_map(|v| match &v.content {
                EnumVariantType::Struct { fields } => {
                    fields.iter().find_map(|f| f.r#type.referenced_schema())
                }
                EnumVariantType::Ref { schema_ref, .. } => schema_ref.as_deref(),
            })
            .collect()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub(crate) struct Field {
    pub(crate) name: String,
    #[serde(serialize_with = "serialize_field_type")]
    pub r#type: FieldType,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    required: bool,
    nullable: bool,
    deprecated: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    example: Option<serde_json::Value>,
    /// If true, this field should be flattened (used for allOf merging)
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    flatten: bool,
}

impl Field {
    fn from_schema(name: String, s: Schema, required: bool) -> anyhow::Result<Self> {
        let obj = match s {
            Schema::Bool(_) => bail!("unsupported bool schema"),
            Schema::Object(o) => o,
        };
        let example = obj.extensions.get("example").cloned();
        let metadata = obj.metadata.clone().unwrap_or_default();

        // Check for OpenAPI 3.0 style nullable extension
        let mut nullable = obj
            .extensions
            .get("nullable")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        // Handle OpenAPI 3.1 oneOf nullable pattern: oneOf: [{type: null}, {actual type}]
        let (field_type, is_oneof_nullable) = FieldType::from_schema_object_with_nullable(obj)?;
        nullable = nullable || is_oneof_nullable;

        Ok(Self {
            name,
            r#type: field_type,
            default: metadata.default,
            description: metadata.description,
            required,
            nullable,
            deprecated: metadata.deprecated,
            example,
            flatten: false,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "camelCase")]
pub(crate) enum EnumVariantType {
    Struct {
        fields: Vec<Field>,
    },
    Ref {
        #[serde(skip_serializing_if = "Option::is_none")]
        schema_ref: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        inner: Option<Type>,
    },
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub(crate) struct SimpleVariant {
    /// Discriminator value that identifies this variant.
    pub name: String,
    #[serde(flatten)]
    pub content: EnumVariantType,
}

/// Supported field type.
///
/// Equivalent to openapi's `type` + `format` + `$ref`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "id")]
pub(crate) enum FieldType {
    Bool,
    Int16,
    UInt16,
    Int32,
    Int64,
    UInt64,
    Float,
    Double,
    String,
    Decimal,
    DateTime,
    Uri,
    /// A JSON object with arbitrary field values.
    JsonObject,
    /// A regular old list.
    List {
        inner: Arc<FieldType>,
    },
    /// List with unique items.
    Set {
        inner: Arc<FieldType>,
    },
    /// A map with a given value type.
    ///
    /// The key type is always `String` in JSON schemas.
    Map {
        value_ty: Arc<FieldType>,
    },
    /// The name of another schema that defines this type.
    SchemaRef {
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        inner: Option<Type>,
    },

    /// A string constant, used as an enum discriminator value.
    StringConst {
        value: String,
    },

    /// An inline string enum that will be promoted to a named top-level type
    /// by `promote_inline_enums`. Should never reach the render stage.
    StringEnum {
        values: Vec<String>,
        /// Title from the OpenAPI schema, used as the promoted type name when set.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },
}

impl FieldType {
    pub(crate) fn from_openapi(format: openapi::ParameterSchemaOrContent) -> anyhow::Result<Self> {
        let openapi::ParameterSchemaOrContent::Schema(s) = format else {
            bail!("found unexpected 'content' data format");
        };
        Self::from_schema(s.json_schema)
    }

    fn from_schema(s: Schema) -> anyhow::Result<Self> {
        let Schema::Object(obj) = s else {
            bail!("found unexpected `true` schema");
        };

        Self::from_schema_object(obj)
    }

    fn from_schema_object(obj: SchemaObject) -> anyhow::Result<Self> {
        let (field_type, _nullable) = Self::from_schema_object_with_nullable(obj)?;
        Ok(field_type)
    }

    /// Parse a schema object, returning the field type and whether it's nullable.
    /// Handles OpenAPI 3.1 patterns like type arrays and oneOf with null.
    fn from_schema_object_with_nullable(obj: SchemaObject) -> anyhow::Result<(Self, bool)> {
        // Check for OpenAPI 3.1 oneOf nullable pattern: oneOf: [{type: null}, {$ref or type}]
        if let Some(ref subschemas) = obj.subschemas
            && let Some(ref one_of) = subschemas.one_of
            && let Some((inner_schema, is_nullable)) = extract_nullable_oneof(one_of)?
        {
            let field_type = Self::from_schema(inner_schema)?;
            return Ok((field_type, is_nullable));
        }

        // Handle OpenAPI 3.1 type arrays like ["string", "null"]
        let (effective_type, is_type_array_nullable) = match &obj.instance_type {
            Some(SingleOrVec::Vec(types)) => {
                let has_null = types.contains(&InstanceType::Null);
                let non_null = extract_non_null_type(types)?;
                (non_null, has_null)
            }
            Some(SingleOrVec::Single(t)) => (Some(*t.clone()), false),
            None => (None, false),
        };

        let result = match effective_type {
            Some(InstanceType::Boolean) => Self::Bool,
            Some(InstanceType::Integer) => match obj.format.as_deref() {
                Some("int16") => Self::Int16,
                Some("uint16") => Self::UInt16,
                Some("int32") => Self::Int32,
                // FIXME: Why do we have int in the spec?
                Some("int" | "int64") => Self::Int64,
                // FIXME: Get rid of uint in the spec..
                Some("uint" | "uint64") => Self::UInt64,
                None => Self::Int64, // Default to i64 for integers without format
                f => bail!("unsupported integer format: `{f:?}`"),
            },
            Some(InstanceType::Number) => match obj.format.as_deref() {
                Some("float") => Self::Float,
                // A number without a format is unbounded in JSON Schema, so pick the wider type.
                Some("double") | None => Self::Double,
                f => bail!("unsupported number format: `{f:?}`"),
            },
            Some(InstanceType::String) => {
                // String consts are the only const / enum values we support, for now.
                // Early return so we don't hit the checks for these two below.
                if let Some(value) = obj.const_value {
                    let serde_json::Value::String(value) = value else {
                        bail!("unsupported: non-string constant as field type");
                    };
                    return Ok((Self::StringConst { value }, is_type_array_nullable));
                }
                if let Some(values) = obj.enum_values {
                    // Single-value enum: treat as a string constant (discriminator)
                    if values.len() == 1 {
                        let serde_json::Value::String(value) = values.into_iter().next().unwrap()
                        else {
                            bail!("unsupported: non-string constant as field type");
                        };
                        return Ok((Self::StringConst { value }, is_type_array_nullable));
                    }
                    // Multi-value enum: inline string enum. Promoted to a named top-level
                    // type by `promote_inline_enums` after parsing.
                    let string_values: Vec<String> = values
                        .into_iter()
                        .map(|v| match v {
                            serde_json::Value::String(s) => Ok(s),
                            _ => bail!("unsupported: non-string value in enum"),
                        })
                        .collect::<anyhow::Result<_>>()?;
                    let title = obj.metadata.as_ref().and_then(|m| m.title.clone());
                    return Ok((
                        Self::StringEnum {
                            values: string_values,
                            title,
                        },
                        is_type_array_nullable,
                    ));
                }

                match obj.format.as_deref() {
                    None | Some("color") | Some("email") => Self::String,
                    Some("decimal") => Self::Decimal,
                    Some("date-time") => Self::DateTime,
                    Some("date") => Self::String, // Date without time, treat as string for now
                    Some("uri") => Self::Uri,
                    Some(f) => {
                        // Unknown formats - treat as string with a warning
                        tracing::warn!(format = f, "treating unknown string format as String");
                        Self::String
                    }
                }
            }
            Some(InstanceType::Array) => {
                let array = obj.array.context("array type must have array props")?;
                ensure!(array.additional_items.is_none(), "not supported");
                let inner = match array.items.context("array type must have items prop")? {
                    SingleOrVec::Single(ty) => ty,
                    SingleOrVec::Vec(types) => {
                        bail!("unsupported multi-typed array parameter: `{types:?}`")
                    }
                };
                let inner = Arc::new(Self::from_schema(*inner)?);
                if array.unique_items == Some(true) {
                    Self::Set { inner }
                } else {
                    Self::List { inner }
                }
            }
            Some(InstanceType::Object) => {
                let obj = obj
                    .object
                    .context("unsupported: object type without further validation")?;
                let additional_properties = obj
                    .additional_properties
                    .context("unsupported: object field type without additional_properties")?;

                ensure!(obj.max_properties.is_none(), "unsupported: max_properties");
                ensure!(obj.min_properties.is_none(), "unsupported: min_properties");
                ensure!(
                    obj.properties.is_empty(),
                    "unsupported: properties on field type"
                );
                ensure!(
                    obj.pattern_properties.is_empty(),
                    "unsupported: pattern_properties"
                );
                // Note: property_names is a JSON Schema validation constraint that limits
                // property names. For code generation, we can safely ignore it.
                // ensure!(obj.property_names.is_none(), "unsupported: property_names");
                ensure!(
                    obj.required.is_empty(),
                    "unsupported: required on field type"
                );

                match *additional_properties {
                    Schema::Bool(true) => Self::JsonObject,
                    Schema::Bool(false) => bail!("unsupported `additional_properties: false`"),
                    Schema::Object(schema_object) => {
                        let value_ty = Arc::new(Self::from_schema_object(schema_object)?);
                        Self::Map { value_ty }
                    }
                }
            }
            Some(ty) => bail!("unsupported type: `{ty:?}`"),
            None => match get_schema_name(obj.reference.as_deref()) {
                Some(name) => Self::SchemaRef { name, inner: None },
                // Empty schema {} means "any JSON" - treat as JsonObject
                None => Self::JsonObject,
            },
        };

        // If we didn't hit the early return above, check that there's no const or enum value(s).
        ensure!(obj.const_value.is_none(), "unsupported const_value");
        ensure!(obj.enum_values.is_none(), "unsupported enum_values");

        Ok((result, is_type_array_nullable))
    }

    fn to_csharp_typename(&self) -> Cow<'_, str> {
        match self {
            Self::Bool => "bool".into(),
            Self::Int16 => "short".into(),
            Self::Int32 => "int".into(),
            Self::Int64 => "long".into(),
            Self::UInt16 => "ushort".into(),
            Self::UInt64 => "ulong".into(),
            Self::Float => "float".into(),
            Self::Double => "double".into(),
            Self::String => "string".into(),
            Self::Decimal => "decimal".into(),
            Self::DateTime => "DateTime".into(),
            Self::Uri => "string".into(),
            Self::JsonObject => "Object".into(),
            Self::Map { value_ty } => {
                format!("Dictionary<string, {}>", value_ty.to_csharp_typename()).into()
            }
            Self::List { inner } | Self::Set { inner } => {
                format!("List<{}>", inner.to_csharp_typename()).into()
            }
            Self::SchemaRef { name, .. } => filter_schema_ref(name, "Object"),
            Self::StringConst { .. } => "string".into(),
            Self::StringEnum { .. } => unreachable_inline_enum(),
        }
    }

    fn to_go_typename(&self) -> Cow<'_, str> {
        match self {
            Self::Bool => "bool".into(),
            Self::Int16 => "int16".into(),
            Self::Int32 => "int32".into(),
            Self::Int64 => "int64".into(),
            Self::UInt16 => "uint16".into(),
            Self::UInt64 => "uint64".into(),
            Self::Float => "float32".into(),
            Self::Double => "float64".into(),
            Self::Uri | Self::String | Self::Decimal => "string".into(),
            Self::DateTime => "time.Time".into(),
            Self::JsonObject => "map[string]any".into(),
            Self::Map { value_ty } => format!("map[string]{}", value_ty.to_go_typename()).into(),
            Self::List { inner } | Self::Set { inner } => {
                format!("[]{}", inner.to_go_typename()).into()
            }
            Self::SchemaRef { name, .. } => filter_schema_ref(name, "map[string]any"),
            Self::StringConst { .. } => "string".into(),
            Self::StringEnum { .. } => unreachable_inline_enum(),
        }
    }

    fn to_kotlin_typename(&self) -> Cow<'_, str> {
        match self {
            Self::Bool => "Boolean".into(),
            Self::Int16 => "Short".into(),
            Self::Int32 => "Int".into(),
            Self::UInt16 => "UShort".into(),
            Self::Int64 => "Long".into(),
            Self::UInt64 => "ULong".into(),
            Self::Float => "Float".into(),
            Self::Double => "Double".into(),
            Self::Uri | Self::String => "String".into(),
            Self::Decimal => "java.math.BigDecimal".into(),
            Self::DateTime => "Instant".into(),
            Self::Map { value_ty } => {
                format!("Map<String,{}>", value_ty.to_kotlin_typename()).into()
            }
            Self::JsonObject => "Map<String,Any>".into(),
            Self::List { inner } => format!("List<{}>", inner.to_kotlin_typename()).into(),
            Self::Set { inner } => format!("Set<{}>", inner.to_kotlin_typename()).into(),
            Self::SchemaRef { name, .. } => filter_schema_ref(name, "Map<String,Any>"),
            Self::StringConst { .. } => "String".into(),
            Self::StringEnum { .. } => unreachable_inline_enum(),
        }
    }

    fn to_js_typename(&self) -> Cow<'_, str> {
        match self {
            Self::Bool => "boolean".into(),
            Self::Int16
            | Self::UInt16
            | Self::Int32
            | Self::Int64
            | Self::UInt64
            | Self::Float
            | Self::Double
            | Self::Decimal => "number".into(),
            Self::String | Self::Uri => "string".into(),
            Self::DateTime => "Date".into(),
            Self::JsonObject => "any".into(),
            Self::List { inner } | Self::Set { inner } => {
                format!("{}[]", inner.to_js_typename()).into()
            }
            Self::Map { value_ty } => {
                format!("{{ [key: string]: {} }}", value_ty.to_js_typename()).into()
            }
            Self::SchemaRef { name, .. } => filter_schema_ref(name, "any"),
            Self::StringConst { .. } => "string".into(),
            Self::StringEnum { .. } => unreachable_inline_enum(),
        }
    }

    fn to_rust_typename(&self) -> Cow<'_, str> {
        match self {
            Self::Bool => "bool".into(),
            Self::Int16 => "i16".into(),
            Self::UInt16 => "u16".into(),
            Self::Int32 |
            // FIXME: All integers in query params are currently i32
            Self::Int64 | Self::UInt64 => "i32".into(),
            Self::Float => "f32".into(),
            Self::Double => "f64".into(),
            // FIXME: Do we want a separate type for Uri?
            Self::Uri | Self::String => "String".into(),
            Self::Decimal => "rust_decimal::Decimal".into(),
            // FIXME: Depends on those chrono imports being in scope, not that great..
            Self::DateTime => "DateTime<Utc>".into(),
            Self::JsonObject => "serde_json::Value".into(),
            // FIXME: Treat set differently? (BTreeSet)
            Self::List { inner } | Self::Set { inner } => {
                format!("Vec<{}>", inner.to_rust_typename()).into()
            }
            Self::Map { value_ty } => format!(
                "std::collections::HashMap<String, {}>",
                value_ty.to_rust_typename(),
            )
            .into(),
            Self::SchemaRef { name,.. } => filter_schema_ref(name, "serde_json::Value"),
            Self::StringConst { .. } => "String".into(),
            Self::StringEnum { .. } => unreachable_inline_enum(),
        }
    }

    pub(crate) fn referenced_schema(&self) -> Option<&str> {
        match self {
            Self::SchemaRef { name, .. } => {
                // Workaround: the `BackgroundTaskFinishedEvent2` struct has a field with type of `Data`
                // which corresponds to an untagged enum. We skip this reference for now.
                if name == "Data" { None } else { Some(name) }
            }
            Self::List { inner: ty } | Self::Set { inner: ty } | Self::Map { value_ty: ty } => {
                ty.referenced_schema()
            }
            _ => None,
        }
    }

    fn to_python_typename(&self) -> Cow<'_, str> {
        match self {
            Self::Bool => "bool".into(),
            Self::Int16 | Self::UInt16 | Self::Int32 | Self::Int64 | Self::UInt64 => "int".into(),
            Self::Float | Self::Double => "float".into(),
            Self::String => "str".into(),
            Self::Decimal => "Decimal".into(),
            Self::DateTime => "datetime".into(),
            Self::SchemaRef { name, .. } => filter_schema_ref(name, "t.Dict[str, t.Any]"),
            Self::Uri => "str".into(),
            Self::JsonObject => "t.Dict[str, t.Any]".into(),
            Self::Set { inner } | Self::List { inner } => {
                format!("t.List[{}]", inner.to_python_typename()).into()
            }
            Self::Map { value_ty } => {
                format!("t.Dict[str, {}]", value_ty.to_python_typename()).into()
            }
            Self::StringConst { .. } => "str".into(),
            Self::StringEnum { .. } => unreachable_inline_enum(),
        }
    }

    fn to_java_typename(&self) -> Cow<'_, str> {
        match self {
            // _ => "String".into(),
            FieldType::Bool => "Boolean".into(),
            FieldType::Int16 => "Short".into(),
            FieldType::UInt16 | FieldType::UInt64 | FieldType::Int64 => "Long".into(),
            FieldType::Int32 => "Integer".into(),
            FieldType::Float => "Float".into(),
            FieldType::Double => "Double".into(),
            FieldType::String => "String".into(),
            FieldType::Decimal => "BigDecimal".into(),
            FieldType::DateTime => "OffsetDateTime".into(),
            FieldType::Uri => "URI".into(),
            FieldType::JsonObject => "Object".into(),
            FieldType::List { inner } => format!("List<{}>", inner.to_java_typename()).into(),
            FieldType::Set { inner: field_type } => {
                format!("Set<{}>", field_type.to_java_typename()).into()
            }
            FieldType::Map { value_ty } => {
                format!("Map<String,{}>", value_ty.to_java_typename()).into()
            }
            FieldType::SchemaRef { name, inner } => {
                // Java doesn't have type aliases, so resolve string alias refs to String
                if let Some(ty) = inner
                    && matches!(ty.data, TypeData::StringAlias)
                {
                    return "String".into();
                }
                filter_schema_ref(name, "Object")
            }
            // backwards compat
            FieldType::StringConst { .. } => "TypeEnum".into(),
            FieldType::StringEnum { .. } => unreachable_inline_enum(),
        }
    }

    /// Check if this field type needs an import statement in Java.
    /// Returns false for primitives, built-in types, and string aliases.
    fn needs_java_import(&self) -> bool {
        match self {
            FieldType::Bool
            | FieldType::Int16
            | FieldType::UInt16
            | FieldType::Int32
            | FieldType::Int64
            | FieldType::UInt64
            | FieldType::Float
            | FieldType::Double
            | FieldType::String
            | FieldType::Decimal
            | FieldType::DateTime
            | FieldType::Uri
            | FieldType::JsonObject
            | FieldType::StringConst { .. } => false,
            FieldType::StringEnum { .. } => unreachable_inline_enum(),
            FieldType::List { inner } | FieldType::Set { inner } => inner.needs_java_import(),
            FieldType::Map { value_ty } => value_ty.needs_java_import(),
            FieldType::SchemaRef { inner, .. } => {
                // String aliases don't need import - they resolve to String
                if let Some(ty) = inner {
                    !matches!(ty.data, TypeData::StringAlias)
                } else {
                    true
                }
            }
        }
    }

    fn to_ruby_typename(&self) -> Cow<'_, str> {
        match self {
            FieldType::SchemaRef { name, .. } => name.clone().into(),
            FieldType::StringConst { .. } => {
                unreachable!("FieldType::const should never be exposed to template code")
            }
            _ => panic!("types? in ruby?!?!, not on my watch!"),
        }
    }

    /// returns `PHPDoc` annotations
    fn to_phpdoc_typename(&self) -> Cow<'_, str> {
        match self {
            FieldType::Bool
            | FieldType::Int16
            | FieldType::UInt16
            | FieldType::Int32
            | FieldType::Int64
            | FieldType::UInt64
            | FieldType::Float
            | FieldType::Double
            | FieldType::String
            | FieldType::Decimal
            | FieldType::DateTime
            | FieldType::Uri
            | FieldType::JsonObject
            | FieldType::StringConst { .. }
            | FieldType::SchemaRef { .. } => self.to_php_typename(),
            FieldType::StringEnum { .. } => unreachable_inline_enum(),
            FieldType::Set { inner } | FieldType::List { inner } => {
                format!("list<{}>", inner.to_phpdoc_typename()).into()
            }
            FieldType::Map { value_ty } => {
                format!("array<string, {}>", value_ty.to_phpdoc_typename()).into()
            }
        }
    }

    fn to_php_typename(&self) -> Cow<'_, str> {
        match self {
            FieldType::Bool => "bool".into(),
            FieldType::UInt16
            | FieldType::Int16
            | FieldType::UInt64
            | FieldType::Int32
            | FieldType::Int64 => "int".into(),
            FieldType::Float | FieldType::Double | FieldType::Decimal => "float".into(),
            FieldType::Uri | FieldType::StringConst { .. } | FieldType::String => "string".into(),
            FieldType::StringEnum { .. } => unreachable_inline_enum(),
            FieldType::DateTime => r#"\DateTimeImmutable"#.into(),

            FieldType::JsonObject
            | FieldType::List { .. }
            | FieldType::Set { .. }
            | FieldType::Map { .. } => "array".into(),
            FieldType::SchemaRef { name, .. } => name.clone().into(),
        }
    }
}

impl minijinja::value::Object for FieldType {
    fn repr(self: &Arc<Self>) -> minijinja::value::ObjectRepr {
        minijinja::value::ObjectRepr::Plain
    }

    fn call_method(
        self: &Arc<Self>,
        _state: &minijinja::State<'_, '_>,
        method: &str,
        args: &[minijinja::Value],
    ) -> Result<minijinja::Value, minijinja::Error> {
        match method {
            "to_python" => {
                ensure_no_args(args, "to_python")?;
                Ok(self.to_python_typename().into())
            }
            "to_csharp" => {
                ensure_no_args(args, "to_csharp")?;
                Ok(self.to_csharp_typename().into())
            }
            "to_go" => {
                ensure_no_args(args, "to_go")?;
                Ok(self.to_go_typename().into())
            }
            "to_js" => {
                ensure_no_args(args, "to_js")?;
                Ok(self.to_js_typename().into())
            }
            "to_kotlin" => {
                ensure_no_args(args, "to_kotlin")?;
                Ok(self.to_kotlin_typename().into())
            }
            "to_rust" => {
                ensure_no_args(args, "to_rust")?;
                Ok(self.to_rust_typename().into())
            }
            "to_java" => {
                ensure_no_args(args, "to_java")?;
                Ok(self.to_java_typename().into())
            }
            "needs_java_import" => {
                ensure_no_args(args, "needs_java_import")?;
                Ok(self.needs_java_import().into())
            }
            "to_ruby" => {
                ensure_no_args(args, "to_ruby")?;
                Ok(self.to_ruby_typename().into())
            }
            "to_php" => {
                ensure_no_args(args, "to_php")?;
                Ok(self.to_php_typename().into())
            }
            "to_phpdoc" => {
                ensure_no_args(args, "to_phpdoc")?;
                Ok(self.to_phpdoc_typename().into())
            }

            "is_datetime" => {
                ensure_no_args(args, "is_datetime")?;
                Ok(matches!(**self, Self::DateTime).into())
            }
            "is_schema_ref" => {
                ensure_no_args(args, "is_schema_ref")?;
                Ok(matches!(**self, Self::SchemaRef { .. }).into())
            }
            "is_list" => {
                ensure_no_args(args, "is_list")?;
                Ok(matches!(**self, Self::List { .. }).into())
            }
            "is_set" => {
                ensure_no_args(args, "is_set")?;
                Ok(matches!(**self, Self::Set { .. }).into())
            }
            "is_map" => {
                ensure_no_args(args, "is_map")?;
                Ok(matches!(**self, Self::Map { .. }).into())
            }
            "is_string" => {
                ensure_no_args(args, "is_string")?;
                Ok(matches!(**self, Self::String).into())
            }
            "is_uri" => {
                ensure_no_args(args, "is_uri")?;
                Ok(matches!(**self, Self::Uri).into())
            }
            "is_bool" => {
                ensure_no_args(args, "is_bool")?;
                Ok(matches!(**self, Self::Bool).into())
            }
            "is_int_or_uint" => {
                ensure_no_args(args, "is_int_or_uint")?;
                use FieldType as F;
                let is_int_or_uint = match &**self {
                    F::Int16 | F::UInt16 | F::Int32 | F::Int64 | F::UInt64 => true,
                    F::Bool
                    | F::Float
                    | F::Double
                    | F::String
                    | F::Decimal
                    | F::DateTime
                    | F::Uri
                    | F::JsonObject
                    | F::List { .. }
                    | F::Set { .. }
                    | F::Map { .. }
                    | F::SchemaRef { .. }
                    | F::StringConst { .. } => false,
                    F::StringEnum { .. } => unreachable_inline_enum(),
                };
                Ok(is_int_or_uint.into())
            }
            "is_json_object" => {
                ensure_no_args(args, "is_json_object")?;
                Ok(matches!(**self, Self::JsonObject).into())
            }
            "is_string_const" => {
                ensure_no_args(args, "is_string_const")?;
                Ok(matches!(**self, Self::StringConst { .. }).into())
            }

            // Returns the inner type of a list or set
            "inner_type" => {
                ensure_no_args(args, "inner_type")?;

                let ty = match &**self {
                    FieldType::List { inner } | FieldType::Set { inner } => {
                        Some(minijinja::Value::from_dyn_object(inner.clone()))
                    }
                    _ => None,
                };
                Ok(ty.into())
            }
            "inner_schema_ref_ty" => {
                ensure_no_args(args, "inner_schema_ref_ty")?;
                let ty = match &**self {
                    FieldType::SchemaRef { inner, .. } => {
                        let i = inner.as_ref().unwrap().clone();
                        Some(minijinja::Value::from_serialize(i))
                    }
                    _ => None,
                };
                Ok(ty.into())
            }
            // Returns the value type of a map
            "value_type" => {
                ensure_no_args(args, "value_type")?;

                let ty = match &**self {
                    FieldType::Map { value_ty } => {
                        Some(minijinja::Value::from_dyn_object(value_ty.clone()))
                    }
                    _ => None,
                };
                Ok(ty.into())
            }
            "string_const_val" => {
                ensure_no_args(args, "string_const_val")?;
                let val = match &**self {
                    Self::StringConst { value } => {
                        Some(minijinja::Value::from_safe_string(value.clone()))
                    }
                    _ => None,
                };
                Ok(val.into())
            }
            _ => Err(minijinja::Error::from(minijinja::ErrorKind::UnknownMethod)),
        }
    }
}

fn ensure_no_args(args: &[minijinja::Value], method_name: &str) -> Result<(), minijinja::Error> {
    if !args.is_empty() {
        return Err(minijinja::Error::new(
            minijinja::ErrorKind::TooManyArguments,
            format!("{method_name} does not take any arguments"),
        ));
    }
    Ok(())
}

/// Serialize a `FieldType`, as an object for minijinja, or
pub(super) fn serialize_field_type<S>(
    field_ty: &FieldType,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if minijinja::value::serializing_for_value() {
        minijinja::Value::from_object(field_ty.clone()).serialize(serializer)
    } else {
        field_ty.serialize(serializer)
    }
}

fn filter_schema_ref<'a>(name: &'a String, json_obj_typename: &'a str) -> Cow<'a, str> {
    // Workaround: the `BackgroundTaskFinishedEvent2` struct has a field with type of `Data`
    // which corresponds to an untagged enum. We use json_obj_typename as fallback.
    if name == "Data" {
        json_obj_typename.into()
    } else {
        name.clone().into()
    }
}

#[cold]
#[inline(never)]
fn unreachable_inline_enum() -> ! {
    panic!("FieldType::StringEnum must be promoted by promote_inline_enums before rendering")
}
