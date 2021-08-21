//! MetaFederation definitions and calculations.

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
#[derive(Copy, Clone, Debug, Deserialize, Display, PartialEq, Serialize, EnumIter, EnumString)]
pub enum MetaFederation {
    /// Federations that are exclusively (non-optionally) tested.
    #[strum(to_string = "fully-tested")]
    FullyTested,

    /// All entries that have "Tested = Yes".
    #[strum(to_string = "all-tested")]
    AllTested,

    #[strum(to_string = "all-argentina")]
    AllArgentina,
    #[strum(to_string = "all-australia")]
    AllAustralia,
    #[strum(to_string = "all-austria")]
    AllAustria,
    #[strum(to_string = "all-azerbaijan")]
    AllAzerbaijan,
    #[strum(to_string = "all-belarus")]
    AllBelarus,
    #[strum(to_string = "all-belgium")]
    AllBelgium,
    #[strum(to_string = "all-bosnia-and-herzegovina")]
    AllBosniaAndHerzegovina,
    #[strum(to_string = "all-brazil")]
    AllBrazil,
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
    #[strum(to_string = "all-czechia")]
    AllCzechia,
    #[strum(to_string = "all-denmark")]
    AllDenmark,
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
    #[strum(to_string = "all-lithuania")]
    AllLithuania,
    #[strum(to_string = "all-malaysia")]
    AllMalaysia,
    #[strum(to_string = "all-mexico")]
    AllMexico,
    #[strum(to_string = "all-moldova")]
    AllMoldova,
    #[strum(to_string = "all-nauru")]
    AllNauru,
    #[strum(to_string = "all-netherlands")]
    AllNetherlands,
    #[strum(to_string = "all-newzealand")]
    AllNewZealand,
    #[strum(to_string = "all-niue")]
    AllNiue,
    #[strum(to_string = "all-norway")]
    AllNorway,
    #[strum(to_string = "all-papuanewguinea")]
    AllPapuaNewGuinea,
    #[strum(to_string = "all-paraguay")]
    AllParaguay,
    #[strum(to_string = "all-philippines")]
    AllPhilippines,
    #[strum(to_string = "all-poland")]
    AllPoland,
    #[strum(to_string = "all-portugal")]
    AllPortugal,
    #[strum(to_string = "all-romania")]
    AllRomania,
    #[strum(to_string = "all-russia")]
    AllRussia,
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
    #[strum(to_string = "all-sweden")]
    AllSweden,
    #[strum(to_string = "all-switzerland")]
    AllSwitzerland,
    #[strum(to_string = "all-thailand")]
    AllThailand,
    #[strum(to_string = "all-turkey")]
    AllTurkey,
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
    #[strum(to_string = "all-usvirginislands")]
    AllUSVirginIslands,

    /// APF, but only Tested entries.
    #[strum(to_string = "aapf")]
    AAPF,

    /// BPU, but only tested entries.
    #[strum(to_string = "abpu")]
    ABPU,

    /// ABS Series, a recurring Irish competition.
    #[strum(to_string = "abs-series")]
    ABSSeries,

    /// AEP, but with international results also.
    #[strum(to_string = "aep")]
    AEP,

    /// AIWBPA, but with international results also.
    #[strum(to_string = "aiwbpa")]
    AIWBPA,

    /// APP, but with international results also.
    #[strum(to_string = "app")]
    APP,

    /// APU, but with international results also.
    #[strum(to_string = "apu")]
    APU,

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

    /// DSF, but with international results also.
    #[strum(to_string = "dsf")]
    DSF,

    /// EJTL, but with international results also.
    #[strum(to_string = "ejtl")]
    EJTL,

    /// EPA, but with BP and international results also.
    #[strum(to_string = "epa")]
    EPA,

    /// FALPO, but with international results also.
    #[strum(to_string = "falpo")]
    FALPO,

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

    /// FPPR, but with international results also.
    #[strum(to_string = "fppr")]
    FPPR,

    /// FPR, but with international results also.
    #[strum(to_string = "fpr")]
    FPR,

    /// FRPL, but with international results also.
    #[strum(to_string = "frpl")]
    FRPL,

    /// GPC-AUS, but excluding non-Australian lifters.
    #[strum(to_string = "gpc-aus")]
    #[serde(rename = "GPC-AUS")]
    GPCAUS,

    /// GPC-GB, but with international results also.
    #[strum(to_string = "gpc-gb")]
    #[serde(rename = "GPC-GB")]
    GPCGB,

    /// GPC-WUAP-CRO, but including HPO results and excluding non-Croatians.
    #[strum(to_string = "gpc-wuap-cro")]
    #[serde(rename = "GPC-WUAP-CRO")]
    GPCWUAPCRO,

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

    /// IrishPF, but with international results also.
    #[strum(to_string = "irishpf")]
    IrishPF,

    /// IrishPO, excluding non-Irish lifters and including WPC results.
    #[strum(to_string = "irishpo")]
    IrishPO,

    /// JPA, but with international results also.
    #[strum(to_string = "jpa")]
    JPA,

    /// KNKFSP, but with international results also.
    #[strum(to_string = "knkf-sp")]
    #[serde(rename = "KNKF-SP")]
    KNKFSP,

    /// KPF, but with international results also.
    #[strum(to_string = "kpf")]
    KPF,

    /// KRAFT, but with international results also.
    #[strum(to_string = "kraft")]
    KRAFT,

    /// LFPH, but with international results also.
    #[strum(to_string = "lfph")]
    LFPH,

    /// LJTF, but with international results also.
    #[strum(to_string = "ljtf")]
    LJTF,

    /// LPF, but with international results also.
    #[strum(to_string = "lpf")]
    LPF,

    /// NauruPF, but with international results also.
    #[strum(to_string = "naurupf")]
    NauruPF,

    /// NIPF, but with BP and international results also.
    #[strum(to_string = "nipf")]
    NIPF,

    /// NSF, but with international results also.
    #[strum(to_string = "nsf")]
    NSF,

    /// NZPF, but with international results also.
    #[strum(to_string = "nzpf")]
    NZPF,

    /// OEVK, but with international results also.
    #[strum(to_string = "oevk")]
    OEVK,

    /// PA, but excluding non-Australian lifters.
    #[strum(to_string = "pa")]
    PA,

    /// PAP, but with international results also.
    #[strum(to_string = "pap")]
    PAP,

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

    /// PZKFiTS, but with international results also.
    #[strum(to_string = "pzkfits")]
    PZKFiTS,

    /// SAPF, but with international results also.
    #[strum(to_string = "sapf")]
    SAPF,

    /// ScottishPL, but with BP and international results also.
    #[strum(to_string = "scottishpl")]
    ScottishPL,

    /// SSF, but with international results also.
    #[strum(to_string = "ssf")]
    SSF,

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

    /// UgandaPF, but with international results also.
    #[strum(to_string = "ugandapf")]
    UgandaPF,

    /// UkrainePF, but with international results also.
    #[strum(to_string = "ukrainepf")]
    UkrainePF,

    /// USAPL, but with international results also.
    #[strum(to_string = "usapl")]
    USAPL,

    /// USPA, plus IPL results for American lifters.
    #[strum(to_string = "uspa")]
    USPA,

    /// USPA MetaFederation, but only for Tested entries.
    #[strum(to_string = "uspa-tested")]
    USPATested,

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

    /// WRPF-USA.
    #[strum(to_string = "wrpf-usa")]
    #[serde(rename = "WRPF-USA")]
    WRPFUSA,
}

/// Helper function for MetaFederation::contains() for AllCountry meta-feds.
#[inline]
fn is_from(country: Country, entry: &Entry, meet: &Meet) -> bool {
    entry.lifter_country == Some(country)
        || (entry.lifter_country == None && meet.federation.home_country() == Some(country))
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
            MetaFederation::AllArgentina => is_from(Country::Argentina, entry, meet),
            MetaFederation::AllAustralia => is_from(Country::Australia, entry, meet),
            MetaFederation::AllAustria => is_from(Country::Austria, entry, meet),
            MetaFederation::AllAzerbaijan => is_from(Country::Azerbaijan, entry, meet),
            MetaFederation::AllBelarus => is_from(Country::Belarus, entry, meet),
            MetaFederation::AllBelgium => is_from(Country::Belgium, entry, meet),
            MetaFederation::AllBosniaAndHerzegovina => {
                is_from(Country::BosniaAndHerzegovina, entry, meet)
            }
            MetaFederation::AllBrazil => is_from(Country::Brazil, entry, meet),
            MetaFederation::AllCanada => {
                entry.lifter_country == Some(Country::Canada)
                    || (entry.lifter_country == None
                        && (meet.federation.home_country() == Some(Country::Canada)
                            || MetaFederation::IPACAN.contains(entry, meets)))
            }
            MetaFederation::AllChile => is_from(Country::Chile, entry, meet),
            MetaFederation::AllChina => is_from(Country::China, entry, meet),
            MetaFederation::AllColombia => is_from(Country::Colombia, entry, meet),
            MetaFederation::AllCroatia => is_from(Country::Croatia, entry, meet),
            MetaFederation::AllCzechia => is_from(Country::Czechia, entry, meet),
            MetaFederation::AllDenmark => is_from(Country::Denmark, entry, meet),
            MetaFederation::AllEstonia => is_from(Country::Estonia, entry, meet),
            MetaFederation::AllFinland => is_from(Country::Finland, entry, meet),
            MetaFederation::AllFrance => is_from(Country::France, entry, meet),
            MetaFederation::AllGeorgia => is_from(Country::Georgia, entry, meet),
            MetaFederation::AllGermany => is_from(Country::Germany, entry, meet),
            MetaFederation::AllGreece => is_from(Country::Greece, entry, meet),
            MetaFederation::AllHongKong => is_from(Country::HongKong, entry, meet),
            MetaFederation::AllHungary => is_from(Country::Hungary, entry, meet),
            MetaFederation::AllIceland => is_from(Country::Iceland, entry, meet),
            MetaFederation::AllIndia => is_from(Country::India, entry, meet),
            MetaFederation::AllIndonesia => is_from(Country::Indonesia, entry, meet),
            // Results for US lifters in the IPF affiliate at the time, for <= 1997-12-5 this is the USPF
            // and for > 1997-12-5 this is the USAPL
            MetaFederation::AllIPFUSA => {
                is_from(Country::USA, entry, meet)
                    && ((meet.federation == Federation::USAPL
                        || meet.federation == NAPF
                        || meet.federation == IPF)
                        || (meet.federation == USPF && meet.date < Date::from_parts(1997, 12, 5)))
            }
            MetaFederation::AllIran => is_from(Country::Iran, entry, meet),
            MetaFederation::AllIreland => is_from(Country::Ireland, entry, meet),
            MetaFederation::AllIsrael => is_from(Country::Israel, entry, meet),
            MetaFederation::AllItaly => is_from(Country::Italy, entry, meet),
            MetaFederation::AllJapan => is_from(Country::Japan, entry, meet),
            MetaFederation::AllKazakhstan => is_from(Country::Kazakhstan, entry, meet),
            MetaFederation::AllKuwait => is_from(Country::Kuwait, entry, meet),
            MetaFederation::AllKyrgyzstan => is_from(Country::Kyrgyzstan, entry, meet),
            MetaFederation::AllLatvia => is_from(Country::Latvia, entry, meet),
            MetaFederation::AllLithuania => is_from(Country::Lithuania, entry, meet),
            MetaFederation::AllMalaysia => is_from(Country::Malaysia, entry, meet),
            MetaFederation::AllMexico => is_from(Country::Mexico, entry, meet),
            MetaFederation::AllMoldova => is_from(Country::Moldova, entry, meet),
            MetaFederation::AllNauru => is_from(Country::Nauru, entry, meet),
            MetaFederation::AllNetherlands => is_from(Country::Netherlands, entry, meet),
            MetaFederation::AllNewZealand => is_from(Country::NewZealand, entry, meet),
            MetaFederation::AllNiue => is_from(Country::Niue, entry, meet),
            MetaFederation::AllNorway => is_from(Country::Norway, entry, meet),
            MetaFederation::AllPapuaNewGuinea => is_from(Country::PapuaNewGuinea, entry, meet),
            MetaFederation::AllParaguay => is_from(Country::Paraguay, entry, meet),
            MetaFederation::AllPhilippines => is_from(Country::Philippines, entry, meet),
            MetaFederation::AllPoland => is_from(Country::Poland, entry, meet),
            MetaFederation::AllPortugal => is_from(Country::Portugal, entry, meet),
            MetaFederation::AllRomania => is_from(Country::Romania, entry, meet),
            MetaFederation::AllRussia => is_from(Country::Russia, entry, meet),
            MetaFederation::AllScotland => is_from(Country::Scotland, entry, meet),
            MetaFederation::AllSerbia => is_from(Country::Serbia, entry, meet),
            MetaFederation::AllSingapore => is_from(Country::Singapore, entry, meet),
            MetaFederation::AllSlovakia => is_from(Country::Slovakia, entry, meet),
            MetaFederation::AllSlovenia => is_from(Country::Slovenia, entry, meet),
            MetaFederation::AllSpain => is_from(Country::Spain, entry, meet),
            MetaFederation::AllSouthAfrica => is_from(Country::SouthAfrica, entry, meet),
            MetaFederation::AllSweden => is_from(Country::Sweden, entry, meet),
            MetaFederation::AllSwitzerland => is_from(Country::Switzerland, entry, meet),
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
                        || (entry.lifter_country == None
                            && meet
                                .federation
                                .home_country()
                                .map_or(false, |c| c.is_in_uk()))
                }
            }
            MetaFederation::AllUganda => is_from(Country::Uganda, entry, meet),
            MetaFederation::AllUKTested => {
                entry.tested && MetaFederation::AllUK.contains(entry, meets)
            }
            MetaFederation::AllUkraine => is_from(Country::Ukraine, entry, meet),
            MetaFederation::AllUSA => is_from(Country::USA, entry, meet),
            MetaFederation::AllVietnam => is_from(Country::Vietnam, entry, meet),
            MetaFederation::AAPF => meet.federation == Federation::APF && entry.tested,
            MetaFederation::ABPU => {
                entry.tested
                    && (meet.federation == Federation::BPU
                        || (meet.federation == Federation::WPC
                            && entry.lifter_country.map_or(false, |c| c.is_in_uk())
                            && meet.date.year() >= 2013))
            }
            MetaFederation::ABSSeries => {
                meet.federation == Federation::IrelandUA && meet.name.starts_with("ABS")
            }
            MetaFederation::AEP => affiliation!(meet, entry, AEP, IPF, EPF),
            MetaFederation::AIWBPA => affiliation!(meet, entry, AIWBPA, IPF, AsianPF),
            MetaFederation::APP => affiliation!(meet, entry, APP, GPA),

            //APU only formed 2018 and became IPF/ORPF affiliate at this time, without checking
            //date we also get PA, AusPF, and AAPLF lifters in IPF/ORPF comps
            MetaFederation::APU => {
                affiliation!(meet, entry, APU, IPF, ORPF) && meet.date.year() >= 2018
            }

            MetaFederation::BelPF => affiliation!(meet, entry, BelPF, IPF, EPF),
            MetaFederation::BP => {
                meet.federation == Federation::BAWLA
                    || meet.federation == Federation::BP
                    || meet.federation == Federation::EPA
                    || meet.federation == Federation::NIPF
                    || meet.federation == Federation::ScottishPL
                    || meet.federation == Federation::WelshPA

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
            MetaFederation::BVDK => match meet.federation {
                // BVDG is the precursor to the BVDK.
                Federation::BVDG | Federation::BVDK => {
                    entry.lifter_country == None || entry.lifter_country == Some(Country::Germany)
                }
                Federation::IPF | Federation::EPF => entry.lifter_country == Some(Country::Germany),
                _ => false,
            },
            MetaFederation::CBLB => affiliation!(meet, entry, CBLB, IPF, FESUPO),
            MetaFederation::CPU => affiliation!(meet, entry, CPU, IPF, NAPF, CommonwealthPF),
            MetaFederation::CSST => affiliation!(meet, entry, CSST, IPF, EPF),
            MetaFederation::DSF => affiliation!(meet, entry, DSF, IPF, EPF, NordicPF),
            MetaFederation::EJTL => affiliation!(meet, entry, EJTL, IPF, EPF, NordicPF),
            MetaFederation::EPA => affiliation!(meet, entry, EPA, IPF, EPF, BP),
            MetaFederation::FALPO => affiliation!(meet, entry, FALPO, IPF, FESUPO),
            MetaFederation::FECAPOLIF => affiliation!(meet, entry, FECAPOLIF, IPF, AfricanPF),
            MetaFederation::FECHIPO => affiliation!(meet, entry, FECHIPO, IPF, FESUPO),
            MetaFederation::FEMEPO => affiliation!(meet, entry, FEMEPO, IPF, NAPF),
            MetaFederation::FFForce => {
                meet.federation == Federation::FFForce
                    // The FFHMFAC is the precursor to the FFForce.
                    || meet.federation == Federation::FFHMFAC
                    // French lifters expect their international results included.
                    || is_from(Country::France, entry, meet) &&
                        (meet.federation == Federation::IPF
                         || meet.federation == Federation::EPF)
            }
            MetaFederation::FIPL => affiliation!(meet, entry, FIPL, IPF, EPF),
            MetaFederation::FPPR => affiliation!(meet, entry, FPPR, IPF, NAPF),
            MetaFederation::FPR => affiliation!(meet, entry, FPR, IPF, EPF),
            MetaFederation::FRPL => affiliation!(meet, entry, FRPL, IPF, EPF),
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
            MetaFederation::GPCWUAPCRO => {
                (meet.federation == Federation::GPCWUAPCRO || meet.federation == Federation::HPO)
                    && (entry.lifter_country == None
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
            MetaFederation::IrishPF => affiliation!(meet, entry, IrishPF, IPF, EPF),
            MetaFederation::IrishPO => affiliation!(meet, entry, IrishPO, WPC),
            MetaFederation::JPA => affiliation!(meet, entry, JPA, IPF, AsianPF),
            MetaFederation::KNKFSP => affiliation!(meet, entry, KNKFSP, NPB, IPF, EPF),
            MetaFederation::LFPH => affiliation!(meet, entry, LFPH, IPF, EPF),
            MetaFederation::LJTF => affiliation!(meet, entry, LJTF, IPF, EPF),
            MetaFederation::LPF => affiliation!(meet, entry, LPF, IPF, EPF),
            MetaFederation::KPF => affiliation!(meet, entry, KPF, IPF, AsianPF),
            MetaFederation::KRAFT => affiliation!(meet, entry, KRAFT, IPF, EPF, NordicPF),
            MetaFederation::NauruPF => affiliation!(meet, entry, NauruPF, IPF, ORPF),
            MetaFederation::NIPF => affiliation!(meet, entry, NIPF, IPF, EPF, BP),
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
            MetaFederation::OEVK => affiliation!(meet, entry, OEVK, IPF, EPF),
            MetaFederation::PA => affiliation!(meet, entry, PA, WP),
            MetaFederation::PAP => affiliation!(meet, entry, PAP, IPF, AsianPF),
            MetaFederation::PI => affiliation!(meet, entry, PI, IPF, AsianPF),
            MetaFederation::PLSS => affiliation!(meet, entry, PLSS, IPF, EPF),
            MetaFederation::PLZS => affiliation!(meet, entry, PLZS, IPF, EPF),
            MetaFederation::PNGPF => affiliation!(meet, entry, PNGPF, IPF, ORPF),
            MetaFederation::PZKFiTS => affiliation!(meet, entry, PZKFiTS, IPF, EPF),
            MetaFederation::SAPF => affiliation!(meet, entry, SAPF, IPF, AfricanPF, CommonwealthPF),
            MetaFederation::ScottishPL => affiliation!(meet, entry, ScottishPL, IPF, EPF, BP),
            MetaFederation::SSF => affiliation!(meet, entry, SSF, IPF, EPF, NordicPF),
            MetaFederation::SVNL => affiliation!(meet, entry, SVNL, IPF, EPF, NordicPF),
            MetaFederation::SwissPL => affiliation!(meet, entry, SwissPL, IPF, EPF),
            MetaFederation::ThaiPF => affiliation!(meet, entry, ThaiPF, IPF, AsianPF),
            MetaFederation::TPSSF => affiliation!(meet, entry, TPSSF, IPF, EPF),
            MetaFederation::UgandaPF => affiliation!(meet, entry, UgandaPF, WP),
            MetaFederation::UkrainePF => affiliation!(meet, entry, UkrainePF, IPF, EPF),

            // Only include USA entries for meets directly affiliated with USAPL at any time,
            // or USA entries for NAPF/IPF meets after USAPL became IPF affiliate on 5 Dec 1997
            MetaFederation::USAPL => {
                is_from(Country::USA, entry, meet)
                    && (meet.federation == Federation::USAPL
                        || ((meet.federation == NAPF || meet.federation == IPF)
                            && meet.date >= Date::from_parts(1997, 12, 5))
                        || (meet.federation == ADFPA && meet.date < Date::from_parts(1997, 12, 5)))
            }

            MetaFederation::USPA => affiliation!(meet, entry, USPA, IPL),
            MetaFederation::USPATested => {
                entry.tested && MetaFederation::USPA.contains(entry, meets)
            }
            MetaFederation::VGPF => affiliation!(meet, entry, VGPF, IPF, EPF),
            MetaFederation::VPF => affiliation!(meet, entry, VPF, IPF, AsianPF),
            MetaFederation::WelshPA => affiliation!(meet, entry, WelshPA, IPF, EPF, BP),
            MetaFederation::WPNauru => affiliation!(meet, entry, WPNauru, WP),
            MetaFederation::WRPFUSA => match meet.federation {
                Federation::WRPF => match entry.lifter_country {
                    Some(Country::USA) => true,
                    None => meet.country == Country::USA,
                    _ => false,
                },
                _ => false,
            },
        }
    }
}

/// Pre-computed list of meets in a MetaFederation.
///
/// A meet is part of the MetaFederation if it contains
/// at least one entry such that `MetaFederation::contains(entry)`.
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
