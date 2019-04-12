//! Defines the `Federation` field for the `meets` table.

use crate::Country;
use crate::Date;
use crate::PointsSystem;

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

    /// Alianza Chilena Powerlifting, GPA/IPO.
    #[strum(to_string = "ACHIPO", serialize = "achipo")]
    ACHIPO,

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

    /// American Frantz Powerlifting Federation
    #[strum(to_string = "AFPF", serialize = "afpf")]
    AFPF,

    /// African Powerlifting Federation, IPF.
    #[strum(to_string = "AfricanPF", serialize = "africanpf")]
    AfricanPF,

    /// All Indonesia Weightlifting, Bodybuilding and Powerlifting
    /// Association, IPF.
    #[strum(to_string = "AIWBPA", serialize = "aiwbpa")]
    AIWBPA,

    /// American Strength Association, an unaffiliated local federation
    /// created to avoid membership fees for local competitions.
    #[strum(to_string = "AmericanSA", serialize = "americansa")]
    AmericanSA,

    /// All Natural Physique and Power Conference (Defunct).
    #[strum(to_string = "ANPPC", serialize = "anppc")]
    ANPPC,

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

    /// Australian Powerlifting Federation, IPF.
    /// PA precursor
    #[strum(to_string = "AusPF", serialize = "auspf")]
    AusPF,

    /// Australian Powerlifting League, IPL.
    #[strum(to_string = "AusPL", serialize = "auspl")]
    AusPL,

    /// Australian Weightlifting Federation, meets pre AAPLF.
    #[strum(to_string = "AWF", serialize = "awf")]
    AWF,

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

    // Belarus Powerlifting Federation, IPF.
    #[strum(to_string = "BelPF", serialize = "belpf")]
    BelPF,

    /// Bench America, short lived Bench invitiational meet.
    #[strum(to_string = "BenchAmerica", serialize = "benchamerica")]
    BenchAmerica,

    /// British Powerlifting, IPF. Formerly named GBPF.
    #[strum(to_string = "BP", serialize = "bp")]
    BP,

    /// Defunct British WPC affiliate.
    #[strum(to_string = "BPC", serialize = "bpc")]
    BPC,

    /// British Powerlifting Federation, WPU/WRPF.
    #[strum(to_string = "BPF", serialize = "bpf")]
    BPF,

    /// British Powerlifting Organization, WPF.
    #[strum(to_string = "BPO", serialize = "bpo")]
    BPO,

    /// British Powerlifting Union.
    #[strum(to_string = "BPU", serialize = "bpu")]
    BPU,

    /// Bundesverbandes Deutscher Gewichtheber, pre-BVDK.
    #[strum(to_string = "BVDG", serialize = "bvdg")]
    BVDG,

    /// Bundesverband Deutscher Kraftdreikämpf, IPF.
    #[strum(to_string = "BVDK", serialize = "bvdk")]
    BVDK,

    /// Australian WPC/GPA affiliate.
    #[strum(to_string = "CAPO", serialize = "capo")]
    CAPO,

    /// Shortlived NZ branch of CAPO.
    #[serde(rename = "CAPO-NZ")]
    #[strum(to_string = "CAPO-NZ", serialize = "capo-nz")]
    CAPONZ,

    /// Česká Asociace Silového Trojboje, GPC/WPC.
    #[strum(to_string = "CAST", serialize = "cast")]
    CAST,

    /// Unaffiliated meets held in Czechia.
    #[serde(rename = "Czechia-UA")]
    #[strum(to_string = "Czechia-UA", serialize = "chzechia-ua")]
    CzechiaUA,

    /// Chinese Powerlifting Association, GPA.
    #[strum(to_string = "ChinaPA", serialize = "chinapa")]
    ChinaPA,

    /// Commonwealth Powerlifting Federation, IPF.
    #[strum(to_string = "CommonwealthPF", serialize = "commonwealthpf")]
    CommonwealthPF,

    /// Canadian Powerlifting Association, WPA.
    #[strum(to_string = "CPA", serialize = "cpa")]
    CPA,

    /// Canadian Powerlifting Congress, WPC.
    #[strum(to_string = "CPC", serialize = "cpc")]
    CPC,

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

    /// Crimean Republic Powerlifting and Extreme Bench Press Association
    #[strum(to_string = "CRPEBA", serialize = "crpeba")]
    CRPEBA,

    /// Unaffiliated meets held in Croatia.
    #[serde(rename = "Croatia-UA")]
    #[strum(to_string = "Croatia-UA", serialize = "croatia-ua")]
    CroatiaUA,

    /// Český svaz silového trojboje, Czech IPF affiliate.
    #[strum(to_string = "CSST", serialize = "csst")]
    CSST,

    /// Deutscher Bodybuilding und Kraftsport Verband, first German federation.
    #[strum(to_string = "DBKV", serialize = "dbkv")]
    DBKV,

    /// Danish IPF affiliate.
    #[strum(to_string = "DSF", serialize = "dsf")]
    DSF,

    /// English Powerlifting Association, IPF.
    #[strum(to_string = "EPA", serialize = "epa")]
    EPA,

    /// European Powerlifting Federation, IPF.
    #[strum(to_string = "EPF", serialize = "epf")]
    EPF,

    /// Ελληνικό Σωματείο Δυναμικού Τριάθλου, multi-fed Greek affiliate.
    #[strum(to_string = "ESDT", serialize = "esdt")]
    ESDT,

    /// Federation Bench Press Double-event, Russian fed.
    #[strum(to_string = "FBPD", serialize = "fbpd")]
    FBPD,

    /// Federación Argentina de Levantamiento de Potencia, IPF.
    #[strum(to_string = "FALPO", serialize = "falpo")]
    FALPO,

    /// Fellowship of Christian Athletes, Defunct US based federation.
    #[strum(to_string = "FCA", serialize = "fca")]
    FCA,

    /// Federace českého silového trojboje, GPC.
    #[strum(to_string = "FCST", serialize = "fcst")]
    FCST,

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

    /// Florida High School Athletics Association.
    #[strum(to_string = "FHSAA", serialize = "fhsaa")]
    FHSAA,

    /// Federazione Italiana Powerlifting, IPF.
    #[strum(to_string = "FIPL", serialize = "fipl")]
    FIPL,

    /// Finland Powerlifting Organization, IPA.
    #[strum(to_string = "FPO", serialize = "fpo")]
    FPO,

    /// Powerlifting Federation of Russia, IPF.
    #[strum(to_string = "FPR", serialize = "fpr")]
    FPR,

    /// Unaffiliated meets held in Germany.
    #[serde(rename = "Germany-UA")]
    #[strum(to_string = "Germany-UA", serialize = "germany-ua")]
    GermanyUA,

    /// Global Powerlifting Union, Ukrainian GPC affiliate.
    #[strum(to_string = "GlobalPU", serialize = "globalpu")]
    GlobalPU,

    /// Global Powerlifting Association.
    #[strum(to_string = "GPA", serialize = "gpa")]
    GPA,

    /// Croatian branch of the GPA.
    #[serde(rename = "GPA-CRO")]
    #[strum(to_string = "GPA-CRO", serialize = "gpa-cro")]
    GPACRO,

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

    /// Latvian branch of the GPC.
    #[serde(rename = "GPC-LAT")]
    #[strum(to_string = "GPC-LAT", serialize = "gpc-lat")]
    GPCLAT,

    /// New Zealand branch of the GPC.
    #[serde(rename = "GPC-NZ")]
    #[strum(to_string = "GPC-NZ", serialize = "gpc-nz")]
    GPCNZ,

    /// Russian branch of the GPC.
    #[serde(rename = "GPC-RUS")]
    #[strum(to_string = "GPC-RUS", serialize = "gpc-rus")]
    GPCRUS,

    /// Global Powerlifting Federation
    #[strum(to_string = "GPF", serialize = "gpf")]
    GPF,

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

    /// Croatian Powerlifting Federation before getting affiliated with IPF
    #[serde(rename = "HPLS-UA")]
    #[strum(to_string = "HPLS-UA", serialize = "hpls-ua")]
    HPLSUA,

    /// Croatian Powerlifting Organization
    #[strum(to_string = "HPO", serialize = "hpo")]
    HPO,

    /// Hantang Powerlifting
    #[strum(to_string = "HTPL", serialize = "htpl")]
    HTPL,

    /// International Blind Sport Assocation.
    #[strum(to_string = "IBSA", serialize = "ibsa")]
    IBSA,

    /// Irish Drug-Free Powerlifting Association.
    #[strum(to_string = "IDFPA", serialize = "idfpa")]
    IDFPA,

    /// Irish Drug-Free Powerlifting Federation.
    #[strum(to_string = "IDFPF", serialize = "idfpf")]
    IDFPF,

    /// Islenska Kraftlyfingafelagid, Icelandic GPC? affiliate.
    #[strum(to_string = "IKF", serialize = "ikf")]
    IKF,

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

    /// International Powerlifting League, New Zealand
    #[serde(rename = "IPL-NZ")]
    #[strum(to_string = "IPL-NZ", serialize = "ipl-nz")]
    IPLNZ,

    /// Unaffiliated meets held in Ireland.
    #[serde(rename = "Ireland-UA")]
    #[strum(to_string = "Ireland-UA", serialize = "ireland-ua")]
    IrelandUA,

    /// Irish Powerlifting Federation, IPF.
    #[strum(to_string = "IrishPF", serialize = "irishpf")]
    IrishPF,

    /// Irish Powerlifting Organization, WPU/IPL.
    #[strum(to_string = "IrishPO", serialize = "irishpo")]
    IrishPO,

    /// International RAW Powerlifting
    #[strum(to_string = "IRP", serialize = "irp")]
    IRP,

    /// Japan Powerlifting Federation, IPF.
    #[strum(to_string = "JPA", serialize = "jpa")]
    JPA,

    /// Kazakhstan IPF affiliate.
    #[strum(to_string = "KPF", serialize = "kpf")]
    KPF,

    /// Icelandic IPF affiliate.
    #[strum(to_string = "KRAFT", serialize = "kraft")]
    KRAFT,

    /// Latvian IPF affiliate.
    #[strum(to_string = "LPF", serialize = "lpf")]
    LPF,

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

    /// Nauru Powerlifting Federation, IPF.
    #[strum(to_string = "NauruPF", serialize = "naurupf")]
    NauruPF,

    /// Northern Ireland Powerlifting Federation.
    #[strum(to_string = "NIPF", serialize = "nipf")]
    NIPF,

    /// NORCAL Powerlifting Federation
    #[strum(to_string = "NORCAL", serialize = "norcal")]
    NORCAL,

    /// Nordic Powerlifting Federation, IPF.
    #[strum(to_string = "NordicPF", serialize = "nordicpf")]
    NordicPF,

    /// National Powerlifting Association of Israel.
    #[strum(to_string = "NPA", serialize = "npa")]
    NPA,

    /// National Powerlifting Association of the Netherlands.
    #[strum(to_string = "NPB", serialize = "npb")]
    NPB,

    /// Norwegian IPF affiliate.
    #[strum(to_string = "NSF", serialize = "nsf")]
    NSF,

    /// New Zealand Open competition, standalone.
    #[strum(to_string = "NZOpen", serialize = "nzopen")]
    NZOpen,

    /// New Zealand Powerlifting Federation, IPF.
    #[strum(to_string = "NZPF", serialize = "nzpf")]
    NZPF,

    /// Oceania Powerlifting Federation, WP.
    #[strum(to_string = "OceaniaPF", serialize = "oceaniapf")]
    OceaniaPF,

    /// Oceania Regional Powerlifting Federation, IPF.
    #[strum(to_string = "ORPF", serialize = "orpf")]
    ORPF,

    /// Paralympic Powerlifting.
    #[strum(to_string = "ParaPL", serialize = "parapl")]
    ParaPL,

    /// Powerlifting Australia, formerly IPF, now WP.
    #[strum(to_string = "PA", serialize = "pa")]
    PA,

    /// Powerlifting Association of the Philippines, IPF.
    #[strum(to_string = "PAP", serialize = "pap")]
    PAP,

    /// Powerlifting zveza Slovenije, IPF.
    #[strum(to_string = "PLZS", serialize = "plzs")]
    PLZS,

    /// Papua New Guinea Powerlifting Federation, IPF.
    #[strum(to_string = "PNGPF", serialize = "pngpf")]
    PNGPF,

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
    #[strum(to_string = "RAW", serialize = "100raw")]
    RAW,

    /// 100% RAW Federation Canada.
    #[serde(rename = "RAW-CAN")]
    #[strum(to_string = "RAW-CAN", serialize = "raw-can")]
    RAWCAN,

    /// Icelandic 100% Raw affiliate, not drug tested.
    #[serde(rename = "RAW-Iceland")]
    #[strum(to_string = "RAW-Iceland", serialize = "raw-iceland")]
    RAWIceland,

    /// 100% RAW Federation Ukraine.
    #[serde(rename = "RAW-UKR")]
    #[strum(to_string = "RAW-UKR", serialize = "raw-ukr")]
    RAWUKR,

    /// Raw United Federation
    #[strum(to_string = "RAWU", serialize = "rawu")]
    RAWU,

    /// Rhino Powerlifting Club, South African GPC Affiliate.
    #[strum(to_string = "RhinoPC", serialize = "rhinopc")]
    RhinoPC,

    /// Revolution Powerlifting Syndicate.
    #[strum(to_string = "RPS", serialize = "rps")]
    RPS,

    /// Russian Powerlifting Union.
    #[strum(to_string = "RPU", serialize = "rpu")]
    RPU,

    /// Raw Unity.
    #[strum(to_string = "RUPC", serialize = "rupc")]
    RUPC,

    /// Unaffiliated meets held in Russia.
    #[serde(rename = "Russia-UA")]
    #[strum(to_string = "Russia-UA", serialize = "russia-ua")]
    RussiaUA,

    /// South African Powerlifting Federation, IPF.
    #[strum(to_string = "SAPF", serialize = "sapf")]
    SAPF,

    /// Slovenská asociásia silového trojboja, Slovakian GPC Affiliate.
    #[strum(to_string = "SAST", serialize = "sast")]
    SAST,

    /// Scottish Powerlifting, IPF.
    #[strum(to_string = "ScottishPL", serialize = "scottishpl")]
    ScottishPL,

    /// State Correctional Institution, I think.
    #[strum(to_string = "SCI", serialize = "sci")]
    SCI,

    /// Super-Cup of Titans, a defunct Russian single-ply meet.
    #[strum(to_string = "SCT", serialize = "sct")]
    SCT,

    /// Son Light Power, US based federation
    #[strum(to_string = "SLP", serialize = "slp")]
    SLP,

    /// Singapore Powerlifting Alliance.
    #[strum(to_string = "SPA", serialize = "spa")]
    SPA,

    /// Southern Powerlifting Federation.
    #[strum(to_string = "SPF", serialize = "spf")]
    SPF,

    /// Societatem Potentis Species Sports, a defunct Russian raw federation.
    #[strum(to_string = "SPSS", serialize = "spss")]
    SPSS,

    /// Syndicated Strength Alliance.
    #[strum(to_string = "SSA", serialize = "ssa")]
    SSA,

    /// Swedish IPF affiliate.
    #[strum(to_string = "SSF", serialize = "ssf")]
    SSF,

    /// Finnish IPF affiliate.
    #[strum(to_string = "SVNL", serialize = "svnl")]
    SVNL,

    /// Swiss IPF affiliate. Previously affiliated to many untested federations.
    #[strum(to_string = "SwissPL", serialize = "swisspl")]
    SwissPL,

    /// Thai IPF affiliate.
    #[strum(to_string = "ThaiPF", serialize = "thaipf")]
    ThaiPF,

    /// Texas High School Powerlifting Association.
    #[strum(to_string = "THSPA", serialize = "thspa")]
    THSPA,

    /// Texas High School Women's Powerlifting Association.
    #[strum(to_string = "THSWPA", serialize = "thswpa")]
    THSWPA,

    /// Ukrainian Drug-Free Powerlifting Federation
    #[strum(to_string = "UDFPF", serialize = "udfpf")]
    UDFPF,

    /// Ukraine Powerlifting Association.
    #[strum(to_string = "UkrainePA", serialize = "ukrainepa")]
    UkrainePA,

    /// Ukraine Powerlifting Organisation.
    #[strum(to_string = "UkrainePO", serialize = "ukrainepo")]
    UkrainePO,

    /// United Powerlifting Association.
    #[strum(to_string = "UPA", serialize = "upa")]
    UPA,

    /// Ukrainian Powerlifting Committee.
    #[strum(to_string = "UPC", serialize = "upc")]
    UPC,

    /// United Powerlifting Congress Germany. WPC, GPC, WUAP.
    #[serde(rename = "UPC-Germany")]
    #[strum(to_string = "UPC-Germany", serialize="upc-germany")]
    UPCGermany,

    /// Ukrainian Powerlifting Federation, IPF.
    #[strum(to_string = "UkrainePF", serialize = "ukrainepf")]
    UkrainePF,

    /// Unaffiliated meets held in the USA.
    #[serde(rename = "USA-UA")]
    #[strum(to_string = "USA-UA", serialize = "usa-ua")]
    USAUA,

    /// USA Powerlifting, IPF.
    #[strum(to_string = "USAPL", serialize = "usapl")]
    USAPL,

    /// USA Raw Bench Press Federation (Defunct).
    #[strum(to_string = "USARawBP", serialize = "usarawbp")]
    USARawBP,

    /// Catch-all for overseas meets done by US Military members
    #[strum(to_string = "USMilAbroad", serialize = "usmilabroad")]
    USMilAbroad,

    /// Ujedinjeni Srpski powerlifting savez.
    #[strum(to_string = "USPS", serialize = "usps")]
    USPS,

    /// US Powerlifting Federation.
    #[strum(to_string = "USPF", serialize = "uspf")]
    USPF,

    /// United States Powerlifting Assocation, IPL.
    #[strum(to_string = "USPA", serialize = "uspa")]
    USPA,

    /// United States Strengthlifting Federation.
    #[strum(to_string = "USSF", serialize = "ussf")]
    USSF,

    /// Unified Strength Sports Federation.
    #[strum(to_string = "USSports", serialize = "ussports")]
    USSports,

    /// Vietnam Powerlifting Alliance, GPA.
    #[strum(to_string = "VietnamPA", serialize = "vietnampa")]
    VietnamPA,

    #[strum(to_string = "Vityaz", serialize = "vityaz")]
    Vityaz,

    /// World Association of Bench Pressers and Deadlifters.
    #[strum(to_string = "WABDL", serialize = "wabdl")]
    WABDL,

    /// Not sure what this stands for, Anthony Clark set a bench record in this fed.
    #[strum(to_string = "WBC", serialize = "wbc")]
    WBC,

    /// World Drug-Free Powerlifting Association.
    #[strum(to_string = "WDFPF", serialize = "wdfpf")]
    WDFPF,

    /// Welsh Powerlifting Association, IPF.
    #[strum(to_string = "WelshPA", serialize = "welshpa")]
    WelshPA,

    /// World Powerlifting, Robert Wilks' federation.
    #[strum(to_string = "WP", serialize = "wp")]
    WP,

    /// World Powerlifting New Zealand.
    #[serde(rename = "WP-NZ")]
    #[strum(to_string = "WP-NZ", serialize = "wp-nz")]
    WPNZ,

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

    /// WPC meets hosted by METAL gym Finland.
    #[serde(rename = "WPC-Finland")]
    #[strum(to_string = "WPC-Finland", serialize = "wpc-finland")]
    WPCFinland,

    /// French WPC affiliate.
    #[serde(rename = "WPC-France")]
    #[strum(to_string = "WPC-France", serialize = "wpc-france")]
    WPCFrance,

    /// German WPC affiliate.
    #[serde(rename = "WPC-Germany")]
    #[strum(to_string = "WPC-Germany", serialize = "wpc-germany")]
    WPCGermany,

    /// Icelandic WPC affiliate.
    #[serde(rename = "WPC-Iceland")]
    #[strum(to_string = "WPC-Iceland", serialize = "wpc-iceland")]
    WPCIceland,

    /// Kazakh WPC affiliate.
    #[serde(rename = "WPC-KAZ")]
    #[strum(to_string = "WPC-KAZ", serialize = "wpc-kaz")]
    WPCKAZ,

    /// Kyrgyzstan WPC affiliate.
    #[serde(rename = "WPC-KGZ")]
    #[strum(to_string = "WPC-KGZ", serialize = "wpc-kgz")]
    WPCKGZ,

    /// Latvian WPC affiliate.
    #[serde(rename = "WPC-Latvia")]
    #[strum(to_string = "WPC-Latvia", serialize = "wpc-latvia")]
    WPCLatvia,

    /// Moldovan WPC affiliate.
    #[serde(rename = "WPC-Moldova")]
    #[strum(to_string = "WPC-Moldova", serialize = "wpc-moldova")]
    WPCMoldova,

    /// Portuguese WPC affiliate.
    #[serde(rename = "WPC-Portugal")]
    #[strum(to_string = "WPC-Portugal", serialize = "wpc-portugal")]
    WPCPortugal,

    /// Russian WPC affiliate.
    #[serde(rename = "WPC-RUS")]
    #[strum(to_string = "WPC-RUS", serialize = "wpc-rus")]
    WPCRUS,

    /// South African WPC affiliate.
    #[serde(rename = "WPC-SA")]
    #[strum(to_string = "WPC-SA", serialize = "wpc-sa")]
    WPCSA,

    /// Ukrainian WPC affiliate.
    #[serde(rename = "WPC-UKR")]
    #[strum(to_string = "WPC-UKR", serialize = "wpc-ukr")]
    WPCUKR,

    /// World Powerlifting Federation.
    #[strum(to_string = "WPF", serialize = "wpf")]
    WPF,

    /// World Powerlifting League.
    #[strum(to_string = "WPLeague", serialize = "wpleague")]
    WPLeague,

    /// World Powerlifting Raw Organisation.
    #[strum(to_string = "WPRO", serialize = "wpro")]
    WPRO,

    /// World Powerlifting Union.
    #[strum(to_string = "WPU", serialize = "wpu")]
    WPU,

    /// World Powerlifting Union of Federations.
    #[strum(to_string = "WPUF", serialize = "wpuf")]
    WPUF,

    /// World Powerlifting Union Russia
    #[serde(rename = "WPU-RUS")]
    #[strum(to_string = "WPU-RUS", serialize = "wpu-rus")]
    WPURUS,

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

    /// Icelandic WRPF affiliate.
    #[serde(rename = "WRPF-Iceland")]
    #[strum(to_string = "WRPF-Iceland", serialize = "wrpf-iceland")]
    WRPFIceland,

    /// Irish WRPF affiliate.
    #[serde(rename = "WRPF-Ireland")]
    #[strum(to_string = "WRPF-Ireland", serialize = "wrpf-ireland")]
    WRPFIreland,

    /// Kazakh WRPF affiliate.
    #[serde(rename = "WRPF-KAZ")]
    #[strum(to_string = "WRPF-KAZ", serialize = "wrpf-kaz")]
    WRPFKAZ,

    /// Polish WRPF affiliate.
    #[serde(rename = "WRPF-POL")]
    #[strum(to_string = "WRPF-POL", serialize = "wrpf-pol")]
    WRPFPOL,

    /// Slovenian WRPF affiliate.
    #[serde(rename = "WRPF-Slovenia")]
    #[strum(to_string = "WRPF-Slovenia", serialize = "wrpf-slovenia")]
    WRPFSlovenia,

    /// Spanish WRPF affiliate.
    #[serde(rename = "WRPF-Spain")]
    #[strum(to_string = "WRPF-Spain", serialize = "wrpf-spain")]
    WRPFSpain,

    /// Serbian WRPF affiliate.
    #[serde(rename = "WRPF-SRB")]
    #[strum(to_string = "WRPF-SRB", serialize = "wrpf-srb")]
    WRPFSRB,

    /// Swedish WRPF affiliate.
    #[serde(rename = "WRPF-Sweden")]
    #[strum(to_string = "WRPF-Sweden", serialize = "wrpf-sweden")]
    WRPFSweden,

    /// World United Amateur Powerlifting.
    #[strum(to_string = "WUAP", serialize = "wuap")]
    WUAP,

    /// Austrian WUAP affiliate.
    #[serde(rename = "WUAP-AUT")]
    #[strum(to_string = "WUAP-AUT", serialize = "wuap-aut")]
    WUAPAUT,

    /// Xtreme Powerlifting Coalition.
    #[strum(to_string = "XPC", serialize = "xpc")]
    XPC,

    /// Polish version of the XPC
    #[serde(rename = "XPC-Poland")]
    #[strum(to_string = "XPC-Poland", serialize = "xpc-poland")]
    XPCPoland,
}

impl Federation {
    /// True iff every division in the federation is drug-tested.
    pub fn is_fully_tested(self) -> bool {
        match self {
            Federation::_365Strong => false,
            Federation::AAP => false,
            Federation::AAPLF => true,
            Federation::AAU => true,
            Federation::ACHIPO => false,
            Federation::ADAU => true,
            Federation::ADFPA => true,
            Federation::ADFPF => true,
            Federation::AEP => true,
            Federation::AFPF => false,
            Federation::AfricanPF => true,
            Federation::AIWBPA => true,
            Federation::AmericanSA => false,
            Federation::ANPPC => false,
            Federation::APA => false,
            Federation::APC => false,
            Federation::APF => false,
            Federation::APU => true,
            Federation::AsianPF => true,
            Federation::AusDFPF => true,
            Federation::AusPF => false,
            Federation::AusPL => false,
            Federation::AWF => false,
            Federation::BAWLA => true,
            Federation::BB => false,
            Federation::BBDD => false,
            Federation::BDFPA => true,
            Federation::BelPF => true,
            Federation::BenchAmerica => false,
            Federation::BP => true,
            Federation::BPC => false,
            Federation::BPF => false,
            Federation::BPO => false,
            Federation::BPU => false,
            Federation::BVDG => true,
            Federation::BVDK => true,
            Federation::CAPO => false,
            Federation::CAPONZ => false,
            Federation::CAST => false,
            Federation::ChinaPA => false,
            Federation::CommonwealthPF => true,
            Federation::CPA => false,
            Federation::CPC => false,
            Federation::CPF => false,
            Federation::CPL => false,
            Federation::CPO => false,
            Federation::CPU => true,
            Federation::CRPEBA => false,
            Federation::CSST => true,
            Federation::DBKV => false,
            Federation::CzechiaUA => false,
            Federation::DSF => true,
            Federation::EPA => true,
            Federation::EPF => true,
            Federation::ESDT => false,
            Federation::FALPO => true,
            Federation::FBPD => false,
            Federation::FCA => false,
            Federation::FCST => false,
            Federation::FEMEPO => true,
            Federation::FEPOA => false,
            Federation::FESUPO => true,
            Federation::FFForce => true,
            Federation::FHSAA => true,
            Federation::FIPL => true,
            Federation::FPO => false,
            Federation::FPR => true,
            Federation::GermanyUA => false,
            Federation::GlobalPU => false,
            Federation::GPA => false,
            Federation::GPACRO => false,
            Federation::GPC => false,
            Federation::GPCAUS => false,
            Federation::GPCGB => false,
            Federation::GPCIRL => false,
            Federation::GPCLAT => false,
            Federation::GPCNZ => false,
            Federation::GPCRUS => false,
            Federation::GPF => false,
            Federation::GPU => false,
            Federation::Hardcore => false,
            Federation::HERC => false,
            Federation::CroatiaUA => false,
            Federation::HPLS => true,
            Federation::HPLSUA => false,
            Federation::HPO => false,
            Federation::HTPL => true,
            Federation::IBSA => true,
            Federation::IDFPA => true,
            Federation::IDFPF => true,
            Federation::IKF => false,
            Federation::GPCCAN => false,
            Federation::IPA => false,
            Federation::IPC => false,
            Federation::IPF => true,
            Federation::IPL => false,
            Federation::IPLNZ => false,
            Federation::IrelandUA => false,
            Federation::IrishPF => true,
            Federation::IrishPO => false,
            Federation::IRP => false,
            Federation::JPA => true,
            Federation::KRAFT => true,
            Federation::KPF => true,
            Federation::LPF => true,
            Federation::MM => false,
            Federation::MPA => false,
            Federation::NAP => false,
            Federation::NAPF => true,
            Federation::NASA => true,
            Federation::NauruPF => true,
            Federation::NIPF => true,
            Federation::NORCAL => true,
            Federation::NordicPF => true,
            Federation::NPA => false,
            Federation::NPB => true,
            Federation::NSF => true,
            Federation::NZOpen => false,
            Federation::NZPF => true,
            Federation::OceaniaPF => true,
            Federation::ORPF => true,
            Federation::ParaPL => true,
            Federation::PA => true,
            Federation::PAP => true,
            Federation::PLZS => true,
            Federation::PNGPF => true,
            Federation::PRIDE => false,
            Federation::ProRaw => false,
            Federation::PRPA => false,
            Federation::PZKFiTS => true,
            Federation::RAW => true,
            Federation::RAWCAN => true,
            Federation::RAWIceland => false,
            Federation::RAWUKR => true,
            Federation::RAWU => false,
            Federation::RhinoPC => false,
            Federation::RPS => false,
            Federation::RPU => false,
            Federation::RUPC => false,
            Federation::RussiaUA => false,
            Federation::SAPF => true,
            Federation::SAST => false,
            Federation::ScottishPL => true,
            Federation::SCI => false,
            Federation::SCT => false,
            Federation::SLP => false,
            Federation::SPA => false,
            Federation::SPF => false,
            Federation::SPSS => false,
            Federation::SSA => false,
            Federation::SSF => true,
            Federation::SVNL => true,
            Federation::SwissPL => false,
            Federation::ThaiPF => true,
            Federation::THSPA => true,
            Federation::THSWPA => true,
            Federation::UDFPF => true,
            Federation::UkrainePA => false,
            Federation::UkrainePO => false,
            Federation::UPA => false,
            Federation::UPC => false,
            Federation::UPCGermany => false,
            Federation::UkrainePF => true,
            Federation::USAUA => false,
            Federation::USAPL => true,
            Federation::USARawBP => false,
            Federation::USMilAbroad => false,
            Federation::USPS => false,
            Federation::USPF => false,
            Federation::USPA => false,
            Federation::USSF => false,
            Federation::USSports => false,
            Federation::VietnamPA => false,
            Federation::Vityaz => false,
            Federation::WABDL => true,
            Federation::WDFPF => true,
            Federation::WelshPA => true,
            Federation::WNPF => true,
            Federation::WP => true,
            Federation::WPA => false,
            Federation::WPARUS => false,
            Federation::WPAU => false,
            Federation::WBC => false,
            Federation::WPC => false,
            Federation::WPCFinland => false,
            Federation::WPCFrance => false,
            Federation::WPCGermany => false,
            Federation::WPCIceland => false,
            Federation::WPCKAZ => false,
            Federation::WPCKGZ => false,
            Federation::WPCLatvia => false,
            Federation::WPCMoldova => false,
            Federation::WPCPortugal => false,
            Federation::WPCRUS => false,
            Federation::WPCSA => false,
            Federation::WPCUKR => false,
            Federation::WPF => false,
            Federation::WPLeague => false,
            Federation::WPNZ => true,
            Federation::WPRO => false,
            Federation::WPU => false,
            Federation::WPUF => false,
            Federation::WPURUS => false,
            Federation::WRPF => false,
            Federation::WRPFAUS => false,
            Federation::WRPFCAN => false,
            Federation::WRPFIceland => false,
            Federation::WRPFIreland => false,
            Federation::WRPFKAZ => false,
            Federation::WRPFPOL => false,
            Federation::WRPFSlovenia => false,
            Federation::WRPFSpain => false,
            Federation::WRPFSRB => false,
            Federation::WRPFSweden => false,
            Federation::WUAP => false,
            Federation::WUAPAUT => false,
            Federation::XPC => false,
            Federation::XPCPoland => false,
        }
    }

    /// Country out of which the federation operates.
    pub fn home_country(self) -> Option<Country> {
        match self {
            Federation::_365Strong => Some(Country::USA),
            Federation::AAP => Some(Country::Argentina),
            Federation::AAPLF => Some(Country::Australia),
            Federation::AAU => Some(Country::USA),
            Federation::ACHIPO => Some(Country::Chile),
            Federation::ADAU => Some(Country::USA),
            Federation::ADFPA => Some(Country::USA),
            Federation::ADFPF => Some(Country::USA),
            Federation::AEP => Some(Country::Spain),
            Federation::AFPF => Some(Country::USA),
            Federation::AfricanPF => None,
            Federation::AIWBPA => Some(Country::Indonesia),
            Federation::AmericanSA => Some(Country::USA),
            Federation::ANPPC => Some(Country::USA),
            Federation::APA => Some(Country::USA),
            Federation::APC => Some(Country::USA),
            Federation::APF => Some(Country::USA),
            Federation::APU => Some(Country::Australia),
            Federation::AsianPF => None,
            Federation::AusDFPF => Some(Country::Australia),
            Federation::AusPF => Some(Country::Australia),
            Federation::AusPL => Some(Country::Australia),
            Federation::AWF => Some(Country::Australia),
            Federation::BAWLA => Some(Country::UK),
            Federation::BB => Some(Country::Russia),
            Federation::BBDD => Some(Country::USA),
            Federation::BDFPA => Some(Country::UK),
            Federation::BelPF => Some(Country::Belarus),
            Federation::BenchAmerica => Some(Country::USA),
            Federation::BP => Some(Country::UK),
            Federation::BPC => Some(Country::UK),
            Federation::BPF => Some(Country::UK),
            Federation::BPO => Some(Country::UK),
            Federation::BPU => Some(Country::UK),
            Federation::BVDG => Some(Country::Germany),
            Federation::BVDK => Some(Country::Germany),
            Federation::CAPO => Some(Country::Australia),
            Federation::CAPONZ => Some(Country::NewZealand),
            Federation::CAST => Some(Country::Czechia),
            Federation::ChinaPA => Some(Country::China),
            Federation::CommonwealthPF => None,
            Federation::CPA => Some(Country::Canada),
            Federation::CPC => Some(Country::Canada),
            Federation::CPF => Some(Country::Canada),
            Federation::CPL => Some(Country::Canada),
            Federation::CPO => Some(Country::Canada),
            Federation::CPU => Some(Country::Canada),
            //Initially Ukraine,until annexation
            Federation::CRPEBA => Some(Country::Russia),
            Federation::CSST => Some(Country::Czechia),
            Federation::DBKV => Some(Country::Germany),
            Federation::CzechiaUA => Some(Country::Czechia),
            Federation::DSF => Some(Country::Denmark),
            Federation::EPA => Some(Country::England),
            Federation::EPF => None,
            Federation::ESDT => Some(Country::Greece),
            Federation::FALPO => Some(Country::Argentina),
            Federation::FBPD => Some(Country::Russia),
            Federation::FCA => Some(Country::USA),
            Federation::FCST => Some(Country::Czechia),
            Federation::FEMEPO => Some(Country::Mexico),
            Federation::FEPOA => Some(Country::Argentina),
            Federation::FESUPO => None,
            Federation::FFForce => Some(Country::France),
            Federation::FHSAA => Some(Country::USA),
            Federation::FIPL => Some(Country::Italy),
            Federation::FPO => Some(Country::Finland),
            Federation::FPR => Some(Country::Russia),
            Federation::GermanyUA => Some(Country::Germany),
            Federation::GlobalPU => Some(Country::Ukraine),
            Federation::GPA => None,
            Federation::GPACRO => Some(Country::Croatia),
            Federation::GPC => None,
            Federation::GPCAUS => Some(Country::Australia),
            Federation::GPCCAN => Some(Country::Canada),
            Federation::GPCGB => Some(Country::UK),
            Federation::GPCIRL => Some(Country::Ireland),
            Federation::GPCLAT => Some(Country::Latvia),
            Federation::GPCNZ => Some(Country::NewZealand),
            Federation::GPCRUS => Some(Country::Russia),
            Federation::GPF => None,
            Federation::GPU => Some(Country::Germany),
            Federation::Hardcore => Some(Country::USA),
            Federation::HERC => Some(Country::USA),
            Federation::CroatiaUA => Some(Country::Croatia),
            Federation::HPLS => Some(Country::Croatia),
            Federation::HPLSUA => Some(Country::Croatia),
            Federation::HPO => Some(Country::Croatia),
            Federation::HTPL => Some(Country::China),
            Federation::IBSA => None,
            Federation::IDFPA => Some(Country::Ireland),
            Federation::IDFPF => Some(Country::Ireland),
            Federation::IKF => Some(Country::Iceland),
            Federation::IPA => Some(Country::USA),
            Federation::IPC => Some(Country::Israel),
            Federation::IPF => None,
            Federation::IPL => None,
            Federation::IPLNZ => Some(Country::NewZealand),
            Federation::IrelandUA => Some(Country::Ireland),
            Federation::IrishPF => Some(Country::Ireland),
            Federation::IrishPO => Some(Country::Ireland),
            Federation::IRP => None,
            Federation::JPA => Some(Country::Japan),
            Federation::KRAFT => Some(Country::Iceland),
            Federation::KPF => Some(Country::Kazakhstan),
            Federation::LPF => Some(Country::Latvia),
            Federation::MM => Some(Country::USA),
            Federation::MPA => Some(Country::Malaysia),
            Federation::NAP => Some(Country::Russia),
            Federation::NAPF => None,
            Federation::NASA => Some(Country::USA),
            Federation::NauruPF => Some(Country::Nauru),
            Federation::NORCAL => Some(Country::USA),
            Federation::NIPF => Some(Country::NorthernIreland),
            Federation::NordicPF => None,
            Federation::NPA => Some(Country::Israel),
            Federation::NPB => Some(Country::Netherlands),
            Federation::NSF => Some(Country::Norway),
            Federation::NZOpen => Some(Country::NewZealand),
            Federation::NZPF => Some(Country::NewZealand),
            Federation::OceaniaPF => None,
            Federation::ORPF => None,
            Federation::ParaPL => None,
            Federation::PA => Some(Country::Australia),
            Federation::PAP => Some(Country::Philippines),
            Federation::PLZS => Some(Country::Slovenia),
            Federation::PNGPF => Some(Country::PapuaNewGuinea),
            Federation::PRIDE => Some(Country::USA),
            Federation::ProRaw => Some(Country::Australia),
            Federation::PRPA => Some(Country::USA),
            Federation::PZKFiTS => Some(Country::Poland),
            Federation::RAW => Some(Country::USA),
            Federation::RAWCAN => Some(Country::Canada),
            Federation::RAWIceland => Some(Country::Iceland),
            Federation::RAWUKR => Some(Country::Ukraine),
            Federation::RAWU => Some(Country::USA),
            Federation::RhinoPC => Some(Country::SouthAfrica),
            Federation::RPS => Some(Country::USA),
            Federation::RPU => Some(Country::Russia),
            Federation::RUPC => Some(Country::USA),
            Federation::RussiaUA => Some(Country::Russia),
            Federation::SAPF => Some(Country::SouthAfrica),
            Federation::SAST => Some(Country::Slovakia),
            Federation::ScottishPL => Some(Country::Scotland),
            Federation::SCI => Some(Country::USA),
            Federation::SCT => Some(Country::Russia),
            Federation::SLP => Some(Country::USA),
            Federation::SPA => Some(Country::Singapore),
            Federation::SPF => Some(Country::USA),
            Federation::SPSS => Some(Country::Russia),
            Federation::SSA => Some(Country::USA),
            Federation::SSF => Some(Country::Sweden),
            Federation::SVNL => Some(Country::Finland),
            Federation::SwissPL => Some(Country::Switzerland),
            Federation::ThaiPF => Some(Country::Thailand),
            Federation::THSPA => Some(Country::USA),
            Federation::THSWPA => Some(Country::USA),
            Federation::UDFPF => Some(Country::Ukraine),
            Federation::UkrainePA => Some(Country::Ukraine),
            Federation::UkrainePO => Some(Country::Ukraine),
            Federation::UPA => Some(Country::USA),
            Federation::UPC => Some(Country::Ukraine),
            Federation::UPCGermany => Some(Country::Germany),
            Federation::UkrainePF => Some(Country::Ukraine),
            Federation::USAUA => Some(Country::USA),
            Federation::USAPL => Some(Country::USA),
            Federation::USARawBP => Some(Country::USA),
            Federation::USMilAbroad => Some(Country::USA),
            Federation::USPS => Some(Country::Serbia),
            Federation::USPF => Some(Country::USA),
            Federation::USPA => Some(Country::USA),
            Federation::USSF => Some(Country::USA),
            Federation::USSports => Some(Country::USA),
            Federation::VietnamPA => Some(Country::Vietnam),
            Federation::Vityaz => Some(Country::Russia),
            Federation::WABDL => Some(Country::USA),
            Federation::WBC => Some(Country::USA),
            Federation::WDFPF => None,
            Federation::WelshPA => Some(Country::Wales),
            Federation::WP => None,
            Federation::WPA => None,
            Federation::WPARUS => Some(Country::Russia),
            Federation::WPAU => Some(Country::Ukraine),
            Federation::WPC => None,
            Federation::WPCFinland => Some(Country::Finland),
            Federation::WPCFrance => Some(Country::France),
            Federation::WPCGermany => Some(Country::Germany),
            Federation::WPCIceland => Some(Country::Iceland),
            Federation::WPCKAZ => Some(Country::Kazakhstan),
            Federation::WPCKGZ => Some(Country::Kyrgyzstan),
            Federation::WPCLatvia => Some(Country::Latvia),
            Federation::WPCMoldova => Some(Country::Moldova),
            Federation::WPCPortugal => Some(Country::Portugal),
            Federation::WPCRUS => Some(Country::Russia),
            Federation::WPCSA => Some(Country::SouthAfrica),
            Federation::WPCUKR => Some(Country::Ukraine),
            Federation::WPF => None,
            Federation::WPLeague => Some(Country::Ukraine),
            Federation::WPNZ => Some(Country::NewZealand),
            Federation::WPRO => Some(Country::Ukraine),
            Federation::WPU => None,
            Federation::WPUF => Some(Country::Ukraine),
            Federation::WPURUS => Some(Country::Russia),
            Federation::WNPF => Some(Country::USA),
            Federation::WRPF => Some(Country::Russia),
            Federation::WRPFAUS => Some(Country::Australia),
            Federation::WRPFCAN => Some(Country::Canada),
            Federation::WRPFIceland => Some(Country::Iceland),
            Federation::WRPFIreland => Some(Country::Ireland),
            Federation::WRPFKAZ => Some(Country::Kazakhstan),
            Federation::WRPFPOL => Some(Country::Poland),
            Federation::WRPFSlovenia => Some(Country::Slovenia),
            Federation::WRPFSpain => Some(Country::Spain),
            Federation::WRPFSRB => Some(Country::Serbia),
            Federation::WRPFSweden => Some(Country::Sweden),
            Federation::WUAP => None,
            Federation::WUAPAUT => Some(Country::Austria),
            Federation::XPC => Some(Country::USA),
            Federation::XPCPoland => Some(Country::Poland),
        }
    }

    /// Helper function for specifying the PointsSystem of federations under IPF rules.
    #[inline]
    fn ipf_rules_on(date: Date) -> PointsSystem {
        // The IPF and their affiliates developed a new federation-specific
        // formula beginning in 2019.
        if date.year() >= 2019 {
            PointsSystem::IPFPoints
        } else {
            PointsSystem::Wilks
        }
    }

    /// Which points system is default for a federation's meet.
    pub fn default_points(self, date: Date) -> PointsSystem {
        match self {
            Federation::_365Strong => PointsSystem::Wilks,
            Federation::AAP => PointsSystem::Wilks,
            Federation::AAPLF => PointsSystem::Wilks,
            Federation::AAU => PointsSystem::Wilks,
            Federation::ACHIPO => PointsSystem::Wilks,
            Federation::ADAU => PointsSystem::Wilks,
            Federation::ADFPA => PointsSystem::Wilks,
            Federation::ADFPF => PointsSystem::Wilks,
            Federation::AEP => Federation::ipf_rules_on(date),
            Federation::AFPF => PointsSystem::Wilks,
            Federation::AfricanPF => Federation::ipf_rules_on(date),
            Federation::AIWBPA => Federation::ipf_rules_on(date),
            Federation::AmericanSA => PointsSystem::Wilks,
            Federation::ANPPC => PointsSystem::Wilks,
            Federation::APA => PointsSystem::Wilks,
            Federation::APC => PointsSystem::Wilks,
            Federation::APF => PointsSystem::Glossbrenner,
            Federation::APU => PointsSystem::Wilks,
            Federation::AsianPF => Federation::ipf_rules_on(date),
            Federation::AusDFPF => PointsSystem::Wilks,
            Federation::AusPF => PointsSystem::Wilks,
            Federation::AusPL => PointsSystem::Wilks,
            Federation::AWF => PointsSystem::Wilks,
            Federation::BAWLA => PointsSystem::Wilks,
            Federation::BB => PointsSystem::Wilks,
            Federation::BBDD => PointsSystem::Wilks,
            Federation::BDFPA => PointsSystem::Wilks,
            Federation::BelPF => Federation::ipf_rules_on(date),
            Federation::BenchAmerica => PointsSystem::Wilks,
            Federation::BP => Federation::ipf_rules_on(date),
            Federation::BPC => PointsSystem::Wilks,
            Federation::BPF => PointsSystem::Wilks,
            Federation::BPO => PointsSystem::Wilks,
            Federation::BPU => PointsSystem::Wilks,
            Federation::BVDG => PointsSystem::Wilks,
            Federation::BVDK => Federation::ipf_rules_on(date),
            Federation::CAPO => PointsSystem::Glossbrenner,
            Federation::CAPONZ => PointsSystem::Glossbrenner,
            Federation::CAST => PointsSystem::Wilks,
            Federation::ChinaPA => PointsSystem::Wilks,
            Federation::CommonwealthPF => Federation::ipf_rules_on(date),
            Federation::CPA => PointsSystem::Wilks,
            Federation::CPC => PointsSystem::Wilks,
            Federation::CPF => PointsSystem::Wilks,
            Federation::CPL => PointsSystem::Wilks,
            Federation::CPO => PointsSystem::Wilks,
            Federation::CPU => Federation::ipf_rules_on(date),
            Federation::CRPEBA => PointsSystem::Wilks,
            Federation::CSST => PointsSystem::Wilks,
            Federation::DBKV => PointsSystem::Wilks,
            Federation::CzechiaUA => PointsSystem::Wilks,
            Federation::DSF => Federation::ipf_rules_on(date),
            Federation::EPA => Federation::ipf_rules_on(date),
            Federation::EPF => Federation::ipf_rules_on(date),
            Federation::ESDT => PointsSystem::Wilks,
            Federation::FALPO => Federation::ipf_rules_on(date),
            Federation::FBPD => PointsSystem::Wilks,
            Federation::FCA => PointsSystem::Wilks,
            Federation::FCST => PointsSystem::Wilks,
            Federation::FEMEPO => Federation::ipf_rules_on(date),
            Federation::FEPOA => PointsSystem::Wilks,
            Federation::FESUPO => Federation::ipf_rules_on(date),
            Federation::FFForce => Federation::ipf_rules_on(date),
            Federation::FHSAA => PointsSystem::Wilks,
            Federation::FIPL => Federation::ipf_rules_on(date),
            Federation::FPO => PointsSystem::Wilks,
            Federation::FPR => Federation::ipf_rules_on(date),
            Federation::GermanyUA => PointsSystem::Wilks,
            Federation::GlobalPU => PointsSystem::Glossbrenner,
            Federation::GPA => PointsSystem::Wilks,
            Federation::GPACRO => PointsSystem::Wilks,
            Federation::GPC => PointsSystem::Glossbrenner,
            Federation::GPCAUS => PointsSystem::Glossbrenner,
            Federation::GPCCAN => PointsSystem::Glossbrenner,
            Federation::GPCGB => PointsSystem::Glossbrenner,
            Federation::GPCIRL => PointsSystem::Glossbrenner,
            Federation::GPCLAT => PointsSystem::Glossbrenner,
            Federation::GPCNZ => PointsSystem::Glossbrenner,
            Federation::GPCRUS => PointsSystem::Glossbrenner,
            Federation::GPF => PointsSystem::Wilks,
            Federation::GPU => PointsSystem::Wilks,
            Federation::Hardcore => PointsSystem::Wilks,
            Federation::HERC => PointsSystem::Wilks,
            Federation::CroatiaUA => PointsSystem::Wilks,
            Federation::HPLS => Federation::ipf_rules_on(date),
            Federation::HPLSUA => PointsSystem::Wilks,
            Federation::HPO => PointsSystem::Wilks,
            Federation::HTPL => PointsSystem::Wilks,
            Federation::IBSA => PointsSystem::Wilks,
            Federation::IDFPA => PointsSystem::Wilks,
            Federation::IDFPF => PointsSystem::Wilks,
            Federation::IKF => PointsSystem::Wilks,
            Federation::IPA => PointsSystem::Wilks,
            Federation::IPC => PointsSystem::Wilks,
            Federation::IPF => Federation::ipf_rules_on(date),
            Federation::IPL => PointsSystem::Wilks,
            Federation::IPLNZ => PointsSystem::Wilks,
            Federation::IrelandUA => PointsSystem::Wilks,
            Federation::IrishPF => Federation::ipf_rules_on(date),
            Federation::IrishPO => PointsSystem::Wilks,
            Federation::IRP => PointsSystem::Wilks,
            Federation::JPA => Federation::ipf_rules_on(date),
            Federation::KPF => Federation::ipf_rules_on(date),
            Federation::KRAFT => Federation::ipf_rules_on(date),
            Federation::LPF => Federation::ipf_rules_on(date),
            Federation::MM => PointsSystem::Wilks,
            Federation::MPA => PointsSystem::Wilks,
            Federation::NAP => PointsSystem::Wilks,
            Federation::NAPF => Federation::ipf_rules_on(date),
            Federation::NASA => PointsSystem::Wilks,
            Federation::NauruPF => Federation::ipf_rules_on(date),
            Federation::NORCAL => PointsSystem::Wilks,
            Federation::NIPF => Federation::ipf_rules_on(date),
            Federation::NordicPF => Federation::ipf_rules_on(date),
            Federation::NPA => PointsSystem::Wilks,
            Federation::NPB => PointsSystem::Wilks,
            Federation::NSF => Federation::ipf_rules_on(date),
            Federation::NZOpen => PointsSystem::Wilks,
            Federation::NZPF => Federation::ipf_rules_on(date),
            Federation::OceaniaPF => PointsSystem::Wilks,
            Federation::ORPF => Federation::ipf_rules_on(date),
            Federation::ParaPL => PointsSystem::Wilks,
            Federation::PA => PointsSystem::Wilks,
            Federation::PAP => Federation::ipf_rules_on(date),
            Federation::PLZS => Federation::ipf_rules_on(date),
            Federation::PNGPF => Federation::ipf_rules_on(date),
            Federation::PRIDE => PointsSystem::Wilks,
            Federation::ProRaw => PointsSystem::Glossbrenner,
            Federation::PRPA => PointsSystem::Wilks,
            Federation::PZKFiTS => Federation::ipf_rules_on(date),
            Federation::RAW => PointsSystem::Wilks,
            Federation::RAWCAN => PointsSystem::Wilks,
            Federation::RAWIceland => PointsSystem::Wilks,
            Federation::RAWUKR => PointsSystem::Wilks,
            Federation::RAWU => PointsSystem::Wilks,
            Federation::RhinoPC => PointsSystem::Glossbrenner,
            Federation::RPS => PointsSystem::Wilks,
            Federation::RPU => PointsSystem::Wilks,
            Federation::RUPC => PointsSystem::Wilks,
            Federation::RussiaUA => PointsSystem::Wilks,
            Federation::SAPF => Federation::ipf_rules_on(date),
            Federation::SAST => PointsSystem::Glossbrenner,
            Federation::ScottishPL => Federation::ipf_rules_on(date),
            Federation::SCI => PointsSystem::Wilks,
            Federation::SCT => PointsSystem::Wilks,
            Federation::SLP => PointsSystem::Wilks,
            Federation::SPA => PointsSystem::Wilks,
            Federation::SPF => PointsSystem::Wilks,
            Federation::SPSS => PointsSystem::Wilks,
            Federation::SSA => PointsSystem::Wilks,
            Federation::SSF => Federation::ipf_rules_on(date),
            Federation::SVNL => Federation::ipf_rules_on(date),
            Federation::SwissPL => PointsSystem::Wilks,
            Federation::ThaiPF => Federation::ipf_rules_on(date),
            Federation::THSPA => PointsSystem::Wilks,
            Federation::THSWPA => PointsSystem::Wilks,
            Federation::UDFPF => PointsSystem::Wilks,
            Federation::UkrainePA => PointsSystem::Wilks,
            Federation::UkrainePO => PointsSystem::Wilks,
            Federation::UPA => PointsSystem::Wilks,
            Federation::UPC => PointsSystem::Wilks,
            Federation::UPCGermany => PointsSystem::Glossbrenner,
            Federation::UkrainePF => Federation::ipf_rules_on(date),
            Federation::USAUA => PointsSystem::Wilks,
            Federation::USAPL => Federation::ipf_rules_on(date),
            Federation::USARawBP => PointsSystem::Wilks,
            Federation::USMilAbroad => PointsSystem::Wilks,
            Federation::USPS => PointsSystem::Wilks,
            Federation::USPF => PointsSystem::Wilks,
            Federation::USPA => PointsSystem::Wilks,
            Federation::USSF => PointsSystem::Wilks,
            Federation::USSports => PointsSystem::Wilks,
            Federation::VietnamPA => PointsSystem::Wilks,
            Federation::Vityaz => PointsSystem::Wilks,
            Federation::WABDL => PointsSystem::Wilks,
            Federation::WBC => PointsSystem::Wilks,
            Federation::WDFPF => PointsSystem::Wilks,
            Federation::WelshPA => Federation::ipf_rules_on(date),
            Federation::WP => PointsSystem::Wilks,
            Federation::WPA => PointsSystem::Wilks,
            Federation::WPARUS => PointsSystem::Wilks,
            Federation::WPAU => PointsSystem::Wilks,
            Federation::WPC => PointsSystem::Glossbrenner,
            Federation::WPCFinland => PointsSystem::Glossbrenner,
            Federation::WPCFrance => PointsSystem::Glossbrenner,
            Federation::WPCGermany => PointsSystem::Glossbrenner,
            Federation::WPCIceland => PointsSystem::Glossbrenner,
            Federation::WPCKAZ => PointsSystem::Glossbrenner,
            Federation::WPCKGZ => PointsSystem::Glossbrenner,
            Federation::WPCLatvia => PointsSystem::Glossbrenner,
            Federation::WPCMoldova => PointsSystem::Glossbrenner,
            Federation::WPCPortugal => PointsSystem::Glossbrenner,
            Federation::WPCRUS => PointsSystem::Glossbrenner,
            Federation::WPCSA => PointsSystem::Glossbrenner,
            Federation::WPCUKR => PointsSystem::Glossbrenner,
            Federation::WPF => PointsSystem::Wilks,
            Federation::WPLeague => PointsSystem::Wilks,
            Federation::WPNZ => PointsSystem::Wilks,
            Federation::WPRO => PointsSystem::Wilks,
            Federation::WPU => PointsSystem::Wilks,
            Federation::WPUF => PointsSystem::Wilks,
            Federation::WPURUS => PointsSystem::Wilks,
            Federation::WNPF => PointsSystem::Wilks,
            Federation::WRPF => PointsSystem::Wilks,
            Federation::WRPFAUS => PointsSystem::Wilks,
            Federation::WRPFCAN => PointsSystem::Wilks,
            Federation::WRPFIceland => PointsSystem::Wilks,
            Federation::WRPFIreland => PointsSystem::Wilks,
            Federation::WRPFKAZ => PointsSystem::Wilks,
            Federation::WRPFPOL => PointsSystem::Wilks,
            Federation::WRPFSlovenia => PointsSystem::Wilks,
            Federation::WRPFSpain => PointsSystem::Wilks,
            Federation::WRPFSRB => PointsSystem::Wilks,
            Federation::WRPFSweden => PointsSystem::Wilks,
            Federation::WUAP => PointsSystem::Wilks,
            Federation::WUAPAUT => PointsSystem::Wilks,
            Federation::XPC => PointsSystem::Wilks,
            Federation::XPCPoland => PointsSystem::Wilks,
        }
    }
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
