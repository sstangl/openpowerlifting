//! Defines the `Federation` field for the `meets` table.

/// Enum of federations.
///
/// `Display` derivation provided by strum.
/// `EnumString` (`FromStr`) derivation provided by strum.
///
/// Note that the deserialization source string (as in the CSV data)
/// may differ from the FromStr source string, which comes from a URL
/// and is generally lowercase.
///
/// The strum `to_string` value defines the default .to_string() result,
/// while *all* of to_string and serialize are parseable.
/// So Federation::APF can be parsed from the strings "APF" and "apf".
#[derive(
    Copy,
    Clone,
    Debug,
    Deserialize,
    Display,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Serialize,
    EnumIter,
    EnumString,
)]
pub enum Federation {
    #[serde(rename = "365Strong")]
    #[strum(to_string = "365Strong", serialize = "365strong")]
    _365Strong,
    #[strum(to_string = "AAP", serialize = "aap")]
    AAP,
    #[strum(to_string = "AAPLF", serialize = "aaplf")]
    AAPLF,
    #[strum(to_string = "AAU", serialize = "aau")]
    AAU,
    #[strum(to_string = "ADAU", serialize = "adau")]
    ADAU,
    #[strum(to_string = "ADFPA", serialize = "adfpa")]
    ADFPA,
    #[strum(to_string = "ADFPF", serialize = "adfpf")]
    ADFPF,
    #[strum(to_string = "AEP", serialize = "aep")]
    AEP,
    #[strum(to_string = "AfricanPF", serialize = "africanpf")]
    AfricanPF,
    #[strum(to_string = "APA", serialize = "apa")]
    AIWBPA,
    #[strum(to_string = "AIWBPA", serialize = "aiwbpa")]
    APA,
    #[strum(to_string = "APC", serialize = "apc")]
    APC,
    #[strum(to_string = "APF", serialize = "apf")]
    APF,
    #[strum(to_string = "APU", serialize = "apu")]
    APU,
    #[strum(to_string = "AsianPF", serialize = "asianpf")]
    AsianPF,
    #[strum(to_string = "AusDFPF", serialize = "ausdfpf")]
    AusDFPF,
    #[strum(to_string = "BAWLA", serialize = "bawla")]
    BAWLA,
    #[strum(to_string = "BB", serialize = "bb")]
    BB,
    #[strum(to_string = "BDFPA", serialize = "bdfpa")]
    BDFPA,
    #[strum(to_string = "BPC", serialize = "bpc")]
    BPC,
    #[strum(to_string = "BPO", serialize = "bpo")]
    BPO,
    #[strum(to_string = "BPU", serialize = "bpu")]
    BPU,
    #[strum(to_string = "BP", serialize = "bp")]
    BP,
    #[strum(to_string = "BVDK", serialize = "bvdk")]
    BVDK,
    #[strum(to_string = "CAPO", serialize = "capo")]
    CAPO,
    #[strum(to_string = "CAST", serialize = "cast")]
    CAST,
    #[strum(to_string = "CommonwealthPF", serialize = "commonwealthpf")]
    CommonwealthPF,
    #[strum(to_string = "CPF", serialize = "cpf")]
    CPF,
    #[strum(to_string = "CPL", serialize = "cpl")]
    CPL,
    #[strum(to_string = "CPO", serialize = "cpo")]
    CPO,
    #[strum(to_string = "CPU", serialize = "cpu")]
    CPU,
    #[strum(to_string = "CSST", serialize = "csst")]
    CSST,
    #[strum(to_string = "DSF", serialize = "dsf")]
    DSF,
    #[strum(to_string = "EPA", serialize = "epa")]
    EPA,
    #[strum(to_string = "EPF", serialize = "epf")]
    EPF,
    #[strum(to_string = "FALPO", serialize = "falpo")]
    FALPO,
    #[strum(to_string = "FEMEPO", serialize = "femepo")]
    FEMEPO,
    #[strum(to_string = "FEPOA", serialize = "fepoa")]
    FEPOA,
    #[strum(to_string = "FESUPO", serialize = "fesupo")]
    FESUPO,
    #[strum(to_string = "FFForce", serialize = "ffforce")]
    FFForce,
    #[strum(to_string = "FPO", serialize = "fpo")]
    FPO,
    #[strum(to_string = "FPR", serialize = "fpr")]
    FPR,
    #[strum(to_string = "GoldenDouble", serialize = "goldendouble")]
    GoldenDouble,
    #[strum(to_string = "GPA", serialize = "gpa")]
    GPA,
    #[strum(to_string = "GPC", serialize = "gpc")]
    GPC,
    #[serde(rename = "GPC-AUS")]
    #[strum(to_string = "GPC-AUS", serialize = "gpc-aus")]
    GPCAUS,
    #[serde(rename = "GPC-CAN")]
    #[strum(to_string = "GPC-CAN", serialize = "gpc-can")]
    GPCCAN,
    #[serde(rename = "GPC-GB")]
    #[strum(to_string = "GPC-GB", serialize = "gpc-gb")]
    GPCGB,
    #[serde(rename = "GPC-IRL")]
    #[strum(to_string = "GPC-IRL", serialize = "gpc-irl")]
    GPCIRL,
    #[serde(rename = "GPC-NZ")]
    #[strum(to_string = "GPC-NZ", serialize = "gpc-nz")]
    GPCNZ,
    #[serde(rename = "GPC-RUS")]
    #[strum(to_string = "GPC-RUS", serialize = "gpc-rus")]
    GPCRUS,
    #[strum(to_string = "GPU", serialize = "gpu")]
    GPU,
    #[strum(to_string = "Hardcore", serialize = "hardcore")]
    Hardcore,
    #[strum(to_string = "HERC", serialize = "herc")]
    HERC,
    #[strum(to_string = "IBSA", serialize = "ibsa")]
    IBSA,
    #[strum(to_string = "IDFPA", serialize = "idfpa")]
    IDFPA,
    #[strum(to_string = "IDFPF", serialize = "idfpf")]
    IDFPF,
    #[strum(to_string = "IPA", serialize = "ipa")]
    IPA,
    #[strum(to_string = "IPC", serialize = "ipc")]
    IPC,
    #[strum(to_string = "IPF", serialize = "ipf")]
    IPF,
    #[strum(to_string = "IPL", serialize = "ipl")]
    IPL,
    #[strum(to_string = "IrishPF", serialize = "irishpf")]
    IrishPF,
    #[strum(to_string = "IrishPO", serialize = "irishpo")]
    IrishPO,
    #[strum(to_string = "JPA", serialize = "jpa")]
    JPA,
    #[strum(to_string = "KRAFT", serialize = "kraft")]
    KRAFT,
    #[strum(to_string = "LPF", serialize = "lpf")]
    LPF,
    #[strum(to_string = "MHP", serialize = "mhp")]
    MHP,
    #[strum(to_string = "MM", serialize = "mm")]
    MM,
    #[strum(to_string = "MPA", serialize = "mpa")]
    MPA,
    #[strum(to_string = "NAP", serialize = "nap")]
    NAP,
    #[strum(to_string = "NAPF", serialize = "napf")]
    NAPF,
    #[strum(to_string = "NASA", serialize = "nasa")]
    NASA,
    #[strum(to_string = "NIPF", serialize = "nipf")]
    NIPF,
    #[strum(to_string = "NordicPF", serialize = "nordicpf")]
    NordicPF,
    #[strum(to_string = "NOTLD", serialize = "notld")]
    NOTLD,
    #[strum(to_string = "NPA", serialize = "npa")]
    NPA,
    #[strum(to_string = "NSF", serialize = "nsf")]
    NSF,
    #[strum(to_string = "NZPF", serialize = "nzpf")]
    NZPF,
    #[strum(to_string = "OceaniaPF", serialize = "oceaniapf")]
    OceaniaPF,
    #[strum(to_string = "OlomouckySilak", serialize = "olomouckysilak")]
    OlomouckySilak,
    #[strum(to_string = "ParaPL", serialize = "parapl")]
    ParaPL,
    #[strum(to_string = "PA", serialize = "pa")]
    PA,
    #[strum(to_string = "PLZS", serialize = "plzs")]
    PLZS,
    #[strum(to_string = "PRIDE", serialize = "pride")]
    PRIDE,
    #[strum(to_string = "ProRaw", serialize = "proraw")]
    ProRaw,
    #[strum(to_string = "PRPA", serialize = "prpa")]
    PRPA,
    #[strum(to_string = "PZKFiTS", serialize = "pzkfits")]
    PZKFiTS,
    #[strum(to_string = "RAW", serialize = "raw")]
    RAW,
    #[serde(rename = "RAW-CAN")]
    #[strum(to_string = "RAW-CAN", serialize = "raw-can")]
    RAWCAN,
    #[serde(rename = "RAW-UKR")]
    #[strum(to_string = "RAW-UKR", serialize = "raw-ukr")]
    RAWUKR,
    #[strum(to_string = "RAWU", serialize = "rawu")]
    RAWU,
    #[strum(to_string = "RPS", serialize = "rps")]
    RPS,
    #[strum(to_string = "RPU", serialize = "rpu")]
    RPU,
    #[strum(to_string = "RUPC", serialize = "rupc")]
    RUPC,
    #[strum(to_string = "ScottishPL", serialize = "scottishpl")]
    ScottishPL,
    #[strum(to_string = "SCT", serialize = "sct")]
    SCT,
    #[strum(to_string = "SPA", serialize = "spa")]
    SPA,
    #[strum(to_string = "SPF", serialize = "spf")]
    SPF,
    #[strum(to_string = "SPSS", serialize = "spss")]
    SPSS,
    #[strum(to_string = "SSF", serialize = "ssf")]
    SSF,
    #[strum(to_string = "SVNL", serialize = "svnl")]
    SVNL,
    #[strum(to_string = "THSPA", serialize = "thspa")]
    THSPA,
    #[strum(to_string = "UkrainePA", serialize = "ukrainepa")]
    UkrainePA,
    #[strum(to_string = "UPA", serialize = "upa")]
    UPA,
    #[strum(to_string = "UPC", serialize = "upc")]
    UPC,
    #[strum(to_string = "USAPL", serialize = "usapl")]
    USAPL,
    #[strum(to_string = "USPF", serialize = "uspf")]
    USPF,
    #[strum(to_string = "USPA", serialize = "uspa")]
    USPA,
    #[strum(to_string = "VietnamPA", serialize = "vietnampa")]
    VietnamPA,
    #[strum(to_string = "Vityaz", serialize = "vityaz")]
    Vityaz,
    #[strum(to_string = "WABDL", serialize = "wabdl")]
    WABDL,
    #[strum(to_string = "WDFPF", serialize = "wdfpf")]
    WDFPF,
    #[strum(to_string = "WelshPA", serialize = "welshpa")]
    WelshPA,
    #[strum(to_string = "WPA", serialize = "wpa")]
    WPA,
    #[serde(rename = "WPA-RUS")]
    #[strum(to_string = "WPA-RUS", serialize = "wpa-rus")]
    WPARUS,
    #[strum(to_string = "WPAU", serialize = "wpau")]
    WPAU,
    #[strum(to_string = "WPC", serialize = "wpc")]
    WPC,
    #[serde(rename = "WPC-Portugal")]
    #[strum(to_string = "WPC-Portugal", serialize = "wpc-portugal")]
    WPCPortugal,
    #[serde(rename = "WPC-RUS")]
    #[strum(to_string = "WPC-RUS", serialize = "wpc-rus")]
    WPCRUS,
    #[serde(rename = "WPC-UKR")]
    #[strum(to_string = "WPC-UKR", serialize = "wpc-ukr")]
    WPCUKR,
    #[strum(to_string = "WPF", serialize = "wpf")]
    WPF,
    #[strum(to_string = "WPU", serialize = "wpu")]
    WPU,
    #[strum(to_string = "WPUF", serialize = "wpuf")]
    WPUF,
    #[strum(to_string = "WNPF", serialize = "wnpf")]
    WNPF,
    #[strum(to_string = "WRPF", serialize = "wrpf")]
    WRPF,
    #[serde(rename = "WRPF-AUS")]
    #[strum(to_string = "WRPF-AUS", serialize = "wrpf-aus")]
    WRPFAUS,
    #[serde(rename = "WRPF-CAN")]
    #[strum(to_string = "WRPF-CAN", serialize = "wrpf-can")]
    WRPFCAN,
    #[strum(to_string = "WUAP", serialize = "wuap")]
    WUAP,
    #[strum(to_string = "XPC", serialize = "xpc")]
    XPC,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_strings() {
        // The lowercase form should parse.
        assert_eq!("wrpf".parse::<Federation>().unwrap(), Federation::WRPF);

        // The default to_string() should be the upper-case form.
        assert_eq!(Federation::WRPF.to_string(), "WRPF");
    }
}
