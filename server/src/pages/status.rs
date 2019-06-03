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
    statuses[APU as usize].status = complete;
    statuses[BB as usize].status = complete;
    statuses[BPU as usize].status = complete;
    statuses[CroatiaUA as usize].status = complete;
    statuses[DSF as usize].status = complete;
    statuses[GPACRO as usize].status = complete;
    statuses[GPCWUAPCRO as usize].status = complete;
    statuses[HERC as usize].status = complete;
    statuses[HPLS as usize].status = complete;
    statuses[HPLSUA as usize].status = complete;
    statuses[HPO as usize].status = complete;
    statuses[IPF as usize].status = complete;
    statuses[IrishPF as usize].status = complete;
    statuses[PA as usize].status = complete;
    statuses[ProRaw as usize].status = complete;
    statuses[RPS as usize].status = complete;
    statuses[SCT as usize].status = complete;
    statuses[SPF as usize].status = complete;
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
    statuses[UPA as usize].has_probe = yes;
    statuses[UPCGermany as usize].has_probe = yes;
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
    let jpg = "JPG Images";
    let magazines = "Magazines";
    let pdf_structured = "PDF (Structured)";
    let pdf_unstructured = "PDF (Unstructured)";
    let xls_unstructured = "XLS (Unstructured)";
    let xls_structured = "XLS (Structured)";
    statuses[_365Strong as usize].format = xls_structured;
    statuses[AEP as usize].format = html;
    statuses[APA as usize].format = html;
    statuses[APF as usize].format = xls_unstructured;
    statuses[APU as usize].format = pdf_structured;
    statuses[AusPL as usize].format = pdf_structured;
    statuses[BB as usize].format = xls_unstructured;
    statuses[BDFPA as usize].format = jpg;
    statuses[BP as usize].format = pdf_unstructured;
    statuses[BPF as usize].format = xls_unstructured;
    statuses[BPU as usize].format = xls_unstructured;
    statuses[BVDG as usize].format = magazines;
    statuses[BVDK as usize].format = pdf_unstructured;
    statuses[CAPO as usize].format = pdf_unstructured;
    statuses[CPF as usize].format = pdf_unstructured;
    statuses[CPU as usize].format = database;
    statuses[CroatiaUA as usize].format = jpg;
    statuses[DBKV as usize].format = magazines;
    statuses[DSF as usize].format = database;
    statuses[EPA as usize].format = pdf_unstructured;
    statuses[EPF as usize].format = html;
    statuses[GPCAUS as usize].format = html;
    statuses[GPCWUAPCRO as usize].format = html;
    statuses[HPLS as usize].format = xls_unstructured;
    statuses[HPLSUA as usize].format = jpg;
    statuses[HPO as usize].format = html;
    statuses[IPA as usize].format = xls_unstructured;
    statuses[IPF as usize].format = html;
    statuses[IPLNZ as usize].format = pdf_structured;
    statuses[IrishPF as usize].format = html;
    statuses[IrishPO as usize].format = xls_unstructured;
    statuses[NASA as usize].format = pdf_structured;
    statuses[NZPF as usize].format = pdf_unstructured;
    statuses[OceaniaPF as usize].format = pdf_structured;
    statuses[OEVK as usize].format = pdf_unstructured;
    statuses[ORPF as usize].format = pdf_structured;
    statuses[PA as usize].format = html;
    statuses[ProRaw as usize].format = xls_unstructured;
    statuses[RAW as usize].format = pdf_unstructured;
    statuses[RPS as usize].format = html;
    statuses[SPF as usize].format = html;
    statuses[SwissPL as usize].format = pdf_structured;
    statuses[THSPA as usize].format = database;
    statuses[THSWPA as usize].format = database;
    statuses[UPA as usize].format = pdf_unstructured;
    statuses[UPCGermany as usize].format = pdf_unstructured;
    statuses[USAPL as usize].format = database;
    statuses[USPA as usize].format = pdf_structured;
    statuses[WelshPA as usize].format = pdf_unstructured;
    statuses[WPC as usize].format = xls_unstructured;
    statuses[WPNZ as usize].format = jpg;
    statuses[WRPFAUS as usize].format = jpg;
    statuses[WUAP as usize].format = pdf_structured;

    // Subjective ease of importation.
    let easy = "Easy";
    let medium = "Medium";
    let difficult = "Difficult";
    let impossible = "Impossible";
    statuses[_365Strong as usize].ease = easy;
    statuses[AEP as usize].ease = medium;
    statuses[APA as usize].ease = medium;
    statuses[APF as usize].ease = difficult;
    statuses[APU as usize].ease = easy;
    statuses[AusPL as usize].ease = medium;
    statuses[BB as usize].ease = difficult;
    statuses[BDFPA as usize].ease = impossible;
    statuses[BP as usize].ease = medium;
    statuses[BPF as usize].ease = easy;
    statuses[BPU as usize].ease = easy;
    statuses[BVDG as usize].ease = difficult;
    statuses[BVDK as usize].ease = medium;
    statuses[CAPO as usize].ease = medium;
    statuses[CPF as usize].ease = medium;
    statuses[CPU as usize].ease = easy;
    statuses[CroatiaUA as usize].ease = difficult;
    statuses[DBKV as usize].ease = medium;
    statuses[DSF as usize].ease = easy;
    statuses[EPA as usize].ease = medium;
    statuses[EPF as usize].ease = easy;
    statuses[GPCAUS as usize].ease = easy;
    statuses[GPCWUAPCRO as usize].ease = medium;
    statuses[HPLS as usize].ease = medium;
    statuses[HPLSUA as usize].ease = difficult;
    statuses[HPO as usize].ease = medium;
    statuses[IPA as usize].ease = difficult;
    statuses[IPF as usize].ease = easy;
    statuses[IPLNZ as usize].ease = medium;
    statuses[IrishPF as usize].ease = easy;
    statuses[IrishPO as usize].ease = easy;
    statuses[NASA as usize].ease = easy;
    statuses[NZPF as usize].ease = difficult;
    statuses[OceaniaPF as usize].ease = medium;
    statuses[OEVK as usize].ease = difficult;
    statuses[ORPF as usize].ease = difficult;
    statuses[PA as usize].ease = easy;
    statuses[ProRaw as usize].ease = medium;
    statuses[RAW as usize].ease = difficult;
    statuses[RPS as usize].ease = easy;
    statuses[SPF as usize].ease = easy;
    statuses[SwissPL as usize].ease = difficult;
    statuses[THSPA as usize].ease = medium;
    statuses[THSWPA as usize].ease = medium;
    statuses[UPA as usize].ease = difficult;
    statuses[UPCGermany as usize].ease = difficult;
    statuses[USAPL as usize].ease = easy;
    statuses[USPA as usize].ease = easy;
    statuses[WPC as usize].ease = medium;
    statuses[WPNZ as usize].ease = difficult;
    statuses[WRPFAUS as usize].ease = impossible;
    statuses[WUAP as usize].ease = difficult;

    // Maintainership variables.
    let email_boris = "<a href=\"mailto:boris@openpowerlifting.org\">boris@</a>";
    let email_enno = "<a href=\"mailto:enno@openpowerlifting.org\">enno@</a>";
    let email_gem = "<a href=\"mailto:gem@openpowerlifting.org\">gem@</a>";
    let email_jo = "<a href=\"mailto:jo@openpowerlifting.org\">jo@</a>";
    let email_matt = "<a href=\"mailto:matt@openpowerlifting.org\">matt@</a>";
    let email_milena = "<a href=\"mailto:milena@openpowerlifting.org\">milena@</a>";
    let email_robby = "<a href=\"mailto:ramasson@hotmail.co.uk\">Robby Masson</a>";
    let email_romi = "<a href=\"mailto:romi@openpowerlifting.org\">Romi@</a>";
    let email_sean = "<a href=\"mailto:sean@openpowerlifting.org\">sean@</a>";
    let email_alan = "<a href=\"mailto:alan.zgb@gmail.com\">alan@</a>";

    // Maintainership information.
    statuses[_365Strong as usize].maintainers = email_sean;
    statuses[AEP as usize].maintainers = email_enno;
    statuses[APF as usize].maintainers = email_gem;
    statuses[APU as usize].maintainers = email_sean;
    statuses[AusPL as usize].maintainers = email_matt;
    statuses[BAWLA as usize].maintainers = email_jo;
    statuses[BP as usize].maintainers = email_jo;
    statuses[BPF as usize].maintainers = email_gem;
    statuses[BPU as usize].maintainers = email_gem;
    statuses[BVDG as usize].maintainers = email_romi;
    statuses[BVDK as usize].maintainers = email_romi;
    statuses[CAPO as usize].maintainers = email_matt;
    statuses[CommonwealthPF as usize].maintainers = email_jo;
    statuses[CPF as usize].maintainers = email_sean;
    statuses[CPU as usize].maintainers = email_sean;
    statuses[CroatiaUA as usize].maintainers = email_alan;
    statuses[DBKV as usize].maintainers = email_romi;
    statuses[EPA as usize].maintainers = email_jo;
    statuses[EPF as usize].maintainers = email_jo;
    statuses[GPACRO as usize].maintainers = email_alan;
    statuses[GPCAUS as usize].maintainers = email_matt;
    statuses[GPCGB as usize].maintainers = email_robby;
    statuses[GPCNZ as usize].maintainers = email_matt;
    statuses[GPCWUAPCRO as usize].maintainers = email_alan;
    statuses[HERC as usize].maintainers = email_sean;
    statuses[HPLS as usize].maintainers = email_alan;
    statuses[HPLSUA as usize].maintainers = email_alan;
    statuses[HPO as usize].maintainers = email_alan;
    statuses[IPF as usize].maintainers = email_jo;
    statuses[IPLNZ as usize].maintainers = email_matt;
    statuses[IrelandUA as usize].maintainers = email_jo;
    statuses[IrishPO as usize].maintainers = email_gem;
    statuses[NASA as usize].maintainers = email_boris;
    statuses[NIPF as usize].maintainers = email_jo;
    statuses[NZPF as usize].maintainers = email_matt;
    statuses[OceaniaPF as usize].maintainers = email_matt;
    statuses[OEVK as usize].maintainers = email_milena;
    statuses[ORPF as usize].maintainers = email_matt;
    statuses[PA as usize].maintainers = email_sean;
    statuses[ProRaw as usize].maintainers = email_sean;
    statuses[RPS as usize].maintainers = email_sean;
    statuses[ScottishPL as usize].maintainers = email_robby;
    statuses[SPF as usize].maintainers = email_sean;
    statuses[USAPL as usize].maintainers = email_sean;
    statuses[USPA as usize].maintainers = email_sean;
    statuses[THSPA as usize].maintainers = email_sean;
    statuses[THSWPA as usize].maintainers = email_sean;
    statuses[UPA as usize].maintainers = email_gem;
    statuses[WelshPA as usize].maintainers = email_jo;
    statuses[WPC as usize].maintainers = email_gem;
    statuses[WPCFinland as usize].maintainers = email_gem;
    statuses[WPCFrance as usize].maintainers = email_gem;
    statuses[WPNZ as usize].maintainers = email_matt;
    statuses[WRPFAUS as usize].maintainers = email_matt;

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
