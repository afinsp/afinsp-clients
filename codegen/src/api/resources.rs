use std::collections::{BTreeMap, BTreeSet};

use aide::openapi::{self, ReferenceOr};
use anyhow::{Context as _, bail};
use indexmap::IndexMap;
use schemars::schema::{InstanceType, Schema};
use serde::{Deserialize, Serialize};

use crate::cli_v1::IncludeMode;

use super::{
    get_schema_name,
    types::{FieldType, resolve_schema_ref_in_field_type_public, serialize_field_type},
};

/// The API operations of the API client we generate.
///
/// Intermediate representation of `paths` from the spec.
pub(crate) type Resources = BTreeMap<String, Resource>;

pub(crate) fn from_openapi(
    paths: openapi::Paths,
    component_schemas: &IndexMap<String, openapi::SchemaObject>,
    include_mode: IncludeMode,
    excluded_operations: &BTreeSet<String>,
    specified_operations: &BTreeSet<String>,
) -> anyhow::Result<Resources> {
    let mut resources = BTreeMap::new();

    for (path, pi) in paths {
        let path_item = pi
            .into_item()
            .context("$ref paths are currently not supported")?;

        if !path_item.parameters.is_empty() {
            tracing::info!("parameters at the path item level are not currently supported");
            continue;
        }

        for (method, op) in path_item {
            if let Some((res_path, op)) = Operation::from_openapi(
                &path,
                method,
                op,
                component_schemas,
                include_mode,
                excluded_operations,
                specified_operations,
            ) {
                let resource = get_or_insert_resource(&mut resources, res_path);
                resource.operations.push(op);
            }
        }
    }

    Ok(resources)
}

pub(crate) fn referenced_components(resources: &Resources) -> impl Iterator<Item = &str> {
    resources.values().flat_map(Resource::referenced_components)
}

/// Resolve SchemaRef inner types in resources (operation query params).
/// This allows to_java() and similar methods to properly convert string aliases to their base types.
pub(crate) fn resolve_schema_refs_in_resources(
    resources: &mut Resources,
    string_alias_names: &BTreeSet<String>,
) {
    for resource in resources.values_mut() {
        resolve_schema_refs_in_resource(resource, string_alias_names);
    }
}

fn resolve_schema_refs_in_resource(resource: &mut Resource, string_alias_names: &BTreeSet<String>) {
    // Resolve in subresources
    for sub in resource.subresources.values_mut() {
        resolve_schema_refs_in_resource(sub, string_alias_names);
    }

    // Resolve in operations
    for op in &mut resource.operations {
        for param in &mut op.query_params {
            resolve_schema_ref_in_field_type_public(&mut param.r#type, string_alias_names);
        }
    }
}

fn get_or_insert_resource(resources: &mut Resources, path: Vec<String>) -> &mut Resource {
    let mut path_iter = path.into_iter();
    let mut name = path_iter.next().expect("path must be non-empty");
    let mut r = resources
        .entry(name.clone())
        .or_insert_with(|| Resource::new(name.clone()));

    for sub_name in path_iter {
        name.push('.');
        name.push_str(&sub_name);

        r = r
            .subresources
            .entry(sub_name)
            .or_insert_with(|| Resource::new(name.clone()));
    }

    r
}

/// A named group of [`Operation`]s.
#[derive(Deserialize, Serialize)]
pub(crate) struct Resource {
    pub name: String,
    pub operations: Vec<Operation>,
    pub subresources: Resources,
}

impl Resource {
    fn new(name: String) -> Self {
        Self {
            name,
            operations: Vec::new(),
            subresources: BTreeMap::new(),
        }
    }

    pub(crate) fn referenced_components(&self) -> BTreeSet<&str> {
        let mut res = BTreeSet::new();

        for resource in self.subresources.values() {
            res.extend(resource.referenced_components());
        }

        for operation in &self.operations {
            for param in &operation.query_params {
                if let Some(name) = param.r#type.referenced_schema() {
                    res.insert(name);
                }
            }
            if let Some(name) = &operation.request_body_schema_name {
                res.insert(name);
            }
            if let Some(name) = &operation.response_body_schema_name {
                res.insert(name);
            }
        }

        res
    }
}

/// A named HTTP endpoint.
#[derive(Deserialize, Serialize)]
pub(crate) struct Operation {
    /// The operation ID from the spec.
    pub(crate) id: String,
    /// The name to use for the operation in code.
    pub(crate) name: String,
    /// Description of the operation to use for documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// Whether this operation is marked as deprecated.
    deprecated: bool,
    /// The HTTP method.
    ///
    /// Encoded as "get", "post" or such because that's what aide's PathItem iterator gives us.
    method: String,
    /// The operation's endpoint path.
    path: String,
    /// Path parameters.
    ///
    /// Only required string-typed parameters are currently supported.
    path_params: Vec<String>,
    /// Header parameters.
    ///
    /// Only string-typed parameters are currently supported.
    header_params: Vec<HeaderParam>,
    /// Query parameters.
    pub(crate) query_params: Vec<QueryParam>,
    /// Name of the request body type, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) request_body_schema_name: Option<String>,
    /// Some request bodies are required, but all the fields are optional (i.e. the CLI can omit
    /// this from the argument list).
    /// Only useful when `request_body_schema_name` is `Some`.
    request_body_all_optional: bool,
    /// True if the request body is application/x-www-form-urlencoded instead of JSON.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    request_body_is_form: bool,
    /// Name of the response body type, if any (only for JSON responses).
    #[serde(skip_serializing_if = "Option::is_none")]
    response_body_schema_name: Option<String>,
    /// True if the response is binary (e.g., application/pdf, application/octet-stream).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    response_is_binary: bool,
    /// True if the response is text (e.g., text/plain, text/html).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    response_is_text: bool,
}

impl Operation {
    #[tracing::instrument(
        name = "operation_from_openapi",
        skip_all,
        fields(path = path, method = method, op_id),
    )]
    fn from_openapi(
        path: &str,
        method: &str,
        op: openapi::Operation,
        component_schemas: &IndexMap<String, aide::openapi::SchemaObject>,
        include_mode: IncludeMode,
        excluded_operations: &BTreeSet<String>,
        specified_operations: &BTreeSet<String>,
    ) -> Option<(Vec<String>, Self)> {
        let Some(op_id) = op.operation_id else {
            // ignore operations without an operationId
            return None;
        };
        tracing::Span::current().record("op_id", &op_id);

        // verbose, but very easy to understand
        let x_internal = op
            .extensions
            .get("x-internal")
            .is_some_and(|val| val == true);
        let include_operation = match include_mode {
            IncludeMode::OnlyPublic => !x_internal,
            IncludeMode::PublicAndInternal => true,
            IncludeMode::OnlyInternal => x_internal,
            IncludeMode::OnlySpecified => specified_operations.contains(&op_id),
        };
        if !include_operation || excluded_operations.contains(&op_id) {
            return None;
        }

        // Use tags as resource name, operation ID as operation name
        let tag = op.tags.first().cloned().unwrap_or_else(|| {
            // Derive resource from path: /api/v1/customers -> customers
            path.split('/')
                .find(|s| !s.is_empty() && *s != "api" && *s != "v1")
                .unwrap_or("default")
                .to_owned()
        });
        // Convert tag to resource name (e.g., "checkout-sessions" -> "checkout_sessions")
        let resource_name = tag.replace('-', "_");
        let res_path = vec![resource_name];
        let op_name = op_id.clone();

        let mut path_params = Vec::new();
        let mut query_params = Vec::new();
        let mut header_params = Vec::new();

        for param in op.parameters {
            match param {
                ReferenceOr::Reference { .. } => {
                    tracing::warn!("$ref parameters are not currently supported");
                    return None;
                }
                ReferenceOr::Item(openapi::Parameter::Path {
                    parameter_data,
                    style: openapi::PathStyle::Simple,
                }) => {
                    assert!(parameter_data.required, "no optional path params");
                    if let Err(e) = enforce_string_parameter(&parameter_data) {
                        tracing::warn!("unsupported path parameter: {e}");
                        return None;
                    }

                    path_params.push(parameter_data.name);
                }
                ReferenceOr::Item(openapi::Parameter::Header {
                    parameter_data,
                    style: openapi::HeaderStyle::Simple,
                }) => {
                    if parameter_data.name != "idempotency-key" {
                        tracing::warn!(name = parameter_data.name, "unknown header parameter");
                    }

                    if let Err(e) = enforce_string_parameter(&parameter_data) {
                        tracing::warn!("unsupported header parameter: {e}");
                        return None;
                    }

                    header_params.push(HeaderParam {
                        name: parameter_data.name,
                        required: parameter_data.required,
                    });
                }
                ReferenceOr::Item(openapi::Parameter::Query {
                    parameter_data,
                    allow_reserved: false,
                    style: openapi::QueryStyle::Form,
                    allow_empty_value: None,
                }) => {
                    let name = parameter_data.name;
                    if method == "post" && name == "get_if_exists" {
                        tracing::debug!("ignoring get_if_exists query parameter");
                        continue;
                    }

                    let _guard = tracing::info_span!("field_type_from_openapi", name).entered();
                    let r#type = match FieldType::from_openapi(parameter_data.format) {
                        Ok(t) => t,
                        Err(e) => {
                            tracing::warn!("unsupported query parameter type: {e}");
                            return None;
                        }
                    };

                    // For `style: form`, OpenAPI's default is `explode: true` — arrays are
                    // serialized as repeated parameters (e.g. `?tags=a&tags=b`). When
                    // `explode: false`, arrays are comma-joined (e.g. `?tags=a,b`).
                    let explode = parameter_data.explode.unwrap_or(true);

                    query_params.push(QueryParam {
                        name,
                        description: parameter_data.description,
                        required: parameter_data.required,
                        r#type,
                        explode,
                    });
                }
                ReferenceOr::Item(parameter) => {
                    tracing::warn!(
                        ?parameter,
                        "this kind of parameter is not currently supported"
                    );
                    return None;
                }
            }
        }

        let request_body_all_optional = op
            .request_body
            .as_ref()
            .map(|r| {
                match r {
                    ReferenceOr::Reference { .. } => {
                        unimplemented!("reference")
                    }
                    ReferenceOr::Item(body) => {
                        if let Some(mt) = body
                            .content
                            .get("application/json")
                            .or_else(|| body.content.get("application/x-www-form-urlencoded"))
                        {
                            match mt.schema.as_ref().map(|so| &so.json_schema) {
                                Some(Schema::Object(schemars::schema::SchemaObject {
                                    object: Some(ov),
                                    ..
                                })) => {
                                    return ov.required.is_empty();
                                }
                                Some(Schema::Object(schemars::schema::SchemaObject {
                                    reference: Some(s),
                                    ..
                                })) => {
                                    match component_schemas
                                        .get(
                                            &get_schema_name(Some(s)).expect("schema should exist"),
                                        )
                                        .map(|so| &so.json_schema)
                                    {
                                        Some(Schema::Object(schemars::schema::SchemaObject {
                                            object: Some(ov),
                                            ..
                                        })) => {
                                            return ov.required.is_empty();
                                        }
                                        _ => unimplemented!("double ref not supported"),
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                false
            })
            .unwrap_or_default();

        let mut request_body_is_form = false;
        let request_body_schema_name = op.request_body.and_then(|b| match b {
            ReferenceOr::Item(mut req_body) => {
                assert!(req_body.required);
                assert!(req_body.extensions.is_empty());
                assert_eq!(req_body.content.len(), 1);
                let is_form = req_body
                    .content
                    .contains_key("application/x-www-form-urlencoded");
                let body = req_body
                    .content
                    .swap_remove("application/json")
                    .or_else(|| {
                        req_body
                            .content
                            .swap_remove("application/x-www-form-urlencoded")
                    })
                    .expect("should have JSON or form-urlencoded body");
                if is_form {
                    request_body_is_form = true;
                }
                assert!(body.extensions.is_empty());
                match body.schema.expect("no body schema?!").json_schema {
                    Schema::Bool(_) => {
                        tracing::error!("unexpected bool schema");
                        None
                    }
                    Schema::Object(obj) => {
                        if !obj.is_ref() {
                            tracing::error!(?obj, "unexpected non-$ref json body schema");
                        }
                        get_schema_name(obj.reference.as_deref())
                    }
                }
            }
            ReferenceOr::Reference { .. } => {
                tracing::error!("$ref request bodies are not currently supported");
                None
            }
        });

        let (response_body_schema_name, response_kind) = op
            .responses
            .map(|r| {
                assert_eq!(r.default, None);
                assert!(r.extensions.is_empty());
                let mut success_responses = r.responses.into_iter().filter(|(st, _)| {
                    match st {
                        openapi::StatusCode::Code(c) => match c {
                            0..100 => tracing::error!("invalid status code < 100"),
                            100..200 => tracing::error!("what is this? status code {c}..."),
                            200..300 => return true,
                            300..400 => tracing::error!("what is this? status code {c}..."),
                            400.. => {}
                        },
                        openapi::StatusCode::Range(_) => {
                            tracing::error!("unsupported status code range");
                        }
                    }

                    false
                });

                let (_, resp) = success_responses
                    .next()
                    .expect("every operation must have one success response");
                let (schema_name, kind) = response_body_info(resp);
                for (_, resp) in success_responses {
                    let (other_name, other_kind) = response_body_info(resp);
                    assert_eq!(schema_name, other_name);
                    assert_eq!(kind, other_kind);
                }

                (schema_name, kind)
            })
            .unwrap_or((None, ResponseKind::None));

        let op = Operation {
            id: op_id,
            name: op_name,
            description: op.description,
            deprecated: op.deprecated,
            method: method.to_owned(),
            path: path.to_owned(),
            path_params,
            header_params,
            query_params,
            request_body_schema_name,
            request_body_all_optional,
            request_body_is_form,
            response_body_schema_name,
            response_is_binary: response_kind == ResponseKind::Binary,
            response_is_text: response_kind == ResponseKind::Text,
        };
        Some((res_path, op))
    }

    pub(crate) fn has_query_or_header_params(&self) -> bool {
        !self.header_params.is_empty() || !self.query_params.is_empty()
    }
}

fn enforce_string_parameter(parameter_data: &openapi::ParameterData) -> anyhow::Result<()> {
    let openapi::ParameterSchemaOrContent::Schema(s) = &parameter_data.format else {
        bail!("found unexpected 'content' data format");
    };
    let Schema::Object(obj) = &s.json_schema else {
        bail!("found unexpected `true` schema");
    };

    // Check for direct string type
    if obj.instance_type == Some(InstanceType::String.into()) {
        return Ok(());
    }

    // Handle OpenAPI 3.1 type arrays like ["string", "null"]
    if let Some(schemars::schema::SingleOrVec::Vec(types)) = &obj.instance_type {
        let has_string = types.contains(&InstanceType::String);
        let all_string_or_null = types
            .iter()
            .all(|t| *t == InstanceType::String || *t == InstanceType::Null);
        if has_string && all_string_or_null {
            return Ok(());
        }
    }

    // Handle oneOf patterns like: oneOf: [{type: null}, {$ref: ...}] or oneOf: [{type: null}, {type: string}]
    if let Some(ref subschemas) = obj.subschemas
        && let Some(ref one_of) = subschemas.one_of
        && one_of.len() == 2
    {
        for schema in one_of {
            if let Schema::Object(inner_obj) = schema {
                // Check if this is a null type - skip it
                let is_null = match &inner_obj.instance_type {
                    Some(schemars::schema::SingleOrVec::Single(t)) => **t == InstanceType::Null,
                    _ => false,
                };
                if is_null {
                    continue;
                }

                // Check if this is a string type
                if inner_obj.instance_type == Some(InstanceType::String.into()) {
                    return Ok(());
                }

                // Check if this is a $ref (path params with refs are typically string IDs)
                if inner_obj.reference.is_some() {
                    return Ok(());
                }
            }
        }
    }

    // If instance_type is None but there's a $ref, accept it (typically string IDs)
    if obj.instance_type.is_none() && obj.reference.is_some() {
        return Ok(());
    }

    bail!("unsupported path parameter type `{:?}`", obj.instance_type);
}

/// Response body type for code generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ResponseKind {
    #[default]
    None,
    Json,
    Binary,
    Text,
}

/// Returns (schema_name, response_kind) for a response.
fn response_body_info(resp: ReferenceOr<openapi::Response>) -> (Option<String>, ResponseKind) {
    match resp {
        ReferenceOr::Item(mut resp_body) => {
            assert!(resp_body.extensions.is_empty());
            if resp_body.content.is_empty() {
                return (None, ResponseKind::None);
            }

            // Check for binary response types
            let binary_types = [
                "application/pdf",
                "application/octet-stream",
                "image/png",
                "image/jpeg",
                "image/gif",
                "application/zip",
            ];
            for binary_type in binary_types {
                if resp_body.content.contains_key(binary_type) {
                    tracing::info!(content_type = binary_type, "detected binary response");
                    return (None, ResponseKind::Binary);
                }
            }

            // Check for text response types
            let text_types = ["text/plain", "text/html", "text/csv"];
            for text_type in text_types {
                if resp_body.content.contains_key(text_type) {
                    tracing::info!(content_type = text_type, "detected text response");
                    return (None, ResponseKind::Text);
                }
            }

            // Handle JSON responses
            let Some(json_body) = resp_body.content.swap_remove("application/json") else {
                tracing::info!(
                    content_types = ?resp_body.content.keys().collect::<Vec<_>>(),
                    "skipping unknown response body type"
                );
                return (None, ResponseKind::None);
            };
            assert!(json_body.extensions.is_empty());
            let schema_name = match json_body.schema.expect("no json body schema?!").json_schema {
                Schema::Bool(_) => {
                    tracing::error!("unexpected bool schema");
                    None
                }
                Schema::Object(obj) => {
                    if !obj.is_ref() {
                        tracing::error!(?obj, "unexpected non-$ref json body schema");
                    }
                    get_schema_name(obj.reference.as_deref())
                }
            };
            (schema_name, ResponseKind::Json)
        }
        ReferenceOr::Reference { .. } => {
            tracing::error!("$ref response bodies are not currently supported");
            (None, ResponseKind::None)
        }
    }
}

#[derive(Deserialize, Serialize)]
struct HeaderParam {
    name: String,
    required: bool,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct QueryParam {
    pub(crate) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    required: bool,
    #[serde(serialize_with = "serialize_field_type")]
    pub(crate) r#type: FieldType,
    /// Whether array values are exploded into repeated query parameters
    /// (`?tags=a&tags=b`) or comma-joined (`?tags=a,b`). Defaults to `true`
    /// per OpenAPI's `style: form` default.
    #[serde(default = "default_explode")]
    pub(crate) explode: bool,
}

fn default_explode() -> bool {
    true
}
