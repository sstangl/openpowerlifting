//! MetaFederation definitions and calculations.

use itertools::Itertools;
use opltypes::*;
use strum::IntoEnumIterator;

use crate::opldb::{Entry, Meet};

/// Enum of MetaFederations. These are the entries in the federation selector
/// that don't correspond neatly to just a single federation value.
///
/// Definition of each MetaFederation is in the `contains` function.
///
/// A MetaFederation may override handling of a Federation by sharing its
/// to_string.
#[derive(
    Copy, Clone, Debug, Deserialize, Display, PartialEq, Serialize, EnumIter, EnumString,
)]
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
    #[strum(to_string = "all-belarus")]
    AllBelarus,
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
    #[strum(to_string = "all-indonesia")]
    AllIndonesia,
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
    #[strum(to_string = "all-norway")]
    AllNorway,
    #[strum(to_string = "all-papuanewguinea")]
    AllPapuaNewGuinea,
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

    #[strum(to_string = "aapf")]
    AAPF,

    /// BPU, but only tested entries.
    #[strum(to_string = "abpu")]
    ABPU,

    /// AEP, but with international results also.
    #[strum(to_string = "aep")]
    AEP,

    /// The BP federation is made up of smaller divisional federations,
    /// but people expect to see them all lumped together.
    #[strum(to_string = "all-bp")]
    AllBP,

    /// BDFPA, but with international results also.
    #[strum(to_string = "bdfpa")]
    BDFPA,

    /// BPU, but with international results also.
    #[strum(to_string = "bpu")]
    BPU,

    /// BVDK, but with international results also.
    #[strum(to_string = "bvdk")]
    BVDK,

    /// CPU, but with international results also.
    #[strum(to_string = "cpu")]
    CPU,

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

    /// HPLS, but excluding non-Croatian lifters.
    #[strum(to_string = "hpls")]
    HPLS,

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

    /// LJTF, but with international results also.
    #[strum(to_string = "ljtf")]
    LJTF,

    /// NPB, but with international results also.
    #[strum(to_string = "npb")]
    NPB,

    /// NZPF, but with international results also.
    #[strum(to_string = "nzpf")]
    NZPF,

    /// OEVK, but with international results also.
    #[strum(to_string = "oevk")]
    OEVK,

    /// PA, but excluding non-Australian lifters.
    #[strum(to_string = "pa")]
    PA,

    /// USAPL, but with international results also.
    USAPL,

    /// USPA, plus IPL results for American lifters.
    #[strum(to_string = "uspa")]
    USPA,

    /// USPA MetaFederation, but only for Tested entries.
    #[strum(to_string = "uspa-tested")]
    USPATested,

    /// ThaiPF, excluding non-Thai lifters and including IPF results.
    ThaiPF,

    /// WRPF-USA.
    #[strum(to_string = "wrpf-usa")]
    #[serde(rename = "WRPF-USA")]
    WRPFUSA,
}

/// Helper function for MetaFederation::contains() for AllCountry meta-feds.
#[inline]
fn is_from(country: Country, entry: &Entry, meet: &Meet) -> bool {
    entry.lifter_country == Some(country)
        || (entry.lifter_country == None
            && meet.federation.home_country() == Some(country))
}

impl MetaFederation {
    /// Defines whether a given `Entry` is part of the MetaFederation.
    ///
    /// Matching is done on Entries instead of on Meets since a MetaFederation
    /// can include Entry-specific information such as Tested status.
    pub fn contains(self, entry: &Entry, meets: &[Meet]) -> bool {
        let meet: &Meet = &meets[entry.meet_id as usize];

        match self {
            MetaFederation::FullyTested => {
                // Still check entry.tested: some fully-tested federations
                // existed before drug-testing was available.
                entry.tested && meet.federation.is_fully_tested()
            }
            MetaFederation::AllTested => entry.tested,
            MetaFederation::AllArgentina => is_from(Country::Argentina, entry, meet),
            MetaFederation::AllAustralia => is_from(Country::Australia, entry, meet),
            MetaFederation::AllAustria => is_from(Country::Austria, entry, meet),
            MetaFederation::AllBelarus => is_from(Country::Belarus, entry, meet),
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
            MetaFederation::AllFinland => is_from(Country::Finland, entry, meet),
            MetaFederation::AllFrance => is_from(Country::France, entry, meet),
            MetaFederation::AllGeorgia => is_from(Country::Georgia, entry, meet),
            MetaFederation::AllGermany => is_from(Country::Germany, entry, meet),
            MetaFederation::AllGreece => is_from(Country::Greece, entry, meet),
            MetaFederation::AllHongKong => is_from(Country::HongKong, entry, meet),
            MetaFederation::AllHungary => is_from(Country::Hungary, entry, meet),
            MetaFederation::AllIceland => is_from(Country::Iceland, entry, meet),
            MetaFederation::AllIndonesia => is_from(Country::Indonesia, entry, meet),
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
            MetaFederation::AllNorway => is_from(Country::Norway, entry, meet),
            MetaFederation::AllPapuaNewGuinea => {
                is_from(Country::PapuaNewGuinea, entry, meet)
            }
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
            MetaFederation::AllUKTested => {
                entry.tested && MetaFederation::AllUK.contains(entry, meets)
            }
            MetaFederation::AllUkraine => is_from(Country::Ukraine, entry, meet),
            MetaFederation::AllUSA => is_from(Country::USA, entry, meet),
            MetaFederation::AllVietnam => is_from(Country::Vietnam, entry, meet),
            MetaFederation::AAPF => meet.federation == Federation::APF && entry.tested,
            MetaFederation::ABPU => {
                entry.tested && MetaFederation::BPU.contains(entry, meets)
            }
            MetaFederation::AEP => match meet.federation {
                Federation::AEP => true,
                Federation::IPF | Federation::EPF => {
                    entry.lifter_country == Some(Country::Spain)
                }
                _ => false,
            },
            MetaFederation::AllBP => {
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
            MetaFederation::BPU => {
                meet.federation == Federation::BPU
                    || (meet.federation == Federation::WPC
                        && entry.lifter_country.map_or(false, |c| c.is_in_uk()))
            }
            MetaFederation::BVDK => match meet.federation {
                // BVDG is the precursor to the BVDK.
                Federation::BVDG | Federation::BVDK => {
                    entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Germany)
                }
                Federation::IPF | Federation::EPF => {
                    entry.lifter_country == Some(Country::Germany)
                }
                _ => false,
            },
            MetaFederation::CPU => match meet.federation {
                Federation::CPU => {
                    entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Canada)
                }
                Federation::IPF | Federation::NAPF | Federation::CommonwealthPF => {
                    entry.lifter_country == Some(Country::Canada)
                }
                _ => false,
            },
            MetaFederation::FRPL => match meet.federation {
                Federation::FRPL => {
                    entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Romania)
                }
                Federation::IPF | Federation::EPF => {
                    entry.lifter_country == Some(Country::Romania)
                }
                _ => false,
            },
            MetaFederation::GPCAUS => {
                meet.federation == Federation::GPCAUS
                    && (entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Australia))
            }
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
                (meet.federation == Federation::GPCWUAPCRO
                    || meet.federation == Federation::HPO)
                    && (entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Croatia))
            }
            MetaFederation::HPLS => {
                meet.federation == Federation::HPLS
                    && (entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Croatia))
            }
            MetaFederation::IPACAN => {
                meet.federation == Federation::IPA && meet.country == Country::Canada
            }
            MetaFederation::IPFAndAffiliates => {
                meet.federation.sanctioning_body(meet.date) == Some(Federation::IPF)
            }
            MetaFederation::IPFInternationals => match meet.federation {
                Federation::IPF
                | Federation::AfricanPF
                | Federation::AsianPF
                | Federation::EPF
                | Federation::FESUPO
                | Federation::NAPF
                | Federation::ORPF
                | Federation::CommonwealthPF => true,
                _ => false,
            },
            MetaFederation::IrishPF => match meet.federation {
                Federation::IrishPF => {
                    entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Ireland)
                }
                Federation::IPF | Federation::EPF => {
                    entry.lifter_country == Some(Country::Ireland)
                }
                _ => false,
            },
            MetaFederation::IrishPO => {
                (meet.federation == Federation::IrishPO
                    && (entry.lifter_country.is_none()
                        || entry.lifter_country == Some(Country::Ireland)))
                    || (meet.federation == Federation::WPC
                        && entry.lifter_country == Some(Country::Ireland))
            }
            MetaFederation::LJTF => match meet.federation {
                Federation::LJTF => {
                    entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Lithuania)
                }
                Federation::IPF | Federation::EPF => {
                    entry.lifter_country == Some(Country::Lithuania)
                }
                _ => false,
            },
            MetaFederation::NPB => match meet.federation {
                Federation::NPB => true,
                Federation::IPF | Federation::EPF => {
                    entry.lifter_country == Some(Country::Netherlands)
                }
                _ => false,
            },
            MetaFederation::NZPF => match meet.federation {
                Federation::NZPF => {
                    entry.lifter_country == None
                        || entry.lifter_country == Some(Country::NewZealand)
                }
                Federation::IPF | Federation::OceaniaPF | Federation::CommonwealthPF => {
                    entry.lifter_country == Some(Country::NewZealand)
                }
                _ => false,
            },
            MetaFederation::OEVK => match meet.federation {
                Federation::OEVK => {
                    entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Austria)
                }
                Federation::IPF | Federation::EPF => {
                    entry.lifter_country == Some(Country::Austria)
                }
                _ => false,
            },
            MetaFederation::PA => match meet.federation {
                Federation::PA => {
                    entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Australia)
                }
                Federation::WP => entry.lifter_country == Some(Country::Australia),
                _ => false,
            },
            MetaFederation::USAPL => match meet.federation {
                Federation::USAPL => {
                    entry.lifter_country == None
                        || entry.lifter_country == Some(Country::USA)
                }
                Federation::IPF | Federation::NAPF => {
                    entry.lifter_country == Some(Country::USA)
                }
                _ => false,
            },
            MetaFederation::USPA => {
                meet.federation == Federation::USPA
                    || (meet.federation == Federation::IPL
                        && entry.lifter_country == Some(Country::USA))
            }
            MetaFederation::USPATested => {
                entry.tested
                    && (meet.federation == Federation::USPA
                        || (meet.federation == Federation::IPL
                            && entry.lifter_country == Some(Country::USA)))
            }
            MetaFederation::ThaiPF => match meet.federation {
                Federation::ThaiPF => {
                    entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Thailand)
                }
                Federation::IPF | Federation::ORPF | Federation::AsianPF => {
                    entry.lifter_country == Some(Country::Thailand)
                }
                _ => false,
            },
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
    pub fn get_meet_ids_for<'a>(&'a self, meta: MetaFederation) -> &'a Vec<u32> {
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
        for _ in 0..num_metafeds {
            ret.push(vec![]);
        }

        // Vector of whether each meet has a match for the
        // given MetaFederation (accessed via index).
        let mut contains: Vec<bool> = Vec::with_capacity(num_metafeds);
        for _ in 0..num_metafeds {
            contains.push(false);
        }

        let mut last_meet_id = 0;

        // Iterate by grouping entries from the same Meet.
        for (meet_id, meet_entries) in entries.iter().group_by(|e| e.meet_id).into_iter()
        {
            // Sanity checking that the entries argument is sorted by meet_id.
            assert!(last_meet_id <= meet_id);
            last_meet_id = meet_id;

            // Check whether any entries are part of each MetaFederation.
            for entry in meet_entries {
                for meta in MetaFederation::iter() {
                    if meta.contains(&entry, &meets) {
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
