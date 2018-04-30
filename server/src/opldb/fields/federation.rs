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
#[derive(Copy, Clone, Debug, Deserialize, Display, PartialEq, Serialize, EnumIter,
         EnumString)]
pub enum Federation {
    #[serde(rename = "365Strong")]
    #[strum(to_string = "365Strong", serialize = "365strong")]
    _365Strong,
    #[strum(to_string = "AAPF", serialize = "aapf")]
    AAPF,
    #[strum(to_string = "AAU", serialize = "aau")]
    AAU,
    #[strum(to_string = "ADFPA", serialize = "adfpa")]
    ADFPA,
    #[strum(to_string = "AEP", serialize = "aep")]
    AEP,
    #[strum(to_string = "AfricanPF", serialize = "africanpf")]
    AfricanPF,
    #[strum(to_string = "APA", serialize = "apa")]
    APA,
    #[strum(to_string = "APC", serialize = "apc")]
    APC,
    #[strum(to_string = "APF", serialize = "apf")]
    APF,
    #[strum(to_string = "AsianPF", serialize = "asianpf")]
    AsianPF,
    #[strum(to_string = "BB", serialize = "bb")]
    BB,
    #[strum(to_string = "BPU", serialize = "bpu")]
    BPU,
    #[strum(to_string = "BP", serialize = "bp")]
    BP,
    #[strum(to_string = "CAPO", serialize = "capo")]
    CAPO,
    #[strum(to_string = "CommonwealthPF", serialize = "commonwealthpf")]
    CommonwealthPF,
    #[strum(to_string = "CPF", serialize = "cpf")]
    CPF,
    #[strum(to_string = "CPL", serialize = "cpl")]
    CPL,
    #[strum(to_string = "CPU", serialize = "cpu")]
    CPU,
    #[strum(to_string = "DSF", serialize = "dsf")]
    DSF,
    #[strum(to_string = "EPA", serialize = "epa")]
    EPA,
    #[strum(to_string = "EPF", serialize = "epf")]
    EPF,
    #[strum(to_string = "FEMEPO", serialize = "femepo")]
    FEMEPO,
    #[strum(to_string = "FESUPO", serialize = "fesupo")]
    FESUPO,
    #[strum(to_string = "FFForce", serialize = "ffforce")]
    FFForce,
    #[strum(to_string = "FPO", serialize = "fpo")]
    FPO,
    #[strum(to_string = "FPR", serialize = "fpr")]
    FPR,
    #[strum(to_string = "GPA", serialize = "gpa")]
    GPA,
    #[strum(to_string = "GPC", serialize = "gpc")]
    GPC,
    #[serde(rename = "GPC-AUS")]
    #[strum(to_string = "GPC-AUS", serialize = "gpc-aus")]
    GPCAUS,
    #[serde(rename = "GPC-GB")]
    #[strum(to_string = "GPC-GB", serialize = "gpc-gb")]
    GPCGB,
    #[serde(rename = "GPC-NZ")]
    #[strum(to_string = "GPC-NZ", serialize = "gpc-nz")]
    GPCNZ,
    #[strum(to_string = "HERC", serialize = "herc")]
    HERC,
    #[strum(to_string = "IDFPF", serialize = "idfpf")]
    IDFPF,
    #[strum(to_string = "IPA", serialize = "ipa")]
    IPA,
    #[strum(to_string = "IPF", serialize = "ipf")]
    IPF,
    #[strum(to_string = "IPL", serialize = "ipl")]
    IPL,
    #[strum(to_string = "IrishPF", serialize = "irishpf")]
    IrishPF,
    #[strum(to_string = "IrishPO", serialize = "irishpo")]
    IrishPO,
    #[strum(to_string = "KRAFT", serialize = "kraft")]
    KRAFT,
    #[strum(to_string = "MHP", serialize = "mhp")]
    MHP,
    #[strum(to_string = "MM", serialize = "mm")]
    MM,
    #[strum(to_string = "NAPF", serialize = "napf")]
    NAPF,
    #[strum(to_string = "NASA", serialize = "nasa")]
    NASA,
    #[strum(to_string = "NIPF", serialize = "nipf")]
    NIPF,
    #[strum(to_string = "NPA", serialize = "npa")]
    NPA,
    #[strum(to_string = "NSF", serialize = "nsf")]
    NSF,
    #[strum(to_string = "NZPF", serialize = "nzpf")]
    NZPF,
    #[strum(to_string = "OceaniaPF", serialize = "oceaniapf")]
    OceaniaPF,
    #[strum(to_string = "ParaPL", serialize = "parapl")]
    ParaPL,
    #[strum(to_string = "PA", serialize = "pa")]
    PA,
    #[strum(to_string = "PLZS", serialize = "plzs")]
    PLZS,
    #[strum(to_string = "ProRaw", serialize = "proraw")]
    ProRaw,
    #[strum(to_string = "PZKFiTS", serialize = "pzkfits")]
    PZKFiTS,
    #[strum(to_string = "RAW", serialize = "raw")]
    RAW,
    #[strum(to_string = "RAWU", serialize = "rawu")]
    RAWU,
    #[strum(to_string = "RPS", serialize = "rps")]
    RPS,
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
    #[strum(to_string = "SVNL", serialize = "svnl")]
    SSF,
    #[strum(to_string = "SSF", serialize = "ssf")]    
    SVNL,
    #[strum(to_string = "THSPA", serialize = "thspa")]
    THSPA,
    #[strum(to_string = "UPA", serialize = "upa")]
    UPA,
    #[strum(to_string = "USAPL", serialize = "usapl")]
    USAPL,
    #[strum(to_string = "USPF", serialize = "uspf")]
    USPF,
    #[strum(to_string = "USPA", serialize = "uspa")]
    USPA,
    #[strum(to_string = "WABDL", serialize = "wabdl")]
    WABDL,
    #[strum(to_string = "WDFPF", serialize = "wdfpf")]
    WDFPF,
    #[strum(to_string = "WelshPA", serialize = "welshpa")]
    WelshPA,
    #[strum(to_string = "WPA", serialize = "wpa")]
    WPA,
    #[strum(to_string = "WPAU", serialize = "wpau")]
    WPAU,
    #[strum(to_string = "WPC", serialize = "wpc")]
    WPC,
    #[serde(rename = "WPC-RUS")]
    #[strum(to_string = "WPC-RUS", serialize = "wpc-rus")]
    WPCRUS,
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
        assert_eq!(
            "wrpf".parse::<Federation>().unwrap(),
            Federation::WRPF
        );

        // The default to_string() should be the upper-case form.
        assert_eq!(Federation::WRPF.to_string(), "WRPF");
    }
}
