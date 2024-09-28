//! MetaFederation definitions and calculations.

#![allow(clippy::zero_prefixed_literal)] // For dates.

use itertools::Itertools;
use opltypes::*;
use strum::IntoEnumIterator;

use crate::{Entry, Meet};

/// Enum of MetaFederations. These are the entries in the federation selector
/// that don't correspond neatly to just a single federation value.
///
/// Definition of each MetaFederation is in the `contains` function.
///
/// A MetaFederation may override handling of a Federation by sharing its
/// to_string.
#[derive(
    Copy, Clone, Debug, Deserialize, Display, PartialEq, Eq, Serialize, EnumIter, EnumString,
)]
pub enum MetaFederation {
    /// Federations that are exclusively (non-optionally) tested.
    #[strum(to_string = "fully-tested")]
    FullyTested,

    /// All entries that have "Tested = Yes".
    #[strum(to_string = "all-tested")]
    AllTested,

    #[strum(to_string = "all-algeria")]
    AllAlgeria,
    #[strum(to_string = "all-argentina")]
    AllArgentina,
    #[strum(to_string = "all-australia")]
    AllAustralia,
    #[strum(to_string = "all-australia-tested")]
    AllAustraliaTested,
    #[strum(to_string = "all-austria")]
    AllAustria,
    #[strum(to_string = "all-azerbaijan")]
    AllAzerbaijan,
    #[strum(to_string = "all-belarus")]
    AllBelarus,
    #[strum(to_string = "all-belgium")]
    AllBelgium,
    /// Results for all Belgian IPF Affiliates.
    #[strum(to_string = "all-ipf-belgium")]
    AllIPFBelgium,
    #[strum(to_string = "all-belize")]
    AllBelize,
    #[strum(to_string = "all-bolivia")]
    AllBolivia,
    #[strum(to_string = "all-bosnia-and-herzegovina")]
    AllBosniaAndHerzegovina,
    #[strum(to_string = "all-brazil")]
    AllBrazil,
    #[strum(to_string = "all-brunei")]
    AllBrunei,
    #[strum(to_string = "all-bulgaria")]
    AllBulgaria,
    #[strum(to_string = "all-canada")]
    AllCanada,
    #[strum(to_string = "all-chile")]
    AllChile,
    #[strum(to_string = "all-china")]
    AllChina,
    #[strum(to_string = "all-colombia")]
    AllColombia,
    #[strum(to_string = "all-croatia")]
    AllCroatia,
    #[strum(to_string = "all-cyprus")]
    AllCyprus,
    #[strum(to_string = "all-czechia")]
    AllCzechia,
    #[strum(to_string = "all-denmark")]
    AllDenmark,
    #[strum(to_string = "all-egypt")]
    AllEgypt,
    #[strum(to_string = "all-estonia")]
    AllEstonia,
    #[strum(to_string = "all-finland")]
    AllFinland,
    #[strum(to_string = "all-france")]
    AllFrance,
    #[strum(to_string = "all-georgia")]
    AllGeorgia,
    #[strum(to_string = "all-germany")]
    AllGermany,
    #[strum(to_string = "all-greece")]
    AllGreece,
    #[strum(to_string = "all-guatemala")]
    AllGuatemala,
    #[strum(to_string = "all-guyana")]
    AllGuyana,
    #[strum(to_string = "all-hongkong")]
    AllHongKong,
    #[strum(to_string = "all-hungary")]
    AllHungary,
    #[strum(to_string = "all-iceland")]
    AllIceland,
    #[strum(to_string = "all-india")]
    AllIndia,
    #[strum(to_string = "all-indonesia")]
    AllIndonesia,
    /// Results for the relevant US IPF Affiliate at the time.
    #[strum(to_string = "all-ipf-usa")]
    AllIPFUSA,
    #[strum(to_string = "all-iran")]
    AllIran,
    #[strum(to_string = "all-iraq")]
    AllIraq,
    #[strum(to_string = "all-ireland")]
    AllIreland,
    #[strum(to_string = "all-israel")]
    AllIsrael,
    #[strum(to_string = "all-italy")]
    AllItaly,
    #[strum(to_string = "all-japan")]
    AllJapan,
    #[strum(to_string = "all-kazakhstan")]
    AllKazakhstan,
    #[strum(to_string = "all-kuwait")]
    AllKuwait,
    #[strum(to_string = "all-kyrgyzstan")]
    AllKyrgyzstan,
    #[strum(to_string = "all-latvia")]
    AllLatvia,
    #[strum(to_string = "all-lebanon")]
    AllLebanon,
    #[strum(to_string = "all-libya")]
    AllLibya,
    #[strum(to_string = "all-lithuania")]
    AllLithuania,
    #[strum(to_string = "all-malaysia")]
    AllMalaysia,
    #[strum(to_string = "all-mexico")]
    AllMexico,
    #[strum(to_string = "all-moldova")]
    AllMoldova,
    #[strum(to_string = "all-mongolia")]
    AllMongolia,
    #[strum(to_string = "all-nauru")]
    AllNauru,
    #[strum(to_string = "all-nepal")]
    AllNepal,
    #[strum(to_string = "all-netherlands")]
    AllNetherlands,
    #[strum(to_string = "all-newzealand")]
    AllNewZealand,
    #[strum(to_string = "all-nicaragua")]
    AllNicaragua,
    #[strum(to_string = "all-niue")]
    AllNiue,
    #[strum(to_string = "all-norway")]
    AllNorway,
    #[strum(to_string = "all-papuanewguinea")]
    AllPapuaNewGuinea,
    #[strum(to_string = "all-oman")]
    AllOman,
    #[strum(to_string = "all-panama")]
    AllPanama,
    #[strum(to_string = "all-paraguay")]
    AllParaguay,
    #[strum(to_string = "all-philippines")]
    AllPhilippines,
    #[strum(to_string = "all-poland")]
    AllPoland,
    #[strum(to_string = "all-portugal")]
    AllPortugal,
    #[strum(to_string = "all-qatar")]
    AllQatar,
    #[strum(to_string = "all-romania")]
    AllRomania,
    #[strum(to_string = "all-russia")]
    AllRussia,
    #[strum(to_string = "all-saudiarabia")]
    AllSaudiArabia,
    #[strum(to_string = "all-scotland")]
    AllScotland,
    #[strum(to_string = "all-serbia")]
    AllSerbia,
    #[strum(to_string = "all-singapore")]
    AllSingapore,
    #[strum(to_string = "all-slovakia")]
    AllSlovakia,
    #[strum(to_string = "all-slovenia")]
    AllSlovenia,
    #[strum(to_string = "all-spain")]
    AllSpain,
    #[strum(to_string = "all-southafrica")]
    AllSouthAfrica,
    #[strum(to_string = "all-srilanka")]
    AllSriLanka,
    #[strum(to_string = "all-sweden")]
    AllSweden,
    #[strum(to_string = "all-switzerland")]
    AllSwitzerland,
    #[strum(to_string = "all-syria")]
    AllSyria,
    #[strum(to_string = "all-taiwan")]
    AllTaiwan,
    #[strum(to_string = "all-thailand")]
    AllThailand,
    #[strum(to_string = "all-turkey")]
    AllTurkey,
    #[strum(to_string = "all-uae")]
    AllUAE,
    #[strum(to_string = "all-uganda")]
    AllUganda,
    #[strum(to_string = "all-uk")]
    AllUK,
    #[strum(to_string = "all-uk-tested")]
    AllUKTested,
    #[strum(to_string = "all-ukraine")]
    AllUkraine,
    #[strum(to_string = "all-vietnam")]
    AllVietnam,
    #[strum(to_string = "all-usa")]
    AllUSA,
    #[strum(to_string = "all-usa-tested")]
    AllUSATested,
    #[strum(to_string = "all-usvirginislands")]
    AllUSVirginIslands,

    /// WPC, but only Tested entries.
    #[strum(to_string = "awpc")]
    AWPC,

    /// APF, but only Tested entries, and with international results.
    #[strum(to_string = "aapf")]
    AAPF,

    /// BPU, but only tested entries.
    #[strum(to_string = "abpu")]
    ABPU,

    /// AEP, but with international results also.
    #[strum(to_string = "aep")]
    AEP,

    /// IPO, but only tested entries.
    #[strum(to_string = "airishpo")]
    AIrishPO,

    /// AIWBPA, but with international results also.
    #[strum(to_string = "aiwbpa")]
    AIWBPA,

    /// AMP, but with international results also.
    #[strum(to_string = "amp")]
    AMP,

    /// APF, buf with international results.
    #[strum(to_string = "apf")]
    APF,

    /// APLA, but with international results also.
    #[strum(to_string = "apla")]
    APLA,

    /// APP, but with international results also.
    #[strum(to_string = "app")]
    APP,

    /// APU, but with international results also.
    #[strum(to_string = "apu")]
    APU,

    /// AusPL Tested
    #[strum(to_string = "auspl-tested")]
    AusPLTested,

    /// BDFPA, but with international results also.
    #[strum(to_string = "bdfpa")]
    BDFPA,

    /// BelPF, but with international results also.
    #[strum(to_string = "belpf")]
    BelPF,

    /// The BP federation is made up of smaller divisional federations,
    /// but people expect to see them all lumped together.
    #[strum(to_string = "bp")]
    BP,

    /// BPC, but with international results also.
    #[strum(to_string = "bpc")]
    BPC,

    /// BPU, but with international results also.
    #[strum(to_string = "bpu")]
    BPU,

    /// BulgarianPF, but with international results also.
    #[strum(to_string = "bulgarianpf")]
    BulgarianPF,

    /// BVDK, but with international results also.
    #[strum(to_string = "bvdk")]
    BVDK,

    /// CBLB, but with international results also.
    #[strum(to_string = "cblb")]
    CBLB,

    /// CPU, but with international results also.
    #[strum(to_string = "cpu")]
    CPU,

    /// CSST, but with international results also.
    #[strum(to_string = "csst")]
    CSST,

    /// CyprusPF, but with international results also.
    #[strum(to_string = "cypruspf")]
    CyprusPF,

    /// DPL, but with international results also.
    #[strum(to_string = "dpl")]
    DPL,

    /// DSF, but with international results also.
    #[strum(to_string = "dsf")]
    DSF,

    /// EgyptPF, but with international results also.
    #[strum(to_string = "egyptpf")]
    EgyptPF,

    /// EJTL, but with international results also.
    #[strum(to_string = "ejtl")]
    EJTL,

    /// EPA, but with BP and international results also.
    #[strum(to_string = "epa")]
    EPA,

    /// FALPO, but with international results also.
    #[strum(to_string = "falpo")]
    FALPO,

    /// FAPL, but with international results also.
    #[strum(to_string = "fapl")]
    FAPL,

    /// FCP, but with international results also
    #[strum(to_string = "fcp")]
    FCP,

    /// FECAPOLIF, but with international results also.
    #[strum(to_string = "fecapolif")]
    FECAPOLIF,

    /// FECHIPO, but with international results also.
    #[strum(to_string = "fechipo")]
    FECHIPO,

    /// FEMEPO, but with international results also.
    #[strum(to_string = "femepo")]
    FEMEPO,

    /// FFForce, but with international results also.
    #[strum(to_string = "ffforce")]
    FFForce,

    /// FIPL, but with international results also.
    #[strum(to_string = "fipl")]
    FIPL,

    /// FPP, but with international results also.
    #[strum(to_string = "fpp")]
    FPP,

    /// FPPR, but with international results also.
    #[strum(to_string = "fppr")]
    FPPR,

    /// FPR, but with international results also.
    #[strum(to_string = "fpr")]
    FPR,

    /// FRPL, but with international results also.
    #[strum(to_string = "frpl")]
    FRPL,

    /// GAPLF, but with international results also.
    #[strum(to_string = "gaplf")]
    GAPLF,

    /// GPC, but with affiliate results also.
    #[strum(to_string = "gpcaff")]
    GPCAff,

    /// GPC-AUS, but excluding non-Australian lifters.
    #[strum(to_string = "gpc-aus")]
    #[serde(rename = "GPC-AUS")]
    GPCAUS,

    /// GPC-GB, but with international results also.
    #[strum(to_string = "gpc-gb")]
    #[serde(rename = "GPC-GB")]
    GPCGB,

    /// GPC-CRO, but including HPO results and excluding non-Croatians.
    #[strum(to_string = "gpc-cro")]
    #[serde(rename = "GPC-CRO")]
    GPCCRO,

    /// HKWPA, but with international results also.
    #[strum(to_string = "hkwpa")]
    HKWPA,

    /// HPLS, but excluding non-Croatian lifters.
    #[strum(to_string = "hpls")]
    HPLS,

    /// Hunpower, but with international results also.
    #[strum(to_string = "hunpower")]
    Hunpower,

    /// IPA, but only counting meets held in Canada.
    #[strum(to_string = "ipa-can")]
    IPACAN,

    /// IPF including all affiliates, local and regional.
    #[strum(to_string = "ipf-and-affiliates")]
    IPFAndAffiliates,

    /// IPF including select international sub-affiliates.
    #[strum(to_string = "ipf-internationals")]
    IPFInternationals,

    /// IPF-China, but with international results also.
    #[strum(to_string = "ipf-china")]
    IPFChina,

    /// IPL-China, plus IPL results for Chinese lifters.
    #[strum(to_string = "ipl-china")]
    IPLChina,

    /// IPL-China MetaFederation, but only for Tested entries.
    #[strum(to_string = "iplchina-tested")]
    IPLChinaTested,

    /// IranBBF, but with international results also.
    #[strum(to_string = "iranbbf")]
    IranBBF,

    /// IraqPF, but with international results also.
    #[strum(to_string = "iraqpf")]
    IraqPF,

    /// IrishPF, but with international results also.
    #[strum(to_string = "irishpf")]
    IrishPF,

    /// IrishPO, excluding non-Irish lifters and including WPC results.
    #[strum(to_string = "irishpo")]
    IrishPO,

    /// JPA, but with international results also.
    #[strum(to_string = "jpa")]
    JPA,

    /// KBGV, but with international results also.
    #[strum(to_string = "kbgv")]
    KBGV,

    /// KDKS, but with international results also.
    #[strum(to_string = "kdks")]
    KDKS,

    /// KNKFSP, but with international results also.
    #[strum(to_string = "knkf-sp")]
    #[serde(rename = "KNKF-SP")]
    KNKFSP,

    /// KPC, but with international results also.
    #[strum(to_string = "kpc")]
    KPC,

    /// KPF, but with international results also.
    #[strum(to_string = "kpf")]
    KPF,

    /// KRAFT, but with international results also.
    #[strum(to_string = "kraft")]
    KRAFT,

    /// lebanonpf, but with international results also.
    #[strum(to_string = "lebanonpf")]
    LebanonPF,

    /// LFPH, but with international results also.
    #[strum(to_string = "lfph")]
    LFPH,

    /// libyapf, but with international results also.
    #[strum(to_string = "libyapf")]
    LibyaPF,

    /// LJTF, but with international results also.
    #[strum(to_string = "ljtf")]
    LJTF,

    /// LPF, but with international results also.
    #[strum(to_string = "lpf")]
    LPF,

    /// ManxPL, but with BP and international results also.
    #[strum(to_string = "manxpl")]
    ManxPL,

    /// MAP, but with international results also.
    #[strum(to_string = "map")]
    MAP,

    /// MUPF, but with international results also.
    #[strum(to_string = "mupf")]
    MUPF,

    /// NauruPF, but with international results also.
    #[strum(to_string = "naurupf")]
    NauruPF,

    /// NIPF, but with BP and international results also.
    #[strum(to_string = "nipf")]
    NIPF,

    /// NPAJ, but with international results also.
    #[strum(to_string = "npaj")]
    NPAJ,

    /// NSF, but with international results also.
    #[strum(to_string = "nsf")]
    NSF,

    /// NZPF, but with international results also.
    #[strum(to_string = "nzpf")]
    NZPF,

    /// OCWP, but with international results also.
    #[strum(to_string = "ocwp")]
    OCWP,

    /// OEVK, but with international results also.
    #[strum(to_string = "oevk")]
    OEVK,

    /// PA, but excluding non-Australian lifters.
    #[strum(to_string = "pa")]
    PA,

    /// PAP, but with international results also.
    #[strum(to_string = "pap")]
    PAP,

    /// PFBD, but with international results also
    #[strum(to_string = "pfbd")]
    PFBD,

    /// PI, but with international results also.
    #[strum(to_string = "pi")]
    PI,

    /// PLSS, but with international results also.
    #[strum(to_string = "plss")]
    PLSS,

    /// PLZS, but with international results also.
    #[strum(to_string = "plzs")]
    PLZS,

    /// PNGPF, but with international results also.
    #[strum(to_string = "pngpf")]
    PNGPF,

    /// PS, but with international results also.
    #[strum(to_string = "ps")]
    PS,

    /// PWFL, but with international results also.
    #[strum(to_string = "pwfl")]
    PWFL,

    /// PZKFiTS, but with international results also.
    #[strum(to_string = "pzkfits")]
    PZKFiTS,

    /// QatarPL, but with international results also.
    #[strum(to_string = "qatarpl")]
    QatarPL,

    /// SAFKST, but with international results also.
    #[strum(to_string = "safkst")]
    SAFKST,

    /// SAFP, but with international results also.
    #[strum(to_string = "safp")]
    SAFP,

    /// SAPF, but with international results also.
    #[strum(to_string = "sapf")]
    SAPF,

    /// ScottishPL, but with BP and international results also.
    #[strum(to_string = "scottishpl")]
    ScottishPL,

    /// SLPF, but with international results also.
    #[strum(to_string = "slpf")]
    SLPF,

    /// SSF, but with international results also.
    #[strum(to_string = "ssf")]
    SSF,

    /// SSSC, but with international results also.
    #[strum(to_string = "sssc")]
    SSSC,

    /// SVNL, but with international results also.
    #[strum(to_string = "svnl")]
    SVNL,

    /// SwissPL, but with international results also.
    #[strum(to_string = "swisspl")]
    SwissPL,

    /// ThaiPF, excluding non-Thai lifters and including IPF results.
    #[strum(to_string = "thaipf")]
    ThaiPF,

    /// TPSSF, but with international results also.
    #[strum(to_string = "tpssf")]
    TPSSF,

    /// UAEPL, but with international results also.
    #[strum(to_string = "uaepl")]
    UAEPL,

    /// UgandaPF, but with international results also.
    #[strum(to_string = "ugandapf")]
    UgandaPF,

    /// UKPU Tested
    #[strum(to_string = "ukpu-tested")]
    UKPUTested,

    /// UkrainePF, but with international results also.
    #[strum(to_string = "ukrainepf")]
    UkrainePF,

    /// USAPL, but with international results also.
    #[strum(to_string = "usapl")]
    USAPL,

    /// USAPL, which apparently has an Australian affiliate now.
    #[strum(to_string = "usapl-australia")]
    #[serde(rename = "USAPL-Australia")]
    USAPLAustralia,

    /// USPA, plus IPL results for American lifters.
    #[strum(to_string = "uspa")]
    USPA,

    /// USPA MetaFederation, but only for Tested entries.
    #[strum(to_string = "uspa-tested")]
    USPATested,

    /// USPC MetaFederation, but only for Tested entries.
    #[strum(to_string = "uspc-tested")]
    USPCTested,

    /// VGPF, but with international results also.
    #[strum(to_string = "vgpf")]
    VGPF,

    /// VPF, but with international results also.
    #[strum(to_string = "vpf")]
    VPF,

    /// WelshPA, but with BP and international results also.
    #[strum(to_string = "welshpa")]
    WelshPA,

    /// WP-Nauru, but with international results also.
    #[strum(to_string = "wp-nauru")]
    WPNauru,

    /// WP-USA, but with international results also.
    #[strum(to_string = "wp-usa")]
    WPUSA,

    /// WRPF including all affiliates, local and regional.
    #[strum(to_string = "wrpf-and-affiliates")]
    WRPFAndAffiliates,

    /// WRPF-CAN, but only for Tested entries.
    #[strum(to_string = "wrpf-can-tested")]
    #[serde(rename = "WRPF-CAN-Tested")]
    WRPFCANTested,

    /// WRPF-USA.
    #[strum(to_string = "wrpf-usa")]
    #[serde(rename = "WRPF-USA")]
    WRPFUSA,

    /// WRPF-USA, but only for Tested entries.
    #[strum(to_string = "wrpf-usa-tested")]
    #[serde(rename = "WRPF-USA-Tested")]
    WRPFUSATested,

    /// WUAP-CRO, but including HPO results and excluding non-Croatians.
    #[strum(to_string = "wuap-cro")]
    #[serde(rename = "WUAP-CRO")]
    WUAPCRO,
}

/// Helper function for MetaFederation::contains() for AllCountry meta-feds.
#[inline]
fn is_from(country: Country, entry: &Entry, meet: &Meet) -> bool {
    entry.lifter_country == Some(country)
        || (entry.lifter_country.is_none() && meet.federation.home_country() == Some(country))
}

/// Helper macro for specifying a country-based federation that allows lifters
/// to also compete in international affiliates.
macro_rules! affiliation {
    ($meet:ident, $entry:ident, $fed:path, $($affiliates:path),+) => {
        {
            let country: Option<Country> = $fed.home_country();
            match $meet.federation {
                $fed => {
                    $entry.lifter_country.is_none()
                        || $entry.lifter_country == country
                }
                $($affiliates)|+ => {
                    $entry.lifter_country == country
                }
                _ => false,
            }
        }
    };
}

#[rustfmt::skip::macros(date)]
impl MetaFederation {
    /// Defines whether a given `Entry` is part of the MetaFederation.
    ///
    /// Matching is done on Entries instead of on Meets since a MetaFederation
    /// can include Entry-specific information such as Tested status.
    pub fn contains(self, entry: &Entry, meets: &[Meet]) -> bool {
        use Federation::*;
        let meet: &Meet = &meets[entry.meet_id as usize];

        match self {
            MetaFederation::FullyTested => {
                // Still check entry.tested: some fully-tested federations
                // existed before drug-testing was available.
                entry.tested && meet.federation.is_fully_tested(meet.date)
            }
            MetaFederation::AllTested => entry.tested,
            MetaFederation::AllAlgeria => is_from(Country::Algeria, entry, meet),
            MetaFederation::AllArgentina => is_from(Country::Argentina, entry, meet),
            MetaFederation::AllAustralia => is_from(Country::Australia, entry, meet),
            MetaFederation::AllAustraliaTested => {
                entry.tested && is_from(Country::Australia, entry, meet)
            }
            MetaFederation::AllAustria => is_from(Country::Austria, entry, meet),
            MetaFederation::AllAzerbaijan => is_from(Country::Azerbaijan, entry, meet),
            MetaFederation::AllBelarus => is_from(Country::Belarus, entry, meet),
            MetaFederation::AllBelgium => is_from(Country::Belgium, entry, meet),
            MetaFederation::AllIPFBelgium => {
                is_from(Country::Belgium, entry, meet)
                    && (meet.federation == Federation::KBGV
                        || meet.federation == Federation::LFPH
                        || meet.federation == Federation::VGPF
                        || meet.federation == Federation::IPF
                        || meet.federation == Federation::EPF)
            }
            MetaFederation::AllBelize => is_from(Country::Belize, entry, meet),
            MetaFederation::AllBolivia => is_from(Country::Bolivia, entry, meet),
            MetaFederation::AllBosniaAndHerzegovina => {
                is_from(Country::BosniaAndHerzegovina, entry, meet)
            }
            MetaFederation::AllBrazil => is_from(Country::Brazil, entry, meet),
            MetaFederation::AllBrunei => is_from(Country::Brunei, entry, meet),
            MetaFederation::AllBulgaria => is_from(Country::Bulgaria, entry, meet),
            MetaFederation::AllCanada => {
                entry.lifter_country == Some(Country::Canada)
                    || (entry.lifter_country.is_none()
                        && (meet.federation.home_country() == Some(Country::Canada)
                            || MetaFederation::IPACAN.contains(entry, meets)))
            }
            MetaFederation::AllChile => is_from(Country::Chile, entry, meet),
            MetaFederation::AllChina => is_from(Country::China, entry, meet),
            MetaFederation::AllColombia => is_from(Country::Colombia, entry, meet),
            MetaFederation::AllCroatia => is_from(Country::Croatia, entry, meet),
            MetaFederation::AllCyprus => is_from(Country::Cyprus, entry, meet),
            MetaFederation::AllCzechia => is_from(Country::Czechia, entry, meet),
            MetaFederation::AllDenmark => is_from(Country::Denmark, entry, meet),
            MetaFederation::AllEgypt => is_from(Country::Egypt, entry, meet),
            MetaFederation::AllEstonia => is_from(Country::Estonia, entry, meet),
            MetaFederation::AllFinland => is_from(Country::Finland, entry, meet),
            MetaFederation::AllFrance => is_from(Country::France, entry, meet),
            MetaFederation::AllGeorgia => is_from(Country::Georgia, entry, meet),
            MetaFederation::AllGermany => is_from(Country::Germany, entry, meet),
            MetaFederation::AllGreece => is_from(Country::Greece, entry, meet),
            MetaFederation::AllGuatemala => is_from(Country::Guatemala, entry, meet),
            MetaFederation::AllGuyana => is_from(Country::Guyana, entry, meet),
            MetaFederation::AllHongKong => is_from(Country::HongKong, entry, meet),
            MetaFederation::AllHungary => is_from(Country::Hungary, entry, meet),
            MetaFederation::AllIceland => is_from(Country::Iceland, entry, meet),
            MetaFederation::AllIndia => is_from(Country::India, entry, meet),
            MetaFederation::AllIndonesia => is_from(Country::Indonesia, entry, meet),
            // Results for USA lifters in the IPF affiliate at the given time.
            MetaFederation::AllIPFUSA => {
                is_from(Country::USA, entry, meet)
                    && (((meet.federation == Federation::USAPL && meet.date <= date!(2021-11-07))
                        || meet.federation == AMP
                        || meet.federation == NAPF
                        || meet.federation == IPF)
                        || (meet.federation == USPF && meet.date < date!(1997-12-05)))
            }
            MetaFederation::AllIran => is_from(Country::Iran, entry, meet),
            MetaFederation::AllIraq => is_from(Country::Iraq, entry, meet),
            MetaFederation::AllIreland => is_from(Country::Ireland, entry, meet),
            MetaFederation::AllIsrael => is_from(Country::Israel, entry, meet),
            MetaFederation::AllItaly => is_from(Country::Italy, entry, meet),
            MetaFederation::AllJapan => is_from(Country::Japan, entry, meet),
            MetaFederation::AllKazakhstan => is_from(Country::Kazakhstan, entry, meet),
            MetaFederation::AllKuwait => is_from(Country::Kuwait, entry, meet),
            MetaFederation::AllKyrgyzstan => is_from(Country::Kyrgyzstan, entry, meet),
            MetaFederation::AllLatvia => is_from(Country::Latvia, entry, meet),
            MetaFederation::AllLebanon => is_from(Country::Lebanon, entry, meet),
            MetaFederation::AllLibya => is_from(Country::Libya, entry, meet),
            MetaFederation::AllLithuania => is_from(Country::Lithuania, entry, meet),
            MetaFederation::AllMalaysia => is_from(Country::Malaysia, entry, meet),
            MetaFederation::AllMexico => is_from(Country::Mexico, entry, meet),
            MetaFederation::AllMoldova => is_from(Country::Moldova, entry, meet),
            MetaFederation::AllMongolia => is_from(Country::Mongolia, entry, meet),
            MetaFederation::AllNauru => is_from(Country::Nauru, entry, meet),
            MetaFederation::AllNepal => is_from(Country::Nepal, entry, meet),
            MetaFederation::AllNetherlands => is_from(Country::Netherlands, entry, meet),
            MetaFederation::AllNewZealand => is_from(Country::NewZealand, entry, meet),
            MetaFederation::AllNicaragua => is_from(Country::Nicaragua, entry, meet),
            MetaFederation::AllNiue => is_from(Country::Niue, entry, meet),
            MetaFederation::AllNorway => is_from(Country::Norway, entry, meet),
            MetaFederation::AllOman => is_from(Country::Oman, entry, meet),
            MetaFederation::AllPanama => is_from(Country::Panama, entry, meet),
            MetaFederation::AllPapuaNewGuinea => is_from(Country::PapuaNewGuinea, entry, meet),
            MetaFederation::AllParaguay => is_from(Country::Paraguay, entry, meet),
            MetaFederation::AllPhilippines => is_from(Country::Philippines, entry, meet),
            MetaFederation::AllPoland => is_from(Country::Poland, entry, meet),
            MetaFederation::AllPortugal => is_from(Country::Portugal, entry, meet),
            MetaFederation::AllQatar => is_from(Country::Qatar, entry, meet),
            MetaFederation::AllRomania => is_from(Country::Romania, entry, meet),
            MetaFederation::AllRussia => is_from(Country::Russia, entry, meet),
            MetaFederation::AllSaudiArabia => is_from(Country::SaudiArabia, entry, meet),
            MetaFederation::AllScotland => is_from(Country::Scotland, entry, meet),
            MetaFederation::AllSerbia => is_from(Country::Serbia, entry, meet),
            MetaFederation::AllSingapore => is_from(Country::Singapore, entry, meet),
            MetaFederation::AllSlovakia => is_from(Country::Slovakia, entry, meet),
            MetaFederation::AllSlovenia => is_from(Country::Slovenia, entry, meet),
            MetaFederation::AllSpain => is_from(Country::Spain, entry, meet),
            MetaFederation::AllSriLanka => is_from(Country::SriLanka, entry, meet),
            MetaFederation::AllSouthAfrica => is_from(Country::SouthAfrica, entry, meet),
            MetaFederation::AllSweden => is_from(Country::Sweden, entry, meet),
            MetaFederation::AllSyria => is_from(Country::Syria, entry, meet),
            MetaFederation::AllSwitzerland => is_from(Country::Switzerland, entry, meet),
            MetaFederation::AllTaiwan => is_from(Country::Taiwan, entry, meet),
            MetaFederation::AllThailand => is_from(Country::Thailand, entry, meet),
            MetaFederation::AllTurkey => is_from(Country::Turkey, entry, meet),
            MetaFederation::AllUSVirginIslands => is_from(Country::USVirginIslands, entry, meet),
            MetaFederation::AllUK => {
                // UK lifters sometimes switch country affiliation from UK to Ireland
                // when they compete IrishPF.
                //
                // Assume that all IrishPF lifting is done for Ireland,
                // even if the lifter is marked as UK.
                if meet.federation == Federation::IrishPF {
                    false
                } else {
                    entry.lifter_country.map_or(false, |c| c.is_in_uk())
                        || (entry.lifter_country.is_none()
                            && meet
                                .federation
                                .home_country()
                                .map_or(false, |c| c.is_in_uk()))
                }
            }
            MetaFederation::AllUAE => is_from(Country::UAE, entry, meet),
            MetaFederation::AllUganda => is_from(Country::Uganda, entry, meet),
            MetaFederation::AllUKTested => {
                entry.tested && MetaFederation::AllUK.contains(entry, meets)
            }
            MetaFederation::AllUkraine => is_from(Country::Ukraine, entry, meet),
            MetaFederation::AllUSA => is_from(Country::USA, entry, meet),
            MetaFederation::AllUSATested => {
                entry.tested && MetaFederation::AllUSA.contains(entry, meets)
            }
            MetaFederation::AllVietnam => is_from(Country::Vietnam, entry, meet),
            MetaFederation::AAPF => entry.tested && affiliation!(meet, entry, APF, WPC),
            MetaFederation::ABPU => {
                entry.tested
                    && (meet.federation == Federation::BPU
                        || (meet.federation == Federation::WPC
                            && entry.lifter_country.map_or(false, |c| c.is_in_uk())
                            && meet.date.year() >= 2013))
            }
            MetaFederation::AEP => affiliation!(meet, entry, AEP, IPF, EPF),
            MetaFederation::AIrishPO => {
                entry.tested
                    && (meet.federation == Federation::IrishPO
                        || (meet.federation == Federation::WPC
                            && entry.lifter_country == Some(Country::Ireland)))
            }
            MetaFederation::AIWBPA => affiliation!(meet, entry, AIWBPA, IPF, AsianPF),
            MetaFederation::AMP => {
                affiliation!(meet, entry, AMP, IPF, NAPF) && meet.date.year() >= 2022
            }
            MetaFederation::APF => affiliation!(meet, entry, APF, WPC),

            // APLA formed in 2024 and became IPF and ORPF/CommonwealthPF affiliate at this time, replacing APU.
            MetaFederation::APLA => {
                affiliation!(meet, entry, APLA, IPF, ORPF, CommonwealthPF)
                    && meet.date.year() >= 2024
            }
            MetaFederation::APP => affiliation!(meet, entry, APP, GPA),

            // APU formed in 2018 and was IPF affiliate until end of 2023.
            // APU was originally affiliated to ORPF but changed to AsianPF from 2021.
            MetaFederation::APU => {
                ((2018..2021).contains(&meet.date.year())
                    && affiliation!(meet, entry, APU, IPF, CommonwealthPF, ORPF))
                    || ((2021..2024).contains(&meet.date.year())
                        && affiliation!(meet, entry, APU, IPF, CommonwealthPF, AsianPF))
                    || (meet.date.year() >= 2024 && affiliation!(meet, entry, APU, WDFPF))
            }
            MetaFederation::AusPLTested => meet.federation == Federation::AusPL && entry.tested,
            MetaFederation::UKPUTested => meet.federation == Federation::UKPU && entry.tested,
            MetaFederation::AWPC => meet.federation == Federation::WPC && entry.tested,
            MetaFederation::BelPF => affiliation!(meet, entry, BelPF, IPF, EPF),
            MetaFederation::BP => {
                meet.federation == Federation::BAWLA
                    || meet.federation == Federation::BP
                    || meet.federation == Federation::EPA
                    || meet.federation == Federation::NIPF
                    || meet.federation == Federation::ScottishPL
                    || meet.federation == Federation::WelshPA
                    || meet.federation == Federation::ManxPL

                    // British lifters expect their international results included.
                    || (entry.lifter_country.map_or(false, |c| c.is_in_uk()) &&
                        (meet.federation == Federation::IPF
                         || meet.federation == Federation::EPF
                         || meet.federation == Federation::CommonwealthPF))
            }
            MetaFederation::BDFPA => {
                meet.federation == Federation::BDFPA
                    || (meet.federation == Federation::WDFPF
                        && entry.lifter_country.map_or(false, |c| c.is_in_uk()))
            }
            MetaFederation::BPC => {
                meet.federation == Federation::BPC
                    || (meet.federation == Federation::WPC
                        && entry.lifter_country.map_or(false, |c| c.is_in_uk())
                        && meet.date.year() <= 2012)
            }
            MetaFederation::BPU => {
                meet.federation == Federation::BPU
                    || (meet.federation == Federation::WPC
                        && entry.lifter_country.map_or(false, |c| c.is_in_uk())
                        && meet.date.year() >= 2013)
            }
            MetaFederation::BulgarianPF => affiliation!(meet, entry, BulgarianPF, IPF, EPF),
            MetaFederation::BVDK => match meet.federation {
                // BVDG is the precursor to the BVDK.
                Federation::BVDG | Federation::BVDK => {
                    entry.lifter_country.is_none() || entry.lifter_country == Some(Country::Germany)
                }
                Federation::IPF | Federation::EPF => entry.lifter_country == Some(Country::Germany),
                _ => false,
            },
            MetaFederation::CBLB => affiliation!(meet, entry, CBLB, IPF, FESUPO),
            MetaFederation::CPU => affiliation!(meet, entry, CPU, IPF, NAPF, CommonwealthPF),
            MetaFederation::CSST => affiliation!(meet, entry, CSST, IPF, EPF),
            MetaFederation::CyprusPF => affiliation!(meet, entry, CyprusPF, IPF, EPF),
            MetaFederation::DPL => affiliation!(meet, entry, DPL, IPL),
            MetaFederation::DSF => affiliation!(meet, entry, DSF, IPF, EPF, NordicPF),
            MetaFederation::EgyptPF => affiliation!(meet, entry, EgyptPF, IPF, AfricanPF),
            MetaFederation::EJTL => affiliation!(meet, entry, EJTL, IPF, EPF, NordicPF),
            MetaFederation::EPA => affiliation!(meet, entry, EPA, IPF, EPF, BP),
            MetaFederation::FALPO => affiliation!(meet, entry, FALPO, IPF, FESUPO),
            MetaFederation::FAPL => affiliation!(meet, entry, FAPL, IPF, AfricanPF),
            MetaFederation::FCP => affiliation!(meet, entry, FCP, IPF, EPF),
            MetaFederation::FECAPOLIF => affiliation!(meet, entry, FECAPOLIF, IPF, AfricanPF),
            MetaFederation::FECHIPO => affiliation!(meet, entry, FECHIPO, IPF, FESUPO),
            MetaFederation::FEMEPO => affiliation!(meet, entry, FEMEPO, IPF, NAPF),
            MetaFederation::FFForce => {
                (
                    meet.federation == Federation::FFForce
                    && (entry.lifter_country.is_none() || entry.lifter_country == Some(Country::France))
                )
                // The FFHMFAC is the precursor to the FFForce.
                || (
                        meet.federation == Federation::FFHMFAC
                        && (entry.lifter_country.is_none() || entry.lifter_country == Some(Country::France))
                    )
                // French lifters expect their international results included.
                || (
                        is_from(Country::France, entry, meet)
                        && (meet.federation == Federation::IPF || meet.federation == Federation::EPF)
                    )
            }
            MetaFederation::FIPL => affiliation!(meet, entry, FIPL, IPF, EPF),
            MetaFederation::FPP => affiliation!(meet, entry, FPP, IPF, NAPF),
            MetaFederation::FPPR => affiliation!(meet, entry, FPPR, IPF, NAPF),
            MetaFederation::FPR => affiliation!(meet, entry, FPR, IPF, EPF),
            MetaFederation::FRPL => affiliation!(meet, entry, FRPL, IPF, EPF),
            MetaFederation::GAPLF => affiliation!(meet, entry, GAPLF, IPF, FESUPO, NAPF),
            MetaFederation::GPCAff => {
                meet.federation.sanctioning_body(meet.date) == Some(Federation::GPC)
            }
            MetaFederation::GPCAUS => affiliation!(meet, entry, GPCAUS, GPC),
            MetaFederation::GPCGB => match meet.federation {
                Federation::GPCGB => true,
                fed => {
                    if let Some(country) = entry.lifter_country {
                        country.is_in_uk()
                            && country != Country::NorthernIreland
                            && fed.sanctioning_body(meet.date) == Some(Federation::GPC)
                    } else {
                        false
                    }
                }
            },
            MetaFederation::GPCCRO => {
                (meet.federation == Federation::GPCCRO || meet.federation == Federation::HPO)
                    && (entry.lifter_country.is_none()
                        || entry.lifter_country == Some(Country::Croatia))
            }
            MetaFederation::HKWPA => affiliation!(meet, entry, HKWPA, IPF, AsianPF),
            MetaFederation::HPLS => affiliation!(meet, entry, HPLS, IPF, EPF),
            MetaFederation::Hunpower => affiliation!(meet, entry, Hunpower, IPF, EPF),
            MetaFederation::IPACAN => {
                meet.federation == Federation::IPA && meet.country == Country::Canada
            }
            MetaFederation::IPFAndAffiliates => {
                meet.federation.sanctioning_body(meet.date) == Some(Federation::IPF)
            }
            MetaFederation::IPFInternationals => matches!(
                meet.federation,
                Federation::IPF
                    | Federation::AfricanPF
                    | Federation::AsianPF
                    | Federation::EPF
                    | Federation::FESUPO
                    | Federation::NAPF
                    | Federation::ORPF
                    | Federation::CommonwealthPF
            ),
            MetaFederation::IPFChina => affiliation!(meet, entry, IPFChina, IPF, AsianPF),
            MetaFederation::IPLChina => affiliation!(meet, entry, IPLChina, IPL),
            MetaFederation::IPLChinaTested => {
                entry.tested && MetaFederation::IPLChina.contains(entry, meets)
            }
            MetaFederation::IranBBF => affiliation!(meet, entry, IranBBF, IPF, AsianPF),
            MetaFederation::IraqPF => affiliation!(meet, entry, IraqPF, IPF, AsianPF),
            MetaFederation::IrishPF => affiliation!(meet, entry, IrishPF, IPF, EPF),
            MetaFederation::IrishPO => affiliation!(meet, entry, IrishPO, WPC),
            MetaFederation::JPA => affiliation!(meet, entry, JPA, IPF, AsianPF),
            MetaFederation::KBGV => affiliation!(meet, entry, KBGV, IPF, EPF),
            MetaFederation::KNKFSP => affiliation!(meet, entry, KNKFSP, NPB, IPF, EPF),
            MetaFederation::LebanonPF => affiliation!(meet, entry, LebanonPF, IPF, AsianPF),
            MetaFederation::LFPH => affiliation!(meet, entry, LFPH, IPF, EPF, KBGV),
            MetaFederation::LibyaPF => affiliation!(meet, entry, LibyaPF, IPF, AfricanPF),
            MetaFederation::LJTF => affiliation!(meet, entry, LJTF, IPF, EPF),
            MetaFederation::LPF => affiliation!(meet, entry, LPF, IPF, EPF),
            MetaFederation::KDKS => {
                affiliation!(meet, entry, KDKS, IPF, EPF) && meet.date.year() >= 2020
            }
            MetaFederation::KPC => affiliation!(meet, entry, KPC, IPF, AsianPF, UAEPL), // Kuwait often use UAE meets for team selection
            MetaFederation::KPF => affiliation!(meet, entry, KPF, IPF, AsianPF),
            MetaFederation::KRAFT => affiliation!(meet, entry, KRAFT, IPF, EPF, NordicPF),
            MetaFederation::ManxPL => affiliation!(meet, entry, ManxPL, IPF, EPF, BP, EPA),
            MetaFederation::MAP => affiliation!(meet, entry, MAP, IPF, AsianPF),
            MetaFederation::MUPF => affiliation!(meet, entry, MUPF, IPF, AsianPF),
            MetaFederation::NauruPF => affiliation!(meet, entry, NauruPF, IPF, ORPF),
            MetaFederation::NIPF => affiliation!(meet, entry, NIPF, IPF, EPF, BP),
            MetaFederation::NPAJ => affiliation!(meet, entry, NPAJ, IPF, NAPF),
            MetaFederation::NSF => affiliation!(meet, entry, NSF, IPF, EPF, NordicPF),
            MetaFederation::NZPF => {
                meet.federation == Federation::NZPF
                    || meet.federation == Federation::NZAWLA
                    // New Zealand lifters expect their international results included.
                    || is_from(Country::NewZealand, entry, meet) &&
                        (meet.federation == Federation::IPF
                         || meet.federation == Federation::CommonwealthPF
                         || meet.federation == Federation::ORPF
                         || (meet.federation == Federation::OceaniaPF && meet.date.year() <= 2017))
            }
            MetaFederation::OCWP => affiliation!(meet, entry, OCWP, IPF, AsianPF),
            MetaFederation::OEVK => affiliation!(meet, entry, OEVK, IPF, EPF),
            MetaFederation::PA => affiliation!(meet, entry, PA, WP),
            MetaFederation::PAP => affiliation!(meet, entry, PAP, IPF, AsianPF),
            MetaFederation::PFBD => affiliation!(meet, entry, PFBD, IPF, AsianPF),
            MetaFederation::PI => affiliation!(meet, entry, PI, IPF, AsianPF),
            MetaFederation::PLSS => affiliation!(meet, entry, PLSS, IPF, EPF),
            MetaFederation::PLZS => affiliation!(meet, entry, PLZS, IPF, EPF),
            MetaFederation::PNGPF => affiliation!(meet, entry, PNGPF, IPF, ORPF),
            MetaFederation::PS => affiliation!(meet, entry, PS, IPF, AsianPF),
            MetaFederation::PWFL => affiliation!(meet, entry, PWFL, IPF, EPF),
            MetaFederation::PZKFiTS => affiliation!(meet, entry, PZKFiTS, IPF, EPF),
            MetaFederation::QatarPL => affiliation!(meet, entry, QatarPL, IPF, AsianPF),
            MetaFederation::SAFKST => affiliation!(meet, entry, SAFKST, IPF, EPF),           
            MetaFederation::SAFP => affiliation!(meet, entry, SAFP, IPF, AsianPF),
            MetaFederation::SAPF => affiliation!(meet, entry, SAPF, IPF, AfricanPF, CommonwealthPF),
            MetaFederation::ScottishPL => affiliation!(meet, entry, ScottishPL, IPF, EPF, BP),
            MetaFederation::SLPF => affiliation!(meet, entry, SLPF, IPF, AsianPF),
            MetaFederation::SSF => affiliation!(meet, entry, SSF, IPF, EPF, NordicPF),
            MetaFederation::SSSC => affiliation!(meet, entry, SSSC, IPF, AsianPF),
            MetaFederation::SVNL => affiliation!(meet, entry, SVNL, IPF, EPF, NordicPF),
            MetaFederation::SwissPL => {
                let country: Option<Country> = SwissPL.home_country();
                match meet.federation {
                    SwissPL => entry.lifter_country.is_none() || entry.lifter_country == country,
                    IPF | EPF => {
                        entry.lifter_country == country
                            && SwissPL.sanctioning_body(meet.date) == Some(IPF)
                    }
                    _ => false,
                }
            }
            MetaFederation::ThaiPF => affiliation!(meet, entry, ThaiPF, IPF, AsianPF),
            MetaFederation::TPSSF => affiliation!(meet, entry, TPSSF, IPF, EPF),
            MetaFederation::UAEPL => affiliation!(meet, entry, UAEPL, IPF, AsianPF, OceaniaPF),
            MetaFederation::UgandaPF => affiliation!(meet, entry, UgandaPF, WP),
            MetaFederation::UkrainePF => affiliation!(meet, entry, UkrainePF, IPF, EPF),

            // Only include USA entries for meets directly affiliated with USAPL at any time,
            // or USA entries for NAPF/IPF meets after USAPL became IPF affiliate on 5 Dec 1997
            MetaFederation::USAPL => {
                is_from(Country::USA, entry, meet)
                    && (meet.federation == Federation::USAPL
                        || ((meet.federation == NAPF || meet.federation == IPF)
                            && meet.date >= date!(1997-12-05)
                            && meet.date <= date!(2021-11-07))
                        || (meet.federation == ADFPA && meet.date < date!(1997-12-05)))
            }
            // Include USAPL Australia results after metafederation formed from 2022-05-14.
            MetaFederation::USAPLAustralia => {
                meet.federation == Federation::USAPL
                    && entry.lifter_country == Some(Country::Australia)
                    && meet.date >= date!(2022-05-14)
            }
            MetaFederation::USPA => affiliation!(meet, entry, USPA, IPL),
            MetaFederation::USPATested => {
                entry.tested && MetaFederation::USPA.contains(entry, meets)
            }
            MetaFederation::USPCTested => meet.federation == Federation::USPC && entry.tested,
            MetaFederation::VGPF => affiliation!(meet, entry, VGPF, IPF, EPF, KBGV),
            MetaFederation::VPF => affiliation!(meet, entry, VPF, IPF, AsianPF),
            MetaFederation::WelshPA => affiliation!(meet, entry, WelshPA, IPF, EPF, BP),
            MetaFederation::WPNauru => affiliation!(meet, entry, WPNauru, WP),
            MetaFederation::WPUSA => affiliation!(meet, entry, WPUSA, WP),
            MetaFederation::WRPFAndAffiliates => {
                meet.federation.sanctioning_body(meet.date) == Some(Federation::WRPF)
            }
            MetaFederation::WRPFCANTested => meet.federation == Federation::WRPFCAN && entry.tested,
            MetaFederation::WRPFUSA => match meet.federation {
                Federation::WRPF => match entry.lifter_country {
                    Some(Country::USA) => true,
                    None => meet.country == Country::USA,
                    _ => false,
                },
                _ => false,
            },
            MetaFederation::WRPFUSATested => {
                entry.tested && MetaFederation::WRPFUSA.contains(entry, meets)
            }
            MetaFederation::WUAPCRO => {
                (meet.federation == Federation::WUAPCRO || meet.federation == Federation::HPO)
                    && (entry.lifter_country.is_none()
                        || entry.lifter_country == Some(Country::Croatia))
            }
        }
    }
}

/// Pre-computed list of meets in a MetaFederation.
///
/// A meet is part of the MetaFederation if it contains
/// at least one entry such that `MetaFederation::contains(entry)`.
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaFederationCache {
    /// Uses (MetaFederation as usize) as index to a list of meet_ids
    /// for that MetaFederation.
    cache: Vec<Vec<u32>>,
}

impl MetaFederationCache {
    pub fn meet_ids_for(&self, meta: MetaFederation) -> &[u32] {
        &self.cache[meta as usize]
    }

    /// Fill in the MetaFederationCache during CSV importation.
    ///
    /// The `entries` vector should be sorted by `entry.meet_id`,
    /// not by `entry.lifter_id` as it is post-importation.
    pub fn make(meets: &[Meet], entries: &[Entry]) -> MetaFederationCache {
        let num_metafeds: usize = MetaFederation::iter().count();

        // Vector of list of meets for each MetaFederation.
        let mut ret: Vec<Vec<u32>> = Vec::with_capacity(num_metafeds);
        ret.resize(num_metafeds, vec![]);

        // Vector of whether each meet has a match for the
        // given MetaFederation (accessed via index).
        let mut contains: Vec<bool> = Vec::with_capacity(num_metafeds);
        contains.resize(num_metafeds, false);

        let mut last_meet_id = 0;

        // Iterate by grouping entries from the same Meet.
        for (meet_id, meet_entries) in entries.iter().group_by(|e| e.meet_id).into_iter() {
            // Sanity checking that the entries argument is sorted by meet_id.
            assert!(last_meet_id <= meet_id);
            last_meet_id = meet_id;

            // Check whether any entries are part of each MetaFederation.
            for entry in meet_entries {
                for meta in MetaFederation::iter() {
                    if meta.contains(entry, meets) {
                        contains[meta as usize] = true;
                    }
                }
            }

            // If any match, add to that MetaFederation's meet list.
            for i in 0..num_metafeds {
                if contains[i] {
                    ret[i].push(meet_id);
                }
                // Reset the vector for the next iteration.
                contains[i] = false;
            }
        }

        // Since we're here already, sort the return vector by Date
        // in reverse order -- that's usually how it's consumed.
        for v in &mut ret {
            v.sort_unstable_by(|&a, &b| {
                meets[a as usize]
                    .date
                    .cmp(&meets[b as usize].date)
                    .reverse()
            });
        }

        MetaFederationCache { cache: ret }
    }
}
