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
    /// 365 Strong Powerlifting Federation.
    #[serde(rename = "365Strong")]
    #[strum(to_string = "365Strong", serialize = "365strong")]
    _365Strong,

    /// Alianza Argentina Powerlifting, GPA/IPO.
    #[strum(to_string = "AAP", serialize = "aap")]
    AAP,

    /// Australian Amateur Powerlifting Federation, defunct IPF affiliate.
    #[strum(to_string = "AAPLF", serialize = "aaplf")]
    AAPLF,

    /// Amateur Athletic Union.
    #[strum(to_string = "AAU", serialize = "aau")]
    AAU,

    /// Anti-Drug Athletes United.
    #[strum(to_string = "ADAU", serialize = "adau")]
    ADAU,

    /// American Drug-Free Powerlifting Association, predecessor of USAPL.
    #[strum(to_string = "ADFPA", serialize = "adfpa")]
    ADFPA,

    /// American Drug-Free Powerlifting Federation, WDFPF.
    #[strum(to_string = "ADFPF", serialize = "adfpf")]
    ADFPF,

    /// Asociación Española de Powerlifting, IPF. 
    #[strum(to_string = "AEP", serialize = "aep")]
    AEP,

    /// African Powerlifting Federation, IPF.
    #[strum(to_string = "AfricanPF", serialize = "africanpf")]
    AfricanPF,

    /// All Indonesia Weightlifting, Bodybuilding and Powerlifting
    /// Association, IPF.
    #[strum(to_string = "AIWBPA", serialize = "aiwbpa")]
    AIWBPA,

    /// American Powerlifting Association, WPA.
    #[strum(to_string = "APA", serialize = "apa")]
    APA,

    /// American Powerlifting Committee.
    #[strum(to_string = "APC", serialize = "apc")]
    APC,

    /// American Powerlifting Federation.
    #[strum(to_string = "APF", serialize = "apf")]
    APF,

    /// Australian Powerlifting Union.
    #[strum(to_string = "APU", serialize = "apu")]
    APU,

    /// Asian Powerlifting Federation, IPF.
    #[strum(to_string = "AsianPF", serialize = "asianpf")]
    AsianPF,

    /// Australian Drug-Free Powerlifting Federation, WDFPF.
    #[strum(to_string = "AusDFPF", serialize = "ausdfpf")]
    AusDFPF,

    /// British Amateur Weightlifting Association, predecessor to BP.
    #[strum(to_string = "BAWLA", serialize = "bawla")]
    BAWLA,

    /// Bogatyr Brotherhood, a stand-alone and short-lived Russian federation.
    #[strum(to_string = "BB", serialize = "bb")]
    BB,

    /// Baddest Bench, Deadliest Deadlift. Yearly meets run by John Inzer.
    #[strum(to_string = "BBDD", serialize = "bbdd")]
    BBDD,

    /// British Drug-Free Powerlifting Assocation.
    #[strum(to_string = "BDFPA", serialize = "bdfpa")]
    BDFPA,

    /// Defunct British WPC affiliate.
    #[strum(to_string = "BPC", serialize = "bpc")]
    BPC,

    /// British Powerlifting Organization, WPF.
    #[strum(to_string = "BPO", serialize = "bpo")]
    BPO,

    /// British Powerlifting Union.
    #[strum(to_string = "BPU", serialize = "bpu")]
    BPU,

    /// British Powerlifting, IPF. Formerly named GBPF.
    #[strum(to_string = "BP", serialize = "bp")]
    BP,

    /// Bundesverband Deutscher Kraftdreikämpf, IPF.
    #[strum(to_string = "BVDK", serialize = "bvdk")]
    BVDK,

    /// Australian WPC/GPA affiliate.
    #[strum(to_string = "CAPO", serialize = "capo")]
    CAPO,

    /// Česká Asociace Silového Trojboje, GPC/WPC.
    #[strum(to_string = "CAST", serialize = "cast")]
    CAST,

    /// Commonwealth Powerlifting Federation, IPF.
    #[strum(to_string = "CommonwealthPF", serialize = "commonwealthpf")]
    CommonwealthPF,

    /// Canadian Powerlifting Federation, WPF.
    #[strum(to_string = "CPF", serialize = "cpf")]
    CPF,

    /// Canadian Powerlifting League, IPL.
    #[strum(to_string = "CPL", serialize = "cpl")]
    CPL,

    /// Canadian Powerlifting Organization, defunct WPC affiliate.
    #[strum(to_string = "CPO", serialize = "cpo")]
    CPO,

    /// Canadian Powerlifting Union, IPF.
    #[strum(to_string = "CPU", serialize = "cpu")]
    CPU,

    /// Český svaz silového trojboje, Czech IPF affiliate.
    #[strum(to_string = "CSST", serialize = "csst")]
    CSST,

    /// Danish IPF affiliate.
    #[strum(to_string = "DSF", serialize = "dsf")]
    DSF,

    /// English Powerlifting Association, IPF.
    #[strum(to_string = "EPA", serialize = "epa")]
    EPA,

    #[strum(to_string = "EPF", serialize = "epf")]
    EPF,

    /// Federación Argentina de Levantamiento de Potencia, IPF.
    #[strum(to_string = "FALPO", serialize = "falpo")]
    FALPO,

    /// Federación Mexicana de Powerlifting A.C., IPF.
    #[strum(to_string = "FEMEPO", serialize = "femepo")]
    FEMEPO,

    /// Federación de Powerlifting Argentino, GPC.
    #[strum(to_string = "FEPOA", serialize = "fepoa")]
    FEPOA,

    /// Federación Sudamericana de Powerlifting, IPF.
    #[strum(to_string = "FESUPO", serialize = "fesupo")]
    FESUPO,

    /// Federation Francaise de Force, IPF.
    #[strum(to_string = "FFForce", serialize = "ffforce")]
    FFForce,

    /// Finland Powerlifting Organization, IPA.
    #[strum(to_string = "FPO", serialize = "fpo")]
    FPO,

    /// Powerlifting Federation of Russia, IPF.
    #[strum(to_string = "FPR", serialize = "fpr")]
    FPR,

    /// Russian stand-alone competition.
    #[strum(to_string = "GoldenDouble", serialize = "goldendouble")]
    GoldenDouble,

    /// Global Powerlifting Association.
    #[strum(to_string = "GPA", serialize = "gpa")]
    GPA,

    /// Global Powerlifting Committee.
    #[strum(to_string = "GPC", serialize = "gpc")]
    GPC,

    /// Australian branch of the GPC.
    #[serde(rename = "GPC-AUS")]
    #[strum(to_string = "GPC-AUS", serialize = "gpc-aus")]
    GPCAUS,

    /// Canadian branch of the GPC.
    #[serde(rename = "GPC-CAN")]
    #[strum(to_string = "GPC-CAN", serialize = "gpc-can")]
    GPCCAN,

    /// British branch of the GPC.
    #[serde(rename = "GPC-GB")]
    #[strum(to_string = "GPC-GB", serialize = "gpc-gb")]
    GPCGB,

    /// Irish branch of the GPC.
    #[serde(rename = "GPC-IRL")]
    #[strum(to_string = "GPC-IRL", serialize = "gpc-irl")]
    GPCIRL,

    /// New Zealand branch of the GPC.
    #[serde(rename = "GPC-NZ")]
    #[strum(to_string = "GPC-NZ", serialize = "gpc-nz")]
    GPCNZ,

    /// Russian branch of the GPC.
    #[serde(rename = "GPC-RUS")]
    #[strum(to_string = "GPC-RUS", serialize = "gpc-rus")]
    GPCRUS,

    /// German Powerlifting Union, WPU.
    #[strum(to_string = "GPU", serialize = "gpu")]
    GPU,

    /// Defunct stand-alone US federation.
    #[strum(to_string = "Hardcore", serialize = "hardcore")]
    Hardcore,

    /// Hercules Gym in Syracuse, NY. Run by Rheta West.
    #[strum(to_string = "HERC", serialize = "herc")]
    HERC,

    /// Croatian IPF affiliate
    #[strum(to_string = "HPLS", serialize = "hpls")]
    HPLS,

    /// International Blind Sport Assocation.
    #[strum(to_string = "IBSA", serialize = "ibsa")]
    IBSA,

    /// Irish Drug-Free Powerlifting Association.
    #[strum(to_string = "IDFPA", serialize = "idfpa")]
    IDFPA,

    /// Irish Drug-Free Powerlifting Federation.
    #[strum(to_string = "IDFPF", serialize = "idfpf")]
    IDFPF,

    #[strum(to_string = "IPA", serialize = "ipa")]
    IPA,

    /// Israel Powerlifting Community.
    #[strum(to_string = "IPC", serialize = "ipc")]
    IPC,

    /// International Powerlifting Federation.
    #[strum(to_string = "IPF", serialize = "ipf")]
    IPF,

    /// International Powerlifting League.
    #[strum(to_string = "IPL", serialize = "ipl")]
    IPL,

    /// Irish Powerlifting Federation, IPF.
    #[strum(to_string = "IrishPF", serialize = "irishpf")]
    IrishPF,

    /// Irish Powerlifting Organization, WPU/IPL.
    #[strum(to_string = "IrishPO", serialize = "irishpo")]
    IrishPO,

    /// Japan Powerlifting Federation, IPF.
    #[strum(to_string = "JPA", serialize = "jpa")]
    JPA,

    /// Icelandic IPF affiliate.
    #[strum(to_string = "KRAFT", serialize = "kraft")]
    KRAFT,

    /// Latvian IPF affiliate.
    #[strum(to_string = "LPF", serialize = "lpf")]
    LPF,

    /// Maximum Human Performance, a vitamin company.
    #[strum(to_string = "MHP", serialize = "mhp")]
    MHP,

    /// Metal Militia, a small, independent federation.
    #[strum(to_string = "MM", serialize = "mm")]
    MM,

    /// Malaysian Powerlifting Alliance.
    #[strum(to_string = "MPA", serialize = "mpa")]
    MPA,

    /// National Association of Powerlifting Russia, IPA.
    #[strum(to_string = "NAP", serialize = "nap")]
    NAP,

    /// North American Powerlifting Federation, IPF.
    #[strum(to_string = "NAPF", serialize = "napf")]
    NAPF,

    /// Natural Athlete Strength Assocation.
    #[strum(to_string = "NASA", serialize = "nasa")]
    NASA,

    /// Northern Ireland Powerlifting Federation.
    #[strum(to_string = "NIPF", serialize = "nipf")]
    NIPF,

    /// Nordic Powerlifting Federation, IPF.
    #[strum(to_string = "NordicPF", serialize = "nordicpf")]
    NordicPF,

    /// Night of the Living Deadlift.
    #[strum(to_string = "NOTLD", serialize = "notld")]
    NOTLD,

    /// National Powerlifting Association of Israel.
    #[strum(to_string = "NPA", serialize = "npa")]
    NPA,

    /// Norwegian IPF affiliate.
    #[strum(to_string = "NSF", serialize = "nsf")]
    NSF,

    /// New Zealand Powerlifting Federation, IPF.
    #[strum(to_string = "NZPF", serialize = "nzpf")]
    NZPF,

    /// Oceania Powerlifting Federation, IPF.
    #[strum(to_string = "OceaniaPF", serialize = "oceaniapf")]
    OceaniaPF,

    /// Olomoucký Silák, a yearly bench competition in Czechia.
    #[strum(to_string = "OlomouckySilak", serialize = "olomouckysilak")]
    OlomouckySilak,

    /// Paralympic Powerlifting.
    #[strum(to_string = "ParaPL", serialize = "parapl")]
    ParaPL,

    /// Powerlifting Australia, formerly IPF, now WP.
    #[strum(to_string = "PA", serialize = "pa")]
    PA,

    /// Powerlifting zveza Slovenije, IPF.
    #[strum(to_string = "PLZS", serialize = "plzs")]
    PLZS,

    /// A defunct stand-alone US federation.
    #[strum(to_string = "PRIDE", serialize = "pride")]
    PRIDE,

    /// Australian stand-alone meets run by Markos Markopoulos.
    #[strum(to_string = "ProRaw", serialize = "proraw")]
    ProRaw,

    /// Professional Raw Powerlifting Assocation.
    #[strum(to_string = "PRPA", serialize = "prpa")]
    PRPA,

    /// Polish IPF affiliate.
    #[strum(to_string = "PZKFiTS", serialize = "pzkfits")]
    PZKFiTS,

    /// 100% RAW Federation, WP.
    #[strum(to_string = "RAW", serialize = "raw")]
    RAW,

    /// 100% RAW Federation Canada.
    #[serde(rename = "RAW-CAN")]
    #[strum(to_string = "RAW-CAN", serialize = "raw-can")]
    RAWCAN,

    /// 100% RAW Federation Ukraine.
    #[serde(rename = "RAW-UKR")]
    #[strum(to_string = "RAW-UKR", serialize = "raw-ukr")]
    RAWUKR,

    /// Raw United Federation
    #[strum(to_string = "RAWU", serialize = "rawu")]
    RAWU,

    /// Revolution Powerlifting Syndicate.
    #[strum(to_string = "RPS", serialize = "rps")]
    RPS,

    /// Russian Powerlifting Union.
    #[strum(to_string = "RPU", serialize = "rpu")]
    RPU,

    /// Raw Unity.
    #[strum(to_string = "RUPC", serialize = "rupc")]
    RUPC,

    /// Scottish Powerlifting, IPF.
    #[strum(to_string = "ScottishPL", serialize = "scottishpl")]
    ScottishPL,

    /// Old US based federation, no idea what this stands for.
    #[strum(to_string = "SCI", serialize = "sci")]
    SCI,

    /// Super-Cup of Titans, a defunct Russian single-ply meet.
    #[strum(to_string = "SCT", serialize = "sct")]
    SCT,

    /// Singapore Powerlifting Alliance.
    #[strum(to_string = "SPA", serialize = "spa")]
    SPA,

    /// Southern Powerlifting Federation.
    #[strum(to_string = "SPF", serialize = "spf")]
    SPF,

    /// Societatem Potentis Species Sports, a defunct Russian raw federation.
    #[strum(to_string = "SPSS", serialize = "spss")]
    SPSS,

    /// Swedish IPF affiliate.
    #[strum(to_string = "SSF", serialize = "ssf")]
    SSF,

    /// Finnish IPF affiliate.
    #[strum(to_string = "SVNL", serialize = "svnl")]
    SVNL,

    /// Texas High School Powerlifting Association.
    #[strum(to_string = "THSPA", serialize = "thspa")]
    THSPA,

    /// Ukraine Powerlifting Association.
    #[strum(to_string = "UkrainePA", serialize = "ukrainepa")]
    UkrainePA,

    /// United Powerlifting Association.
    #[strum(to_string = "UPA", serialize = "upa")]
    UPA,

    /// Ukrainian Powerlifting Committee.
    #[strum(to_string = "UPC", serialize = "upc")]
    UPC,

    /// Ukrainian Powerlifting Federation, IPF.
    #[strum(to_string = "UkrainePF", serialize = "ukrainepf")]
    UkrainePF,

    /// USA Powerlifting, IPF.
    #[strum(to_string = "USAPL", serialize = "usapl")]
    USAPL,

    /// US Powerlifting Federation.
    #[strum(to_string = "USPF", serialize = "uspf")]
    USPF,

    /// United States Powerlifting Assocation, IPL.
    #[strum(to_string = "USPA", serialize = "uspa")]
    USPA,

    /// Vietnam Powerlifting Alliance, GPA.
    #[strum(to_string = "VietnamPA", serialize = "vietnampa")]
    VietnamPA,

    #[strum(to_string = "Vityaz", serialize = "vityaz")]
    Vityaz,

    /// World Association of Bench Pressers and Deadlifters.
    #[strum(to_string = "WABDL", serialize = "wabdl")]
    WABDL,

    /// World Drug-Free Powerlifting Association.
    #[strum(to_string = "WDFPF", serialize = "wdfpf")]
    WDFPF,

    /// Welsh Powerlifting Association, IPF.
    #[strum(to_string = "WelshPA", serialize = "welshpa")]
    WelshPA,

    /// World Powerlifting, Robert Wilks' federation.
    #[strum(to_string = "WP", serialize = "wp")]
    WP,

    /// World Powerlifting Alliance.
    #[strum(to_string = "WPA", serialize = "wpa")]
    WPA,

    /// World Powerlifting Alliance Russia.
    #[serde(rename = "WPA-RUS")]
    #[strum(to_string = "WPA-RUS", serialize = "wpa-rus")]
    WPARUS,

    /// World Powerlifting Alliance Ukraine.
    #[strum(to_string = "WPAU", serialize = "wpau")]
    WPAU,

    /// World Powerlifting Committee.
    #[strum(to_string = "WPC", serialize = "wpc")]
    WPC,

    /// German WPC affiliate.
    #[serde(rename = "WPC-Germany")]
    #[strum(to_string = "WPC-Germany", serialize = "wpc-germany")]
    WPCGermany,

    /// Portuguese WPC affiliate.
    #[serde(rename = "WPC-Portugal")]
    #[strum(to_string = "WPC-Portugal", serialize = "wpc-portugal")]
    WPCPortugal,

    /// Russian WPC affiliate.
    #[serde(rename = "WPC-RUS")]
    #[strum(to_string = "WPC-RUS", serialize = "wpc-rus")]
    WPCRUS,

    /// Ukrainian WPC affiliate.
    #[serde(rename = "WPC-UKR")]
    #[strum(to_string = "WPC-UKR", serialize = "wpc-ukr")]
    WPCUKR,

    /// World Powerlifting Federation.
    #[strum(to_string = "WPF", serialize = "wpf")]
    WPF,

    /// World Powerlifting Union.
    #[strum(to_string = "WPU", serialize = "wpu")]
    WPU,

    /// World Powerlifting Union of Federations.
    #[strum(to_string = "WPUF", serialize = "wpuf")]
    WPUF,

    /// World Natural Powerlifting Federation.
    #[strum(to_string = "WNPF", serialize = "wnpf")]
    WNPF,

    /// World Raw Powerlifting Federation.
    #[strum(to_string = "WRPF", serialize = "wrpf")]
    WRPF,

    /// Australian WRPF affiliate.
    #[serde(rename = "WRPF-AUS")]
    #[strum(to_string = "WRPF-AUS", serialize = "wrpf-aus")]
    WRPFAUS,

    /// Canadian WRPF affiliate.
    #[serde(rename = "WRPF-CAN")]
    #[strum(to_string = "WRPF-CAN", serialize = "wrpf-can")]
    WRPFCAN,

    /// World United Amateur Powerlifting.
    #[strum(to_string = "WUAP", serialize = "wuap")]
    WUAP,

    /// Xtreme Powerlifting Coalition.
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
