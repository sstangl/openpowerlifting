//! Defines the `Federation` field for the `meets` table.

use opldb::{Entry, Meet};
use opldb::fields::Country;

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
#[derive(Copy, Clone, Debug, Deserialize, Display, PartialEq, PartialOrd, Ord, Eq,
         Serialize, EnumIter, EnumString)]
pub enum Federation {
    #[serde(rename = "365Strong")]
    #[strum(to_string = "365Strong", serialize = "365strong")]
    _365Strong,
    #[strum(to_string = "AAP", serialize = "aap")]
    AAP,
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
    APA,
    #[strum(to_string = "APC", serialize = "apc")]
    APC,
    #[strum(to_string = "APF", serialize = "apf")]
    APF,
    #[strum(to_string = "APU", serialize = "apu")]
    APU,
    #[strum(to_string = "AsianPF", serialize = "asianpf")]
    AsianPF,
    #[strum(to_string = "BB", serialize = "bb")]
    BB,
    #[strum(to_string = "BPC", serialize = "bpc")]
    BPC,
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
    #[strum(to_string = "PZKFiTS", serialize = "pzkfits")]
    PZKFiTS,
    #[strum(to_string = "RAW", serialize = "raw")]
    RAW,
    #[serde(rename = "RAW-CAN")]
    #[strum(to_string = "RAW-CAN", serialize = "raw-can")]
    RAWCAN,
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
    #[strum(to_string = "UPA", serialize = "upa")]
    UPA,
    #[strum(to_string = "USAPL", serialize = "usapl")]
    USAPL,
    #[strum(to_string = "USPF", serialize = "uspf")]
    USPF,
    #[strum(to_string = "USPA", serialize = "uspa")]
    USPA,
    #[strum(to_string = "VietnamPA", serialize = "vietnampa")]
    VietnamPA,
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
    #[serde(rename = "WPC-Portugal")]
    #[strum(to_string = "WPC-Portugal", serialize = "wpc-portugal")]
    WPCPortugal,
    #[serde(rename = "WPC-RUS")]
    #[strum(to_string = "WPC-RUS", serialize = "wpc-rus")]
    WPCRUS,
    #[strum(to_string = "WPF", serialize = "wpf")]
    WPF,
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

/// Enum of Meta-Federations. These are the entries in the federation selector
/// that don't correspond neatly to just a single federation value.
///
/// Definition of each Meta-Federation is in the `contains` function.
#[derive(Copy, Clone, Debug, Deserialize, Display, PartialEq, Serialize, EnumIter,
         EnumString)]
pub enum MetaFederation {
    /// Federations that are exclusively (non-optionally) tested.
    #[strum(to_string = "all-tested")]
    AllTested,
    /// All entries that have "Tested = Yes".
    #[strum(to_string = "all-amateur")]
    AllAmateur,
    #[strum(to_string = "all-argentina")]
    AllArgentina,
    #[strum(to_string = "all-australia")]
    AllAustralia,
    #[strum(to_string = "all-canada")]
    AllCanada,
    #[strum(to_string = "all-czechia")]
    AllCzechia,
    #[strum(to_string = "all-finland")]
    AllFinland,
    #[strum(to_string = "all-germany")]
    AllGermany,
    #[strum(to_string = "all-ireland")]
    AllIreland,
    #[strum(to_string = "all-israel")]
    AllIsrael,
    #[strum(to_string = "all-russia")]
    AllRussia,
    #[strum(to_string = "all-uk")]
    AllUK,
    #[strum(to_string = "all-ukraine")]
    AllUkraine,
    #[strum(to_string = "all-usa")]
    AllUSA,

    #[strum(to_string = "aapf")]
    AAPF,
    #[strum(to_string = "abpu")]
    ABPU,
    /// The BP federation is made up of smaller divisional federations,
    /// but people expect to see them all lumped together.
    #[strum(to_string = "all-bp")]
    AllBP,
    /// IPA, but only counting meets held in Canada.
    #[strum(to_string = "ipa-can")]
    IPACAN,
    #[strum(to_string = "uspa-tested")]
    USPATested,
}

impl MetaFederation {
    /// Defines whether a given `Entry` is part of the MetaFederation.
    ///
    /// Matching is done on Entries instead of on Meets since a MetaFederation
    /// can include Entry-specific information such as Tested status.
    pub fn contains(self, entry: &Entry, meets: &Vec<Meet>) -> bool {
        let meet: &Meet = &meets[entry.meet_id as usize];

        match self {
            MetaFederation::AllTested => {
                meet.federation == Federation::AAU
                    || meet.federation == Federation::ADAU
                    || meet.federation == Federation::ADFPA
                    || meet.federation == Federation::ADFPF
                    || meet.federation == Federation::AEP
                    || meet.federation == Federation::AfricanPF
                    || meet.federation == Federation::APU
                    || meet.federation == Federation::AsianPF
                    || meet.federation == Federation::BP
                    || meet.federation == Federation::BVDK
                    || meet.federation == Federation::CommonwealthPF
                    || meet.federation == Federation::CPU
                    || meet.federation == Federation::CSST
                    || meet.federation == Federation::DSF
                    || meet.federation == Federation::EPA
                    || meet.federation == Federation::EPF
                    || meet.federation == Federation::FALPO
                    || meet.federation == Federation::FEMEPO
                    || meet.federation == Federation::FESUPO
                    || meet.federation == Federation::FFForce
                    || meet.federation == Federation::FPR
                    || meet.federation == Federation::IDFPA
                    || meet.federation == Federation::IDFPF
                    || meet.federation == Federation::IPF
                    || meet.federation == Federation::IrishPF
                    || meet.federation == Federation::JPA
                    || meet.federation == Federation::KRAFT
                    || meet.federation == Federation::LPF
                    || meet.federation == Federation::NAPF
                    || meet.federation == Federation::NASA
                    || meet.federation == Federation::NIPF
                    || meet.federation == Federation::NordicPF
                    || meet.federation == Federation::NSF
                    || meet.federation == Federation::NZPF
                    || meet.federation == Federation::OceaniaPF
                    || meet.federation == Federation::ParaPL
                    || meet.federation == Federation::PA
                    || meet.federation == Federation::PLZS
                    || meet.federation == Federation::PZKFiTS
                    || meet.federation == Federation::RAW
                    || meet.federation == Federation::RAWCAN
                    || meet.federation == Federation::ScottishPL
                    || meet.federation == Federation::SSF
                    || meet.federation == Federation::SVNL
                    || meet.federation == Federation::THSPA
                    || meet.federation == Federation::USAPL
                    || meet.federation == Federation::WABDL
                    || meet.federation == Federation::WDFPF
                    || meet.federation == Federation::WelshPA
                    || meet.federation == Federation::WNPF
            }
            MetaFederation::AllAmateur => entry.tested,
            MetaFederation::AllArgentina => {
                meet.federation == Federation::AAP
                    || meet.federation == Federation::FALPO
                    || meet.federation == Federation::FEPOA
            }
            MetaFederation::AllAustralia => {
                meet.federation == Federation::APU
                    || meet.federation == Federation::CAPO
                    || meet.federation == Federation::GPCAUS
                    || meet.federation == Federation::PA
                    || meet.federation == Federation::ProRaw
            }
            MetaFederation::AllCanada => {
                meet.federation == Federation::CPF
                    || meet.federation == Federation::CPL
                    || meet.federation == Federation::CPU
                    || meet.federation == Federation::GPCCAN
                    || MetaFederation::IPACAN.contains(entry, meets)
                    || meet.federation == Federation::RAWCAN
                    || meet.federation == Federation::WRPFCAN
            }
            MetaFederation::AllCzechia => {
                meet.federation == Federation::CAST
                    || meet.federation == Federation::CSST
                    || meet.federation == Federation::OlomouckySilak
            }
            MetaFederation::AllFinland => {
                meet.federation == Federation::FPO || meet.federation == Federation::SVNL
            }
            MetaFederation::AllGermany => {
                meet.federation == Federation::BVDK || meet.federation == Federation::GPU
            }
            MetaFederation::AllIreland => {
                meet.federation == Federation::GPCIRL
                    || meet.federation == Federation::IDFPA
                    || meet.federation == Federation::IDFPF
                    || meet.federation == Federation::IrishPF
                    || meet.federation == Federation::IrishPO
            }
            MetaFederation::AllIsrael => {
                meet.federation == Federation::IPC || meet.federation == Federation::NPA
            }
            MetaFederation::AllRussia => {
                meet.federation == Federation::BB
                    || meet.federation == Federation::FPR
                    || meet.federation == Federation::GPCRUS
                    || meet.federation == Federation::NAP
                    || meet.federation == Federation::RPU
                    || meet.federation == Federation::SCT
                    || meet.federation == Federation::SPSS
                    || meet.federation == Federation::WPCRUS
                    || meet.federation == Federation::WRPF
            }
            MetaFederation::AllUK => {
                meet.federation == Federation::BP
                    || meet.federation == Federation::BPC
                    || meet.federation == Federation::BPU
                    || meet.federation == Federation::EPA
                    || meet.federation == Federation::GPCGB
                    || meet.federation == Federation::NIPF
                    || meet.federation == Federation::ScottishPL
                    || meet.federation == Federation::WelshPA
            }
            MetaFederation::AllUkraine => {
                meet.federation == Federation::WPAU || meet.federation == Federation::WPUF
            }
            MetaFederation::AllUSA => {
                meet.federation == Federation::_365Strong
                    || meet.federation == Federation::AAU
                    || meet.federation == Federation::ADAU
                    || meet.federation == Federation::ADFPA
                    || meet.federation == Federation::ADFPF
                    || meet.federation == Federation::APA
                    || meet.federation == Federation::APC
                    || meet.federation == Federation::APF
                    || meet.federation == Federation::Hardcore
                    || meet.federation == Federation::HERC
                    || meet.federation == Federation::IPA
                    || meet.federation == Federation::PRIDE
                    || meet.federation == Federation::MHP
                    || meet.federation == Federation::MM
                    || meet.federation == Federation::NASA
                    || meet.federation == Federation::NOTLD
                    || meet.federation == Federation::RAW
                    || meet.federation == Federation::RAWU
                    || meet.federation == Federation::RPS
                    || meet.federation == Federation::RUPC
                    || meet.federation == Federation::SPF
                    || meet.federation == Federation::THSPA
                    || meet.federation == Federation::UPA
                    || meet.federation == Federation::USAPL
                    || meet.federation == Federation::USPA
                    || meet.federation == Federation::USPF
                    || meet.federation == Federation::XPC
                    || meet.federation == Federation::WNPF
            }
            MetaFederation::AAPF => meet.federation == Federation::APF && entry.tested,
            MetaFederation::ABPU => meet.federation == Federation::BPU && entry.tested,
            MetaFederation::AllBP => {
                meet.federation == Federation::BP
                    || meet.federation == Federation::EPA
                    || meet.federation == Federation::NIPF
                    || meet.federation == Federation::ScottishPL
                    || meet.federation == Federation::WelshPA
            }
            MetaFederation::IPACAN => {
                meet.federation == Federation::IPA && meet.country == Country::Canada
            }
            MetaFederation::USPATested => {
                meet.federation == Federation::USPA && entry.tested
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_url_strings() {
        // The lowercase form should parse.
        assert_eq!("wrpf".parse::<Federation>().unwrap(), Federation::WRPF);

        // The default to_string() should be the upper-case form.
        assert_eq!(Federation::WRPF.to_string(), "WRPF");
    }

    /// The Federation and MetaFederation enums must serialize
    /// to unique strings.
    #[test]
    fn test_fed_metafed_conflicts() {
        let v1: Vec<String> = Federation::iter().map(|f| f.to_string()).collect();
        let v2: Vec<String> = MetaFederation::iter().map(|f| f.to_string()).collect();
        for v in v1 {
            assert!(!v2.contains(&v));
        }
    }
}
