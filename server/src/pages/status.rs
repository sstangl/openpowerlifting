//! Logic for the project status page.

use opltypes::*;
use strum::IntoEnumIterator;

use crate::langpack::{self, Language, Locale};
use crate::opldb;

/// The context object passed to `templates/status.html.tera`
#[derive(Serialize)]
pub struct Context<'a> {
    pub page_title: &'a str,
    pub language: Language,
    pub strings: &'a langpack::Translations,
    pub units: WeightUnits,
    pub fed_statuses: Vec<FederationStatus>,
    pub num_entries: u32,
    pub num_meets: u32,
    pub num_lifters: u32,
}

#[derive(Serialize)]
pub struct FederationStatus {
    pub fed: Federation,
    pub meet_count: usize,

    /// Status of completion, where "Complete" means up-to-date.
    pub status: &'static str,

    /// Probes are used to detect when new meets occur.
    pub has_probe: &'static str,

    /// Description of the format(s) used for new results.
    pub format: &'static str,

    /// A short, subjective description of the difficulty in entering these
    /// results.
    pub ease: &'static str,

    /// People who have committed to keeping this federation updated.
    pub maintainers: &'static str,
}

impl FederationStatus {
    fn new(fed: Federation) -> FederationStatus {
        FederationStatus {
            fed,
            meet_count: 0,
            status: "Incomplete",
            has_probe: "No",
            format: "",
            ease: "",
            maintainers:
                "None (<a href=\"mailto:updates@openpowerlifting.org\">Apply</a>)",
        }
    }
}

fn set_hardcoded_strings(statuses: &mut Vec<FederationStatus>) {
    use Federation::*;

    // Completeness.
    let complete = "Complete";
    statuses[_365Strong as usize].status = complete;
    statuses[BB as usize].status = complete;
    statuses[HERC as usize].status = complete;
    statuses[PA as usize].status = complete;
    statuses[RPS as usize].status = complete;
    statuses[SCT as usize].status = complete;
    statuses[THSPA as usize].status = complete;
    statuses[THSWPA as usize].status = complete;
    statuses[USAPL as usize].status = "Since 2014";
    statuses[USPA as usize].status = complete;

    // Probes.
    let yes = "Yes";
    statuses[AAP as usize].has_probe = yes;
    statuses[AAU as usize].has_probe = yes;
    statuses[AEP as usize].has_probe = yes;
    statuses[APA as usize].has_probe = yes;
    statuses[APC as usize].has_probe = yes;
    statuses[APF as usize].has_probe = yes;
    statuses[APU as usize].has_probe = yes;
    statuses[AusPL as usize].has_probe = yes;
    statuses[BP as usize].has_probe = yes;
    statuses[BPU as usize].has_probe = yes;
    statuses[CAPO as usize].has_probe = yes;
    statuses[CommonwealthPF as usize].has_probe = yes;
    statuses[CPA as usize].has_probe = yes;
    statuses[CPF as usize].has_probe = yes;
    statuses[CPL as usize].has_probe = yes;
    statuses[CSST as usize].has_probe = yes;
    statuses[DSF as usize].has_probe = yes;
    statuses[EPF as usize].has_probe = yes;
    statuses[FEMEPO as usize].has_probe = yes;
    statuses[FEPOA as usize].has_probe = yes;
    statuses[FESUPO as usize].has_probe = yes;
    statuses[FFForce as usize].has_probe = yes;
    statuses[FPO as usize].has_probe = yes;
    statuses[FPR as usize].has_probe = yes;
    statuses[GPA as usize].has_probe = yes;
    statuses[GPCAUS as usize].has_probe = yes;
    statuses[HERC as usize].has_probe = yes;
    statuses[IPA as usize].has_probe = yes;
    statuses[IPF as usize].has_probe = yes;
    statuses[IrishPF as usize].has_probe = yes;
    statuses[IrishPO as usize].has_probe = yes;
    statuses[KRAFT as usize].has_probe = yes;
    statuses[LPF as usize].has_probe = yes;
    statuses[NAPF as usize].has_probe = yes;
    statuses[NASA as usize].has_probe = yes;
    statuses[NIPF as usize].has_probe = yes;
    statuses[NordicPF as usize].has_probe = yes;
    statuses[NPB as usize].has_probe = yes;
    statuses[NSF as usize].has_probe = yes;
    statuses[NZPF as usize].has_probe = yes;
    statuses[OceaniaPF as usize].has_probe = yes;
    statuses[PA as usize].has_probe = yes;
    statuses[PLZS as usize].has_probe = yes;
    statuses[RAW as usize].has_probe = yes;
    statuses[RPS as usize].has_probe = yes;
    statuses[RPU as usize].has_probe = yes;
    statuses[ScottishPL as usize].has_probe = yes;
    statuses[SPF as usize].has_probe = yes;
    statuses[SSF as usize].has_probe = yes;
    statuses[SVNL as usize].has_probe = yes;
    statuses[THSPA as usize].has_probe = yes;
    statuses[THSWPA as usize].has_probe = yes;
    statuses[UkrainePF as usize].has_probe = yes;
    statuses[UnitedPC as usize].has_probe = yes;
    statuses[UPA as usize].has_probe = yes;
    statuses[USAPL as usize].has_probe = yes;
    statuses[USPA as usize].has_probe = yes;
    statuses[USPF as usize].has_probe = yes;
    statuses[WABDL as usize].has_probe = yes;
    statuses[WPAU as usize].has_probe = yes;
    statuses[WPCRUS as usize].has_probe = yes;
    statuses[WRPFAUS as usize].has_probe = yes;
    statuses[WRPFCAN as usize].has_probe = yes;
    statuses[WRPFSpain as usize].has_probe = yes;
    statuses[WRPF as usize].has_probe = yes;
    statuses[XPC as usize].has_probe = yes;

    // Results format.
    let database = "Database";
    let html = "HTML";
    let pdf_structured = "PDF (Structured)";
    let pdf_unstructured = "PDF (Unstructured)";
    let xls_unstructured = "XLS (Unstructured)";
    statuses[_365Strong as usize].format = xls_unstructured;
    statuses[BB as usize].format = xls_unstructured;
    statuses[CPU as usize].format = database;
    statuses[EPF as usize].format = html;
    statuses[IPA as usize].format = xls_unstructured;
    statuses[IPF as usize].format = html;
    statuses[IrishPF as usize].format = html;
    statuses[PA as usize].format = html;
    statuses[RAW as usize].format = pdf_unstructured;
    statuses[RPS as usize].format = html;
    statuses[SPF as usize].format = html;
    statuses[THSPA as usize].format = database;
    statuses[THSWPA as usize].format = database;
    statuses[UPA as usize].format = pdf_unstructured;
    statuses[USAPL as usize].format = database;
    statuses[USPA as usize].format = pdf_structured;

    // Subjective ease of importation.
    let easy = "Easy";
    let medium = "Medium";
    let difficult = "Difficult";
    statuses[_365Strong as usize].ease = difficult;
    statuses[BB as usize].ease = difficult;
    statuses[CPU as usize].ease = easy;
    statuses[EPF as usize].ease = easy;
    statuses[IPA as usize].ease = difficult;
    statuses[IPF as usize].ease = easy;
    statuses[IrishPF as usize].ease = easy;
    statuses[PA as usize].ease = easy;
    statuses[RAW as usize].ease = difficult;
    statuses[RPS as usize].ease = easy;
    statuses[SPF as usize].ease = easy;
    statuses[THSPA as usize].ease = medium;
    statuses[THSWPA as usize].ease = medium;
    statuses[UPA as usize].ease = difficult;
    statuses[USAPL as usize].ease = easy;
    statuses[USPA as usize].ease = easy;

    // Maintainership variables.
    let email_sean = "<a href=\"mailto:sean@openpowerlifting.org\">sean@</a>";

    // Maintainership information.
    statuses[_365Strong as usize].maintainers = email_sean;
    statuses[CPU as usize].maintainers = email_sean;
    statuses[HERC as usize].maintainers = email_sean;
    statuses[PA as usize].maintainers = email_sean;
    statuses[RPS as usize].maintainers = email_sean;
    statuses[SPF as usize].maintainers = email_sean;
    statuses[USAPL as usize].maintainers = email_sean;
    statuses[USPA as usize].maintainers = email_sean;
    statuses[THSPA as usize].maintainers = email_sean;
    statuses[THSWPA as usize].maintainers = email_sean;

    // Don't ask for maintainership applications for defunct, completed federations.
    statuses[BB as usize].maintainers = "";
    statuses[SCT as usize].maintainers = "";
}

impl<'a> Context<'a> {
    pub fn new(opldb: &'a opldb::OplDb, locale: &'a Locale) -> Context<'a> {
        let mut statuses: Vec<FederationStatus> =
            Federation::iter().map(FederationStatus::new).collect();

        for meet in opldb.get_meets() {
            let idx = meet.federation as usize;
            statuses[idx].meet_count += 1;
        }

        set_hardcoded_strings(&mut statuses);

        Context {
            page_title: &locale.strings.header.status,
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            fed_statuses: statuses,
            num_entries: opldb.get_entries().len() as u32,
            num_meets: opldb.get_meets().len() as u32,
            num_lifters: opldb.get_lifters().len() as u32,
        }
    }
}
