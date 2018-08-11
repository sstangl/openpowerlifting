//! MetaFederation definitions and calculations.

use itertools::Itertools;
use opltypes::*;
use strum::IntoEnumIterator;

use opldb::{Entry, Meet};

/// Enum of MetaFederations. These are the entries in the federation selector
/// that don't correspond neatly to just a single federation value.
///
/// Definition of each MetaFederation is in the `contains` function.
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
    pub fn contains(self, entry: &Entry, meets: &[Meet]) -> bool {
        let meet: &Meet = &meets[entry.meet_id as usize];

        match self {
            MetaFederation::AllTested => {
                meet.federation == Federation::AAPLF
                    || meet.federation == Federation::AAU
                    || meet.federation == Federation::ADAU
                    || meet.federation == Federation::ADFPA
                    || meet.federation == Federation::ADFPF
                    || meet.federation == Federation::AEP
                    || meet.federation == Federation::AfricanPF
                    || meet.federation == Federation::APU
                    || meet.federation == Federation::AsianPF
                    || meet.federation == Federation::AusDFPF
                    || meet.federation == Federation::BAWLA
                    || meet.federation == Federation::BDFPA
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
                    || meet.federation == Federation::IBSA
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
                meet.federation == Federation::AAPLF
                    || meet.federation == Federation::APU
                    || meet.federation == Federation::AusDFPF
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
                    || meet.federation == Federation::GoldenDouble
                    || meet.federation == Federation::GPCRUS
                    || meet.federation == Federation::NAP
                    || meet.federation == Federation::RPU
                    || meet.federation == Federation::SCT
                    || meet.federation == Federation::SPSS
                    || meet.federation == Federation::WPARUS
                    || meet.federation == Federation::WPCRUS
                    || meet.federation == Federation::WRPF
            }
            MetaFederation::AllUK => {
                meet.federation == Federation::BAWLA
                    || meet.federation == Federation::BDFPA
                    || meet.federation == Federation::BP
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
                    || meet.federation == Federation::PRPA
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
                meet.federation == Federation::BAWLA
                    || meet.federation == Federation::BP
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

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

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

