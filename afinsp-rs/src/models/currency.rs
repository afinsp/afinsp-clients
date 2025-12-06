// this file is @generated
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Currency {
    #[default]
    #[serde(rename = "AED")]
    Aed,

    #[serde(rename = "AFN")]
    Afn,

    #[serde(rename = "ALL")]
    All,

    #[serde(rename = "AMD")]
    Amd,

    #[serde(rename = "ANG")]
    Ang,

    #[serde(rename = "AOA")]
    Aoa,

    #[serde(rename = "ARS")]
    Ars,

    #[serde(rename = "AUD")]
    Aud,

    #[serde(rename = "AWG")]
    Awg,

    #[serde(rename = "AZN")]
    Azn,

    #[serde(rename = "BAM")]
    Bam,

    #[serde(rename = "BBD")]
    Bbd,

    #[serde(rename = "BDT")]
    Bdt,

    #[serde(rename = "BGN")]
    Bgn,

    #[serde(rename = "BHD")]
    Bhd,

    #[serde(rename = "BIF")]
    Bif,

    #[serde(rename = "BMD")]
    Bmd,

    #[serde(rename = "BND")]
    Bnd,

    #[serde(rename = "BOB")]
    Bob,

    #[serde(rename = "BRL")]
    Brl,

    #[serde(rename = "BSD")]
    Bsd,

    #[serde(rename = "BTN")]
    Btn,

    #[serde(rename = "BWP")]
    Bwp,

    #[serde(rename = "BYN")]
    Byn,

    #[serde(rename = "BZD")]
    Bzd,

    #[serde(rename = "CAD")]
    Cad,

    #[serde(rename = "CDF")]
    Cdf,

    #[serde(rename = "CHF")]
    Chf,

    #[serde(rename = "CLP")]
    Clp,

    #[serde(rename = "CNH")]
    Cnh,

    #[serde(rename = "CNY")]
    Cny,

    #[serde(rename = "COP")]
    Cop,

    #[serde(rename = "CRC")]
    Crc,

    #[serde(rename = "CUC")]
    Cuc,

    #[serde(rename = "CUP")]
    Cup,

    #[serde(rename = "CVE")]
    Cve,

    #[serde(rename = "CZK")]
    Czk,

    #[serde(rename = "DJF")]
    Djf,

    #[serde(rename = "DKK")]
    Dkk,

    #[serde(rename = "DOP")]
    Dop,

    #[serde(rename = "DZD")]
    Dzd,

    #[serde(rename = "EGP")]
    Egp,

    #[serde(rename = "ERN")]
    Ern,

    #[serde(rename = "ETB")]
    Etb,

    #[serde(rename = "EUR")]
    Eur,

    #[serde(rename = "FJD")]
    Fjd,

    #[serde(rename = "FKP")]
    Fkp,

    #[serde(rename = "GBP")]
    Gbp,

    #[serde(rename = "GEL")]
    Gel,

    #[serde(rename = "GHS")]
    Ghs,

    #[serde(rename = "GIP")]
    Gip,

    #[serde(rename = "GMD")]
    Gmd,

    #[serde(rename = "GNF")]
    Gnf,

    #[serde(rename = "GTQ")]
    Gtq,

    #[serde(rename = "GYD")]
    Gyd,

    #[serde(rename = "HKD")]
    Hkd,

    #[serde(rename = "HNL")]
    Hnl,

    #[serde(rename = "HRK")]
    Hrk,

    #[serde(rename = "HTG")]
    Htg,

    #[serde(rename = "HUF")]
    Huf,

    #[serde(rename = "IDR")]
    Idr,

    #[serde(rename = "ILS")]
    Ils,

    #[serde(rename = "INR")]
    Inr,

    #[serde(rename = "IQD")]
    Iqd,

    #[serde(rename = "IRR")]
    Irr,

    #[serde(rename = "ISK")]
    Isk,

    #[serde(rename = "JMD")]
    Jmd,

    #[serde(rename = "JOD")]
    Jod,

    #[serde(rename = "JPY")]
    Jpy,

    #[serde(rename = "KES")]
    Kes,

    #[serde(rename = "KGS")]
    Kgs,

    #[serde(rename = "KHR")]
    Khr,

    #[serde(rename = "KMF")]
    Kmf,

    #[serde(rename = "KPW")]
    Kpw,

    #[serde(rename = "KRW")]
    Krw,

    #[serde(rename = "KWD")]
    Kwd,

    #[serde(rename = "KYD")]
    Kyd,

    #[serde(rename = "KZT")]
    Kzt,

    #[serde(rename = "LAK")]
    Lak,

    #[serde(rename = "LBP")]
    Lbp,

    #[serde(rename = "LKR")]
    Lkr,

    #[serde(rename = "LRD")]
    Lrd,

    #[serde(rename = "LSL")]
    Lsl,

    #[serde(rename = "LYD")]
    Lyd,

    #[serde(rename = "MAD")]
    Mad,

    #[serde(rename = "MDL")]
    Mdl,

    #[serde(rename = "MGA")]
    Mga,

    #[serde(rename = "MKD")]
    Mkd,

    #[serde(rename = "MMK")]
    Mmk,

    #[serde(rename = "MNT")]
    Mnt,

    #[serde(rename = "MOP")]
    Mop,

    #[serde(rename = "MRU")]
    Mru,

    #[serde(rename = "MUR")]
    Mur,

    #[serde(rename = "MVR")]
    Mvr,

    #[serde(rename = "MWK")]
    Mwk,

    #[serde(rename = "MXN")]
    Mxn,

    #[serde(rename = "MYR")]
    Myr,

    #[serde(rename = "MZN")]
    Mzn,

    #[serde(rename = "NAD")]
    Nad,

    #[serde(rename = "NGN")]
    Ngn,

    #[serde(rename = "NIO")]
    Nio,

    #[serde(rename = "NOK")]
    Nok,

    #[serde(rename = "NPR")]
    Npr,

    #[serde(rename = "NZD")]
    Nzd,

    #[serde(rename = "OMR")]
    Omr,

    #[serde(rename = "PAB")]
    Pab,

    #[serde(rename = "PEN")]
    Pen,

    #[serde(rename = "PGK")]
    Pgk,

    #[serde(rename = "PHP")]
    Php,

    #[serde(rename = "PKR")]
    Pkr,

    #[serde(rename = "PLN")]
    Pln,

    #[serde(rename = "PYG")]
    Pyg,

    #[serde(rename = "QAR")]
    Qar,

    #[serde(rename = "RON")]
    Ron,

    #[serde(rename = "RSD")]
    Rsd,

    #[serde(rename = "RUB")]
    Rub,

    #[serde(rename = "RWF")]
    Rwf,

    #[serde(rename = "SAR")]
    Sar,

    #[serde(rename = "SBD")]
    Sbd,

    #[serde(rename = "SCR")]
    Scr,

    #[serde(rename = "SDG")]
    Sdg,

    #[serde(rename = "SEK")]
    Sek,

    #[serde(rename = "SGD")]
    Sgd,

    #[serde(rename = "SHP")]
    Shp,

    #[serde(rename = "SLL")]
    Sll,

    #[serde(rename = "SOS")]
    Sos,

    #[serde(rename = "SRD")]
    Srd,

    #[serde(rename = "SSP")]
    Ssp,

    #[serde(rename = "STD")]
    Std,

    #[serde(rename = "STN")]
    Stn,

    #[serde(rename = "SVC")]
    Svc,

    #[serde(rename = "SYP")]
    Syp,

    #[serde(rename = "SZL")]
    Szl,

    #[serde(rename = "THB")]
    Thb,

    #[serde(rename = "TJS")]
    Tjs,

    #[serde(rename = "TMT")]
    Tmt,

    #[serde(rename = "TND")]
    Tnd,

    #[serde(rename = "TOP")]
    Top,

    #[serde(rename = "TRY")]
    Try,

    #[serde(rename = "TTD")]
    Ttd,

    #[serde(rename = "TWD")]
    Twd,

    #[serde(rename = "TZS")]
    Tzs,

    #[serde(rename = "UAH")]
    Uah,

    #[serde(rename = "UGX")]
    Ugx,

    #[serde(rename = "USD")]
    Usd,

    #[serde(rename = "UYU")]
    Uyu,

    #[serde(rename = "UZS")]
    Uzs,

    #[serde(rename = "VES")]
    Ves,

    #[serde(rename = "VND")]
    Vnd,

    #[serde(rename = "VUV")]
    Vuv,

    #[serde(rename = "WST")]
    Wst,

    #[serde(rename = "XAF")]
    Xaf,

    #[serde(rename = "XCD")]
    Xcd,

    #[serde(rename = "XOF")]
    Xof,

    #[serde(rename = "XPF")]
    Xpf,

    #[serde(rename = "YER")]
    Yer,

    #[serde(rename = "ZAR")]
    Zar,

    #[serde(rename = "ZMW")]
    Zmw,

    #[serde(rename = "ZWL")]
    Zwl,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Self::Aed => "AED",
            Self::Afn => "AFN",
            Self::All => "ALL",
            Self::Amd => "AMD",
            Self::Ang => "ANG",
            Self::Aoa => "AOA",
            Self::Ars => "ARS",
            Self::Aud => "AUD",
            Self::Awg => "AWG",
            Self::Azn => "AZN",
            Self::Bam => "BAM",
            Self::Bbd => "BBD",
            Self::Bdt => "BDT",
            Self::Bgn => "BGN",
            Self::Bhd => "BHD",
            Self::Bif => "BIF",
            Self::Bmd => "BMD",
            Self::Bnd => "BND",
            Self::Bob => "BOB",
            Self::Brl => "BRL",
            Self::Bsd => "BSD",
            Self::Btn => "BTN",
            Self::Bwp => "BWP",
            Self::Byn => "BYN",
            Self::Bzd => "BZD",
            Self::Cad => "CAD",
            Self::Cdf => "CDF",
            Self::Chf => "CHF",
            Self::Clp => "CLP",
            Self::Cnh => "CNH",
            Self::Cny => "CNY",
            Self::Cop => "COP",
            Self::Crc => "CRC",
            Self::Cuc => "CUC",
            Self::Cup => "CUP",
            Self::Cve => "CVE",
            Self::Czk => "CZK",
            Self::Djf => "DJF",
            Self::Dkk => "DKK",
            Self::Dop => "DOP",
            Self::Dzd => "DZD",
            Self::Egp => "EGP",
            Self::Ern => "ERN",
            Self::Etb => "ETB",
            Self::Eur => "EUR",
            Self::Fjd => "FJD",
            Self::Fkp => "FKP",
            Self::Gbp => "GBP",
            Self::Gel => "GEL",
            Self::Ghs => "GHS",
            Self::Gip => "GIP",
            Self::Gmd => "GMD",
            Self::Gnf => "GNF",
            Self::Gtq => "GTQ",
            Self::Gyd => "GYD",
            Self::Hkd => "HKD",
            Self::Hnl => "HNL",
            Self::Hrk => "HRK",
            Self::Htg => "HTG",
            Self::Huf => "HUF",
            Self::Idr => "IDR",
            Self::Ils => "ILS",
            Self::Inr => "INR",
            Self::Iqd => "IQD",
            Self::Irr => "IRR",
            Self::Isk => "ISK",
            Self::Jmd => "JMD",
            Self::Jod => "JOD",
            Self::Jpy => "JPY",
            Self::Kes => "KES",
            Self::Kgs => "KGS",
            Self::Khr => "KHR",
            Self::Kmf => "KMF",
            Self::Kpw => "KPW",
            Self::Krw => "KRW",
            Self::Kwd => "KWD",
            Self::Kyd => "KYD",
            Self::Kzt => "KZT",
            Self::Lak => "LAK",
            Self::Lbp => "LBP",
            Self::Lkr => "LKR",
            Self::Lrd => "LRD",
            Self::Lsl => "LSL",
            Self::Lyd => "LYD",
            Self::Mad => "MAD",
            Self::Mdl => "MDL",
            Self::Mga => "MGA",
            Self::Mkd => "MKD",
            Self::Mmk => "MMK",
            Self::Mnt => "MNT",
            Self::Mop => "MOP",
            Self::Mru => "MRU",
            Self::Mur => "MUR",
            Self::Mvr => "MVR",
            Self::Mwk => "MWK",
            Self::Mxn => "MXN",
            Self::Myr => "MYR",
            Self::Mzn => "MZN",
            Self::Nad => "NAD",
            Self::Ngn => "NGN",
            Self::Nio => "NIO",
            Self::Nok => "NOK",
            Self::Npr => "NPR",
            Self::Nzd => "NZD",
            Self::Omr => "OMR",
            Self::Pab => "PAB",
            Self::Pen => "PEN",
            Self::Pgk => "PGK",
            Self::Php => "PHP",
            Self::Pkr => "PKR",
            Self::Pln => "PLN",
            Self::Pyg => "PYG",
            Self::Qar => "QAR",
            Self::Ron => "RON",
            Self::Rsd => "RSD",
            Self::Rub => "RUB",
            Self::Rwf => "RWF",
            Self::Sar => "SAR",
            Self::Sbd => "SBD",
            Self::Scr => "SCR",
            Self::Sdg => "SDG",
            Self::Sek => "SEK",
            Self::Sgd => "SGD",
            Self::Shp => "SHP",
            Self::Sll => "SLL",
            Self::Sos => "SOS",
            Self::Srd => "SRD",
            Self::Ssp => "SSP",
            Self::Std => "STD",
            Self::Stn => "STN",
            Self::Svc => "SVC",
            Self::Syp => "SYP",
            Self::Szl => "SZL",
            Self::Thb => "THB",
            Self::Tjs => "TJS",
            Self::Tmt => "TMT",
            Self::Tnd => "TND",
            Self::Top => "TOP",
            Self::Try => "TRY",
            Self::Ttd => "TTD",
            Self::Twd => "TWD",
            Self::Tzs => "TZS",
            Self::Uah => "UAH",
            Self::Ugx => "UGX",
            Self::Usd => "USD",
            Self::Uyu => "UYU",
            Self::Uzs => "UZS",
            Self::Ves => "VES",
            Self::Vnd => "VND",
            Self::Vuv => "VUV",
            Self::Wst => "WST",
            Self::Xaf => "XAF",
            Self::Xcd => "XCD",
            Self::Xof => "XOF",
            Self::Xpf => "XPF",
            Self::Yer => "YER",
            Self::Zar => "ZAR",
            Self::Zmw => "ZMW",
            Self::Zwl => "ZWL",
        };
        f.write_str(value)
    }
}

impl crate::request::QueryParamValue for Currency {
    fn encode(&self) -> String {
        self.to_string()
    }
}
