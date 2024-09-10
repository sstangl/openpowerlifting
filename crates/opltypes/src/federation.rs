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

    /// Alianza Boliviana Powerlifting, GPA/IPO.
    #[strum(to_string = "ABP", serialize = "abp")]
    ABP,

    /// Alianza Chilena Powerlifting, GPA/IPO.
    #[strum(to_string = "ACHIPO", serialize = "achipo")]
    ACHIPO,

    /// African Continental Powerlifting Alliance, WPA.
    #[strum(to_string = "ACPA", serialize = "acpa")]
    ACPA,

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

    /// Powerlifting America. Replaced USAPL as USA IPF affiliate in 2022.
    #[strum(to_string = "AMP", serialize = "amp")]
    AMP,

    /// All Natural Physique and Power Conference (Defunct).
    #[strum(to_string = "ANPPC", serialize = "anppc")]
    ANPPC,

    /// American Powerlifting Association, WPA.
    #[strum(to_string = "APA", serialize = "apa")]
    APA,

    /// American Powerlifting Committee, WUAP.
    #[strum(to_string = "APC", serialize = "apc")]
    APC,

    /// American Powerlifting Federation.
    #[strum(to_string = "APF", serialize = "apf")]
    APF,

    /// Australian Powerlifting Alliance, IPF.
    #[strum(to_string = "APLA", serialize = "apla")]
    APLA,

    /// Alianza Paraguaya de Powerlifting, Paraguay GPA affiliate.
    #[strum(to_string = "APP", serialize = "app")]
    APP,

    /// Australian Powerlifting Union, formerly IPF, now WDFPF.
    #[strum(to_string = "APU", serialize = "apu")]
    APU,

    /// Asociación Powerlifting Unidos de Argentina, WABDL.
    #[strum(to_string = "APUA", serialize = "apua")]
    APUA,

    /// Argentina Powerlifting League, IPL.
    #[strum(to_string = "ARPL", serialize = "arpl")]
    ARPL,

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

    /// Bahamas Powerlifting Federation, IPF.
    #[strum(to_string = "BahamasPF", serialize = "bahamasPF")]
    BahamasPF,

    /// British Amateur Weightlifting Association, predecessor to BP.
    #[strum(to_string = "BAWLA", serialize = "bawla")]
    BAWLA,

    /// Bogatyr Brotherhood, a stand-alone and short-lived Russian federation.
    #[strum(to_string = "BB", serialize = "bb")]
    BB,

    /// British Drug-Free Powerlifting Assocation, WDFPF.
    #[strum(to_string = "BDFPA", serialize = "bdfpa")]
    BDFPA,

    /// Belgian Drug-Free Powerlifting Federation, WDFPF.
    #[strum(to_string = "BDFPF", serialize = "bdfpf")]
    BDFPF,

    /// Belarus Powerlifting Federation, IPF.
    #[strum(to_string = "BelPF", serialize = "belpf")]
    BelPF,

    /// British Powerlifting, IPF. Formerly named GBPF.
    #[strum(to_string = "BP", serialize = "bp")]
    BP,

    /// Belize Powerlifting Association, IPF.
    #[strum(to_string = "BPA", serialize = "bpa")]
    BPA,

    /// Defunct British WPC affiliate.
    #[strum(to_string = "BPC", serialize = "bpc")]
    BPC,

    /// British Powerlifting Federation, WPU affiliate, formerly IPL/WRPF.
    #[strum(to_string = "BPF", serialize = "bpf")]
    BPF,

    /// British Powerlifting Organization, WPF.
    #[strum(to_string = "BPO", serialize = "bpo")]
    BPO,

    /// British Powerlifting Union.
    #[strum(to_string = "BPU", serialize = "bpu")]
    BPU,

    /// Bulgarian Powerlifting Federation, IPF.
    #[strum(to_string = "BulgarianPF", serialize = "bulgarianpf")]
    BulgarianPF,

    /// Bundesverbandes Deutscher Gewichtheber, pre-BVDK.
    #[strum(to_string = "BVDG", serialize = "bvdg")]
    BVDG,

    /// Bundesverband Deutscher Kraftdreikämpf, IPF.
    #[strum(to_string = "BVDK", serialize = "bvdk")]
    BVDK,

    /// Unaffiliated meets held in Canada.
    #[serde(rename = "Canada-UA")]
    #[strum(to_string = "Canada-UA", serialize = "canada-ua")]
    CanadaUA,

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

    /// Confederação Brasileira de Levantamentos Básicos, IPF.
    #[strum(to_string = "CBLB", serialize = "cblb")]
    CBLB,

    /// Central Bench Press League, US 90s "Fed"
    #[strum(to_string = "CBPL", serialize = "cbpl")]
    CBPL,

    /// Chinese Powerlifting Association, GPA.
    #[strum(to_string = "ChinaPA", serialize = "chinapa")]
    ChinaPA,

    /// Comite National de Force Athletique, old french IPF affiliate.
    #[strum(to_string = "CNFA", serialize = "cnfa")]
    CNFA,

    /// Colombian Powerlifting Federation, IPF.
    #[strum(to_string = "ColPF", serialize = "colpf")]
    ColPF,

    /// Commonwealth Powerlifting Federation, IPF.
    #[strum(to_string = "CommonwealthPF", serialize = "commonwealthpf")]
    CommonwealthPF,

    /// Canadian Powerlifting Association, WPA.
    #[strum(to_string = "CPA", serialize = "cpa")]
    CPA,

    /// Canadian Powerlifting Congress, WPC.
    #[strum(to_string = "CPC", serialize = "cpc")]
    CPC,

    /// Canadian Powerlifting Federation, GPA/IPO.
    ///
    /// * 2020-06-17: the CPF dropped their WPC affiliation over WPC's continued
    ///   acceptance of Metal gear despite the Metal owner's racism.
    ///
    /// * 2020-07-24: the CPF announced a merger with the CPL for 2021-01-01.
    ///   The CPF does not exist from that date.
    ///
    #[strum(to_string = "CPF", serialize = "cpf")]
    CPF,

    /// Confederación de Powerlifting Ibérica.
    #[strum(to_string = "CPI", serialize = "cpi")]
    CPI,

    /// Canadian Powerlifting League, IPL.
    #[strum(to_string = "CPL", serialize = "cpl")]
    CPL,

    /// Canadian Powerlifting Organization, defunct WPC affiliate.
    #[strum(to_string = "CPO", serialize = "cpo")]
    CPO,

    /// Canadian Powerlifting Union, IPF.
    #[strum(to_string = "CPU", serialize = "cpu")]
    CPU,

    /// Unaffiliated meets held in Croatia.
    #[serde(rename = "Croatia-UA")]
    #[strum(to_string = "Croatia-UA", serialize = "croatia-ua")]
    CroatiaUA,

    /// Costa Rica Powerlifting, not federated internationally
    #[strum(to_string = "CRPL", serialize = "crpl")]
    CRPL,

    /// Český svaz silového trojboje, Czech IPF affiliate.
    #[strum(to_string = "CSST", serialize = "csst")]
    CSST,

    /// Cyrpus Powerlifting Federation, Cyprus IPF affiliate.
    #[strum(to_string = "CyprusPF", serialize = "cypruspf")]
    CyprusPF,

    /// Unaffiliated meets held in Czechia.
    #[serde(rename = "Czechia-UA")]
    #[strum(to_string = "Czechia-UA", serialize = "czechia-ua")]
    CzechiaUA,

    /// Deutscher Bodybuilding und Kraftsport Verband, first German federation.
    #[strum(to_string = "DBKV", serialize = "dbkv")]
    DBKV,

    /// Drug Free Powerlifting Federation Netherlands.
    #[strum(to_string = "DFPFNL", serialize = "dfpfnl")]
    DFPFNL,

    /// Dutch Powerlifting League.
    #[strum(to_string = "DPL", serialize = "dpl")]
    DPL,

    /// Danish IPF affiliate.
    #[strum(to_string = "DSF", serialize = "dsf")]
    DSF,

    /// EGYPT POWERLIFTING FEDERATION, IPF.
    #[strum(to_string = "EgyptPF", serialize = "egyptpf")]
    EgyptPF,

    /// Estonian IPF affiliate, IPF.
    #[strum(to_string = "EJTL", serialize = "ejtl")]
    EJTL,

    /// Elite Powerlifting Canada, IPL-affiliated prior to 2018.
    #[strum(to_string = "EPC", serialize = "epc")]
    EPC,

    /// Unaffiliated meets held in England.
    #[serde(rename = "England-UA")]
    #[strum(to_string = "England-UA", serialize = "england-ua")]
    EnglandUA,

    /// English Powerlifting Association, IPF.
    #[strum(to_string = "EPA", serialize = "epa")]
    EPA,

    /// European Powerlifting Federation, IPF.
    #[strum(to_string = "EPF", serialize = "epf")]
    EPF,

    /// Ελληνικό Σωματείο Δυναμικού Τριάθλου, multi-fed Greek affiliate.
    #[strum(to_string = "ESDT", serialize = "esdt")]
    ESDT,

    /// Federación Argentina de Levantamiento de Potencia, IPF.
    #[strum(to_string = "FALPO", serialize = "falpo")]
    FALPO,

    /// FEDERATION ALGERIENNE DE POWERLIFTING, IPF.
    #[strum(to_string = "FAPL", serialize = "fapl")]
    FAPL,

    /// Federation Bench Press Double-event, Russian fed.
    #[strum(to_string = "FBPD", serialize = "fbpd")]
    FBPD,

    /// Fédération Sportive de Force Athlétique, WDFPF.
    #[strum(to_string = "FSFA", serialize = "fsfa")]
    FSFA,

    /// Fellowship of Christian Athletes, Defunct US based federation.
    #[strum(to_string = "FCA", serialize = "fca")]
    FCA,

    ///Federacao De Culturismo Epowerlifting De Portugal, Portuguese IPF Affiliate
    #[strum(to_string = "FCP", serialize = "fcp")]
    FCP,

    /// Federace českého silového trojboje, GPC.
    #[strum(to_string = "FCST", serialize = "fcst")]
    FCST,

    /// Federation Camerounaise de Powerlifting et Disciplines Affinitaires.
    /// Cameroon IPF and WDFPF affiliate.
    #[strum(to_string = "FECAPOLIF", serialize = "fecapolif")]
    FECAPOLIF,

    /// La Federación Chilena de Powerlifting, IPF.
    #[strum(to_string = "FECHIPO", serialize = "fechipo")]
    FECHIPO,

    /// Federación Nacional de Levantamiento de Potencia, Guatemalan IPF affiliate.
    #[strum(to_string = "Fedepotencia", serialize = "fedepotencia")]
    Fedepotencia,

    /// Federación de Lifterspower de México A.C., WP.
    #[strum(to_string = "FELIPOME", serialize = "felipome")]
    FELIPOME,

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

    /// Fédération Française d’Haltérophilie, Musculation, Force Athlétique et Culturisme, IPF.
    /// FFForce precursor
    #[strum(to_string = "FFHMFAC", serialize = "ffhmfac")]
    FFHMFAC,

    /// Florida High School Athletics Association.
    #[strum(to_string = "FHSAA", serialize = "fhsaa")]
    FHSAA,

    /// Federazione Italiana Atletica Pesante.
    #[strum(to_string = "FIAP", serialize = "fiap")]
    FIAP,

    /// Unaffiliated meets held in Finland.
    #[serde(rename = "Finland-UA")]
    #[strum(to_string = "Finland-UA", serialize = "finland-ua")]
    FinlandUA,

    /// Federazione Italiana Powerlifting, IPF.
    #[strum(to_string = "FIPL", serialize = "fipl")]
    FIPL,

    /// Finland Powerlifting Organization, IPA.
    #[strum(to_string = "FPO", serialize = "fpo")]
    FPO,

    /// Federación Panameña de Potencia, IPF.
    #[strum(to_string = "FPP", serialize = "fpp")]
    FPP,

    /// Federación de Powerlifting de Puerto Rico, IPF.
    #[strum(to_string = "FPPR", serialize = "fppr")]
    FPPR,

    /// Powerlifting Federation of Russia, IPF.
    #[strum(to_string = "FPR", serialize = "fpr")]
    FPR,

    /// Federatia Romana de Powerlifting, Romanian IPF affiliate.
    #[strum(to_string = "FRPL", serialize = "frpl")]
    FRPL,

    /// German Drug-Free Powerlifting Federation, WDFPF.
    #[strum(to_string = "GDFPF", serialize = "gdfpf")]
    GDFPF,

    /// Unaffiliated meets held in Germany.
    #[serde(rename = "Germany-UA")]
    #[strum(to_string = "Germany-UA", serialize = "germany-ua")]
    GermanyUA,

    /// Global Federation of Powerlifting
    #[strum(to_string = "GFP", serialize = "gfp")]
    GFP,

    /// Global Powerlifting Union, Ukrainian GPC affiliate.
    #[strum(to_string = "GlobalPU", serialize = "globalpu")]
    GlobalPU,

    /// Global Powerlifting Association.
    #[strum(to_string = "GPA", serialize = "gpa")]
    GPA,

    /// Brazilian branch of the GPA.
    #[serde(rename = "GPA-Brazil")]
    #[strum(to_string = "GPA-Brazil", serialize = "gpa-brazil")]
    GPABrazil,

    /// Colombian branch of the GPA.
    #[serde(rename = "GPA-COL")]
    #[strum(to_string = "GPA-COL", serialize = "gpa-col")]
    GPACOL,

    /// Croatian branch of the GPA.
    #[serde(rename = "GPA-CRO")]
    #[strum(to_string = "GPA-CRO", serialize = "gpa-cro")]
    GPACRO,

    /// Finnish branch of the GPA.
    #[serde(rename = "GPA-Finland")]
    #[strum(to_string = "GPA-Finland", serialize = "gpa-finland")]
    GPAFinland,

    /// Global Powerlifting Committee.
    #[strum(to_string = "GPC", serialize = "gpc")]
    GPC,

    /// Australian branch of the GPC.
    #[serde(rename = "GPC-AUS")]
    #[strum(to_string = "GPC-AUS", serialize = "gpc-aus")]
    GPCAUS,

    /// Confederacao Brasileira De Powerlifting, GPC. Formerly CONBRAP.
    #[serde(rename = "GPC-Brazil")]
    #[strum(to_string = "GPC-Brazil", serialize = "gpc-brazil")]
    GPCBrazil,

    /// Canadian branch of the GPC.
    #[serde(rename = "GPC-CAN")]
    #[strum(to_string = "GPC-CAN", serialize = "gpc-can")]
    GPCCAN,

    /// Frencg branch of the GPC.
    #[serde(rename = "GPC-France")]
    #[strum(to_string = "GPC-France", serialize = "gpc-france")]
    GPCFrance,

    /// British branch of the GPC.
    #[serde(rename = "GPC-GB")]
    #[strum(to_string = "GPC-GB", serialize = "gpc-gb")]
    GPCGB,

    /// United Powerlifting Congress Global Union Powerlifting Ukraine.
    ///
    /// Sister federation to GPC-UKR.
    #[serde(rename = "GPC-GUPU")]
    #[strum(to_string = "GPC-GUPU", serialize = "gpc-gupu")]
    GPCGUPU,

    /// Irish branch of the GPC.
    #[serde(rename = "GPC-IRL")]
    #[strum(to_string = "GPC-IRL", serialize = "gpc-irl")]
    GPCIRL,

    /// Israeli branch of the GPC.
    #[serde(rename = "GPC-ISR")]
    #[strum(to_string = "GPC-ISR", serialize = "gpc-isr")]
    GPCISR,

    /// Latvian branch of the GPC.
    #[serde(rename = "GPC-LAT")]
    #[strum(to_string = "GPC-LAT", serialize = "gpc-lat")]
    GPCLAT,

    /// New Zealand branch of the GPC.
    #[serde(rename = "GPC-NZ")]
    #[strum(to_string = "GPC-NZ", serialize = "gpc-nz")]
    GPCNZ,

    /// Poland branch of the GPC.
    #[serde(rename = "GPC-POL")]
    #[strum(to_string = "GPC-POL", serialize = "gpc-pol")]
    GPCPOL,

    /// Portugese branch of the GPC.
    #[serde(rename = "GPC-Portugal")]
    #[strum(to_string = "GPC-Portugal", serialize = "gpc-portugal")]
    GPCPortugal,

    /// Scotland branch of the GPC.
    #[serde(rename = "GPC-Scotland")]
    #[strum(to_string = "GPC-Scotland", serialize = "gpc-scotland")]
    GPCScotland,

    /// Ukrainian branch of the GPC.
    #[serde(rename = "GPC-UKR")]
    #[strum(to_string = "GPC-UKR", serialize = "gpc-ukr")]
    GPCUKR,

    /// USA branch of the GPC.
    #[serde(rename = "GPC-USA")]
    #[strum(to_string = "GPC-USA", serialize = "gpc-usa")]
    GPCUSA,

    /// Russian branch of the GPC.
    #[serde(rename = "GPC-RUS")]
    #[strum(to_string = "GPC-RUS", serialize = "gpc-rus")]
    GPCRUS,

    /// Croatian branch of the GPC. Successor to HPO.
    #[serde(rename = "GPC-CRO")]
    #[strum(to_string = "GPC-CRO", serialize = "gpc-cro")]
    GPCCRO,

    /// Global Powerlifting Federation.
    #[strum(to_string = "GPF", serialize = "gpf")]
    GPF,

    /// German Powerlifting Union, WPU.
    #[strum(to_string = "GPU", serialize = "gpu")]
    GPU,

    /// German RAW Association, IRP.
    #[strum(to_string = "GRAWA", serialize = "grawa")]
    GRAWA,

    /// Greek Powerlifting League.
    #[strum(to_string = "GPL", serialize = "gpl")]
    GPL,

    /// GSF-Belarus.
    #[serde(rename = "GSF-Belarus")]
    #[strum(to_string = "GSF-Belarus", serialize = "gsf-belarus")]
    GSFBelarus,

    /// Defunct stand-alone US federation.
    #[strum(to_string = "Hardcore", serialize = "hardcore")]
    Hardcore,

    /// Hong Kong Powerlifting Federation, WP.
    #[strum(to_string = "HKPF", serialize = "hkpf")]
    HKPF,

    /// Hong Kong Weightlifting and Powerlifting, IPF.
    #[strum(to_string = "HKWPA", serialize = "hkwpa")]
    HKWPA,

    /// Hungarian Powerlifting Congress, WPC.
    #[strum(to_string = "HPC", serialize = "hpc")]
    HPC,

    /// Hellas Powerlifting Federation, IPF.
    #[strum(to_string = "HPF", serialize = "hpf")]
    HPF,

    /// Croatian IPF affiliate.
    #[strum(to_string = "HPLS", serialize = "hpls")]
    HPLS,

    /// Croatian Powerlifting Federation before getting affiliated with the IPF.
    #[serde(rename = "HPLS-UA")]
    #[strum(to_string = "HPLS-UA", serialize = "hpls-ua")]
    HPLSUA,

    /// Croatian Powerlifting Organization. Defunct: became GPC-CRO + WUAP-CRO.
    #[strum(to_string = "HPO", serialize = "hpo")]
    HPO,

    /// Hantang Powerlifting, from China.
    #[strum(to_string = "HTPL", serialize = "htpl")]
    HTPL,

    /// Magyar Erőemelő Szövetség, Hungarian IPF affiliate.
    ///
    /// They call themselves "Hunpower" for short.
    #[strum(to_string = "Hunpower", serialize = "hunpower")]
    Hunpower,

    /// Unaffiliated meets held in Hungary.
    #[serde(rename = "Hungary-UA")]
    #[strum(to_string = "Hungary-UA", serialize = "hungary-ua")]
    HungaryUA,

    /// International Blind Sport Assocation.
    #[strum(to_string = "IBSA", serialize = "ibsa")]
    IBSA,

    /// Irish Drug-Free Powerlifting Association.
    #[strum(to_string = "IDFPA", serialize = "idfpa")]
    IDFPA,

    /// Irish Drug-Free Powerlifting Federation.
    #[strum(to_string = "IDFPF", serialize = "idfpf")]
    IDFPF,

    /// Ilinois High School Powerlifting Association.
    #[strum(to_string = "IHSPLA", serialize = "ihspla")]
    IHSPLA,

    /// Islenska Kraftlyfingafelagid, Icelandic GPC? affiliate.
    #[strum(to_string = "IKF", serialize = "ikf")]
    IKF,

    /// Israel Powerlifting Federation.
    #[strum(to_string = "ILPA", serialize = "ilpa")]
    ILPA,

    /// Israeli Powerlifting.
    #[strum(to_string = "ILPF", serialize = "ilpf")]
    ILPF,

    /// International Nova Strength Association, tiny Texas "fed".
    ///
    /// <https://web.archive.org/web/20040807073200/http://www.novafitness.com/insa_fronts/index.htm>
    #[strum(to_string = "INSA", serialize = "insa")]
    INSA,

    /// International Powerlifting Association.
    #[strum(to_string = "IPA", serialize = "ipa")]
    IPA,

    /// International Powerlifting Association, Azerbaijan
    #[serde(rename = "IPA-AZE")]
    #[strum(to_string = "IPA-AZE", serialize = "ipa-aze")]
    IPAAZE,

    /// Israel Powerlifting Community.
    #[strum(to_string = "IPC", serialize = "ipc")]
    IPC,

    /// International Powerlifting Federation.
    #[strum(to_string = "IPF", serialize = "ipf")]
    IPF,

    /// International Powerlifting Federation, China
    #[serde(rename = "IPF-China")]
    #[strum(to_string = "IPF-China", serialize = "ipf-china")]
    IPFChina,

    /// International Powerlifting League, China
    #[serde(rename = "IPL-China")]
    #[strum(to_string = "IPL-China", serialize = "ipl-china")]
    IPLChina,

    /// International Powerlifting League.
    #[strum(to_string = "IPL", serialize = "ipl")]
    IPL,

    /// International Powerlifting League, New Zealand
    #[serde(rename = "IPL-NZ")]
    #[strum(to_string = "IPL-NZ", serialize = "ipl-nz")]
    IPLNZ,

    /// International Powerlifting League, Spain
    #[serde(rename = "IPL-Spain")]
    #[strum(to_string = "IPL-Spain", serialize = "ipl-spain")]
    IPLSpain,

    /// UK International Powerlifting League
    #[serde(rename = "UKIPL")]
    #[strum(to_string = "UKIPL", serialize = "ukipl")]
    UKIPL,

    /// Iran Bodybuilding & Fitness, IPF.
    #[strum(to_string = "IranBBF", serialize = "iranbbf")]
    IranBBF,

    /// Iraq Powerlifting Federation, IPF.
    #[strum(to_string = "IraqPF", serialize = "iraqpf")]
    IraqPF,

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

    /// Iron Boy Powerlifting
    #[strum(to_string = "IronBoy", serialize = "ironboy")]
    IronBoy,

    /// International RAW Powerlifting.
    #[strum(to_string = "IRP", serialize = "irp")]
    IRP,

    /// Unaffiliated meets held in Italy.
    #[serde(rename = "Italy-UA")]
    #[strum(to_string = "Italy-UA", serialize = "italy-ua")]
    ItalyUA,

    /// Japan Powerlifting Federation, IPF.
    #[strum(to_string = "JPA", serialize = "jpa")]
    JPA,

    /// Koninklijk Belgisch Gewichtheffers Verbond, Belgian IPF affiliate.
    #[strum(to_string = "KBGV", serialize = "kbgv")]
    KBGV,

    /// Swiss Powerlifting Federation, IPF affiliate, Switzerland.
    #[strum(to_string = "KDKS", serialize = "kdks")]
    KDKS,

    /// Dutch IPF affiliate (Netherlands).
    #[serde(rename = "KNKF-SP")]
    #[strum(to_string = "KNKF-SP", serialize = "knkf-sp")]
    KNKFSP,

    /// Unaffiliated meets held in South Korea.
    #[serde(rename = "Korea-UA")]
    #[strum(to_string = "Korea-UA", serialize = "korea-ua")]
    KoreaUA,

    /// Kazakhstan IPF affiliate.
    #[strum(to_string = "KPF", serialize = "kpf")]
    KPF,

    /// Icelandic IPF affiliate.
    #[strum(to_string = "KRAFT", serialize = "kraft")]
    KRAFT,

    /// Kuwait Powerlifting Committe, IPF. Previously Kuwait Powerlifting & Armwrestling Federation
    #[strum(to_string = "KPC", serialize = "kpc")]
    KPC,

    /// Kuwait Powerlifting League, IPL.
    #[strum(to_string = "KuwaitPL", serialize = "kuwaitpl")]
    KuwaitPL,

    /// Lebanon Powerlifting Federation, IPF.
    #[strum(to_string = "LebanonPF", serialize = "lebanonpf")]
    LebanonPF,

    /// Ligue Francophone des Poids & Haltères,
    /// the Walloon Belgian IPF affiliate.
    #[strum(to_string = "LFPH", serialize = "lfph")]
    LFPH,

    /// LGBT Powerlifting.
    #[strum(to_string = "LGBT", serialize = "lgbt")]
    LGBT,

    /// Louisiana High School Powerlifting Association.
    #[strum(to_string = "LHSPLA", serialize = "lhspla")]
    LHSPLA,

    /// Libyan Powerlifting Federation, IPF.
    #[strum(to_string = "LibyaPF", serialize = "libyapf")]
    LibyaPF,

    /// Lietuvos Jėgos Trikovės Federacija, Lithuanian IPF affiliate.
    #[strum(to_string = "LJTF", serialize = "ljtf")]
    LJTF,

    /// Liga Mexicana de Powerlifting, Mexican IPL affiliate.
    #[strum(to_string = "LMP", serialize = "lmp")]
    LMP,

    /// Latvian IPF affiliate.
    #[strum(to_string = "LPF", serialize = "lpf")]
    LPF,

    /// Unaffiliated meets held in Malaysia.
    #[serde(rename = "Malaysia-UA")]
    #[strum(to_string = "Malaysia-UA", serialize = "malaysia-ua")]
    MalaysiaUA,

    /// Malta Powerlifting Association, IPF.
    #[strum(to_string = "MaltaPA", serialize = "maltapa")]
    MaltaPA,

    /// Manx Powerlifting, IPF.
    #[strum(to_string = "ManxPL", serialize = "manxpl")]
    ManxPL,

    /// Malta Drug Free Powerlifting Association.
    #[strum(to_string = "MDFPA", serialize = "mdfpa")]
    MDFPA,

    /// Moldova Drug Free Powerlifting Federation.
    #[strum(to_string = "MDFPF", serialize = "mdfpf")]
    MDFPF,

    /// Mississippi High School Athletic Association.
    #[strum(to_string = "MHSAA", serialize = "mhsaa")]
    MHSAA,

    /// Michigan High School Powerlifting Association.
    #[strum(to_string = "MHSPLA", serialize = "mhspla")]
    MHSPLA,

    /// Metal Militia, a small, independent federation.
    #[strum(to_string = "MM", serialize = "mm")]
    MM,

    /// Metal Militia Australia
    #[serde(rename = "MM-AUS")]
    #[strum(to_string = "MM-AUS", serialize = "mm-aus")]
    MMAUS,

    /// Malaysian Association for Powerlifting, IPF.
    #[strum(to_string = "MAP", serialize = "map")]
    MAP,

    /// Malaysian Powerlifting Alliance.
    #[strum(to_string = "MPA", serialize = "mpa")]
    MPA,

    /// MONGOLIAN UNITED POWERLIFTING FEDERATION, IPF.
    #[strum(to_string = "MUPF", serialize = "mupf")]
    MUPF,

    /// National Association of Powerlifting Russia, IPA.
    #[strum(to_string = "NAP", serialize = "nap")]
    NAP,

    /// North American Powerlifting Federation, IPF.
    #[strum(to_string = "NAPF", serialize = "napf")]
    NAPF,

    /// Natural Athlete Strength Assocation.
    #[strum(to_string = "NASA", serialize = "nasa")]
    NASA,

    /// Natural Powerlifting Assocation, early 80's Drug Free Fed
    #[strum(to_string = "NaturalPA", serialize = "naturalpa")]
    NaturalPA,

    /// Nauru Powerlifting Federation, IPF.
    #[strum(to_string = "NauruPF", serialize = "naurupf")]
    NauruPF,

    /// NextGenPF, a USA-IN Push/Pull/Curl federation.
    #[strum(to_string = "NextGenPF", serialize = "nextgenpf")]
    NextGenPF,

    /// Northern Ireland Powerlifting Federation.
    #[strum(to_string = "NIPF", serialize = "nipf")]
    NIPF,

    /// Northern California Powerlifting Federation.
    #[strum(to_string = "NORCAL", serialize = "norcal")]
    NORCAL,

    /// Nordic Powerlifting Federation, IPF.
    #[strum(to_string = "NordicPF", serialize = "nordicpf")]
    NordicPF,

    /// Northern Virginia Raw Powerlifting.
    #[strum(to_string = "NOVA", serialize = "nova")]
    NOVA,

    /// National Powerlifting Association of Israel.
    #[strum(to_string = "NPA", serialize = "npa")]
    NPA,

    /// National Powerlifting Association of Jamaica, IPF.
    #[strum(to_string = "NPAJ", serialize = "npaj")]
    NPAJ,

    /// National Powerlifting League.
    #[strum(to_string = "NPL", serialize = "npl")]
    NPL,

    /// Nederlandse Powerlifting Bond, IPF.
    #[strum(to_string = "NPB", serialize = "npb")]
    NPB,

    /// Norwegian IPF affiliate.
    #[strum(to_string = "NSF", serialize = "nsf")]
    NSF,

    /// Nepali Youth Fitness & Calisthetics, WP affiliate
    #[strum(to_string = "NYFC", serialize = "nyfc")]
    NYFC,

    /// New Zealand Amateur Weightlifting Association, IPF. (NPZF Precursor)
    #[strum(to_string = "NZAWLA", serialize = "nzawla")]
    NZAWLA,

    /// New Zealand Powerlifting Federation, IPF.
    #[strum(to_string = "NZPF", serialize = "nzpf")]
    NZPF,

    /// Unaffiliated meets held in NZ.
    #[serde(rename = "NZ-UA")]
    #[strum(to_string = "NZ-UA", serialize = "nz-ua")]
    NZUA,

    /// Oceania Powerlifting Federation, WP.
    #[strum(to_string = "OceaniaPF", serialize = "oceaniapf")]
    OceaniaPF,

    /// Oceania Powerlifting Organisation, WPC.
    #[strum(to_string = "OceaniaPO", serialize = "oceaniapo")]
    OceaniaPO,

    /// Oman Committee for Weightlifting and Powerlifting, IPF.
    #[strum(to_string = "OCWP", serialize = "ocwp")]
    OCWP,

    /// Oceania Regional Powerlifting Federation, IPF.
    #[strum(to_string = "ORPF", serialize = "orpf")]
    ORPF,

    /// Österreichischer Verband für Kraftdreikampf, IPF.
    #[strum(to_string = "OEVK", serialize = "oevk")]
    OEVK,

    /// Powerlifting Australia, formerly IPF, now WP.
    #[strum(to_string = "PA", serialize = "pa")]
    PA,

    /// Powerlifting Association Germany eV, WPF.
    #[strum(to_string = "PAGermany", serialize = "pagermany")]
    PAGermany,

    /// Powerlifting Association of the Philippines, IPF.
    #[strum(to_string = "PAP", serialize = "pap")]
    PAP,

    /// Armenian Powerlifting Federation, WRPF/WPA/IPU/GPC.
    #[strum(to_string = "PFA", serialize = "pfa")]
    PFA,

    ///Powerlifting Federation Brunei Darussalam, IPF
    #[strum(to_string = "PFBD", serialize = "pfbd")]
    PFBD,

    /// Philippine Powerlifting, GPA/APA.
    #[strum(to_string = "PHPL", serialize = "phpl")]
    PHPL,

    /// Powerlifting India, IPF
    #[strum(to_string = "PI", serialize = "pi")]
    PI,

    /// Powerlifting Holland, WPF.
    #[strum(to_string = "PLH", serialize = "plh")]
    PLH,

    /// Powerlifting Republica Dominicana, IPF.
    #[strum(to_string = "PLRD", serialize = "plrd")]
    PLRD,

    /// Power Lifting Savez Srbije, IPF.
    #[strum(to_string = "PLSS", serialize = "plss")]
    PLSS,

    /// Polska Liga Trójboju RAW, unaffiliated.
    #[strum(to_string = "PLTRAW", serialize = "pltraw")]
    PLTRAW,

    /// Powerlifting zveza Slovenije, IPF.
    #[strum(to_string = "PLZS", serialize = "plzs")]
    PLZS,

    /// Papua New Guinea Powerlifting Federation, IPF.
    #[strum(to_string = "PNGPF", serialize = "pngpf")]
    PNGPF,

    /// Unaffiliated meets held in Poland.
    #[serde(rename = "Poland-UA")]
    #[strum(to_string = "Poland-UA", serialize = "poland-ua")]
    PolandUA,

    /// Police Athletic League, a defunct US Fed.
    #[strum(to_string = "PoliceAL", serialize = "policeal")]
    PoliceAL,

    /// Unaffiliated meets held in Portugal.
    #[serde(rename = "Portugal-UA")]
    #[strum(to_string = "Portugal-UA", serialize = "portugal-ua")]
    PortugalUA,

    /// A defunct stand-alone US federation.
    #[strum(to_string = "PRIDE", serialize = "pride")]
    PRIDE,

    /// Australian stand-alone meets run by Markos Markopoulos.
    #[strum(to_string = "ProRaw", serialize = "proraw")]
    ProRaw,

    /// Professional Raw Powerlifting Assocation.
    #[strum(to_string = "PRPA", serialize = "prpa")]
    PRPA,

    /// Powerlifting Singapore, IPF
    #[strum(to_string = "PS", serialize = "ps")]
    PS,

    /// Powerlifting & Weightlifting Federation Luxembourg, Luxembourgish IPF affiliate.
    #[strum(to_string = "PWFL", serialize = "pwfl")]
    PWFL,

    /// Polish IPF affiliate.
    #[strum(to_string = "PZKFiTS", serialize = "pzkfits")]
    PZKFiTS,

    /// Qatar Powerlifting, IPF.
    #[strum(to_string = "QatarPL", serialize = "qatarpl")]
    QatarPL,

    /// Unaffiliated meets held in Qatar.
    #[serde(rename = "Qatar-UA")]
    #[strum(to_string = "Qatar-UA", serialize = "qatar-ua")]
    QatarUA,

    /// 100% RAW Federation, backronym of Redeemed Among the World.
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

    /// Raw Iron Powerlifting League, an independent Texas federation.
    ///
    /// * On 2020-06-20, they switched from tested to untested.
    ///
    #[strum(to_string = "RawIronPL", serialize = "rawironpl")]
    RawIronPL,

    /// Finnish Series from 2002 to ~2011
    #[strum(to_string = "RawPower", serialize = "rawpower")]
    RawPower,

    /// 100% RAW Federation Ukraine.
    #[serde(rename = "RAW-UKR")]
    #[strum(to_string = "RAW-UKR", serialize = "raw-ukr")]
    RAWUKR,

    /// Raw United Federation.
    #[strum(to_string = "RAWU", serialize = "rawu")]
    RAWU,

    /// Russian Drug Free Powerlifting Federation.
    #[strum(to_string = "RDFPF", serialize = "rdfpf")]
    RDFPF,

    /// Rhino Powerlifting Club, South African GPC Affiliate.
    #[strum(to_string = "RhinoPC", serialize = "rhinopc")]
    RhinoPC,

    /// Unaffiliated meets held in the Taiwan.
    #[strum(to_string = "Taiwan-UA", serialize = "taiwan-ua")]
    TaiwanUA,

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

    /// Strong Athletes Against Steroids, US 90s "Fed"
    #[strum(to_string = "SAAS", serialize = "saas")]
    SAAS,

    /// Unaffiliated meets in Saudi Arabia.
    #[strum(to_string = "Saudi-UA", serialize = "saudi-ua")]
    SaudiUA,    

    /// Slovenská Asociácia Fitnes, Kulturistiky a Silového Trojboja.
    /// Slovakian IPF Affilate.
    #[strum(to_string = "SAFKST", serialize = "safkst")]
    SAFKST,

    /// Syrian Arab Federation of Powerlifting, IPF.
    #[strum(to_string = "SAFP", serialize = "safp")]
    SAFP,

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

    /// Swiss Drug-Free Powerlifting Federation, Swiss WDFPF Affiliate.
    #[strum(to_string = "SDFPF", serialize = "sdfpf")]
    SDFPF,

    /// Son Light Power, US based federation.
    #[strum(to_string = "SLP", serialize = "slp")]
    SLP,

    /// SRI LANKA POWERLIFTING FEDERATION, IPF.
    #[strum(to_string = "SLPF", serialize = "slpf")]
    SLPF,

    /// Singapore Powerlifting Alliance.
    #[strum(to_string = "SPA", serialize = "spa")]
    SPA,

    /// Southern Powerlifting Federation.
    #[strum(to_string = "SPF", serialize = "spf")]
    SPF,

    /// South Australian Drug-Free Powerlifting Association.
    #[strum(to_string = "SADFPA", serialize = "sadfpa")]
    SADFPA,

    /// Southern Powerlifting Federation Ireland.
    #[serde(rename = "SPF-IRL")]
    #[strum(to_string = "SPF-IRL", serialize = "spf-irl")]
    SPFIRL,

    /// Societatem Potentis Species Sports, a defunct Russian raw federation.
    #[strum(to_string = "SPSS", serialize = "spss")]
    SPSS,

    /// Syndicated Strength Alliance.
    #[strum(to_string = "SSA", serialize = "ssa")]
    SSA,

    /// Swedish IPF affiliate.
    #[strum(to_string = "SSF", serialize = "ssf")]
    SSF,

    /// Saudi Strength Sports Committee, IPF.
    #[strum(to_string = "SSSC", serialize = "sssc")]
    SSSC,

    /// Finnish IPF affiliate.
    #[strum(to_string = "SVNL", serialize = "svnl")]
    SVNL,

    /// Swiss Powerlifting. Previously IPF affiliate. Now affiliated to many untested federations.
    #[strum(to_string = "SwissPL", serialize = "swisspl")]
    SwissPL,

    /// Thai Amateur Association of Powerlifting
    /// New IPF affiliate for Thailand as of 2023
    #[strum(to_string = "TAAP", serialize = "taap")]
    TAAP,

    /// Ex-Thai IPF affiliate.
    #[strum(to_string = "ThaiPF", serialize = "thaipf")]
    ThaiPF,

    /// Texas High School Powerlifting Association.
    #[strum(to_string = "THSPA", serialize = "thspa")]
    THSPA,

    /// Texas High School Women's Powerlifting Association.
    #[strum(to_string = "THSWPA", serialize = "thswpa")]
    THSWPA,

    /// Powerlifting Strongman ve Streetworkout Federasyonu,
    /// the Turkish IPF affiliate.
    #[strum(to_string = "TPSSF", serialize = "tpssf")]
    TPSSF,

    /// United Arab Emirates Powerlifting Association, IPF.
    ///
    /// Also called the United Arab Emirates Powerlifting Committee.
    #[strum(to_string = "UAEPL", serialize = "uaepl")]
    UAEPL,

    /// Unaffiliated meets held in the UAE.
    #[strum(to_string = "UAE-UA", serialize = "uae-ua")]
    UAEUA,

    /// Ukrainian Drug-Free Powerlifting Federation
    #[strum(to_string = "UDFPF", serialize = "udfpf")]
    UDFPF,

    /// Uganda Powerlifting Association, WPA.
    #[strum(to_string = "UgandaPA", serialize = "ugandapa")]
    UgandaPA,

    /// Uganda Powerlifting Federation, WP.
    #[strum(to_string = "UgandaPF", serialize = "ugandapf")]
    UgandaPF,

    /// Ukraine Powerlifting Association.
    #[strum(to_string = "UkrainePA", serialize = "ukrainepa")]
    UkrainePA,

    /// Ukrainian Powerlifting Federation, IPF.
    #[strum(to_string = "UkrainePF", serialize = "ukrainepf")]
    UkrainePF,

    /// Ukraine Powerlifting Organisation.
    #[strum(to_string = "UkrainePO", serialize = "ukrainepo")]
    UkrainePO,

    /// Unified Strength Alliance, short-lived NorthEastern MP fed.
    #[strum(to_string = "UnifiedSA", serialize = "unifiedsa")]
    UnifiedSA,

    /// United Powerlifting Association.
    #[strum(to_string = "UPA", serialize = "upa")]
    UPA,

    /// Ukrainian Powerlifting Committee.
    #[strum(to_string = "UPC", serialize = "upc")]
    UPC,

    /// United Powerlifting Congress Germany. WPC, GPC, WUAP.
    #[serde(rename = "UPC-Germany")]
    #[strum(to_string = "UPC-Germany", serialize = "upc-germany")]
    UPCGermany,

    /// Ukrainian Powerlifting League, IPL.
    #[strum(to_string = "UPL", serialize = "upl")]
    UPL,

    /// Ukrainian Raw Powerlifting Federation, WRPF
    #[strum(to_string = "URPF", serialize = "urpf")]
    URPF,

    /// United States Association of Blind Athletes, IBSA.
    #[strum(to_string = "USABA", serialize = "usaba")]
    USABA,

    /// USA Bench Press Association, unaffiliated.
    #[strum(to_string = "USABPA", serialize = "usabpa")]
    USABPA,

    /// Unaffiliated meets held in the USA.
    #[serde(rename = "USA-UA")]
    #[strum(to_string = "USA-UA", serialize = "usa-ua")]
    USAUA,

    /// USA Powerlifting, IPF from 1997-2021, currently unaffiliated.
    ///
    /// - On 2021-11-07, the IPF voted to remove USAPL, effective immediately.
    #[strum(to_string = "USAPL", serialize = "usapl")]
    USAPL,

    /// USA Raw Bench Press Federation (Defunct).
    #[strum(to_string = "USARawBP", serialize = "usarawbp")]
    USARawBP,

    /// Catch-all for overseas meets done by US Military members.
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

    /// United States Powerlifting Coalition, WP.
    ///
    /// The USPC splintered from USPA East in 2020 after the latter attempted to remove
    /// a convicted child rapist from their board of directors.
    #[strum(to_string = "USPC", serialize = "uspc")]
    USPC,

    /// United States Strengthlifting Federation.
    #[strum(to_string = "USSF", serialize = "ussf")]
    USSF,

    /// Unified Strength Sports Federation.
    #[strum(to_string = "USSports", serialize = "ussports")]
    USSports,

    /// US Virgin Islands Powerlifting Federation, IPF.
    #[strum(to_string = "USVIPF", serialize = "usvipf")]
    USVIPF,

    /// Victorian Drug-Free Powerlifting Association, WDFPF (via ADFPF)
    /// 1986-2014, independent (but WDFPF rules)
    /// 2015-2016, 100% Raw 2017, defunct post-2017.
    #[strum(to_string = "VDFPA", serialize = "vdfpa")]
    VDFPA,

    /// Vlaamse Gewichtheffers en Powerlifting Federatie,
    /// the Flemish Belgian IPF affiliate.
    #[strum(to_string = "VGPF", serialize = "vgpf")]
    VGPF,

    /// Vietnam Powerlifting Alliance, GPA.
    #[strum(to_string = "VietnamPA", serialize = "vietnampa")]
    VietnamPA,

    /// Unaffiliated meets held Vietnam.
    #[serde(rename = "Vietnam-UA")]
    #[strum(to_string = "Vietnam-UA", serialize = "vietnam-ua")]
    VietnamUA,

    /// Defunct Russian meet.
    #[strum(to_string = "Vityaz", serialize = "vityaz")]
    Vityaz,

    /// Vietnam Powerlifting Federation, IPF.
    #[strum(to_string = "VPF", serialize = "vpf")]
    VPF,

    /// World Association of Bench Pressers and Deadlifters.
    #[strum(to_string = "WABDL", serialize = "wabdl")]
    WABDL,

    /// World Association of Iron Athletes, a local USA federation.
    #[strum(to_string = "WAIA", serialize = "waia")]
    WAIA,

    /// Warrior Powerlifting Federation, the continuation of Son Light Power.
    ///
    /// The federation renamed itself to WarriorPLF around August 2019.
    #[strum(to_string = "WarriorPLF", serialize = "warriorplf")]
    WarriorPLF,

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

    /// World Powerlifting China.
    #[serde(rename = "WP-China")]
    #[strum(to_string = "WP-China", serialize = "wp-china")]
    WPChina,

    /// World Powerlifting India.
    #[serde(rename = "WP-India")]
    #[strum(to_string = "WP-India", serialize = "wp-india")]
    WPIndia,

    /// World Powerlifting Nauru.
    #[serde(rename = "WP-Nauru")]
    #[strum(to_string = "WP-Nauru", serialize = "wp-nauru")]
    WPNauru,

    /// WP-Niue
    #[serde(rename = "WP-Niue")]
    #[strum(to_string = "WP-Niue", serialize = "wp-niue")]
    WPNiue,

    /// WP-USA, the drug-tested half of the USPC.
    #[serde(rename = "WP-USA")]
    #[strum(to_string = "WP-USA", serialize = "wp-usa")]
    WPUSA,

    /// World Powerlifting Alliance.
    #[strum(to_string = "WPA", serialize = "wpa")]
    WPA,

    /// World Powerlifting Alliance Georgia.
    #[serde(rename = "WPA-GEO")]
    #[strum(to_string = "WPA-GEO", serialize = "wpa-geo")]
    WPAGEO,

    /// World Powerlifting Alliance Russia.
    #[serde(rename = "WPA-Poland")]
    #[strum(to_string = "WPA-Poland", serialize = "wpa-poland")]
    WPAPoland,

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

    /// World Powerlifting Committee Canada Powerlifting.
    ///
    /// Formed as the Canadian WPC affiliate after the CPF dropped WPC
    /// affiliation in 2020.
    #[strum(to_string = "WPCCP", serialize = "wpccp")]
    WPCCP,

    /// WPC meets hosted in Egypt.
    #[serde(rename = "WPC-Egypt")]
    #[strum(to_string = "WPC-Egypt", serialize = "wpc-egypt")]
    WPCEgypt,

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

    /// Indian WPC affiliate.
    #[serde(rename = "WPC-India")]
    #[strum(to_string = "WPC-India", serialize = "wpc-india")]
    WPCIndia,

    /// Israeli WPC affiliate.
    #[serde(rename = "WPC-Israel")]
    #[strum(to_string = "WPC-Israel", serialize = "wpc-israel")]
    WPCIsrael,

    /// Italian WPC affiliate.
    #[serde(rename = "WPC-Italy")]
    #[strum(to_string = "WPC-Italy", serialize = "wpc-italy")]
    WPCItaly,

    /// Kazakh WPC affiliate.
    #[serde(rename = "WPC-KAZ")]
    #[strum(to_string = "WPC-KAZ", serialize = "wpc-kaz")]
    WPCKAZ,

    /// Kyrgyzstan WPC affiliate.
    #[serde(rename = "WPC-KGZ")]
    #[strum(to_string = "WPC-KGZ", serialize = "wpc-kgz")]
    WPCKGZ,

    /// Korean WPC affiliate.
    #[serde(rename = "WPC-Korea")]
    #[strum(to_string = "WPC-Korea", serialize = "wpc-korea")]
    WPCKorea,

    /// Latvian WPC affiliate.
    #[serde(rename = "WPC-Latvia")]
    #[strum(to_string = "WPC-Latvia", serialize = "wpc-latvia")]
    WPCLatvia,

    /// Moldovan WPC affiliate.
    #[serde(rename = "WPC-Moldova")]
    #[strum(to_string = "WPC-Moldova", serialize = "wpc-moldova")]
    WPCMoldova,

    /// Polish WPC affiliate.
    #[serde(rename = "WPC-Poland")]
    #[strum(to_string = "WPC-Poland", serialize = "wpc-poland")]
    WPCPoland,

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

    /// Slovakian WPC affiliate.
    #[serde(rename = "WPC-SVK")]
    #[strum(to_string = "WPC-SVK", serialize = "wpc-svk")]
    WPCSVK,

    /// Ukrainian WPC affiliate.
    #[serde(rename = "WPC-UKR")]
    #[strum(to_string = "WPC-UKR", serialize = "wpc-ukr")]
    WPCUKR,

    /// World Powerlifting Federation.
    #[strum(to_string = "WPF", serialize = "wpf")]
    WPF,

    /// World Police and Fire Games.
    #[strum(to_string = "WPFG", serialize = "wpfg")]
    WPFG,

    /// World Powerlifting Federation KRAWA (Not a WPF affiliate..)
    #[serde(rename = "WPF-KRAWA")]
    #[strum(to_string = "WPF-KRAWA", serialize = "wpf-krawa")]
    WPFKRAWA,

    /// World Powerlifting Federation Russia.
    #[serde(rename = "WPF-RUS")]
    #[strum(to_string = "WPF-RUS", serialize = "wpf-rus")]
    WPFRUS,

    /// World Powerlifting League.
    #[strum(to_string = "WPLeague", serialize = "wpleague")]
    WPLeague,

    /// World Powerlifting New Zealand.
    #[serde(rename = "WP-NZ")]
    #[strum(to_string = "WP-NZ", serialize = "wp-nz")]
    WPNZ,

    /// World Powerlifting Organization.
    #[strum(to_string = "WPO", serialize = "wpo")]
    WPO,

    /// World Power Power League.
    #[strum(to_string = "WPPL", serialize = "wppl")]
    WPPL,

    /// World Power Power League, Argentina.
    #[serde(rename = "WPPL-Argentina")]
    #[strum(to_string = "WPPL-Argentina", serialize = "wppl-argentina")]
    WPPLArgentina,

    /// World Power Power League, Belarus.
    #[serde(rename = "WPPL-Belarus")]
    #[strum(to_string = "WPPL-Belarus", serialize = "wppl-belarus")]
    WPPLBelarus,

    /// World Power Power League, Brazil.
    #[serde(rename = "WPPL-Brazil")]
    #[strum(to_string = "WPPL-Brazil", serialize = "wppl-brazil")]
    WPPLBrazil,

    /// World Power Power League, Georgia.
    #[serde(rename = "WPPL-Georgia")]
    #[strum(to_string = "WPPL-Georgia", serialize = "wppl-georgia")]
    WPPLGeorgia,

    /// World Power Power League, Ireland.
    #[serde(rename = "WPPL-Ireland")]
    #[strum(to_string = "WPPL-Ireland", serialize = "wppl-ireland")]
    WPPLIreland,

    /// World Power Power League, Mexico.
    #[serde(rename = "WPPL-Mexico")]
    #[strum(to_string = "WPPL-Mexico", serialize = "wppl-mexico")]
    WPPLMexico,

    /// World Power Power League, Peru.
    #[serde(rename = "WPPL-Peru")]
    #[strum(to_string = "WPPL-Peru", serialize = "wppl-peru")]
    WPPLPeru,

    /// World Power Power League, Russia.
    #[serde(rename = "WPPL-Russia")]
    #[strum(to_string = "WPPL-Russia", serialize = "wppl-russia")]
    WPPLRussia,

    /// World Power Power League, Ukraine.
    #[serde(rename = "WPPL-Ukraine")]
    #[strum(to_string = "WPPL-Ukraine", serialize = "wppl-ukraine")]
    WPPLUkraine,

    /// World Paralympic Powerlifting (formerly ParaPL).
    #[strum(to_string = "WPPO", serialize = "wppo")]
    WPPO,

    /// World Powerlifting Raw Organisation.
    #[strum(to_string = "WPRO", serialize = "wpro")]
    WPRO,

    /// World Power Sport Federation.
    #[strum(to_string = "WPSF", serialize = "wpsf")]
    WPSF,

    /// World Power Sport Federation, Belarus.
    #[serde(rename = "WPSF-Belarus")]
    #[strum(to_string = "WPSF-Belarus", serialize = "wpsf-belarus")]
    WPSFBelarus,

    /// World Powerlifting Union.
    #[strum(to_string = "WPU", serialize = "wpu")]
    WPU,

    /// World Powerlifting Union of Federations.
    #[strum(to_string = "WPUF", serialize = "wpuf")]
    WPUF,

    /// World Natural Powerlifting Federation.
    /// Was briefly called NPU (Natural Powerlifters United) in 1999.
    #[strum(to_string = "WNPF", serialize = "wnpf")]
    WNPF,

    /// World Raw Powerlifting Federation.
    #[strum(to_string = "WRPF", serialize = "wrpf")]
    WRPF,

    /// Argentinian WRPF affiliate.
    ///
    /// Previously Power League, an Argentina invitational.
    #[serde(rename = "WRPF-Argentina")]
    #[strum(to_string = "WRPF-Argentina", serialize = "wrpf-argentina")]
    WRPFArgentina,

    /// Australian WRPF affiliate.
    #[serde(rename = "WRPF-AUS")]
    #[strum(to_string = "WRPF-AUS", serialize = "wrpf-aus")]
    WRPFAUS,

    /// Belarusian WRPF affiliate.
    #[serde(rename = "WRPF-Belarus")]
    #[strum(to_string = "WRPF-Belarus", serialize = "wrpf-belarus")]
    WRPFBelarus,

    /// Bolivian WRPF affiliate.
    #[serde(rename = "WRPF-Bolivia")]
    #[strum(to_string = "WRPF-Bolivia", serialize = "wrpf-bolivia")]
    WRPFBolivia,

    /// Brazilian WRPF affiliate.
    #[serde(rename = "WRPF-Brazil")]
    #[strum(to_string = "WRPF-Brazil", serialize = "wrpf-brazil")]
    WRPFBrazil,

    /// Bulgarian WRPF affiliate.
    #[serde(rename = "WRPF-Bulgaria")]
    #[strum(to_string = "WRPF-Bulgaria", serialize = "wrpf-bulgaria")]
    WRPFBulgaria,

    /// Canadian WRPF affiliate.
    #[serde(rename = "WRPF-CAN")]
    #[strum(to_string = "WRPF-CAN", serialize = "wrpf-can")]
    WRPFCAN,

    /// Chilean WRPF affiliate.
    #[serde(rename = "WRPF-Chile")]
    #[strum(to_string = "WRPF-Chile", serialize = "wrpf-chile")]
    WRPFChile,

    /// Colombian WRPF affiliate.
    #[serde(rename = "WRPF-Colombia")]
    #[strum(to_string = "WRPF-Colombia", serialize = "wrpf-colombia")]
    WRPFColombia,

    /// Croatian WRPF affiliate.
    #[serde(rename = "WRPF-CRO")]
    #[strum(to_string = "WRPF-CRO", serialize = "wrpf-cro")]
    WRPFCRO,

    /// Hungarian WRPF affiliate.
    #[serde(rename = "WRPF-HUN")]
    #[strum(to_string = "WRPF-HUN", serialize = "wrpf-hun")]
    WRPFHUN,

    /// Icelandic WRPF affiliate.
    #[serde(rename = "WRPF-Iceland")]
    #[strum(to_string = "WRPF-Iceland", serialize = "wrpf-iceland")]
    WRPFIceland,

    /// Irish WRPF affiliate.
    #[serde(rename = "WRPF-Ireland")]
    #[strum(to_string = "WRPF-Ireland", serialize = "wrpf-ireland")]
    WRPFIreland,

    /// New Irish WRPF affiliate.
    #[serde(rename = "WRPF-EIRE")]
    #[strum(to_string = "WRPF-EIRE", serialize = "wrpf-eire")]
    WRPFEIRE,

    /// Italian WRPF affiliate.
    #[serde(rename = "WRPF-Italy")]
    #[strum(to_string = "WRPF-Italy", serialize = "wrpf-italy")]
    WRPFItaly,

    /// Kazakh WRPF affiliate.
    #[serde(rename = "WRPF-KAZ")]
    #[strum(to_string = "WRPF-KAZ", serialize = "wrpf-kaz")]
    WRPFKAZ,

    /// Latvian WRPF affiliate.
    #[serde(rename = "WRPF-Latvia")]
    #[strum(to_string = "WRPF-Latvia", serialize = "wrpf-latvia")]
    WRPFLatvia,

    /// Lithuanian WRPF affiliate.
    #[serde(rename = "WRPF-Lithuania")]
    #[strum(to_string = "WRPF-Lithuania", serialize = "wrpf-lithuania")]
    WRPFLithuania,

    /// Mexican WRPF affiliate.
    #[serde(rename = "WRPF-MEX")]
    #[strum(to_string = "WRPF-MEX", serialize = "wrpf-mex")]
    WRPFMEX,

    /// Nicaraguan WRPF affiliate.
    #[serde(rename = "WRPF-NIC")]
    #[strum(to_string = "WRPF-NIC", serialize = "wrpf-nic")]
    WRPFNIC,

    /// Polish WRPF affiliate.
    #[serde(rename = "WRPF-POL")]
    #[strum(to_string = "WRPF-POL", serialize = "wrpf-pol")]
    WRPFPOL,

    /// Qatari WRPF affiliate.
    #[serde(rename = "WRPF-Qatar")]
    #[strum(to_string = "WRPF-Qatar", serialize = "wrpf-qatar")]
    WRPFQatar,

    /// Portugese WRPF affiliate.
    #[serde(rename = "WRPF-Portugal")]
    #[strum(to_string = "WRPF-Portugal", serialize = "wrpf-portugal")]
    WRPFPortugal,

    /// Slovakian WRPF affiliate.
    #[serde(rename = "WRPF-Slovakia")]
    #[strum(to_string = "WRPF-Slovakia", serialize = "wrpf-slovakia")]
    WRPFSlovakia,

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

    /// UK WRPF affiliate.
    #[serde(rename = "WRPF-UK")]
    #[strum(to_string = "WRPF-UK", serialize = "wrpf-uk")]
    WRPFUK,

    /// Vietnamese WRPF affiliate.
    #[serde(rename = "WRPF-Vietnam")]
    #[strum(to_string = "WRPF-Vietnam", serialize = "wrpf-vietnam")]
    WRPFVietnam,

    /// Washington State High School Powerlifting.
    #[strum(to_string = "WSHSPL", serialize = "wshspl")]
    WSHSPL,

    /// World United Amateur Powerlifting.
    #[strum(to_string = "WUAP", serialize = "wuap")]
    WUAP,

    /// Austrian WUAP affiliate.
    #[serde(rename = "WUAP-AUT")]
    #[strum(to_string = "WUAP-AUT", serialize = "wuap-aut")]
    WUAPAUT,

    /// Croatian WUAP affiliate. Successor to HPO.
    #[serde(rename = "WUAP-CRO")]
    #[strum(to_string = "WUAP-CRO", serialize = "wuap-cro")]
    WUAPCRO,

    /// Czechian WUAP affiliate.
    #[serde(rename = "WUAP-CZ")]
    #[strum(to_string = "WUAP-CZ", serialize = "wuap-cz")]
    WUAPCZ,

    /// German WUAP affiliate.
    #[serde(rename = "WUAP-Germany")]
    #[strum(to_string = "WUAP-Germany", serialize = "wuap-germany")]
    WUAPGermany,

    /// Russian WUAP affiliate.
    #[serde(rename = "WUAP-RUS")]
    #[strum(to_string = "WUAP-RUS", serialize = "wuap-rus")]
    WUAPRUS,

    /// Slovakian WUAP affiliate.
    #[serde(rename = "WUAP-SVK")]
    #[strum(to_string = "WUAP-SVK", serialize = "wuap-svk")]
    WUAPSVK,

    /// US WUAP affiliate.
    #[serde(rename = "WUAP-USA")]
    #[strum(to_string = "WUAP-USA", serialize = "wuap-usa")]
    WUAPUSA,

    /// Xtreme Powerlifting Coalition.
    #[strum(to_string = "XPC", serialize = "xpc")]
    XPC,

    /// Polish version of the XPC.
    #[serde(rename = "XPC-Poland")]
    #[strum(to_string = "XPC-Poland", serialize = "xpc-poland")]
    XPCPoland,

    /// Extreme Performance and Strength, formerly known as HERC.
    ///
    /// On 2021-07-29, Rheta West requested that the name change from HERC to XPS,
    /// in order to sanction events outside of the Hercules Gym.
    #[strum(to_string = "XPS", serialize = "xps")]
    XPS,
}

#[rustfmt::skip::macros(date)]
impl Federation {
    /// True if every division in the federation is drug-tested.
    pub fn is_fully_tested(self, date: Date) -> bool {
        const FULLY_TESTED: bool = true;

        match self {
            Federation::_365Strong => false,
            Federation::AAP => false,
            Federation::AAPLF => FULLY_TESTED,
            Federation::AAU => {
                // The AAU was untested in it's original form
                if date.year() >= 1995 {
                    FULLY_TESTED
                } else {
                    false
                }
            }
            Federation::ABP => false,
            Federation::ACHIPO => false,
            Federation::ACPA => false,
            Federation::ADAU => FULLY_TESTED,
            Federation::ADFPA => FULLY_TESTED,
            Federation::ADFPF => FULLY_TESTED,
            Federation::AEP => FULLY_TESTED,
            Federation::AFPF => false,
            Federation::AfricanPF => FULLY_TESTED,
            Federation::AIWBPA => FULLY_TESTED,
            Federation::AmericanSA => false,
            Federation::AMP => FULLY_TESTED,
            Federation::ANPPC => false,
            Federation::APA => false,
            Federation::APC => false,
            Federation::APF => false,
            Federation::APLA => FULLY_TESTED,
            Federation::APP => false,
            Federation::APU => FULLY_TESTED,
            Federation::APUA => FULLY_TESTED,
            Federation::ARPL => false,
            Federation::AsianPF => FULLY_TESTED,
            Federation::AusDFPF => FULLY_TESTED,
            Federation::AusPF => false,
            Federation::AusPL => false,
            Federation::AWF => false,
            Federation::BahamasPF => FULLY_TESTED,
            Federation::BAWLA => FULLY_TESTED,
            Federation::BB => false,
            Federation::BDFPA => FULLY_TESTED,
            Federation::BDFPF => FULLY_TESTED,
            Federation::BelPF => FULLY_TESTED,
            Federation::BP => FULLY_TESTED,
            Federation::BPA => FULLY_TESTED,
            Federation::BPC => false,
            Federation::BPF => false,
            Federation::BPO => false,
            Federation::BPU => false,
            Federation::BulgarianPF => FULLY_TESTED,
            Federation::BVDG => FULLY_TESTED,
            Federation::BVDK => FULLY_TESTED,
            Federation::CanadaUA => false,
            Federation::CAPO => false,
            Federation::CAPONZ => false,
            Federation::CAST => false,
            Federation::CBLB => FULLY_TESTED,
            Federation::CBPL => false,
            Federation::ChinaPA => false,
            Federation::CNFA => FULLY_TESTED,
            Federation::ColPF => FULLY_TESTED,
            Federation::CommonwealthPF => FULLY_TESTED,
            Federation::CPA => false,
            Federation::CPC => false,
            Federation::CPF => false,
            Federation::CPI => false,
            Federation::CPL => false,
            Federation::CPO => false,
            Federation::CPU => FULLY_TESTED,
            Federation::CroatiaUA => false,
            Federation::CRPL => false,
            Federation::CSST => FULLY_TESTED,
            Federation::DBKV => false,
            Federation::DFPFNL => FULLY_TESTED,
            Federation::DPL => false,
            Federation::CyprusPF => FULLY_TESTED,
            Federation::CzechiaUA => false,
            Federation::DSF => FULLY_TESTED,
            Federation::EgyptPF => FULLY_TESTED,
            Federation::EJTL => FULLY_TESTED,
            Federation::EPC => false,
            Federation::EnglandUA => false,
            Federation::EPA => FULLY_TESTED,
            Federation::EPF => FULLY_TESTED,
            Federation::ESDT => false,
            Federation::FALPO => FULLY_TESTED,
            Federation::FAPL => FULLY_TESTED,
            Federation::FBPD => false,
            Federation::FCA => false,
            Federation::FCP => FULLY_TESTED,
            Federation::FCST => false,
            Federation::FECAPOLIF => FULLY_TESTED,
            Federation::FECHIPO => FULLY_TESTED,
            Federation::Fedepotencia => FULLY_TESTED,
            Federation::FELIPOME => FULLY_TESTED,
            Federation::FEMEPO => FULLY_TESTED,
            Federation::FEPOA => false,
            Federation::FESUPO => FULLY_TESTED,
            Federation::FFForce => FULLY_TESTED,
            Federation::FFHMFAC => FULLY_TESTED,
            Federation::FHSAA => FULLY_TESTED,
            Federation::FIAP => FULLY_TESTED,
            Federation::FinlandUA => false,
            Federation::FIPL => FULLY_TESTED,
            Federation::FPO => false,
            Federation::FPP => FULLY_TESTED,
            Federation::FPPR => FULLY_TESTED,
            Federation::FPR => FULLY_TESTED,
            Federation::FRPL => FULLY_TESTED,
            Federation::FSFA => FULLY_TESTED,
            Federation::GDFPF => FULLY_TESTED,
            Federation::GermanyUA => false,
            Federation::GFP => false,
            Federation::GlobalPU => false,
            Federation::GPA => false,
            Federation::GPABrazil => false,
            Federation::GPACOL => false,
            Federation::GPACRO => false,
            Federation::GPAFinland => false,
            Federation::GPC => false,
            Federation::GPCAUS => false,
            Federation::GPCBrazil => false,
            Federation::GPCCAN => false,
            Federation::GPCFrance => false,
            Federation::GPCGB => false,
            Federation::GPCGUPU => false,
            Federation::GPCIRL => false,
            Federation::GPCISR => false,
            Federation::GPCLAT => false,
            Federation::GPCNZ => false,
            Federation::GPCPOL => false,
            Federation::GPCPortugal => false,
            Federation::GPCScotland => false,
            Federation::GPCUKR => false,
            Federation::GPCUSA => false,
            Federation::GPCRUS => false,
            Federation::GPCCRO => false,
            Federation::GPF => false,
            Federation::GPL => false,
            Federation::GPU => false,
            Federation::GRAWA => false,
            Federation::GSFBelarus => false,
            Federation::Hardcore => false,
            Federation::HKPF => FULLY_TESTED,
            Federation::HKWPA => FULLY_TESTED,
            Federation::HPC => false,
            Federation::HPF => FULLY_TESTED,
            Federation::HPLS => FULLY_TESTED,
            Federation::HPLSUA => false,
            Federation::HPO => false,
            Federation::HTPL => FULLY_TESTED,
            Federation::Hunpower => FULLY_TESTED,
            Federation::HungaryUA => false,
            Federation::IBSA => FULLY_TESTED,
            Federation::IDFPA => FULLY_TESTED,
            Federation::IDFPF => FULLY_TESTED,
            Federation::IKF => false,
            Federation::IHSPLA => FULLY_TESTED,
            Federation::ILPA => false,
            Federation::ILPF => {
                // ILPF switched to IPF and drug-tested
                if date.year() >= 2023 {
                    FULLY_TESTED
                } else {
                    false
                }
            }
            Federation::INSA => false,
            Federation::IPA => false,
            Federation::IPAAZE => false,
            Federation::IPC => false,
            Federation::IPF => FULLY_TESTED,
            Federation::IPFChina => FULLY_TESTED,
            Federation::IPLChina => false,
            Federation::IPL => false,
            Federation::IPLNZ => false,
            Federation::IPLSpain => false,
            Federation::UKIPL => false,
            Federation::IranBBF => FULLY_TESTED,
            Federation::IraqPF => FULLY_TESTED,
            Federation::IrelandUA => false,
            Federation::IrishPF => FULLY_TESTED,
            Federation::IrishPO => false,
            Federation::IronBoy => FULLY_TESTED,
            Federation::IRP => false,
            Federation::ItalyUA => false,
            Federation::JPA => FULLY_TESTED,
            Federation::KBGV => FULLY_TESTED,
            Federation::KDKS => FULLY_TESTED,
            Federation::KNKFSP => FULLY_TESTED,
            Federation::KoreaUA => false,
            Federation::KPF => FULLY_TESTED,
            Federation::KRAFT => FULLY_TESTED,
            Federation::KPC => FULLY_TESTED,
            Federation::KuwaitPL => false,
            Federation::LebanonPF => FULLY_TESTED,
            Federation::LFPH => FULLY_TESTED,
            Federation::LGBT => false,
            Federation::LHSPLA => false,
            Federation::LibyaPF => FULLY_TESTED,
            Federation::LJTF => FULLY_TESTED,
            Federation::LMP => false,
            Federation::LPF => FULLY_TESTED,
            Federation::MalaysiaUA => false,
            Federation::MaltaPA => FULLY_TESTED,
            Federation::ManxPL => FULLY_TESTED,
            Federation::MAP => FULLY_TESTED,
            Federation::MDFPA => FULLY_TESTED,
            Federation::MDFPF => FULLY_TESTED,
            Federation::MHSAA => false,
            Federation::MHSPLA => false,
            Federation::MM => false,
            Federation::MMAUS => false,
            Federation::MPA => false,
            Federation::MUPF => FULLY_TESTED,
            Federation::NAP => false,
            Federation::NAPF => FULLY_TESTED,
            Federation::NASA => FULLY_TESTED,
            Federation::NaturalPA => FULLY_TESTED,
            Federation::NauruPF => FULLY_TESTED,
            Federation::NextGenPF => false,
            Federation::NIPF => FULLY_TESTED,
            Federation::NORCAL => FULLY_TESTED,
            Federation::NordicPF => FULLY_TESTED,
            Federation::NOVA => false,
            Federation::NPA => false,
            Federation::NPAJ => FULLY_TESTED,
            Federation::NPB => FULLY_TESTED,
            Federation::NPL => false,
            Federation::NSF => FULLY_TESTED,
            Federation::NYFC => FULLY_TESTED,
            Federation::NZPF => FULLY_TESTED,
            Federation::NZAWLA => FULLY_TESTED,
            Federation::NZUA => false,
            Federation::OceaniaPF => FULLY_TESTED,
            Federation::OceaniaPO => false,
            Federation::OCWP => FULLY_TESTED,
            Federation::ORPF => FULLY_TESTED,
            Federation::OEVK => FULLY_TESTED,
            Federation::PA => FULLY_TESTED,
            Federation::PoliceAL => false,
            Federation::PAGermany => false,
            Federation::PAP => FULLY_TESTED,
            Federation::PFA => false,
            Federation::PFBD => FULLY_TESTED,
            Federation::PHPL => false,
            Federation::PI => FULLY_TESTED,
            Federation::PLH => false,
            Federation::PLRD => FULLY_TESTED,
            Federation::PLSS => FULLY_TESTED,
            Federation::PLTRAW => false,
            Federation::PLZS => FULLY_TESTED,
            Federation::PNGPF => FULLY_TESTED,
            Federation::PolandUA => false,
            Federation::PRIDE => false,
            Federation::PortugalUA => false,
            Federation::ProRaw => false,
            Federation::PRPA => false,
            Federation::PS => FULLY_TESTED,
            Federation::PWFL => FULLY_TESTED,
            Federation::PZKFiTS => FULLY_TESTED,
            Federation::QatarPL => FULLY_TESTED,
            Federation::QatarUA => false,
            Federation::RAW => FULLY_TESTED,
            Federation::RAWCAN => FULLY_TESTED,
            Federation::RAWIceland => false,
            Federation::RawIronPL => {
                // RawIronPL switched to untested in 2020.
                if date >= date!(2020-06-20) {
                    false
                } else {
                    FULLY_TESTED
                }
            }
            Federation::RawPower => false,
            Federation::RAWUKR => FULLY_TESTED,
            Federation::RAWU => false,
            Federation::RDFPF => FULLY_TESTED,
            Federation::RhinoPC => false,
            Federation::TaiwanUA => false,
            Federation::RPS => false,
            Federation::RPU => false,
            Federation::RUPC => false,
            Federation::RussiaUA => false,
            Federation::SAAS => false,
            Federation::SaudiUA => false,
            Federation::SADFPA => FULLY_TESTED,
            Federation::SAFKST => FULLY_TESTED,
            Federation::SAFP => FULLY_TESTED,
            Federation::SAPF => FULLY_TESTED,
            Federation::SAST => false,
            Federation::ScottishPL => FULLY_TESTED,
            Federation::SCI => false,
            Federation::SCT => false,
            Federation::SDFPF => FULLY_TESTED,
            Federation::SLP => false,
            Federation::SLPF => FULLY_TESTED,
            Federation::SPA => false,
            Federation::SPF => false,
            Federation::SPFIRL => false,
            Federation::SPSS => false,
            Federation::SSA => false,
            Federation::SSF => FULLY_TESTED,
            Federation::SSSC => FULLY_TESTED,
            Federation::SVNL => FULLY_TESTED,
            Federation::SwissPL => false,
            Federation::TAAP => FULLY_TESTED,
            Federation::ThaiPF => FULLY_TESTED,
            Federation::THSPA => FULLY_TESTED,
            Federation::THSWPA => FULLY_TESTED,
            Federation::TPSSF => FULLY_TESTED,
            Federation::UAEPL => FULLY_TESTED,
            Federation::UAEUA => false,
            Federation::UDFPF => FULLY_TESTED,
            Federation::UgandaPA => false,
            Federation::UgandaPF => FULLY_TESTED,
            Federation::UkrainePA => false,
            Federation::UkrainePO => false,
            Federation::UnifiedSA => false,
            Federation::UPA => false,
            Federation::UPC => false,
            Federation::UPCGermany => false,
            Federation::UkrainePF => FULLY_TESTED,
            Federation::UPL => false,
            Federation::URPF => false,
            Federation::USABA => FULLY_TESTED,
            Federation::USABPA => false,
            Federation::USAUA => false,
            Federation::USAPL => FULLY_TESTED,
            Federation::USARawBP => false,
            Federation::USMilAbroad => false,
            Federation::USPS => false,
            Federation::USPF => false,
            Federation::USPA => false,
            Federation::USPC => false,
            Federation::USSF => false,
            Federation::USSports => false,
            Federation::USVIPF => FULLY_TESTED,
            Federation::VDFPA => FULLY_TESTED,
            Federation::VGPF => FULLY_TESTED,
            Federation::VietnamPA => false,
            Federation::VietnamUA => false,
            Federation::Vityaz => false,
            Federation::VPF => FULLY_TESTED,
            Federation::WABDL => FULLY_TESTED,
            Federation::WAIA => false,
            Federation::WarriorPLF => false,
            Federation::WDFPF => FULLY_TESTED,
            Federation::WelshPA => FULLY_TESTED,
            Federation::WNPF => FULLY_TESTED,
            Federation::WP => FULLY_TESTED,
            Federation::WPChina => FULLY_TESTED,
            Federation::WPIndia => FULLY_TESTED,
            Federation::WPNauru => FULLY_TESTED,
            Federation::WPNiue => FULLY_TESTED,
            Federation::WPUSA => FULLY_TESTED,
            Federation::WPA => false,
            Federation::WPAGEO => false,
            Federation::WPAPoland => false,
            Federation::WPARUS => false,
            Federation::WPAU => false,
            Federation::WBC => false,
            Federation::WPC => false,
            Federation::WPCCP => false,
            Federation::WPCEgypt => false,
            Federation::WPCFinland => false,
            Federation::WPCFrance => false,
            Federation::WPCGermany => false,
            Federation::WPCIceland => false,
            Federation::WPCIndia => false,
            Federation::WPCIsrael => false,
            Federation::WPCItaly => false,
            Federation::WPCKAZ => false,
            Federation::WPCKGZ => false,
            Federation::WPCKorea => false,
            Federation::WPCLatvia => false,
            Federation::WPCMoldova => false,
            Federation::WPCPortugal => false,
            Federation::WPCPoland => false,
            Federation::WPCRUS => false,
            Federation::WPCSA => false,
            Federation::WPCSVK => false,
            Federation::WPCUKR => false,
            Federation::WPF => false,
            Federation::WPFG => false,
            Federation::WPFKRAWA => false,
            Federation::WPFRUS => false,
            Federation::WPLeague => false,
            Federation::WPNZ => FULLY_TESTED,
            Federation::WPO => false,
            Federation::WPPL => false,
            Federation::WPPLArgentina => false,
            Federation::WPPLBelarus => false,
            Federation::WPPLBrazil => false,
            Federation::WPPLGeorgia => false,
            Federation::WPPLIreland => false,
            Federation::WPPLMexico => false,
            Federation::WPPLPeru => false,
            Federation::WPPLRussia => false,
            Federation::WPPLUkraine => false,
            Federation::WPPO => FULLY_TESTED,
            Federation::WPRO => false,
            Federation::WPSF => false,
            Federation::WPSFBelarus => false,
            Federation::WPU => false,
            Federation::WPUF => false,
            Federation::WRPF => false,
            Federation::WRPFArgentina => false,
            Federation::WRPFAUS => false,
            Federation::WRPFBelarus => false,
            Federation::WRPFBolivia => false,
            Federation::WRPFBrazil => false,
            Federation::WRPFBulgaria => false,
            Federation::WRPFCAN => false,
            Federation::WRPFChile => false,
            Federation::WRPFColombia => false,
            Federation::WRPFCRO => false,
            Federation::WRPFEIRE => false,
            Federation::WRPFHUN => false,
            Federation::WRPFIceland => false,
            Federation::WRPFIreland => false,
            Federation::WRPFItaly => false,
            Federation::WRPFKAZ => false,
            Federation::WRPFLatvia => false,
            Federation::WRPFLithuania => false,
            Federation::WRPFMEX => false,
            Federation::WRPFNIC => false,
            Federation::WRPFPOL => false,
            Federation::WRPFQatar => false,
            Federation::WRPFPortugal => false,
            Federation::WRPFSlovakia => false,
            Federation::WRPFSlovenia => false,
            Federation::WRPFSpain => false,
            Federation::WRPFSRB => false,
            Federation::WRPFSweden => false,
            Federation::WRPFUK => false,
            Federation::WRPFVietnam => false,
            Federation::WSHSPL => false,
            Federation::WUAP => false,
            Federation::WUAPAUT => false,
            Federation::WUAPCRO => false,
            Federation::WUAPCZ => false,
            Federation::WUAPGermany => false,
            Federation::WUAPRUS => false,
            Federation::WUAPSVK => false,
            Federation::WUAPUSA => false,
            Federation::XPC => false,
            Federation::XPCPoland => false,
            Federation::XPS => false,
        }
    }

    /// Country out of which the federation operates.
    pub fn home_country(self) -> Option<Country> {
        match self {
            Federation::_365Strong => Some(Country::USA),
            Federation::AAP => Some(Country::Argentina),
            Federation::AAPLF => Some(Country::Australia),
            Federation::AAU => Some(Country::USA),
            Federation::ABP => Some(Country::Bolivia),
            Federation::ACHIPO => Some(Country::Chile),
            Federation::ACPA => None,
            Federation::ADAU => Some(Country::USA),
            Federation::ADFPA => Some(Country::USA),
            Federation::ADFPF => Some(Country::USA),
            Federation::AEP => Some(Country::Spain),
            Federation::AFPF => Some(Country::USA),
            Federation::AfricanPF => None,
            Federation::AIWBPA => Some(Country::Indonesia),
            Federation::AmericanSA => Some(Country::USA),
            Federation::AMP => Some(Country::USA),
            Federation::ANPPC => Some(Country::USA),
            Federation::APA => Some(Country::USA),
            Federation::APC => Some(Country::USA),
            Federation::APF => Some(Country::USA),
            Federation::APLA => Some(Country::Australia),
            Federation::APP => Some(Country::Paraguay),
            Federation::APU => Some(Country::Australia),
            Federation::APUA => Some(Country::Argentina),
            Federation::ARPL => Some(Country::Argentina),
            Federation::AsianPF => None,
            Federation::AusDFPF => Some(Country::Australia),
            Federation::AusPF => Some(Country::Australia),
            Federation::AusPL => Some(Country::Australia),
            Federation::AWF => Some(Country::Australia),
            Federation::BahamasPF => Some(Country::Bahamas),
            Federation::BAWLA => Some(Country::UK),
            Federation::BB => Some(Country::Russia),
            Federation::BDFPA => Some(Country::UK),
            Federation::BDFPF => Some(Country::Belgium),
            Federation::BelPF => Some(Country::Belarus),
            Federation::BP => Some(Country::UK),
            Federation::BPA => Some(Country::Belize),
            Federation::BPC => Some(Country::UK),
            Federation::BPF => Some(Country::UK),
            Federation::BPO => Some(Country::UK),
            Federation::BPU => Some(Country::UK),
            Federation::BulgarianPF => Some(Country::Bulgaria),
            Federation::BVDG => Some(Country::Germany),
            Federation::BVDK => Some(Country::Germany),
            Federation::CanadaUA => Some(Country::Canada),
            Federation::CAPO => Some(Country::Australia),
            Federation::CAPONZ => Some(Country::NewZealand),
            Federation::CAST => Some(Country::Czechia),
            Federation::CBLB => Some(Country::Brazil),
            Federation::CBPL => Some(Country::USA),
            Federation::ChinaPA => Some(Country::China),
            Federation::CNFA => Some(Country::France),
            Federation::ColPF => Some(Country::Colombia),
            Federation::CommonwealthPF => None,
            Federation::CPA => Some(Country::Canada),
            Federation::CPC => Some(Country::Canada),
            Federation::CPF => Some(Country::Canada),
            Federation::CPI => Some(Country::Spain),
            Federation::CPL => Some(Country::Canada),
            Federation::CPO => Some(Country::Canada),
            Federation::CPU => Some(Country::Canada),
            Federation::CRPL => Some(Country::CostaRica),
            Federation::CSST => Some(Country::Czechia),
            Federation::DBKV => Some(Country::Germany),
            Federation::DFPFNL => Some(Country::Netherlands),
            Federation::DPL => Some(Country::Netherlands),
            Federation::CyprusPF => Some(Country::Cyprus),
            Federation::CzechiaUA => Some(Country::Czechia),
            Federation::DSF => Some(Country::Denmark),
            Federation::EgyptPF => Some(Country::Egypt),
            Federation::EJTL => Some(Country::Estonia),
            Federation::EPC => Some(Country::Canada),
            Federation::EnglandUA => Some(Country::England),
            Federation::EPA => Some(Country::England),
            Federation::EPF => None,
            Federation::ESDT => Some(Country::Greece),
            Federation::FALPO => Some(Country::Argentina),
            Federation::FAPL => Some(Country::Algeria),
            Federation::FBPD => Some(Country::Russia),
            Federation::FCA => Some(Country::USA),
            Federation::FCP => Some(Country::Portugal),
            Federation::FCST => Some(Country::Czechia),
            Federation::FECAPOLIF => Some(Country::Cameroon),
            Federation::FECHIPO => Some(Country::Chile),
            Federation::Fedepotencia => Some(Country::Guatemala),
            Federation::FELIPOME => Some(Country::Mexico),
            Federation::FEMEPO => Some(Country::Mexico),
            Federation::FEPOA => Some(Country::Argentina),
            Federation::FESUPO => None,
            Federation::FFForce => Some(Country::France),
            Federation::FFHMFAC => Some(Country::France),
            Federation::FHSAA => Some(Country::USA),
            Federation::FIAP => Some(Country::Italy),
            Federation::FinlandUA => Some(Country::Finland),
            Federation::FIPL => Some(Country::Italy),
            Federation::FPO => Some(Country::Finland),
            Federation::FPP => Some(Country::Panama),
            Federation::FPPR => Some(Country::PuertoRico),
            Federation::FPR => Some(Country::Russia),
            Federation::FRPL => Some(Country::Romania),
            Federation::FSFA => Some(Country::France),
            Federation::GDFPF => Some(Country::Germany),
            Federation::GermanyUA => Some(Country::Germany),
            Federation::GFP => Some(Country::Russia),
            Federation::GlobalPU => Some(Country::Ukraine),
            Federation::GPA => None,
            Federation::GPABrazil => Some(Country::Brazil),
            Federation::GPACOL => Some(Country::Colombia),
            Federation::GPACRO => Some(Country::Croatia),
            Federation::GPAFinland => Some(Country::Finland),
            Federation::GPC => None,
            Federation::GPCAUS => Some(Country::Australia),
            Federation::GPCBrazil => Some(Country::Brazil),
            Federation::GPCCAN => Some(Country::Canada),
            Federation::GPCFrance => Some(Country::France),
            Federation::GPCGB => Some(Country::UK),
            Federation::GPCGUPU => Some(Country::Ukraine),
            Federation::GPCIRL => Some(Country::Ireland),
            Federation::GPCISR => Some(Country::Israel),
            Federation::GPCLAT => Some(Country::Latvia),
            Federation::GPCNZ => Some(Country::NewZealand),
            Federation::GPCPOL => Some(Country::Poland),
            Federation::GPCPortugal => Some(Country::Portugal),
            Federation::GPCScotland => Some(Country::Scotland),
            Federation::GPCUKR => Some(Country::Ukraine),
            Federation::GPCUSA => Some(Country::USA),
            Federation::GPCRUS => Some(Country::Russia),
            Federation::GPCCRO => Some(Country::Croatia),
            Federation::GPF => None,
            Federation::GPL => Some(Country::Greece),
            Federation::GPU => Some(Country::Germany),
            Federation::GRAWA => Some(Country::Germany),
            Federation::GSFBelarus => Some(Country::Belarus),
            Federation::Hardcore => Some(Country::USA),
            Federation::HKPF => Some(Country::HongKong),
            Federation::CroatiaUA => Some(Country::Croatia),
            Federation::HKWPA => Some(Country::HongKong),
            Federation::HPC => Some(Country::Hungary),
            Federation::HPF => Some(Country::Greece),
            Federation::HPLS => Some(Country::Croatia),
            Federation::HPLSUA => Some(Country::Croatia),
            Federation::HPO => Some(Country::Croatia),
            Federation::HTPL => Some(Country::China),
            Federation::Hunpower => Some(Country::Hungary),
            Federation::HungaryUA => Some(Country::Hungary),
            Federation::IBSA => None,
            Federation::IDFPA => Some(Country::Ireland),
            Federation::IDFPF => Some(Country::Ireland),
            Federation::IHSPLA => Some(Country::USA),
            Federation::IKF => Some(Country::Iceland),
            Federation::ILPA => Some(Country::Israel),
            Federation::ILPF => Some(Country::Israel),
            Federation::INSA => Some(Country::USA),
            Federation::IPA => Some(Country::USA),
            Federation::IPAAZE => Some(Country::Azerbaijan),
            Federation::IPC => Some(Country::Israel),
            Federation::IPF => None,
            Federation::IPFChina => Some(Country::China),
            Federation::IPLChina => Some(Country::China),
            Federation::IPL => None,
            Federation::IPLNZ => Some(Country::NewZealand),
            Federation::IPLSpain => Some(Country::Spain),
            Federation::UKIPL => Some(Country::UK),
            Federation::IranBBF => Some(Country::Iran),
            Federation::IraqPF => Some(Country::Iraq),
            Federation::IrelandUA => Some(Country::Ireland),
            Federation::IrishPF => Some(Country::Ireland),
            Federation::IrishPO => Some(Country::Ireland),
            Federation::IronBoy => Some(Country::USA),
            Federation::IRP => None,
            Federation::ItalyUA => Some(Country::Italy),
            Federation::JPA => Some(Country::Japan),
            Federation::KBGV => Some(Country::Belgium),
            Federation::KDKS => Some(Country::Switzerland),
            Federation::KNKFSP => Some(Country::Netherlands),
            Federation::KoreaUA => Some(Country::SouthKorea),
            Federation::KPF => Some(Country::Kazakhstan),
            Federation::KRAFT => Some(Country::Iceland),
            Federation::KPC => Some(Country::Kuwait),
            Federation::KuwaitPL => Some(Country::Kuwait),
            Federation::LebanonPF => Some(Country::Lebanon),
            Federation::LFPH => Some(Country::Belgium),
            Federation::LGBT => None,
            Federation::LHSPLA => Some(Country::USA),
            Federation::LibyaPF => Some(Country::Libya),
            Federation::LJTF => Some(Country::Lithuania),
            Federation::LMP => Some(Country::Mexico),
            Federation::LPF => Some(Country::Latvia),
            Federation::MalaysiaUA => Some(Country::Malaysia),
            Federation::MaltaPA => Some(Country::Malta),
            Federation::ManxPL => Some(Country::IsleOfMan),
            Federation::MAP => Some(Country::Malaysia),
            Federation::MHSAA => Some(Country::USA),
            Federation::MDFPA => Some(Country::Malta),
            Federation::MDFPF => Some(Country::Moldova),
            Federation::MHSPLA => Some(Country::USA),
            Federation::MM => Some(Country::USA),
            Federation::MMAUS => Some(Country::Australia),
            Federation::MPA => Some(Country::Malaysia),
            Federation::MUPF => Some(Country::Mongolia),
            Federation::NAP => Some(Country::Russia),
            Federation::NAPF => None,
            Federation::NASA => Some(Country::USA),
            Federation::NaturalPA => Some(Country::USA),
            Federation::NauruPF => Some(Country::Nauru),
            Federation::NextGenPF => Some(Country::USA),
            Federation::NORCAL => Some(Country::USA),
            Federation::NIPF => Some(Country::NorthernIreland),
            Federation::NordicPF => None,
            Federation::NOVA => Some(Country::USA),
            Federation::NPA => Some(Country::Israel),
            Federation::NPAJ => Some(Country::Jamaica),
            Federation::NPB => Some(Country::Netherlands),
            Federation::NPL => Some(Country::USA),
            Federation::NSF => Some(Country::Norway),
            Federation::NYFC => Some(Country::Nepal),
            Federation::NZPF => Some(Country::NewZealand),
            Federation::NZAWLA => Some(Country::NewZealand),
            Federation::NZUA => Some(Country::NewZealand),
            Federation::OceaniaPF => None,
            Federation::OceaniaPO => Some(Country::Australia),
            Federation::OCWP => Some(Country::Oman),
            Federation::ORPF => None,
            Federation::OEVK => Some(Country::Austria),
            Federation::PA => Some(Country::Australia),
            Federation::PoliceAL => Some(Country::USA),
            Federation::PAGermany => Some(Country::Germany),
            Federation::PAP => Some(Country::Philippines),
            Federation::PFA => Some(Country::Armenia),
            Federation::PFBD => Some(Country::Brunei),
            Federation::PHPL => Some(Country::Philippines),
            Federation::PI => Some(Country::India),
            Federation::PLH => Some(Country::Netherlands),
            Federation::PLRD => Some(Country::DominicanRepublic),
            Federation::PLSS => Some(Country::Serbia),
            Federation::PLTRAW => Some(Country::Poland),
            Federation::PLZS => Some(Country::Slovenia),
            Federation::PNGPF => Some(Country::PapuaNewGuinea),
            Federation::PolandUA => Some(Country::Poland),
            Federation::PRIDE => Some(Country::USA),
            Federation::PortugalUA => Some(Country::Portugal),
            Federation::ProRaw => Some(Country::Australia),
            Federation::PRPA => Some(Country::USA),
            Federation::PS => Some(Country::Singapore),
            Federation::PWFL => Some(Country::Luxembourg),
            Federation::PZKFiTS => Some(Country::Poland),
            Federation::QatarPL => Some(Country::Qatar),
            Federation::QatarUA => Some(Country::Qatar),
            Federation::RAW => Some(Country::USA),
            Federation::RAWCAN => Some(Country::Canada),
            Federation::RAWIceland => Some(Country::Iceland),
            Federation::RawIronPL => Some(Country::USA),
            Federation::RawPower => Some(Country::Finland),
            Federation::RAWUKR => Some(Country::Ukraine),
            Federation::RAWU => Some(Country::USA),
            Federation::RDFPF => Some(Country::Russia),
            Federation::RhinoPC => Some(Country::SouthAfrica),
            Federation::TaiwanUA => Some(Country::Taiwan),
            Federation::RPS => Some(Country::USA),
            Federation::RPU => Some(Country::Russia),
            Federation::RUPC => Some(Country::USA),
            Federation::RussiaUA => Some(Country::Russia),
            Federation::SAAS => Some(Country::USA),
            Federation::SaudiUA => Some(Country::SaudiArabia),
            Federation::SADFPA => Some(Country::Australia),
            Federation::SAFKST => Some(Country::Slovakia),
            Federation::SAFP => Some(Country::Syria),
            Federation::SAPF => Some(Country::SouthAfrica),
            Federation::SAST => Some(Country::Slovakia),
            Federation::ScottishPL => Some(Country::Scotland),
            Federation::SCI => Some(Country::USA),
            Federation::SCT => Some(Country::Russia),
            Federation::SDFPF => Some(Country::Switzerland),
            Federation::SLP => Some(Country::USA),
            Federation::SLPF => Some(Country::SriLanka),
            Federation::SPA => Some(Country::Singapore),
            Federation::SPF => Some(Country::USA),
            Federation::SPFIRL => Some(Country::Ireland),
            Federation::SPSS => Some(Country::Russia),
            Federation::SSA => Some(Country::USA),
            Federation::SSF => Some(Country::Sweden),
            Federation::SSSC => Some(Country::SaudiArabia),
            Federation::SVNL => Some(Country::Finland),
            Federation::SwissPL => Some(Country::Switzerland),
            Federation::TAAP => Some(Country::Thailand),
            Federation::ThaiPF => Some(Country::Thailand),
            Federation::THSPA => Some(Country::USA),
            Federation::THSWPA => Some(Country::USA),
            Federation::TPSSF => Some(Country::Turkey),
            Federation::UAEPL => Some(Country::UAE),
            Federation::UAEUA => Some(Country::UAE),
            Federation::UDFPF => Some(Country::Ukraine),
            Federation::UgandaPA => Some(Country::Uganda),
            Federation::UgandaPF => Some(Country::Uganda),
            Federation::UkrainePA => Some(Country::Ukraine),
            Federation::UkrainePO => Some(Country::Ukraine),
            Federation::UnifiedSA => Some(Country::USA),
            Federation::UPA => Some(Country::USA),
            Federation::UPC => Some(Country::Ukraine),
            Federation::UPCGermany => Some(Country::Germany),
            Federation::UkrainePF => Some(Country::Ukraine),
            Federation::UPL => Some(Country::Ukraine),
            Federation::URPF => Some(Country::Ukraine),
            Federation::USABA => Some(Country::USA),
            Federation::USABPA => Some(Country::USA),
            Federation::USAUA => Some(Country::USA),
            Federation::USAPL => Some(Country::USA),
            Federation::USARawBP => Some(Country::USA),
            Federation::USMilAbroad => Some(Country::USA),
            Federation::USPS => Some(Country::Serbia),
            Federation::USPF => Some(Country::USA),
            Federation::USPA => Some(Country::USA),
            Federation::USPC => Some(Country::USA),
            Federation::USSF => Some(Country::USA),
            Federation::USSports => Some(Country::USA),
            Federation::USVIPF => Some(Country::USVirginIslands),
            Federation::VDFPA => Some(Country::Australia),
            Federation::VGPF => Some(Country::Belgium),
            Federation::VietnamPA => Some(Country::Vietnam),
            Federation::VietnamUA => Some(Country::Vietnam),
            Federation::Vityaz => Some(Country::Russia),
            Federation::VPF => Some(Country::Vietnam),
            Federation::WABDL => Some(Country::USA),
            Federation::WAIA => Some(Country::USA),
            Federation::WarriorPLF => Some(Country::USA),
            Federation::WBC => Some(Country::USA),
            Federation::WDFPF => None,
            Federation::WelshPA => Some(Country::Wales),
            Federation::WP => None,
            Federation::WPChina => Some(Country::China),
            Federation::WPIndia => Some(Country::India),
            Federation::WPNauru => Some(Country::Nauru),
            Federation::WPNiue => Some(Country::Niue),
            Federation::WPUSA => Some(Country::USA),
            Federation::WPA => None,
            Federation::WPAGEO => Some(Country::Georgia),
            Federation::WPAPoland => Some(Country::Poland),
            Federation::WPARUS => Some(Country::Russia),
            Federation::WPAU => Some(Country::Ukraine),
            Federation::WPC => None,
            Federation::WPCCP => Some(Country::Canada),
            Federation::WPCEgypt => Some(Country::Egypt),
            Federation::WPCFinland => Some(Country::Finland),
            Federation::WPCFrance => Some(Country::France),
            Federation::WPCGermany => Some(Country::Germany),
            Federation::WPCIceland => Some(Country::Iceland),
            Federation::WPCIndia => Some(Country::India),
            Federation::WPCIsrael => Some(Country::Israel),
            Federation::WPCItaly => Some(Country::Italy),
            Federation::WPCKAZ => Some(Country::Kazakhstan),
            Federation::WPCKGZ => Some(Country::Kyrgyzstan),
            Federation::WPCKorea => Some(Country::SouthKorea),
            Federation::WPCLatvia => Some(Country::Latvia),
            Federation::WPCMoldova => Some(Country::Moldova),
            Federation::WPCPortugal => Some(Country::Portugal),
            Federation::WPCPoland => Some(Country::Poland),
            Federation::WPCRUS => Some(Country::Russia),
            Federation::WPCSA => Some(Country::SouthAfrica),
            Federation::WPCSVK => Some(Country::Slovakia),
            Federation::WPCUKR => Some(Country::Ukraine),
            Federation::WPF => None,
            Federation::WPFG => None,
            Federation::WPFKRAWA => Some(Country::Ukraine),
            Federation::WPFRUS => Some(Country::Russia),
            Federation::WPLeague => Some(Country::Ukraine),
            Federation::WPNZ => Some(Country::NewZealand),
            Federation::WPO => None,
            Federation::WPPL => None,
            Federation::WPPLArgentina => Some(Country::Argentina),
            Federation::WPPLBelarus => Some(Country::Belarus),
            Federation::WPPLBrazil => Some(Country::Brazil),
            Federation::WPPLGeorgia => Some(Country::Georgia),
            Federation::WPPLIreland => Some(Country::Ireland),
            Federation::WPPLMexico => Some(Country::Mexico),
            Federation::WPPLPeru => Some(Country::Peru),
            Federation::WPPLRussia => Some(Country::Russia),
            Federation::WPPLUkraine => Some(Country::Ukraine),
            Federation::WPPO => None,
            Federation::WPRO => Some(Country::Ukraine),
            Federation::WPSF => None,
            Federation::WPSFBelarus => Some(Country::Belarus),
            Federation::WPU => None,
            Federation::WPUF => Some(Country::Ukraine),
            Federation::WNPF => Some(Country::USA),
            Federation::WRPF => Some(Country::Russia),
            Federation::WRPFArgentina => Some(Country::Argentina),
            Federation::WRPFAUS => Some(Country::Australia),
            Federation::WRPFBelarus => Some(Country::Belarus),
            Federation::WRPFBolivia => Some(Country::Bolivia),
            Federation::WRPFBrazil => Some(Country::Brazil),
            Federation::WRPFBulgaria => Some(Country::Bulgaria),
            Federation::WRPFCAN => Some(Country::Canada),
            Federation::WRPFChile => Some(Country::Chile),
            Federation::WRPFColombia => Some(Country::Colombia),
            Federation::WRPFCRO => Some(Country::Croatia),
            Federation::WRPFEIRE => Some(Country::Ireland),
            Federation::WRPFHUN => Some(Country::Hungary),
            Federation::WRPFIceland => Some(Country::Iceland),
            Federation::WRPFIreland => Some(Country::Ireland),
            Federation::WRPFItaly => Some(Country::Italy),
            Federation::WRPFKAZ => Some(Country::Kazakhstan),
            Federation::WRPFLatvia => Some(Country::Latvia),
            Federation::WRPFLithuania => Some(Country::Lithuania),
            Federation::WRPFMEX => Some(Country::Mexico),
            Federation::WRPFNIC => Some(Country::Nicaragua),
            Federation::WRPFPOL => Some(Country::Poland),
            Federation::WRPFQatar => Some(Country::Qatar),
            Federation::WRPFPortugal => Some(Country::Portugal),
            Federation::WRPFSlovakia => Some(Country::Slovakia),
            Federation::WRPFSlovenia => Some(Country::Slovenia),
            Federation::WRPFSpain => Some(Country::Spain),
            Federation::WRPFSRB => Some(Country::Serbia),
            Federation::WRPFSweden => Some(Country::Sweden),
            Federation::WRPFUK => Some(Country::UK),
            Federation::WRPFVietnam => Some(Country::Vietnam),
            Federation::WSHSPL => Some(Country::USA),
            Federation::WUAP => None,
            Federation::WUAPAUT => Some(Country::Austria),
            Federation::WUAPCRO => Some(Country::Croatia),
            Federation::WUAPCZ => Some(Country::Czechia),
            Federation::WUAPGermany => Some(Country::Germany),
            Federation::WUAPRUS => Some(Country::Russia),
            Federation::WUAPSVK => Some(Country::Slovakia),
            Federation::WUAPUSA => Some(Country::USA),
            Federation::XPC => Some(Country::USA),
            Federation::XPCPoland => Some(Country::Poland),
            Federation::XPS => Some(Country::USA),
        }
    }
    /// The parent federation that provides sanction, if any.
    pub fn sanctioning_body(self, date: Date) -> Option<Federation> {
        match self {
            Federation::_365Strong => None,
            Federation::AAP => Some(Federation::GPA),
            Federation::AAPLF => Some(Federation::IPF),
            Federation::AAU => None,
            Federation::ABP => Some(Federation::GPA),
            Federation::ACHIPO => Some(Federation::GPA),
            Federation::ACPA => Some(Federation::WPA),
            Federation::ADAU => None,
            Federation::ADFPA => {
                // The ADFPA replaced the USPF as IPF affiliate in late 1997.
                if date >= date!(1997-12-05) {
                    Some(Federation::IPF)
                } else {
                    None
                }
            }
            Federation::ADFPF => Some(Federation::WDFPF),
            Federation::AEP => Some(Federation::IPF),
            Federation::AFPF => None,
            Federation::AfricanPF => Some(Federation::IPF),
            Federation::AIWBPA => Some(Federation::IPF),
            Federation::AmericanSA => None,
            Federation::AMP => Some(Federation::IPF),
            Federation::ANPPC => None,
            Federation::APA => Some(Federation::WPA),
            Federation::APC => Some(Federation::WUAP),
            Federation::APF => Some(Federation::WPC),
            Federation::APLA => Some(Federation::IPF),
            Federation::APP => Some(Federation::GPA),
            Federation::APU => {
                // The APU withdrew association with the IPF and affiliated with WDFPF from 2024-01-01 onwards.
                if date >= date!(2024-01-01) {
                    Some(Federation::WDFPF)
                } else {
                    Some(Federation::IPF)
                }
            }
            Federation::APUA => Some(Federation::WABDL),
            Federation::ARPL => Some(Federation::IPL),
            Federation::AsianPF => Some(Federation::IPF),
            Federation::AusDFPF => Some(Federation::WDFPF),
            Federation::AusPF => Some(Federation::IPF),
            Federation::AusPL => {
                // AusPL dropped affiliation due to harassment scandals in the USA affiliate.
                if date >= date!(2023-02-08) {
                    None
                } else {
                    Some(Federation::IPL)
                }
            }
            Federation::AWF => None,
            Federation::BahamasPF => Some(Federation::IPF),
            Federation::BAWLA => Some(Federation::IPF),
            Federation::BB => None,
            Federation::BDFPA => Some(Federation::WDFPF),
            Federation::BDFPF => Some(Federation::WDFPF),
            Federation::BelPF => Some(Federation::IPF),
            Federation::BP => Some(Federation::IPF),
            Federation::BPA => Some(Federation::IPF),
            Federation::BPC => {
                // The BPC was WPC-affiliated until 2012.
                if date.year() >= 2012 {
                    None
                } else {
                    Some(Federation::WPC)
                }
            }
            Federation::BPF => {
                if date.year() >= 2022 {
                    Some(Federation::IPL)
                } else {
                    Some(Federation::WRPF)
                }
            }
            Federation::BPO => Some(Federation::WPF),
            Federation::BPU => {
                // The BPU has been WPC-affiliated since 2013.
                if date.year() >= 2013 {
                    Some(Federation::WPC)
                } else {
                    None
                }
            }
            Federation::BulgarianPF => Some(Federation::IPF),
            Federation::BVDG => Some(Federation::IPF),
            Federation::BVDK => Some(Federation::IPF),
            Federation::CanadaUA => None,
            Federation::CAPO => Some(Federation::GPA),
            Federation::CAPONZ => Some(Federation::GPA),
            Federation::CAST => Some(Federation::GPC),
            Federation::CBLB => Some(Federation::IPF),
            Federation::CBPL => None,
            Federation::ChinaPA => Some(Federation::GPA),
            Federation::CNFA => Some(Federation::IPF),
            Federation::ColPF => Some(Federation::IPF),
            Federation::CommonwealthPF => Some(Federation::IPF),
            Federation::CPA => Some(Federation::WPA),
            Federation::CPC => Some(Federation::WPC),
            Federation::CPF => {
                if date >= date!(2020-06-17) {
                    None
                } else {
                    Some(Federation::WPC)
                }
            }
            Federation::CPI => None,
            Federation::CPL => Some(Federation::IPL),
            Federation::CPO => Some(Federation::WPC),
            Federation::CPU => Some(Federation::IPF),
            Federation::CRPL => None,
            Federation::CSST => Some(Federation::IPF),
            Federation::CyprusPF => Some(Federation::IPF),
            Federation::CzechiaUA => None,
            Federation::DBKV => None,
            Federation::DFPFNL => Some(Federation::WDFPF),
            Federation::DPL => Some(Federation::IPL),
            Federation::DSF => Some(Federation::IPF),
            Federation::EgyptPF => Some(Federation::IPF),
            Federation::EJTL => Some(Federation::IPF),
            Federation::EPC => {
                // The EPC was IPL-affiliated until 2018.
                if date.year() >= 2018 {
                    None
                } else {
                    Some(Federation::IPL)
                }
            }
            Federation::EnglandUA => None,
            Federation::EPA => Some(Federation::IPF),
            Federation::EPF => Some(Federation::IPF),
            Federation::ESDT => Some(Federation::WP),
            Federation::FALPO => Some(Federation::IPF),
            Federation::FAPL => Some(Federation::IPF),
            Federation::FBPD => None,
            Federation::FCA => None,
            Federation::FCP => Some(Federation::IPF),
            Federation::FCST => Some(Federation::GPC),
            Federation::FECAPOLIF => Some(Federation::IPF),
            Federation::FECHIPO => Some(Federation::IPF),
            Federation::Fedepotencia => Some(Federation::IPF),
            Federation::FELIPOME => Some(Federation::WP),
            Federation::FEMEPO => Some(Federation::IPF),
            Federation::FEPOA => Some(Federation::GPC),
            Federation::FESUPO => Some(Federation::IPF),
            Federation::FFForce => Some(Federation::IPF),
            Federation::FFHMFAC => Some(Federation::IPF),
            Federation::FHSAA => None,
            Federation::FIAP => None,
            Federation::FinlandUA => None,
            Federation::FIPL => Some(Federation::IPF),
            Federation::FPO => Some(Federation::IPA),
            Federation::FPP => Some(Federation::IPF),
            Federation::FPPR => Some(Federation::IPF),
            Federation::FPR => {
                // FPR was suspended by the IPF 2022-03-01, effective immediately.
                if date > date!(2022-03-01) {
                    None
                } else {
                    Some(Federation::IPF)
                }
            }
            Federation::FRPL => Some(Federation::IPF),
            Federation::FSFA => Some(Federation::WDFPF),
            Federation::GDFPF => Some(Federation::WDFPF),
            Federation::GermanyUA => None,
            Federation::GFP => None,
            Federation::GlobalPU => Some(Federation::GPC),
            Federation::GPA => Some(Federation::GPA),
            Federation::GPABrazil => Some(Federation::GPA),
            Federation::GPACOL => Some(Federation::GPA),
            Federation::GPACRO => Some(Federation::GPA),
            Federation::GPAFinland => Some(Federation::GPA),
            Federation::GPC => Some(Federation::GPC),
            Federation::GPCAUS => Some(Federation::GPC),
            Federation::GPCBrazil => Some(Federation::GPC),
            Federation::GPCCAN => Some(Federation::GPC),
            Federation::GPCFrance => Some(Federation::GPC),
            Federation::GPCGB => Some(Federation::GPC),
            Federation::GPCGUPU => Some(Federation::GPC),
            Federation::GPCIRL => Some(Federation::GPC),
            Federation::GPCISR => Some(Federation::GPC),
            Federation::GPCLAT => Some(Federation::GPC),
            Federation::GPCNZ => Some(Federation::GPC),
            Federation::GPCPOL => Some(Federation::GPC),
            Federation::GPCPortugal => Some(Federation::GPC),
            Federation::GPCScotland => Some(Federation::GPC),
            Federation::GPCUKR => Some(Federation::GPC),
            Federation::GPCUSA => Some(Federation::GPC),
            Federation::GPCRUS => Some(Federation::GPC),
            Federation::GPCCRO => Some(Federation::GPC),
            Federation::GPF => None,
            Federation::GPL => Some(Federation::IPL),
            Federation::GPU => Some(Federation::WPU),
            Federation::GRAWA => Some(Federation::IRP),
            Federation::GSFBelarus => None,
            Federation::Hardcore => None,
            Federation::CroatiaUA => None,
            Federation::HKPF => Some(Federation::WP),
            Federation::HKWPA => Some(Federation::IPF),
            Federation::HPC => Some(Federation::WPC),
            Federation::HPF => Some(Federation::IPF),
            Federation::HPLS => Some(Federation::IPF),
            Federation::HPLSUA => None,
            Federation::HPO => None,
            Federation::HTPL => None,
            Federation::Hunpower => Some(Federation::IPF),
            Federation::HungaryUA => None,
            Federation::IBSA => None,
            Federation::IDFPA => None,
            Federation::IDFPF => Some(Federation::WDFPF),
            Federation::IHSPLA => None,
            Federation::IKF => Some(Federation::GPC),
            Federation::ILPA => Some(Federation::GPA),
            Federation::ILPF => {
                // Israeli IPF affiliate from 2024
                if date.year() >= 2024 {
                    Some(Federation::IPF)
                } else {
                    None
                }
            }
            Federation::INSA => None,
            Federation::IPA => None,
            Federation::IPAAZE => Some(Federation::IPA),
            Federation::IPC => None,
            Federation::IPF => Some(Federation::IPF),
            Federation::IPFChina => Some(Federation::IPF),
            Federation::IPLChina => Some(Federation::IPL),
            Federation::IPL => Some(Federation::IPL),
            Federation::IPLNZ => Some(Federation::IPL),
            Federation::IPLSpain => Some(Federation::IPL),
            Federation::UKIPL => {
                // UK IPL affiliate from 2024
                if date.year() >= 2024 {
                    Some(Federation::IPL)
                } else {
                    None
                }
            }
            Federation::IranBBF => Some(Federation::IPF),
            Federation::IraqPF => Some(Federation::IPF),
            Federation::IrelandUA => None,
            Federation::IrishPF => Some(Federation::IPF),
            Federation::IrishPO => Some(Federation::WPC),
            Federation::IronBoy => None,
            Federation::IRP => None,
            Federation::ItalyUA => None,
            Federation::JPA => Some(Federation::IPF),
            Federation::KBGV => Some(Federation::IPF),
            Federation::KDKS => Some(Federation::IPF),
            Federation::KNKFSP => Some(Federation::IPF),
            Federation::KoreaUA => None,
            Federation::KPF => Some(Federation::IPF),
            Federation::KRAFT => Some(Federation::IPF),
            Federation::KPC => Some(Federation::IPF),
            Federation::KuwaitPL => Some(Federation::IPL),
            Federation::LebanonPF => Some(Federation::IPF),
            Federation::LFPH => Some(Federation::IPF),
            Federation::LGBT => None,
            Federation::LHSPLA => None,
            Federation::LibyaPF => Some(Federation::IPF),
            Federation::LJTF => Some(Federation::IPF),
            Federation::LMP => Some(Federation::IPL),
            Federation::LPF => Some(Federation::IPF),
            Federation::MalaysiaUA => None,
            Federation::MaltaPA => Some(Federation::IPF),
            Federation::ManxPL => Some(Federation::IPF),
            Federation::MAP => Some(Federation::IPF),
            Federation::MDFPA => Some(Federation::WDFPF),
            Federation::MDFPF => Some(Federation::WDFPF),
            Federation::MHSAA => None,
            Federation::MHSPLA => None,
            Federation::MM => None,
            Federation::MMAUS => Some(Federation::MM),
            Federation::MPA => None,
            Federation::MUPF => Some(Federation::IPF),
            Federation::NAP => Some(Federation::IPA),
            Federation::NAPF => Some(Federation::IPF),
            Federation::NASA => None,
            Federation::NaturalPA => None,
            Federation::NauruPF => Some(Federation::IPF),
            Federation::NextGenPF => None,
            Federation::NIPF => Some(Federation::IPF),
            Federation::NORCAL => None,
            Federation::NordicPF => Some(Federation::IPF),
            Federation::NOVA => None,
            Federation::NPA => None,
            Federation::NPAJ => Some(Federation::IPF),
            Federation::NPB => Some(Federation::IPF),
            Federation::NPL => None,
            Federation::NSF => Some(Federation::IPF),
            Federation::NYFC => Some(Federation::WP),
            Federation::NZPF => Some(Federation::IPF),
            Federation::NZAWLA => Some(Federation::IPF),
            Federation::NZUA => None,
            Federation::OceaniaPF => {
                // PA lost IPF affiliation in 2018, replaced by the ORPF.
                if date.year() >= 2018 {
                    Some(Federation::WP)
                } else {
                    Some(Federation::IPF)
                }
            }
            Federation::OceaniaPO => Some(Federation::WPC),
            Federation::OCWP => Some(Federation::IPF),
            Federation::ORPF => Some(Federation::IPF),
            Federation::OEVK => Some(Federation::IPF),
            Federation::PA => {
                // PA lost IPF affiliation in 2018, replaced by the APU.
                if date.year() >= 2018 {
                    Some(Federation::WP)
                } else {
                    Some(Federation::IPF)
                }
            }
            Federation::PAGermany => Some(Federation::WPF),
            Federation::PAP => Some(Federation::IPF),
            Federation::PFA => Some(Federation::WRPF),
            Federation::PFBD => Some(Federation::IPF),
            Federation::PHPL => Some(Federation::GPA),
            Federation::PI => Some(Federation::IPF),
            Federation::PLH => Some(Federation::WPF),
            Federation::PLRD => Some(Federation::IPF),
            Federation::PLSS => Some(Federation::IPF),
            Federation::PLTRAW => None,
            Federation::PLZS => Some(Federation::IPF),
            Federation::PNGPF => Some(Federation::IPF),
            Federation::PolandUA => None,
            Federation::PoliceAL => None,
            Federation::PortugalUA => None,
            Federation::PRIDE => None,
            Federation::ProRaw => None,
            Federation::PRPA => None,
            Federation::PS => Some(Federation::IPF),
            Federation::PWFL => Some(Federation::IPF),
            Federation::PZKFiTS => Some(Federation::IPF),
            Federation::QatarPL => Some(Federation::IPF),
            Federation::QatarUA => None,
            Federation::RAW => None,
            Federation::RAWCAN => None,
            Federation::RAWIceland => None,
            Federation::RawIronPL => None,
            Federation::RawPower => None,
            Federation::RAWUKR => None,
            Federation::RAWU => None,
            Federation::RDFPF => Some(Federation::WDFPF),
            Federation::RhinoPC => Some(Federation::GPC),
            Federation::TaiwanUA => None,
            Federation::RPS => None,
            Federation::RPU => None,
            Federation::RUPC => None,
            Federation::RussiaUA => None,
            Federation::SAAS => None,
            Federation::SaudiUA => None,
            Federation::SADFPA => Some(Federation::WDFPF),
            Federation::SAFKST => Some(Federation::IPF),
            Federation::SAFP => Some(Federation::IPF),
            Federation::SAPF => Some(Federation::IPF),
            Federation::SAST => Some(Federation::GPC),
            Federation::ScottishPL => Some(Federation::IPF),
            Federation::SCI => None,
            Federation::SCT => None,
            Federation::SDFPF => Some(Federation::WDFPF),
            Federation::SLP => None,
            Federation::SLPF => Some(Federation::IPF),
            Federation::SPA => None,
            Federation::SPF => None,
            Federation::SPFIRL => Some(Federation::SPF),
            Federation::SPSS => None,
            Federation::SSA => None,
            Federation::SSF => Some(Federation::IPF),
            Federation::SSSC => Some(Federation::IPF),
            Federation::SVNL => Some(Federation::IPF),
            Federation::SwissPL => {
                // Not sure about the exact date of the switch to IPF; ended in 2020.
                if date.year() >= 2018 && date.year() < 2020 {
                    Some(Federation::IPF)
                } else {
                    None
                }
            }
            Federation::TAAP => Some(Federation::IPF),
            Federation::ThaiPF => Some(Federation::IPF),
            Federation::THSPA => None,
            Federation::THSWPA => None,
            Federation::TPSSF => Some(Federation::IPF),
            Federation::UAEPL => Some(Federation::IPF),
            Federation::UAEUA => None,
            Federation::UDFPF => Some(Federation::WDFPF),
            Federation::UgandaPA => Some(Federation::WPA),
            Federation::UgandaPF => Some(Federation::WP),
            Federation::UkrainePA => None,
            Federation::UkrainePO => None,
            Federation::UnifiedSA => None,
            Federation::UPA => None,
            Federation::UPC => Some(Federation::UPC),
            Federation::UPCGermany => Some(Federation::UPC),
            Federation::UkrainePF => Some(Federation::IPF),
            Federation::UPL => Some(Federation::IPL),
            Federation::URPF => Some(Federation::WRPF),
            Federation::USABA => Some(Federation::IBSA),
            Federation::USABPA => None,
            Federation::USAUA => None,
            Federation::USAPL => {
                // The USAPL was removed from the IPF by vote on 2021-11-07, effective immediately.
                if date > date!(2021-11-07) {
                    None
                } else {
                    Some(Federation::IPF)
                }
            }
            Federation::USARawBP => None,
            Federation::USMilAbroad => None,
            Federation::USPS => None,
            Federation::USPF => {
                // The USPF was an IPF affiliate until late 1997, replaced by ADFPA.
                if date >= date!(1997-12-05) {
                    None
                } else {
                    Some(Federation::IPF)
                }
            }
            Federation::USPA => Some(Federation::IPL),
            Federation::USPC => None,
            Federation::USSF => None,
            Federation::USSports => None,
            Federation::USVIPF => Some(Federation::IPF),
            Federation::VDFPA => {
                if date < date!(2015-01-01) {
                    Some(Federation::WDFPF)
                } else if date < date!(2017-03-05) {
                    None
                } else {
                    Some(Federation::RAW)
                }
            }
            Federation::VGPF => Some(Federation::IPF),
            Federation::VietnamPA => Some(Federation::GPA),
            Federation::VietnamUA => None,
            Federation::Vityaz => None,
            Federation::VPF => Some(Federation::IPF),
            Federation::WABDL => None,
            Federation::WAIA => None,
            Federation::WarriorPLF => None,
            Federation::WBC => None,
            Federation::WDFPF => Some(Federation::WDFPF),
            Federation::WelshPA => Some(Federation::IPF),
            Federation::WP => Some(Federation::WP),
            Federation::WPChina => Some(Federation::WP),
            Federation::WPIndia => Some(Federation::WP),
            Federation::WPNauru => Some(Federation::WP),
            Federation::WPNiue => Some(Federation::WP),
            Federation::WPUSA => Some(Federation::WP),
            Federation::WPA => None,
            Federation::WPAGEO => Some(Federation::WPA),
            Federation::WPAPoland => Some(Federation::WPA),
            Federation::WPARUS => Some(Federation::WPA),
            Federation::WPAU => None,
            Federation::WPC => Some(Federation::WPC),
            Federation::WPCCP => Some(Federation::WPC),
            Federation::WPCEgypt => Some(Federation::WPC),
            Federation::WPCFinland => Some(Federation::WPC),
            Federation::WPCFrance => Some(Federation::WPC),
            Federation::WPCGermany => Some(Federation::WPC),
            Federation::WPCIceland => Some(Federation::WPC),
            Federation::WPCIndia => Some(Federation::WPC),
            Federation::WPCIsrael => Some(Federation::WPC),
            Federation::WPCItaly => Some(Federation::WPC),
            Federation::WPCKAZ => Some(Federation::WPC),
            Federation::WPCKGZ => Some(Federation::WPC),
            Federation::WPCKorea => Some(Federation::WPC),
            Federation::WPCLatvia => Some(Federation::WPC),
            Federation::WPCMoldova => Some(Federation::WPC),
            Federation::WPCPortugal => Some(Federation::WPC),
            Federation::WPCPoland => Some(Federation::WPC),
            Federation::WPCRUS => Some(Federation::WPC),
            Federation::WPCSA => Some(Federation::WPC),
            Federation::WPCSVK => Some(Federation::WPC),
            Federation::WPCUKR => Some(Federation::WPC),
            Federation::WPF => None,
            Federation::WPFG => None,
            Federation::WPFKRAWA => None,
            Federation::WPFRUS => Some(Federation::WPF),
            Federation::WPLeague => None,
            Federation::WPNZ => Some(Federation::WP),
            Federation::WPO => Some(Federation::WPO),
            Federation::WPPL => Some(Federation::WPPL),
            Federation::WPPLArgentina => Some(Federation::WPPL),
            Federation::WPPLBelarus => Some(Federation::WPPL),
            Federation::WPPLBrazil => Some(Federation::WPPL),
            Federation::WPPLGeorgia => Some(Federation::WPPL),
            Federation::WPPLIreland => Some(Federation::WPPL),
            Federation::WPPLMexico => Some(Federation::WPPL),
            Federation::WPPLPeru => Some(Federation::WPPL),
            Federation::WPPLRussia => Some(Federation::WPPL),
            Federation::WPPLUkraine => Some(Federation::WPPL),
            Federation::WPPO => None,
            Federation::WPRO => None,
            Federation::WPSF => Some(Federation::WPSF),
            Federation::WPSFBelarus => Some(Federation::WPSF),
            Federation::WPU => None,
            Federation::WPUF => None,
            Federation::WNPF => None,
            Federation::WRPF => Some(Federation::WRPF),
            Federation::WRPFArgentina => Some(Federation::WRPF),
            Federation::WRPFAUS => Some(Federation::WRPF),
            Federation::WRPFBelarus => Some(Federation::WRPF),
            Federation::WRPFBolivia => Some(Federation::WRPF),
            Federation::WRPFBrazil => Some(Federation::WRPF),
            Federation::WRPFBulgaria => Some(Federation::WRPF),
            Federation::WRPFCAN => Some(Federation::WRPF),
            Federation::WRPFChile => Some(Federation::WRPF),
            Federation::WRPFColombia => Some(Federation::WRPF),
            Federation::WRPFCRO => Some(Federation::WRPF),
            Federation::WRPFEIRE => Some(Federation::WRPF),
            Federation::WRPFHUN => Some(Federation::WRPF),
            Federation::WRPFIceland => Some(Federation::WRPF),
            Federation::WRPFIreland => Some(Federation::WRPF),
            Federation::WRPFItaly => Some(Federation::WRPF),
            Federation::WRPFKAZ => Some(Federation::WRPF),
            Federation::WRPFLatvia => Some(Federation::WRPF),
            Federation::WRPFLithuania => Some(Federation::WRPF),
            Federation::WRPFMEX => Some(Federation::WRPF),
            Federation::WRPFNIC => Some(Federation::WRPF),
            Federation::WRPFPOL => Some(Federation::WRPF),
            Federation::WRPFQatar => Some(Federation::WRPF),
            Federation::WRPFPortugal => Some(Federation::WRPF),
            Federation::WRPFSlovakia => Some(Federation::WRPF),
            Federation::WRPFSlovenia => Some(Federation::WRPF),
            Federation::WRPFSpain => Some(Federation::WRPF),
            Federation::WRPFSRB => Some(Federation::WRPF),
            Federation::WRPFSweden => Some(Federation::WRPF),
            Federation::WRPFUK => Some(Federation::WRPF),
            Federation::WRPFVietnam => Some(Federation::WRPF),
            Federation::WSHSPL => None,
            Federation::WUAP => Some(Federation::WUAP),
            Federation::WUAPAUT => Some(Federation::WUAP),
            Federation::WUAPCRO => Some(Federation::WUAP),
            Federation::WUAPCZ => Some(Federation::WUAP),
            Federation::WUAPGermany => Some(Federation::WUAP),
            Federation::WUAPRUS => Some(Federation::WUAP),
            Federation::WUAPSVK => Some(Federation::WUAP),
            Federation::WUAPUSA => Some(Federation::WUAP),
            Federation::XPC => Some(Federation::XPC),
            Federation::XPCPoland => Some(Federation::XPC),
            Federation::XPS => None,
        }
    }

    /// Helper function for specifying the PointsSystem of federations under IPF rules.
    #[inline]
    fn ipf_rules_on(date: Date) -> PointsSystem {
        if date >= date!(2020-05-01) {
            // The IPF switched to Goodlift Points from 2020-05-01 onward.
            PointsSystem::Goodlift
        } else if date.year() >= 2019 {
            // The IPF and their affiliates developed a new federation-specific
            // formula beginning in 2019.
            PointsSystem::IPFPoints
        } else if date.year() >= 1997 {
            PointsSystem::Wilks
        } else {
            PointsSystem::SchwartzMalone
        }
    }

    /// Helper function for specifying the PointsSystem of federations under IPL rules.
    #[inline]
    fn ipl_rules_on(date: Date) -> PointsSystem {
        if date >= date!(2020-11-11) {
            // The IPL changed from Wilks2020 to Dots, presumably because Robert Wilks
            // brought on the USPC (which forked from the USPA) as a WP affiliate.
            PointsSystem::Dots
        } else if date >= date!(2020-03-04) {
            // The IPL silently switched from Wilks to Wilks2020 on 2020-03-04.
            // The change was implemented only by notifying their meet directors
            // and requiring use of an upgraded version of  their IronComp meet software.
            PointsSystem::Wilks2020
        } else {
            PointsSystem::Wilks
        }
    }

    /// Helper function for specifying the PointsSystem of federations under WP rules.
    #[inline]
    fn wp_rules_on(date: Date) -> PointsSystem {
        if date.year() >= 2020 {
            PointsSystem::Wilks2020
        } else {
            PointsSystem::Wilks
        }
    }

    /// Which points system is default for a federation's meet.
    pub fn default_points(self, date: Date) -> PointsSystem {
        match self {
            Federation::_365Strong => PointsSystem::Wilks,
            Federation::AAP => PointsSystem::Wilks,
            Federation::AAPLF => PointsSystem::SchwartzMalone,
            Federation::AAU => PointsSystem::Wilks,
            Federation::ABP => PointsSystem::Wilks,
            Federation::ACHIPO => PointsSystem::Wilks,
            Federation::ACPA => PointsSystem::Wilks,
            Federation::ADAU => PointsSystem::Wilks,
            Federation::ADFPA => PointsSystem::SchwartzMalone,
            Federation::ADFPF => PointsSystem::SchwartzMalone,
            Federation::AEP => Federation::ipf_rules_on(date),
            Federation::AFPF => PointsSystem::Wilks,
            Federation::AfricanPF => Federation::ipf_rules_on(date),
            Federation::AIWBPA => Federation::ipf_rules_on(date),
            Federation::AmericanSA => PointsSystem::Wilks,
            Federation::AMP => Federation::ipf_rules_on(date),
            Federation::ANPPC => PointsSystem::Wilks,
            Federation::APA => PointsSystem::Wilks,
            Federation::APC => PointsSystem::Wilks,
            Federation::APF => PointsSystem::Glossbrenner,
            Federation::APLA => Federation::ipf_rules_on(date),
            Federation::APP => PointsSystem::Wilks,
            Federation::APU => {
                // Due to change of affiliation from IPF to WDFPF in 2024.
                if date.year() >= 2024 {
                    PointsSystem::SchwartzMalone
                } else {
                    Federation::ipf_rules_on(date)
                }
            }
            Federation::APUA => PointsSystem::Wilks,
            Federation::ARPL => Federation::ipl_rules_on(date),
            Federation::AsianPF => Federation::ipf_rules_on(date),
            Federation::AusDFPF => PointsSystem::SchwartzMalone,
            Federation::AusPF => PointsSystem::Wilks,
            Federation::AusPL => Federation::ipl_rules_on(date),
            Federation::AWF => PointsSystem::Wilks,
            Federation::BahamasPF => PointsSystem::Wilks,
            Federation::BAWLA => PointsSystem::Wilks,
            Federation::BB => PointsSystem::Wilks,
            Federation::BDFPA => PointsSystem::Wilks,
            Federation::BDFPF => PointsSystem::Wilks,
            Federation::BelPF => Federation::ipf_rules_on(date),
            Federation::BP => Federation::ipf_rules_on(date),
            Federation::BPA => Federation::ipf_rules_on(date),
            Federation::BPC => PointsSystem::Wilks,
            Federation::BPF => {
                if date.year() >= 2022 {
                    Federation::ipl_rules_on(date)
                } else {
                    PointsSystem::Wilks
                }
            }
            Federation::BPO => PointsSystem::Wilks,
            Federation::BPU => PointsSystem::Glossbrenner,
            Federation::BulgarianPF => PointsSystem::Wilks,
            Federation::BVDG => PointsSystem::Wilks,
            Federation::BVDK => {
                // Federation voted in Nov 2019 to switch to Dots in 2020.
                if date.year() >= 2020 {
                    PointsSystem::Dots
                } else {
                    Federation::ipf_rules_on(date)
                }
            }
            Federation::CanadaUA => PointsSystem::Wilks,
            Federation::CAPO => PointsSystem::Glossbrenner,
            Federation::CAPONZ => PointsSystem::Glossbrenner,
            Federation::CAST => PointsSystem::Wilks,
            Federation::CBLB => Federation::ipf_rules_on(date),
            Federation::CBPL => PointsSystem::Wilks,
            Federation::ChinaPA => PointsSystem::Wilks,
            Federation::CNFA => Federation::ipf_rules_on(date),
            Federation::ColPF => Federation::ipf_rules_on(date),
            Federation::CommonwealthPF => Federation::ipf_rules_on(date),
            Federation::CPA => PointsSystem::Wilks,
            Federation::CPC => PointsSystem::Wilks,
            Federation::CPF => PointsSystem::Wilks,
            Federation::CPI => PointsSystem::Dots,
            Federation::CPL => Federation::ipl_rules_on(date),
            Federation::CPO => PointsSystem::Wilks,
            Federation::CPU => Federation::ipf_rules_on(date),
            Federation::CRPL => PointsSystem::Wilks,
            Federation::CSST => PointsSystem::Wilks,
            Federation::DBKV => PointsSystem::Wilks,
            Federation::DFPFNL => PointsSystem::Wilks,
            Federation::DPL => PointsSystem::Dots,
            Federation::CyprusPF => Federation::ipf_rules_on(date),
            Federation::CzechiaUA => PointsSystem::Wilks,
            Federation::DSF => Federation::ipf_rules_on(date),
            Federation::EgyptPF => Federation::ipf_rules_on(date),
            Federation::EJTL => Federation::ipf_rules_on(date),
            Federation::EPC => PointsSystem::Wilks,
            Federation::EnglandUA => PointsSystem::Wilks,
            Federation::EPA => Federation::ipf_rules_on(date),
            Federation::EPF => Federation::ipf_rules_on(date),
            Federation::ESDT => PointsSystem::Wilks,
            Federation::FALPO => Federation::ipf_rules_on(date),
            Federation::FAPL => Federation::ipf_rules_on(date),
            Federation::FBPD => PointsSystem::Wilks,
            Federation::FCA => PointsSystem::Wilks,
            Federation::FCP => Federation::ipf_rules_on(date),
            Federation::FCST => PointsSystem::Wilks,
            Federation::FECAPOLIF => Federation::ipf_rules_on(date),
            Federation::FECHIPO => Federation::ipf_rules_on(date),
            Federation::Fedepotencia => Federation::ipf_rules_on(date),
            Federation::FELIPOME => Federation::wp_rules_on(date),
            Federation::FEMEPO => Federation::ipf_rules_on(date),
            Federation::FEPOA => PointsSystem::Wilks,
            Federation::FESUPO => Federation::ipf_rules_on(date),
            Federation::FFForce => Federation::ipf_rules_on(date),
            Federation::FFHMFAC => Federation::ipf_rules_on(date),
            Federation::FHSAA => PointsSystem::Wilks,
            Federation::FIAP => Federation::ipf_rules_on(date),
            Federation::FinlandUA => PointsSystem::Wilks,
            Federation::FIPL => Federation::ipf_rules_on(date),
            Federation::FPO => PointsSystem::Wilks,
            Federation::FPP => Federation::ipf_rules_on(date),
            Federation::FPPR => Federation::ipf_rules_on(date),
            Federation::FPR => Federation::ipf_rules_on(date),
            Federation::FRPL => Federation::ipf_rules_on(date),
            Federation::FSFA => PointsSystem::Wilks,
            Federation::GDFPF => PointsSystem::Wilks,
            Federation::GermanyUA => PointsSystem::Wilks,
            Federation::GFP => PointsSystem::Wilks,
            Federation::GlobalPU => PointsSystem::Glossbrenner,
            Federation::GPA => PointsSystem::Wilks,
            Federation::GPABrazil => PointsSystem::Wilks,
            Federation::GPACOL => PointsSystem::Wilks,
            Federation::GPACRO => PointsSystem::Wilks,
            Federation::GPAFinland => PointsSystem::Wilks,
            Federation::GPC => PointsSystem::Reshel,
            Federation::GPCAUS => PointsSystem::Glossbrenner,
            Federation::GPCBrazil => PointsSystem::Glossbrenner,
            Federation::GPCCAN => PointsSystem::Glossbrenner,
            Federation::GPCFrance => PointsSystem::Glossbrenner,
            Federation::GPCGB => PointsSystem::Reshel,
            Federation::GPCGUPU => PointsSystem::Glossbrenner,
            Federation::GPCIRL => PointsSystem::Glossbrenner,
            Federation::GPCISR => PointsSystem::Glossbrenner,
            Federation::GPCLAT => PointsSystem::Glossbrenner,
            Federation::GPCNZ => PointsSystem::Glossbrenner,
            Federation::GPCPOL => PointsSystem::Glossbrenner,
            Federation::GPCPortugal => PointsSystem::Reshel,
            Federation::GPCScotland => PointsSystem::Reshel,
            Federation::GPCUKR => PointsSystem::Glossbrenner,
            Federation::GPCUSA => PointsSystem::Glossbrenner,
            Federation::GPCRUS => PointsSystem::Glossbrenner,
            Federation::GPCCRO => PointsSystem::Glossbrenner,
            Federation::GPF => PointsSystem::Wilks,
            Federation::GPL => PointsSystem::Dots,
            Federation::GPU => PointsSystem::Wilks,
            Federation::GRAWA => PointsSystem::Wilks,
            Federation::GSFBelarus => PointsSystem::Wilks,
            Federation::Hardcore => PointsSystem::Wilks,
            Federation::CroatiaUA => PointsSystem::Wilks,
            Federation::HKPF => Federation::wp_rules_on(date),
            Federation::HKWPA => Federation::ipf_rules_on(date),
            Federation::HPC => PointsSystem::Wilks,
            Federation::HPF => Federation::ipf_rules_on(date),
            Federation::HPLS => Federation::ipf_rules_on(date),
            Federation::HPLSUA => PointsSystem::Wilks,
            Federation::HPO => PointsSystem::Wilks,
            Federation::HTPL => PointsSystem::Wilks,
            Federation::Hunpower => Federation::ipf_rules_on(date),
            Federation::HungaryUA => Federation::ipf_rules_on(date),
            Federation::IBSA => PointsSystem::Wilks,
            Federation::IDFPA => PointsSystem::Wilks,
            Federation::IDFPF => PointsSystem::SchwartzMalone,
            Federation::IHSPLA => PointsSystem::Wilks,
            Federation::IKF => PointsSystem::Wilks,
            Federation::ILPA => PointsSystem::Wilks,
            Federation::ILPF => Federation::ipf_rules_on(date),
            Federation::INSA => PointsSystem::Wilks,
            Federation::IPA => PointsSystem::Wilks,
            Federation::IPAAZE => PointsSystem::Wilks,
            Federation::IPC => PointsSystem::Wilks,
            Federation::IPF => Federation::ipf_rules_on(date),
            Federation::IPFChina => Federation::ipf_rules_on(date),
            Federation::IPLChina => Federation::ipl_rules_on(date),
            Federation::IPL => Federation::ipl_rules_on(date),
            Federation::IPLNZ => Federation::ipl_rules_on(date),
            Federation::IPLSpain => Federation::ipl_rules_on(date),
            Federation::UKIPL => Federation::ipl_rules_on(date),
            Federation::IranBBF => Federation::ipf_rules_on(date),
            Federation::IraqPF => Federation::ipf_rules_on(date),
            Federation::IrelandUA => PointsSystem::Wilks,
            Federation::IrishPF => {
                // On 2020-02-16, IrishPF voted to immediately switch to Dots.
                if date > date!(2020-02-16) {
                    PointsSystem::Dots
                } else {
                    Federation::ipf_rules_on(date)
                }
            }
            Federation::IrishPO => PointsSystem::Glossbrenner,
            Federation::IronBoy => PointsSystem::Wilks,
            Federation::IRP => PointsSystem::Wilks,
            Federation::ItalyUA => PointsSystem::Wilks,
            Federation::JPA => Federation::ipf_rules_on(date),
            Federation::KBGV => Federation::ipf_rules_on(date),
            Federation::KDKS => PointsSystem::Dots,
            Federation::KNKFSP => Federation::ipf_rules_on(date),
            Federation::KoreaUA => PointsSystem::Dots,
            Federation::KPF => Federation::ipf_rules_on(date),
            Federation::KRAFT => {
                // On 2020-03-04, KRAFT announced that they voted for Dots since 02-29.
                if date >= date!(2020-02-29) {
                    PointsSystem::Dots
                } else {
                    Federation::ipf_rules_on(date)
                }
            }
            Federation::KPC => Federation::ipf_rules_on(date),
            Federation::KuwaitPL => Federation::ipl_rules_on(date),
            Federation::LebanonPF => Federation::ipf_rules_on(date),
            Federation::LFPH => Federation::ipf_rules_on(date),
            Federation::LGBT => PointsSystem::Wilks,
            Federation::LHSPLA => PointsSystem::Wilks,
            Federation::LibyaPF => Federation::ipf_rules_on(date),
            Federation::LJTF => Federation::ipf_rules_on(date),
            Federation::LMP => Federation::ipl_rules_on(date),
            Federation::LPF => Federation::ipf_rules_on(date),
            Federation::MalaysiaUA => PointsSystem::Wilks,
            Federation::MaltaPA => Federation::ipf_rules_on(date),
            Federation::ManxPL => Federation::ipf_rules_on(date),
            Federation::MAP => Federation::ipf_rules_on(date),
            Federation::MDFPA => PointsSystem::SchwartzMalone,
            Federation::MDFPF => PointsSystem::SchwartzMalone,
            Federation::MHSAA => PointsSystem::Wilks,
            Federation::MHSPLA => PointsSystem::Wilks,
            Federation::MM => PointsSystem::SchwartzMalone,
            Federation::MMAUS => PointsSystem::SchwartzMalone,
            Federation::MPA => PointsSystem::Wilks,
            Federation::MUPF => Federation::ipf_rules_on(date),
            Federation::NAP => PointsSystem::Wilks,
            Federation::NAPF => Federation::ipf_rules_on(date),
            Federation::NASA => PointsSystem::NASA,
            Federation::NaturalPA => PointsSystem::Wilks,
            Federation::NauruPF => Federation::ipf_rules_on(date),
            Federation::NextGenPF => PointsSystem::Wilks,
            Federation::NORCAL => PointsSystem::Wilks,
            Federation::NIPF => Federation::ipf_rules_on(date),
            Federation::NordicPF => Federation::ipf_rules_on(date),
            Federation::NOVA => PointsSystem::Wilks,
            Federation::NPA => PointsSystem::Wilks,
            Federation::NPAJ => Federation::ipf_rules_on(date),
            Federation::NPB => Federation::ipf_rules_on(date),
            Federation::NPL => PointsSystem::Dots,
            Federation::NSF => Federation::ipf_rules_on(date),
            Federation::NYFC => Federation::wp_rules_on(date),
            Federation::NZPF => Federation::ipf_rules_on(date),
            Federation::NZAWLA => Federation::ipf_rules_on(date),
            Federation::NZUA => PointsSystem::Wilks,
            Federation::OceaniaPF => Federation::wp_rules_on(date),
            Federation::OceaniaPO => PointsSystem::Glossbrenner,
            Federation::OCWP => Federation::ipf_rules_on(date),
            Federation::ORPF => Federation::ipf_rules_on(date),
            Federation::OEVK => Federation::ipf_rules_on(date),
            Federation::PA => {
                if date.year() >= 2020 {
                    PointsSystem::Wilks2020
                } else if date.year() >= 1997 {
                    PointsSystem::Wilks
                } else {
                    PointsSystem::SchwartzMalone
                }
            }
            Federation::PAGermany => PointsSystem::Wilks,
            Federation::PoliceAL => PointsSystem::Wilks,
            Federation::PAP => Federation::ipf_rules_on(date),
            Federation::PFA => PointsSystem::Dots,
            Federation::PFBD => Federation::ipf_rules_on(date),
            Federation::PHPL => PointsSystem::Reshel,
            Federation::PI => Federation::ipf_rules_on(date),
            Federation::PLH => PointsSystem::Reshel,
            Federation::PLRD => Federation::ipf_rules_on(date),
            Federation::PLSS => Federation::ipf_rules_on(date),
            Federation::PLTRAW => PointsSystem::Dots,
            Federation::PLZS => Federation::ipf_rules_on(date),
            Federation::PNGPF => Federation::ipf_rules_on(date),
            Federation::PolandUA => PointsSystem::Wilks,
            Federation::PRIDE => PointsSystem::Wilks,
            Federation::PortugalUA => PointsSystem::Dots,
            Federation::ProRaw => PointsSystem::Glossbrenner,
            Federation::PRPA => PointsSystem::Wilks,
            Federation::PS => Federation::ipf_rules_on(date),
            Federation::PWFL => Federation::ipf_rules_on(date),
            Federation::PZKFiTS => Federation::ipf_rules_on(date),
            Federation::QatarPL => Federation::ipf_rules_on(date),
            Federation::QatarUA => Federation::ipf_rules_on(date),
            Federation::RAW => PointsSystem::SchwartzMalone,
            Federation::RAWCAN => PointsSystem::Wilks,
            Federation::RAWIceland => PointsSystem::Wilks,
            Federation::RawIronPL => {
                if date >= date!(2020-12-01) {
                    PointsSystem::Dots
                } else {
                    PointsSystem::Wilks
                }
            }
            Federation::RawPower => PointsSystem::Wilks,
            Federation::RAWUKR => PointsSystem::Wilks,
            Federation::RAWU => PointsSystem::Wilks,
            Federation::RDFPF => PointsSystem::SchwartzMalone,
            Federation::RhinoPC => PointsSystem::Glossbrenner,
            Federation::TaiwanUA => Federation::ipf_rules_on(date),
            Federation::RPS => PointsSystem::Wilks,
            Federation::RPU => PointsSystem::Wilks,
            Federation::RUPC => PointsSystem::Wilks,
            Federation::RussiaUA => PointsSystem::Wilks,
            Federation::SAAS => PointsSystem::Wilks,
            Federation::SaudiUA => PointsSystem::Dots,
            Federation::SADFPA => PointsSystem::Wilks,
            Federation::SAFKST => Federation::ipf_rules_on(date),
            Federation::SAFP => Federation::ipf_rules_on(date),
            Federation::SAPF => Federation::ipf_rules_on(date),
            Federation::SAST => PointsSystem::Glossbrenner,
            Federation::ScottishPL => Federation::ipf_rules_on(date),
            Federation::SCI => PointsSystem::Wilks,
            Federation::SCT => PointsSystem::Wilks,
            Federation::SDFPF => PointsSystem::Wilks,
            Federation::SLP => PointsSystem::Wilks,
            Federation::SLPF => Federation::ipf_rules_on(date),
            Federation::SPA => PointsSystem::Wilks,
            Federation::SPF => PointsSystem::SchwartzMalone,
            Federation::SPFIRL => PointsSystem::SchwartzMalone,
            Federation::SPSS => PointsSystem::Wilks,
            Federation::SSA => PointsSystem::Wilks,
            Federation::SSF => Federation::ipf_rules_on(date),
            Federation::SSSC => Federation::ipf_rules_on(date),
            Federation::SVNL => Federation::ipf_rules_on(date),
            Federation::SwissPL => {
                // Federation voted in Nov 2019 to switch to Dots in 2020.
                if date.year() >= 2020 {
                    PointsSystem::Dots
                } else {
                    Federation::ipf_rules_on(date)
                }
            }
            Federation::TAAP => Federation::ipf_rules_on(date),
            Federation::ThaiPF => Federation::ipf_rules_on(date),
            Federation::THSPA => PointsSystem::Wilks,
            Federation::THSWPA => PointsSystem::Wilks,
            Federation::TPSSF => Federation::ipf_rules_on(date),
            Federation::UAEPL => Federation::ipf_rules_on(date),
            Federation::UAEUA => PointsSystem::Dots,
            Federation::UDFPF => PointsSystem::Wilks,
            Federation::UgandaPA => PointsSystem::Wilks,
            Federation::UgandaPF => Federation::wp_rules_on(date),
            Federation::UkrainePA => PointsSystem::Wilks,
            Federation::UkrainePO => PointsSystem::Wilks,
            Federation::UnifiedSA => PointsSystem::Wilks,
            Federation::UPA => PointsSystem::Wilks,
            Federation::UPC => PointsSystem::Wilks,
            Federation::UPCGermany => PointsSystem::Glossbrenner,
            Federation::UkrainePF => Federation::ipf_rules_on(date),
            Federation::UPL => Federation::ipl_rules_on(date),
            Federation::URPF => PointsSystem::Wilks,
            Federation::USABA => PointsSystem::Wilks,
            Federation::USABPA => PointsSystem::Wilks,
            Federation::USAUA => PointsSystem::Wilks,
            Federation::USAPL => {
                // The USAPL started using Dots in 2022.
                if date.year() >= 2022 {
                    PointsSystem::Dots
                } else {
                    Federation::ipf_rules_on(date)
                }
            }
            Federation::USARawBP => PointsSystem::Wilks,
            Federation::USMilAbroad => PointsSystem::Wilks,
            Federation::USPS => PointsSystem::Wilks,
            Federation::USPF => PointsSystem::Wilks,
            Federation::USPA => Federation::ipl_rules_on(date),
            Federation::USPC => {
                if date >= date!(2023-01-01) {
                    PointsSystem::Dots
                } else if date >= date!(2020-11-01) {
                    PointsSystem::Wilks2020
                } else {
                    PointsSystem::Dots
                }
            }
            Federation::USSF => PointsSystem::Wilks,
            Federation::USSports => PointsSystem::Wilks,
            Federation::USVIPF => Federation::ipf_rules_on(date),
            Federation::VDFPA => PointsSystem::SchwartzMalone,
            Federation::VGPF => Federation::ipf_rules_on(date),
            Federation::VietnamPA => PointsSystem::Wilks,
            Federation::VietnamUA => PointsSystem::Dots,
            Federation::Vityaz => PointsSystem::Wilks,
            Federation::VPF => Federation::ipf_rules_on(date),
            Federation::WABDL => PointsSystem::Wilks,
            Federation::WAIA => PointsSystem::Wilks,
            Federation::WarriorPLF => PointsSystem::Wilks,
            Federation::WBC => PointsSystem::Wilks,
            Federation::WDFPF => PointsSystem::Wilks,
            Federation::WelshPA => Federation::ipf_rules_on(date),
            Federation::WP => Federation::wp_rules_on(date),
            Federation::WPChina => Federation::wp_rules_on(date),
            Federation::WPIndia => Federation::wp_rules_on(date),
            Federation::WPNauru => Federation::wp_rules_on(date),
            Federation::WPNiue => Federation::wp_rules_on(date),
            Federation::WPUSA => Federation::wp_rules_on(date),
            Federation::WPA => PointsSystem::Wilks,
            Federation::WPAGEO => PointsSystem::Wilks,
            Federation::WPAPoland => PointsSystem::Wilks,
            Federation::WPARUS => PointsSystem::Wilks,
            Federation::WPAU => PointsSystem::Wilks,
            Federation::WPC => PointsSystem::Glossbrenner,
            Federation::WPCCP => PointsSystem::Glossbrenner,
            Federation::WPCEgypt => PointsSystem::Glossbrenner,
            Federation::WPCFinland => PointsSystem::Glossbrenner,
            Federation::WPCFrance => PointsSystem::Glossbrenner,
            Federation::WPCGermany => PointsSystem::Glossbrenner,
            Federation::WPCIceland => PointsSystem::Glossbrenner,
            Federation::WPCIndia => PointsSystem::Glossbrenner,
            Federation::WPCIsrael => PointsSystem::Glossbrenner,
            Federation::WPCItaly => PointsSystem::Glossbrenner,
            Federation::WPCKAZ => PointsSystem::Glossbrenner,
            Federation::WPCKGZ => PointsSystem::Glossbrenner,
            Federation::WPCKorea => PointsSystem::Glossbrenner,
            Federation::WPCLatvia => PointsSystem::Glossbrenner,
            Federation::WPCMoldova => PointsSystem::Glossbrenner,
            Federation::WPCPortugal => PointsSystem::Glossbrenner,
            Federation::WPCPoland => PointsSystem::Glossbrenner,
            Federation::WPCRUS => PointsSystem::Glossbrenner,
            Federation::WPCSA => PointsSystem::Glossbrenner,
            Federation::WPCSVK => PointsSystem::Glossbrenner,
            Federation::WPCUKR => PointsSystem::Glossbrenner,
            Federation::WPF => PointsSystem::Wilks,
            Federation::WPFG => PointsSystem::Wilks,
            Federation::WPFKRAWA => PointsSystem::Wilks,
            Federation::WPFRUS => PointsSystem::Wilks,
            Federation::WPLeague => PointsSystem::Wilks,
            Federation::WPNZ => PointsSystem::Wilks,
            Federation::WPO => PointsSystem::Glossbrenner,
            Federation::WPPL => PointsSystem::Wilks,
            Federation::WPPLArgentina => PointsSystem::Wilks,
            Federation::WPPLBelarus => PointsSystem::Wilks,
            Federation::WPPLBrazil => PointsSystem::Wilks,
            Federation::WPPLGeorgia => PointsSystem::Wilks,
            Federation::WPPLIreland => PointsSystem::Wilks,
            Federation::WPPLMexico => PointsSystem::Wilks,
            Federation::WPPLPeru => PointsSystem::Wilks,
            Federation::WPPLRussia => PointsSystem::Wilks,
            Federation::WPPLUkraine => PointsSystem::Wilks,
            Federation::WPPO => PointsSystem::AH,
            Federation::WPRO => PointsSystem::Wilks,
            Federation::WPSF => PointsSystem::Wilks,
            Federation::WPSFBelarus => PointsSystem::Wilks,
            Federation::WPU => PointsSystem::Wilks,
            Federation::WPUF => PointsSystem::Wilks,
            Federation::WNPF => PointsSystem::Wilks,
            Federation::WRPF => {
                if date.year() >= 2020 {
                    PointsSystem::Dots
                } else {
                    PointsSystem::Wilks
                }
            }
            Federation::WRPFArgentina => PointsSystem::Wilks,
            Federation::WRPFAUS => PointsSystem::Wilks,
            Federation::WRPFBelarus => PointsSystem::Wilks,
            Federation::WRPFBolivia => PointsSystem::Wilks,
            Federation::WRPFBrazil => PointsSystem::Wilks,
            Federation::WRPFBulgaria => PointsSystem::Wilks,
            Federation::WRPFCAN => PointsSystem::Wilks,
            Federation::WRPFChile => PointsSystem::Wilks,
            Federation::WRPFColombia => PointsSystem::Wilks,
            Federation::WRPFCRO => PointsSystem::Wilks,
            Federation::WRPFEIRE => PointsSystem::Wilks,
            Federation::WRPFHUN => PointsSystem::Wilks,
            Federation::WRPFIceland => PointsSystem::Wilks,
            Federation::WRPFIreland => PointsSystem::Wilks,
            Federation::WRPFItaly => PointsSystem::Wilks,
            Federation::WRPFKAZ => PointsSystem::Wilks,
            Federation::WRPFLatvia => PointsSystem::Wilks,
            Federation::WRPFLithuania => PointsSystem::Wilks,
            Federation::WRPFMEX => PointsSystem::Wilks,
            Federation::WRPFNIC => PointsSystem::Wilks,
            Federation::WRPFPOL => PointsSystem::Wilks,
            Federation::WRPFQatar => PointsSystem::Wilks,
            Federation::WRPFPortugal => PointsSystem::Wilks,
            Federation::WRPFSlovakia => PointsSystem::Wilks,
            Federation::WRPFSlovenia => PointsSystem::Wilks,
            Federation::WRPFSpain => PointsSystem::Wilks,
            Federation::WRPFSRB => PointsSystem::Wilks,
            Federation::WRPFSweden => PointsSystem::Wilks,
            Federation::WRPFUK => PointsSystem::Wilks,
            Federation::WRPFVietnam => PointsSystem::Wilks,
            Federation::WSHSPL => PointsSystem::Wilks,
            Federation::WUAP => PointsSystem::Wilks,
            Federation::WUAPAUT => PointsSystem::Wilks,
            Federation::WUAPCRO => PointsSystem::Wilks,
            Federation::WUAPCZ => PointsSystem::Wilks,
            Federation::WUAPGermany => PointsSystem::Wilks,
            Federation::WUAPRUS => PointsSystem::Wilks,
            Federation::WUAPSVK => PointsSystem::Wilks,
            Federation::WUAPUSA => PointsSystem::Wilks,
            Federation::XPC => PointsSystem::Wilks,
            Federation::XPCPoland => PointsSystem::Wilks,
            Federation::XPS => PointsSystem::Wilks,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_strings() {
        // The lowercase form should parse.
        assert_eq!("wrpf".parse::<Federation>().unwrap(), Federation::WRPF);

        // The default to_string() should be the upper-case form.
        assert_eq!(Federation::WRPF.to_string(), "WRPF");
    }
}
