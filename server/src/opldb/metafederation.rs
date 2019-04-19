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
    #[strum(to_string = "all-canada")]
    AllCanada,
    #[strum(to_string = "all-china")]
    AllChina,
    #[strum(to_string = "all-croatia")]
    AllCroatia,
    #[strum(to_string = "all-czechia")]
    AllCzechia,
    #[strum(to_string = "all-finland")]
    AllFinland,
    #[strum(to_string = "all-france")]
    AllFrance,
    #[strum(to_string = "all-germany")]
    AllGermany,
    #[strum(to_string = "all-greece")]
    AllGreece,
    #[strum(to_string = "all-iceland")]
    AllIceland,
    #[strum(to_string = "all-ireland")]
    AllIreland,
    #[strum(to_string = "all-israel")]
    AllIsrael,
    #[strum(to_string = "all-kazakhstan")]
    AllKazakhstan,
    #[strum(to_string = "all-latvia")]
    AllLatvia,
    #[strum(to_string = "all-newzealand")]
    AllNewZealand,
    #[strum(to_string = "all-poland")]
    AllPoland,
    #[strum(to_string = "all-russia")]
    AllRussia,
    #[strum(to_string = "all-serbia")]
    AllSerbia,
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
    #[strum(to_string = "all-uk")]
    AllUK,
    #[strum(to_string = "all-uk-tested")]
    AllUKTested,
    #[strum(to_string = "all-ukraine")]
    AllUkraine,
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

    /// BPU, but with international results also.
    #[strum(to_string = "bpu")]
    BPU,

    /// BVDK, but with international results also.
    #[strum(to_string = "bvdk")]
    BVDK,

    /// GPC-AUS, but excluding non-Australian lifters.
    #[strum(to_string = "gpc-aus")]
    GPCAUS,

    /// HPLS, but excluding non-Croatian lifters.
    #[strum(to_string = "hpls")]
    HPLS,

    /// IPA, but only counting meets held in Canada.
    #[strum(to_string = "ipa-can")]
    IPACAN,

    /// IrishPO, excluding non-Irish lifters and including WPC results.
    #[strum(to_string = "irishpo")]
    IrishPO,

    /// NPB, but with international results also.
    #[strum(to_string = "npb")]
    NPB,

    /// USAPL, but with international results also.
    USAPL,

    /// USPA, plus IPL results for American lifters.
    #[strum(to_string = "uspa")]
    USPA,

    /// USPA MetaFederation, but only for Tested entries.
    #[strum(to_string = "uspa-tested")]
    USPATested,
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
            MetaFederation::AllArgentina => {
                entry.lifter_country == Some(Country::Argentina)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Argentina))
            }
            MetaFederation::AllAustralia => {
                entry.lifter_country == Some(Country::Australia)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Australia))
            }
            MetaFederation::AllCanada => {
                entry.lifter_country == Some(Country::Canada)
                    || (entry.lifter_country == None
                        && (meet.federation.home_country() == Some(Country::Canada)
                            || MetaFederation::IPACAN.contains(entry, meets)))
            }
            MetaFederation::AllChina => {
                entry.lifter_country == Some(Country::China)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::China))
            }
            MetaFederation::AllCroatia => {
                entry.lifter_country == Some(Country::Croatia)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Croatia))
            }
            MetaFederation::AllCzechia => {
                entry.lifter_country == Some(Country::Czechia)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Czechia))
            }
            MetaFederation::AllFinland => {
                entry.lifter_country == Some(Country::Finland)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Finland))
            }
            MetaFederation::AllFrance => {
                entry.lifter_country == Some(Country::France)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::France))
            }
            MetaFederation::AllGermany => {
                entry.lifter_country == Some(Country::Germany)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Germany))
            }
            MetaFederation::AllGreece => {
                entry.lifter_country == Some(Country::Greece)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Greece))
            }
            MetaFederation::AllIceland => {
                entry.lifter_country == Some(Country::Iceland)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Iceland))
            }
            MetaFederation::AllIreland => {
                entry.lifter_country == Some(Country::Ireland)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Ireland))
            }
            MetaFederation::AllIsrael => {
                entry.lifter_country == Some(Country::Israel)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Israel))
            }
            MetaFederation::AllKazakhstan => {
                entry.lifter_country == Some(Country::Kazakhstan)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Kazakhstan))
            }
            MetaFederation::AllLatvia => {
                entry.lifter_country == Some(Country::Latvia)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Latvia))
            }
            MetaFederation::AllNewZealand => {
                entry.lifter_country == Some(Country::NewZealand)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::NewZealand))
            }
            MetaFederation::AllPoland => {
                entry.lifter_country == Some(Country::Poland)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Poland))
            }
            MetaFederation::AllRussia => {
                entry.lifter_country == Some(Country::Russia)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Russia))
            }
            MetaFederation::AllSerbia => {
                entry.lifter_country == Some(Country::Serbia)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Serbia))
            }
            MetaFederation::AllSlovakia => {
                entry.lifter_country == Some(Country::Slovakia)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Slovakia))
            }
            MetaFederation::AllSlovenia => {
                entry.lifter_country == Some(Country::Slovenia)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Slovenia))
            }
            MetaFederation::AllSpain => {
                entry.lifter_country == Some(Country::Spain)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Spain))
            }
            MetaFederation::AllSouthAfrica => {
                entry.lifter_country == Some(Country::SouthAfrica)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::SouthAfrica))
            }
            MetaFederation::AllSweden => {
                entry.lifter_country == Some(Country::Sweden)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Sweden))
            }
            MetaFederation::AllSwitzerland => {
                entry.lifter_country == Some(Country::Switzerland)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Switzerland))
            }
            MetaFederation::AllUK => {
                // UK lifters can set UK records abroad, except in Ireland.
                if meet.country == Country::Ireland {
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
            MetaFederation::AllUkraine => {
                entry.lifter_country == Some(Country::Ukraine)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::Ukraine))
            }
            MetaFederation::AllUSA => {
                entry.lifter_country == Some(Country::USA)
                    || (entry.lifter_country == None
                        && meet.federation.home_country() == Some(Country::USA))
            }
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
            MetaFederation::BPU => {
                meet.federation == Federation::BPU
                    || (meet.federation == Federation::WPC
                        && entry.lifter_country.map_or(false, |c| c.is_in_uk()))
            }
            MetaFederation::BVDK => match meet.federation {
                Federation::BVDG => true, // Precursor to the BVDK.
                Federation::BVDK => true,
                Federation::IPF | Federation::EPF => {
                    entry.lifter_country == Some(Country::Germany)
                }
                _ => false,
            },
            MetaFederation::GPCAUS => {
                meet.federation == Federation::GPCAUS
                    && (entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Australia))
            }
            MetaFederation::HPLS => {
                meet.federation == Federation::HPLS
                    && (entry.lifter_country == None
                        || entry.lifter_country == Some(Country::Croatia))
            }
            MetaFederation::IPACAN => {
                meet.federation == Federation::IPA && meet.country == Country::Canada
            }
            MetaFederation::IrishPO => {
                (meet.federation == Federation::IrishPO
                    && (entry.lifter_country.is_none()
                        || entry.lifter_country == Some(Country::Ireland)))
                    || (meet.federation == Federation::WPC
                        && entry.lifter_country == Some(Country::Ireland))
            }
            MetaFederation::NPB => match meet.federation {
                Federation::NPB => true,
                Federation::IPF | Federation::EPF => {
                    entry.lifter_country == Some(Country::Netherlands)
                }
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
