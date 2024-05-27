//! Logic for the project status page.

use langpack::{Language, Locale};
use opltypes::*;

use strum::IntoEnumIterator;

/// The context object passed to `templates/status.html.tera`
#[derive(Serialize)]
pub struct Context {
    pub urlprefix: &'static str,
    pub page_title: &'static str,
    pub page_description: &'static str,
    pub language: Language,
    pub strings: &'static langpack::Translations,
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

    /// The federation's Instagram account.
    pub instagram: &'static str,
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
            maintainers: "None (<a href=\"mailto:updates@openpowerlifting.org\">Apply</a>)",
            instagram: "",
        }
    }
}

fn set_hardcoded_strings(statuses: &mut [FederationStatus]) {
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
    statuses[GPCAUS as usize].status = complete;
    statuses[GPCCRO as usize].status = complete;
    statuses[HPLS as usize].status = complete;
    statuses[HPLSUA as usize].status = complete;
    statuses[HPO as usize].status = complete;
    statuses[IPF as usize].status = complete;
    statuses[IrishPF as usize].status = complete;
    statuses[KDKS as usize].status = complete;
    statuses[KPC as usize].status = complete;
    statuses[LGBT as usize].status = complete;
    statuses[PA as usize].status = complete;
    statuses[ProRaw as usize].status = complete;
    statuses[RPS as usize].status = complete;
    statuses[SCT as usize].status = complete;
    statuses[SPF as usize].status = complete;
    statuses[SSSC as usize].status = complete;
    statuses[THSPA as usize].status = complete;
    statuses[THSWPA as usize].status = complete;
    statuses[USAPL as usize].status = "Since 2014";
    statuses[USPA as usize].status = complete;
    statuses[USPC as usize].status = complete;
    statuses[WP as usize].status = complete;
    statuses[WUAPCRO as usize].status = complete;
    statuses[XPS as usize].status = complete;

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
    statuses[IPA as usize].has_probe = yes;
    statuses[IPF as usize].has_probe = yes;
    statuses[IrishPF as usize].has_probe = yes;
    statuses[IrishPO as usize].has_probe = yes;
    statuses[KNKFSP as usize].has_probe = yes;
    statuses[KRAFT as usize].has_probe = yes;
    statuses[LPF as usize].has_probe = yes;
    statuses[NAPF as usize].has_probe = yes;
    statuses[NASA as usize].has_probe = yes;
    statuses[NIPF as usize].has_probe = yes;
    statuses[NordicPF as usize].has_probe = yes;
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
    statuses[XPS as usize].has_probe = yes;

    // Results format.
    let database = "Database";
    let html = "HTML";
    let jpg = "JPG Images";
    let magazines = "Magazines";
    let openlifter = "OpenLifter";
    let pdf_structured = "PDF (Structured)";
    let pdf_unstructured = "PDF (Unstructured)";
    let xls_unstructured = "XLS (Unstructured)";
    let xls_structured = "XLS (Structured)";
    statuses[_365Strong as usize].format = xls_structured;
    statuses[AAP as usize].format = pdf_structured;
    statuses[AEP as usize].format = html;
    statuses[APA as usize].format = openlifter;
    statuses[APF as usize].format = xls_unstructured;
    statuses[APU as usize].format = html;
    statuses[AusPL as usize].format = openlifter;
    statuses[BB as usize].format = xls_unstructured;
    statuses[BDFPA as usize].format = jpg;
    statuses[BP as usize].format = pdf_unstructured;
    statuses[BPF as usize].format = xls_unstructured;
    statuses[BPU as usize].format = xls_unstructured;
    statuses[BVDG as usize].format = magazines;
    statuses[BVDK as usize].format = pdf_unstructured;
    statuses[CAPO as usize].format = openlifter;
    statuses[CPF as usize].format = pdf_unstructured;
    statuses[CPU as usize].format = database;
    statuses[CroatiaUA as usize].format = jpg;
    statuses[DBKV as usize].format = magazines;
    statuses[DSF as usize].format = database;
    statuses[EPA as usize].format = pdf_unstructured;
    statuses[EPF as usize].format = html;
    statuses[GPCAUS as usize].format = openlifter;
    statuses[GPCCRO as usize].format = xls_structured;
    statuses[HPLS as usize].format = xls_unstructured;
    statuses[HPLSUA as usize].format = jpg;
    statuses[HPO as usize].format = html;
    statuses[Hunpower as usize].format = xls_structured;
    statuses[IPA as usize].format = xls_unstructured;
    statuses[IPF as usize].format = html;
    statuses[IPLNZ as usize].format = pdf_structured;
    statuses[IrishPF as usize].format = html;
    statuses[IrishPO as usize].format = xls_unstructured;
    statuses[LGBT as usize].format = xls_unstructured;
    statuses[KDKS as usize].format = openlifter;
    statuses[MMAUS as usize].format = openlifter;
    statuses[NASA as usize].format = pdf_structured;
    statuses[NZPF as usize].format = pdf_unstructured;
    statuses[OceaniaPF as usize].format = pdf_structured;
    statuses[OEVK as usize].format = pdf_unstructured;
    statuses[ORPF as usize].format = pdf_structured;
    statuses[PA as usize].format = html;
    statuses[ProRaw as usize].format = xls_unstructured;
    statuses[PS as usize].format = openlifter;
    statuses[RAW as usize].format = pdf_unstructured;
    statuses[RPS as usize].format = html;
    statuses[SDFPF as usize].format = pdf_unstructured;
    statuses[SPF as usize].format = html;
    statuses[SSSC as usize].format = xls_structured;
    statuses[SwissPL as usize].format = pdf_structured;
    statuses[THSPA as usize].format = database;
    statuses[THSWPA as usize].format = database;
    statuses[UAEPL as usize].format = xls_structured;
    statuses[UPA as usize].format = pdf_unstructured;
    statuses[UPCGermany as usize].format = pdf_unstructured;
    statuses[USAPL as usize].format = database;
    statuses[USPA as usize].format = pdf_structured;
    statuses[USPC as usize].format = openlifter;
    statuses[WelshPA as usize].format = pdf_unstructured;
    statuses[WPC as usize].format = xls_unstructured;
    statuses[WPCItaly as usize].format = jpg;
    statuses[WPNZ as usize].format = jpg;
    statuses[WRPFAUS as usize].format = openlifter;
    statuses[WRPFCRO as usize].format = database;
    statuses[WUAP as usize].format = pdf_structured;
    statuses[WUAPCRO as usize].format = database;

    // Subjective ease of importation.
    let easy = "Easy";
    let medium = "Medium";
    let difficult = "Difficult";
    let impossible = "Impossible";
    statuses[_365Strong as usize].ease = easy;
    statuses[AAP as usize].ease = difficult;
    statuses[AEP as usize].ease = medium;
    statuses[APA as usize].ease = easy;
    statuses[APF as usize].ease = medium;
    statuses[APU as usize].ease = easy;
    statuses[AusPL as usize].ease = easy;
    statuses[BB as usize].ease = difficult;
    statuses[BDFPA as usize].ease = impossible;
    statuses[BP as usize].ease = medium;
    statuses[BPF as usize].ease = easy;
    statuses[BPU as usize].ease = easy;
    statuses[BVDG as usize].ease = difficult;
    statuses[BVDK as usize].ease = medium;
    statuses[CAPO as usize].ease = easy;
    statuses[CPF as usize].ease = medium;
    statuses[CPU as usize].ease = easy;
    statuses[CroatiaUA as usize].ease = difficult;
    statuses[DBKV as usize].ease = medium;
    statuses[DSF as usize].ease = easy;
    statuses[EPA as usize].ease = medium;
    statuses[EPF as usize].ease = easy;
    statuses[GPCAUS as usize].ease = easy;
    statuses[GPCCRO as usize].ease = easy;
    statuses[HPLS as usize].ease = medium;
    statuses[HPLSUA as usize].ease = difficult;
    statuses[HPO as usize].ease = medium;
    statuses[Hunpower as usize].ease = easy;
    statuses[IPA as usize].ease = difficult;
    statuses[IPF as usize].ease = easy;
    statuses[IPLNZ as usize].ease = medium;
    statuses[IrishPF as usize].ease = easy;
    statuses[IrishPO as usize].ease = easy;
    statuses[KDKS as usize].ease = easy;
    statuses[LGBT as usize].ease = easy;
    statuses[MMAUS as usize].ease = easy;
    statuses[NASA as usize].ease = easy;
    statuses[NZPF as usize].ease = difficult;
    statuses[OceaniaPF as usize].ease = medium;
    statuses[OEVK as usize].ease = difficult;
    statuses[ORPF as usize].ease = difficult;
    statuses[PA as usize].ease = easy;
    statuses[ProRaw as usize].ease = medium;
    statuses[PS as usize].ease = easy;
    statuses[RAW as usize].ease = difficult;
    statuses[RPS as usize].ease = easy;
    statuses[SDFPF as usize].ease = difficult;
    statuses[SPF as usize].ease = easy;
    statuses[SSSC as usize].ease = easy;
    statuses[SwissPL as usize].ease = difficult;
    statuses[THSPA as usize].ease = medium;
    statuses[THSWPA as usize].ease = medium;
    statuses[UAEPL as usize].ease = easy;
    statuses[UPA as usize].ease = difficult;
    statuses[UPCGermany as usize].ease = difficult;
    statuses[USAPL as usize].ease = easy;
    statuses[USPA as usize].ease = easy;
    statuses[USPC as usize].ease = easy;
    statuses[WPC as usize].ease = medium;
    statuses[WPCItaly as usize].ease = difficult;
    statuses[WPNZ as usize].ease = difficult;
    statuses[WRPFAUS as usize].ease = easy;
    statuses[WRPFCRO as usize].ease = easy;
    statuses[WUAP as usize].ease = difficult;
    statuses[WUAPCRO as usize].ease = easy;

    // Maintainership variables.
    let email_alan = "<a href=\"mailto:alan.zgb@gmail.com\">alan@</a>";
    let email_artem = "<a href=\"mailto:artem.rodygin@gmail.com\">Artem Rodygin</a>";
    let email_boris = "<a href=\"mailto:boris@openpowerlifting.org\">boris@</a>";
    let email_enno = "<a href=\"mailto:enno@openpowerlifting.org\">enno@</a>";
    let email_gem = "<a href=\"mailto:gem@openpowerlifting.org\">gem@</a>";
    let email_james = "<a href=\"mailto:issues@openpowerlifting.org\">James@</a>";
    let email_jo = "<a href=\"mailto:jo@openpowerlifting.org\">jo@</a>";
    let email_laszlo = "<a href=\"mailto:laszlopota00@gmail.com\">laszlo@</a>";
    let email_laura = "<a href=\"mailto:rettigx+opl@gmail.com\">laura@</a>";
    let email_matt = "<a href=\"mailto:matt@openpowerlifting.org\">matt@</a>";
    let email_mayed = "<a href=\"mailto:mayed.alredha@gmail.com\">Mayed Alredha</a>";
    let email_mbeelen = "<a href=\"mailto:mbeelen@openpowerlifting.org\">mbeelen@</a>";
    let email_milena = "<a href=\"mailto:milena@openpowerlifting.org\">milena@</a>";
    let email_robby = "<a href=\"mailto:ramasson@hotmail.co.uk\">Robby Masson</a>";
    let email_romi = "<a href=\"mailto:romi@openpowerlifting.org\">Romi@</a>";
    let email_rosie = "<a href=\"mailto:issues@openpowerlifting.org\">Rosie@</a>";
    let email_sean = "<a href=\"mailto:sean@openpowerlifting.org\">sean@</a>";
    let email_stefanie = "<a href=\"mailto:stefanie@openpowerlifting.org\">stefanie@</a>";

    // Maintainership information.
    statuses[_365Strong as usize].maintainers = email_sean;
    statuses[AEP as usize].maintainers = email_enno;
    statuses[APF as usize].maintainers = email_gem;
    statuses[APA as usize].maintainers = email_sean;
    statuses[APU as usize].maintainers = email_rosie;
    statuses[AusPL as usize].maintainers = email_james;
    statuses[BAWLA as usize].maintainers = email_jo;
    statuses[BP as usize].maintainers = email_jo;
    statuses[BPF as usize].maintainers = email_gem;
    statuses[BPU as usize].maintainers = email_gem;
    statuses[BVDG as usize].maintainers = email_romi;
    statuses[BVDK as usize].maintainers = email_romi;
    statuses[CAPO as usize].maintainers = email_james;
    statuses[CommonwealthPF as usize].maintainers = email_jo;
    statuses[CPF as usize].maintainers = email_sean;
    statuses[CPU as usize].maintainers = email_sean;
    statuses[CroatiaUA as usize].maintainers = email_alan;
    statuses[DBKV as usize].maintainers = email_romi;
    statuses[EPA as usize].maintainers = email_jo;
    statuses[EPF as usize].maintainers = email_jo;
    statuses[GPCAUS as usize].maintainers = email_james;
    statuses[GPCGB as usize].maintainers = email_gem;
    statuses[GPCNZ as usize].maintainers = email_matt;
    statuses[GPCPortugal as usize].maintainers = email_gem;
    statuses[GPCScotland as usize].maintainers = email_robby;
    statuses[HPLS as usize].maintainers = email_alan;
    statuses[HPLSUA as usize].maintainers = email_alan;
    statuses[Hunpower as usize].maintainers = email_laszlo;
    statuses[IPF as usize].maintainers = email_jo;
    statuses[IPLNZ as usize].maintainers = email_matt;
    statuses[IrelandUA as usize].maintainers = email_jo;
    statuses[IrishPO as usize].maintainers = email_gem;
    statuses[KBGV as usize].maintainers = email_stefanie;
    statuses[KDKS as usize].maintainers = email_laura;
    statuses[KNKFSP as usize].maintainers = email_mbeelen;
    statuses[KPC as usize].maintainers = email_mayed;
    statuses[LFPH as usize].maintainers = email_stefanie;
    statuses[LGBT as usize].maintainers = email_gem;
    statuses[MMAUS as usize].maintainers = email_james;
    statuses[NASA as usize].maintainers = email_boris;
    statuses[NIPF as usize].maintainers = email_jo;
    statuses[NPB as usize].maintainers = email_mbeelen;
    statuses[NZPF as usize].maintainers = email_artem;
    statuses[OceaniaPF as usize].maintainers = email_matt;
    statuses[OEVK as usize].maintainers = email_milena;
    statuses[ORPF as usize].maintainers = email_matt;
    statuses[PA as usize].maintainers = email_james;
    statuses[ProRaw as usize].maintainers = email_james;
    statuses[PS as usize].maintainers = email_mayed;
    statuses[QatarPL as usize].maintainers = email_mayed;
    statuses[RPS as usize].maintainers = email_sean;
    statuses[ScottishPL as usize].maintainers = email_robby;
    statuses[SPF as usize].maintainers = email_sean;
    statuses[SSSC as usize].maintainers = email_mayed;
    statuses[SwissPL as usize].maintainers = email_laura;
    statuses[UAEPL as usize].maintainers = email_mayed;
    statuses[USAPL as usize].maintainers = email_sean;
    statuses[USPA as usize].maintainers = email_sean;
    statuses[USPC as usize].maintainers = email_sean;
    statuses[THSPA as usize].maintainers = email_sean;
    statuses[THSWPA as usize].maintainers = email_sean;
    statuses[UPA as usize].maintainers = email_gem;
    statuses[VGPF as usize].maintainers = email_stefanie;
    statuses[WelshPA as usize].maintainers = email_jo;
    statuses[WPC as usize].maintainers = email_gem;
    statuses[WPCFinland as usize].maintainers = email_gem;
    statuses[WPCFrance as usize].maintainers = email_gem;
    statuses[WPCItaly as usize].maintainers = email_gem;
    statuses[WPCPoland as usize].maintainers = email_gem;
    statuses[WPCPortugal as usize].maintainers = email_gem;
    statuses[WPNZ as usize].maintainers = email_matt;
    statuses[WRPFAUS as usize].maintainers = email_james;
    statuses[XPS as usize].maintainers = email_sean;

    // Don't ask for maintainership applications for defunct, completed federations.
    statuses[BB as usize].maintainers = "";
    statuses[SCT as usize].maintainers = "";

    // Federation Instagram accounts.
    statuses[_365Strong as usize].instagram = "365_strongwpf";
    statuses[AAP as usize].instagram = "alianzaargentinapowerlifting_";
    statuses[APA as usize].instagram = "apawpa_official";
    statuses[APF as usize].instagram = "apf_powerlifting";
    statuses[APU as usize].instagram = "australianpowerliftingunion";
    statuses[ARPL as usize].instagram = "powerlifting.apl";
    statuses[AsianPF as usize].instagram = "asian.powerlifting.federation";
    statuses[AusPL as usize].instagram = "aplpowerlifting";
    statuses[BVDK as usize].instagram = "german_powerlifting";
    statuses[BPU as usize].instagram = "british_powerlifting_union";
    statuses[CAPO as usize].instagram = "capopowerlifting";
    statuses[ChinaPA as usize].instagram = "gpachina";
    statuses[DSF as usize].instagram = "danskstyrkeloeftforbund";
    statuses[FFForce as usize].instagram = "ffforce__";
    statuses[FPO as usize].instagram = "fpo.ry";
    statuses[GPCAUS as usize].instagram = "gpcaustralia";
    statuses[GPCGB as usize].instagram = "gpc_gb";
    statuses[GPCIRL as usize].instagram = "gpcireland";
    statuses[GPCNZ as usize].instagram = "gpcnewzealand";
    statuses[GPCScotland as usize].instagram = "gpc.scotland";
    statuses[HPLS as usize].instagram = "hrvatskipowerliftingsavez";
    statuses[Hunpower as usize].instagram = "hunpowerlifting";
    statuses[IPF as usize].instagram = "theipf";
    statuses[IPL as usize].instagram = "iplpowerlifting";
    statuses[IrishPF as usize].instagram = "irishpowerliftingfederation";
    statuses[IrishPO as usize].instagram = "ipoaipowpc";
    statuses[KDKS as usize].instagram = "kdkschweiz";
    statuses[KNKFSP as usize].instagram = "powerliften";
    statuses[KPC as usize].instagram = "kuwait.powerlifting";
    statuses[LJTF as usize].instagram = "lithuanianpowerlifting";
    statuses[NPB as usize].instagram = "powerliften";
    statuses[NIPF as usize].instagram = "nipowerlifting";
    statuses[NZPF as usize].instagram = "newzealandpowerlifting";
    statuses[ProRaw as usize].instagram = "prorawpowerlifting";
    statuses[RawIronPL as usize].instagram = "rawironpowerliftingleague";
    statuses[RAWUKR as usize].instagram = "raw100power";
    statuses[RPS as usize].instagram = "rps_powerlifting";
    statuses[ScottishPL as usize].instagram = "scottishpowerlifting";
    statuses[SDFPF as usize].instagram = "sdfpf";
    statuses[SPF as usize].instagram = "southernpowerliftingfederation";
    statuses[SVNL as usize].instagram = "voimanostoliitto";
    statuses[SSSC as usize].instagram = "ksa_strength";
    statuses[SwissPL as usize].instagram = "swiss_powerlifting";
    statuses[ThaiPF as usize].instagram = "thaipowerlifting";
    statuses[UAEPL as usize].instagram = "powerlifting_uae";
    statuses[UPA as usize].instagram = "upa_events_";
    statuses[UPC as usize].instagram = "powerliftingupc";
    statuses[USAPL as usize].instagram = "usapowerlifting";
    statuses[USPA as usize].instagram = "uspapower";
    statuses[USPC as usize].instagram = "uspc.pl";
    statuses[WelshPA as usize].instagram = "welsh_powerlifting";
    statuses[WNPF as usize].instagram = "wnpf_powerlifting";
    statuses[WPAU as usize].instagram = "wpaukraine";
    statuses[WPPO as usize].instagram = "parapowerlifting";
    statuses[WPCFinland as usize].instagram = "wpcfinland";
    statuses[WPCSA as usize].instagram = "wpc_powerlifting_cpt";
    statuses[WPSFBelarus as usize].instagram = "wpro_wpsf_belarus";
    statuses[WRPF as usize].instagram = "wrp_official";
    statuses[WRPFAUS as usize].instagram = "wrpfaustralia";
    statuses[WRPFBelarus as usize].instagram = "wrpf_belarus";
    statuses[WRPFIreland as usize].instagram = "wrpfireland";
    statuses[WRPFKAZ as usize].instagram = "wrpfkazakhstan";
    statuses[WRPFLithuania as usize].instagram = "wrpf_lithuania";
    statuses[WRPFSlovenia as usize].instagram = "wrpfslovenia";
    statuses[WRPFSpain as usize].instagram = "wrpfspain";
    statuses[WRPFSweden as usize].instagram = "wrpf.sweden";
    statuses[WUAP as usize].instagram = "wuapusa";
    statuses[WUAPUSA as usize].instagram = "wuapusa";
}

impl Context {
    pub fn new(
        opldb: &opldb::OplDb,
        locale: &Locale,
        fed_filter: Option<fn(Federation) -> bool>,
    ) -> Context {
        let mut statuses: Vec<FederationStatus> =
            Federation::iter().map(FederationStatus::new).collect();

        for meet in opldb.meets() {
            let idx = meet.federation as usize;
            statuses[idx].meet_count += 1;
        }

        set_hardcoded_strings(&mut statuses);

        // Apply a filter after hardcoded strings are set.
        // This changes the indices of each vector, so `federation as usize` logic
        // is invalid after this point.
        if let Some(f) = fed_filter {
            statuses.retain(|s| f(s.fed))
        }

        Context {
            urlprefix: "/",
            page_title: locale.strings.header.status,
            page_description: locale.strings.html_header.description,
            language: locale.language,
            strings: locale.strings,
            units: locale.units,
            fed_statuses: statuses,
            num_entries: opldb.entries().len() as u32,
            num_meets: opldb.meets().len() as u32,
            num_lifters: opldb.lifters().len() as u32,
        }
    }
}
