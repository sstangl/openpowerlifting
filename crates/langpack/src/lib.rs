//! Internationalization facilities.

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate strum_macros;

use opltypes::*;
use serde::ser::Serialize;
use strum::IntoEnumIterator;

use std::fmt;

/// List of languages accepted by the project, in ISO 639-1 code.
#[allow(non_camel_case_types)]
#[derive(
    Clone, Copy, Debug, EnumIter, EnumString, IntoStaticStr, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum Language {
    /// Arabic.
    ar,
    /// Czech.
    cs,
    /// German, without regional variance.
    de,
    /// Greek.
    el,
    /// English, without regional variance (US).
    en,
    /// Esperanto.
    eo,
    /// Spanish.
    es,
    /// Finnish.
    fi,
    /// French.
    fr,
    /// Croatian.
    hr,
    /// Hungarian.
    hu,
    /// Italian.
    it,
    /// Japanese.
    ja,
    /// Korean.
    ko,
    /// Lithuanian.
    lt,
    /// Dutch.
    nl,
    /// Polish.
    pl,
    /// Portuguese.
    pt,
    /// Slovak.
    sk,
    /// Slovenian.
    sl,
    /// Serbian,
    sr,
    /// Swedish.
    sv,
    /// Russian.
    ru,
    /// Turkish.
    tr,
    /// Ukrainian.
    uk,
    /// Vietnamese.
    vi,
    /// Chinese, written in Traditional Chinese script.
    #[serde(rename = "zh-Hant")]
    #[strum(to_string = "zh-Hant")]
    zh_hant,
    /// Chinese, written in Simplified Chinese script.
    #[serde(rename = "zh-Hans")]
    #[strum(to_string = "zh-Hans")]
    zh_hans,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_str: &'static str = self.into();
        write!(f, "{as_str}")
    }
}

impl Language {
    /// Returns the units associated with the language.
    pub fn default_units(self) -> WeightUnits {
        // English variants are decided by common::select_weight_units().
        WeightUnits::Kg
    }

    /// Returns a list of available languages as strings.
    pub fn string_list() -> Vec<&'static str> {
        Language::iter().map(|lang| lang.into()).collect()
    }
}

/// Helper struct to pass around language information.
pub struct Locale {
    pub language: Language,
    pub strings: &'static Translations,
    pub number_format: NumberFormat,
    pub units: WeightUnits,
}

impl Locale {
    pub fn new(language: Language, units: WeightUnits) -> Locale {
        let langinfo = LangInfo::from_global();
        Locale {
            language,
            strings: langinfo.translations(language),
            number_format: language.number_format(),
            units,
        }
    }

    /// Localizes an arbitrary ordinal number.
    ///
    /// The Sex refers to either the sex of the lifter or the grammatical
    /// gender.
    pub fn ordinal(&self, n: u32, sex: Sex) -> LocalizedOrdinal {
        LocalizedOrdinal::from(n, self.language, sex)
    }

    /// Localizes a `Place`.
    ///
    /// Sex refers to the sex of the lifter.
    pub fn place(&self, place: Place, sex: Sex) -> LocalizedPlace {
        LocalizedPlace::from(place, self.language, sex)
    }
}

#[derive(Serialize)]
pub struct UnitsTranslations {
    pub lbs: &'static str,
    pub kg: &'static str,
}

#[derive(Serialize)]
pub struct EquipmentTranslations {
    pub raw: &'static str,
    pub wraps: &'static str,
    pub single: &'static str,
    pub multi: &'static str,
    pub unlimited: &'static str,
    pub straps: &'static str,

    /// Terminology for OpenIPF, meaning "Raw".
    pub classic: &'static str,
    /// Terminology for OpenIPF, meaning "Single-ply".
    pub equipped: &'static str,
}

#[derive(Serialize)]
pub struct SexTranslations {
    pub m: &'static str,
    pub f: &'static str,
    pub mx: &'static str,
}

#[derive(Serialize)]
pub struct CountryTranslations {
    // Continents are placed here rather than make a new struct.
    // We realize these aren't countries.
    pub africa: &'static str,
    pub antarctica: &'static str,
    pub asia: &'static str,
    pub europe: &'static str,
    pub south_america: &'static str,
    pub north_america: &'static str,
    pub oceania: &'static str,

    pub abkhazia: &'static str,
    pub afghanistan: &'static str,
    pub albania: &'static str,
    pub algeria: &'static str,
    pub americansamoa: &'static str,
    pub angola: &'static str,
    pub argentina: &'static str,
    pub armenia: &'static str,
    pub aruba: &'static str,
    pub australia: &'static str,
    pub austria: &'static str,
    pub azerbaijan: &'static str,
    pub bahamas: &'static str,
    pub bahrain: &'static str,
    pub bangladesh: &'static str,
    pub belarus: &'static str,
    pub belgium: &'static str,
    pub belize: &'static str,
    pub benin: &'static str,
    pub bolivia: &'static str,
    pub bosniaandherzegovina: &'static str,
    pub botswana: &'static str,
    pub brazil: &'static str,
    pub britishvirginislands: &'static str,
    pub brunei: &'static str,
    pub bulgaria: &'static str,
    pub burkinafaso: &'static str,
    pub caboverde: &'static str,
    pub cambodia: &'static str,
    pub cameroon: &'static str,
    pub canada: &'static str,
    pub caymanislands: &'static str,
    pub centralafricanrepublic: &'static str,
    pub chile: &'static str,
    pub china: &'static str,
    pub colombia: &'static str,
    pub comoros: &'static str,
    pub congo: &'static str,
    pub cookislands: &'static str,
    pub costarica: &'static str,
    pub croatia: &'static str,
    pub cuba: &'static str,
    pub cyprus: &'static str,
    pub czechia: &'static str,
    pub czechoslovakia: &'static str,
    pub denmark: &'static str,
    pub djibouti: &'static str,
    pub dominicanrepublic: &'static str,
    pub eastgermany: &'static str,
    pub easttimor: &'static str,
    pub ecuador: &'static str,
    pub egypt: &'static str,
    pub elsalvador: &'static str,
    pub england: &'static str,
    pub estonia: &'static str,
    pub eswatini: &'static str,
    pub ethiopia: &'static str,
    pub fiji: &'static str,
    pub finland: &'static str,
    pub france: &'static str,
    pub gabon: &'static str,
    pub georgia: &'static str,
    pub germany: &'static str,
    pub ghana: &'static str,
    pub gibraltar: &'static str,
    pub greece: &'static str,
    pub grenada: &'static str,
    pub guatemala: &'static str,
    pub guinea: &'static str,
    pub guineabissau: &'static str,
    pub guyana: &'static str,
    pub haiti: &'static str,
    pub honduras: &'static str,
    pub hongkong: &'static str,
    pub hungary: &'static str,
    pub iceland: &'static str,
    pub india: &'static str,
    pub indonesia: &'static str,
    pub ireland: &'static str,
    pub israel: &'static str,
    pub italy: &'static str,
    pub iran: &'static str,
    pub iraq: &'static str,
    pub isleofman: &'static str,
    pub ivorycoast: &'static str,
    pub jamaica: &'static str,
    pub japan: &'static str,
    pub jordan: &'static str,
    pub kazakhstan: &'static str,
    pub kenya: &'static str,
    pub kiribati: &'static str,
    pub kuwait: &'static str,
    pub kyrgyzstan: &'static str,
    pub laos: &'static str,
    pub latvia: &'static str,
    pub lebanon: &'static str,
    pub lesotho: &'static str,
    pub liberia: &'static str,
    pub libya: &'static str,
    pub lithuania: &'static str,
    pub luxembourg: &'static str,
    pub madagascar: &'static str,
    pub malaysia: &'static str,
    pub mali: &'static str,
    pub malta: &'static str,
    pub marshallislands: &'static str,
    pub mauritania: &'static str,
    pub mauritius: &'static str,
    pub mexico: &'static str,
    pub moldova: &'static str,
    pub monaco: &'static str,
    pub mongolia: &'static str,
    pub montenegro: &'static str,
    pub morocco: &'static str,
    pub myanmar: &'static str,
    pub namibia: &'static str,
    pub nauru: &'static str,
    pub nepal: &'static str,
    pub netherlands: &'static str,
    pub netherlandsantilles: &'static str,
    pub newcaledonia: &'static str,
    pub newzealand: &'static str,
    pub nicaragua: &'static str,
    pub niger: &'static str,
    pub nigeria: &'static str,
    pub niue: &'static str,
    pub northmacedonia: &'static str,
    pub norway: &'static str,
    pub northernireland: &'static str,
    pub oman: &'static str,
    pub pakistan: &'static str,
    pub palestine: &'static str,
    pub panama: &'static str,
    pub papuanewguinea: &'static str,
    pub paraguay: &'static str,
    pub peru: &'static str,
    pub philippines: &'static str,
    pub poland: &'static str,
    pub portugal: &'static str,
    pub puertorico: &'static str,
    pub qatar: &'static str,
    pub rhodesia: &'static str,
    pub romania: &'static str,
    pub russia: &'static str,
    pub rwanda: &'static str,
    pub samoa: &'static str,
    pub saudiarabia: &'static str,
    pub scotland: &'static str,
    pub senegal: &'static str,
    pub serbia: &'static str,
    pub serbiaandmontenegro: &'static str,
    pub sierraleone: &'static str,
    pub singapore: &'static str,
    pub slovakia: &'static str,
    pub slovenia: &'static str,
    pub solomonislands: &'static str,
    pub southafrica: &'static str,
    pub southkorea: &'static str,
    pub spain: &'static str,
    pub srilanka: &'static str,
    pub sudan: &'static str,
    pub suriname: &'static str,
    pub sweden: &'static str,
    pub switzerland: &'static str,
    pub syria: &'static str,
    pub tahiti: &'static str,
    pub taiwan: &'static str,
    pub tajikistan: &'static str,
    pub tanzania: &'static str,
    pub thailand: &'static str,
    pub thegambia: &'static str,
    pub togo: &'static str,
    pub tonga: &'static str,
    pub transnistria: &'static str,
    pub trinidadandtobago: &'static str,
    pub tunisia: &'static str,
    pub turkey: &'static str,
    pub turkmenistan: &'static str,
    pub tuvalu: &'static str,
    pub uae: &'static str,
    pub uk: &'static str,
    pub ukraine: &'static str,
    pub uganda: &'static str,
    pub uruguay: &'static str,
    pub usa: &'static str,
    pub ussr: &'static str,
    pub usvirginislands: &'static str,
    pub uzbekistan: &'static str,
    pub vanuatu: &'static str,
    pub venezuela: &'static str,
    pub vietnam: &'static str,
    pub wales: &'static str,
    pub wallisandfutuna: &'static str,
    pub westgermany: &'static str,
    pub yemen: &'static str,
    pub yugoslavia: &'static str,
    pub zambia: &'static str,
    pub zimbabwe: &'static str,
}

#[derive(Serialize)]
pub struct PageTitleTranslations {
    pub rankings: &'static str,
    pub records: &'static str,
    pub meets: &'static str,
}

#[derive(Serialize)]
pub struct HtmlHeaderTranslations {
    pub description: &'static str,
}

#[derive(Serialize)]
pub struct HeaderTranslations {
    pub rankings: &'static str,
    pub records: &'static str,
    pub meets: &'static str,
    pub data: &'static str,
    pub apps: &'static str,
    pub status: &'static str,
    pub faq: &'static str,
    pub contact: &'static str,
    pub shop: &'static str,
    pub supportus: &'static str,
}

#[derive(Serialize)]
pub struct ColumnTranslations {
    pub place: &'static str,
    pub formulaplace: &'static str,
    pub liftername: &'static str,
    pub federation: &'static str,
    pub date: &'static str,
    pub location: &'static str,
    pub home: &'static str,
    pub meetname: &'static str,
    pub division: &'static str,
    pub sex: &'static str,
    pub age: &'static str,
    pub equipment: &'static str,
    pub weightclass: &'static str,
    pub bodyweight: &'static str,
    pub squat: &'static str,
    pub bench: &'static str,
    pub deadlift: &'static str,
    pub total: &'static str,
    pub wilks: &'static str,
    pub wilks2020: &'static str,
    pub mcculloch: &'static str,
    pub glossbrenner: &'static str,
    pub ipfpoints: &'static str,
    pub dots: &'static str,
    pub goodlift: &'static str,
    pub num_lifters: &'static str,
}

#[derive(Serialize)]
pub struct ButtonTranslations {
    pub search: &'static str,
    pub download_as_csv: &'static str,
}

#[derive(Serialize)]
pub struct LabelTranslations {
    pub sort: &'static str,
    pub category: &'static str,
}

#[derive(Serialize)]
pub struct SelectorTranslations {
    pub equipment: EquipmentSelectorTranslations,
    pub weightclass: WeightClassSelectorTranslations,
    pub sort: SortSelectorTranslations,
    pub year: YearSelectorTranslations,
    pub sex: SexSelectorTranslations,
    pub event: EventSelectorTranslations,
    pub fed: FedSelectorTranslations,
    pub ageclass: AgeClassSelectorTranslations,
}

#[derive(Serialize)]
pub struct EquipmentSelectorTranslations {
    pub raw: &'static str,
    pub wraps: &'static str,
    pub raw_wraps: &'static str,
    pub single: &'static str,
    pub multi: &'static str,
    pub unlimited: &'static str,
}

#[derive(Serialize)]
pub struct WeightClassSelectorTranslations {
    pub all: &'static str,
    pub traditional: &'static str,
    pub expanded: &'static str,
    pub ipfmen: &'static str,
    pub ipfwomen: &'static str,
    pub para_men: &'static str,
    pub para_women: &'static str,
    pub wp_men: &'static str,
    pub wp_women: &'static str,
}

#[derive(Serialize)]
pub struct SortSelectorTranslations {
    pub by_squat: &'static str,
    pub by_bench: &'static str,
    pub by_deadlift: &'static str,
    pub by_total: &'static str,
    pub by_ah: &'static str,
    pub by_allometric: &'static str,
    pub by_dots: &'static str,
    pub by_glossbrenner: &'static str,
    pub by_goodlift: &'static str,
    pub by_ipfpoints: &'static str,
    pub by_mcculloch: &'static str,
    pub by_nasa: &'static str,
    pub by_reshel: &'static str,
    pub by_schwartzmalone: &'static str,
    pub by_wilks: &'static str,
    pub by_wilks2020: &'static str,
    pub by_division: &'static str,
    pub weight: &'static str,
    pub points: &'static str,
}

#[derive(Serialize)]
pub struct YearSelectorTranslations {
    pub all: &'static str,
}

#[derive(Serialize)]
pub struct SexSelectorTranslations {
    pub all: &'static str,
    pub m: &'static str,
    pub f: &'static str,
}

#[derive(Serialize)]
pub struct EventSelectorTranslations {
    pub all: &'static str,
    pub full_power: &'static str,
    pub push_pull: &'static str,
    pub squat_only: &'static str,
    pub bench_only: &'static str,
    pub deadlift_only: &'static str,
}

#[derive(Serialize)]
pub struct FedSelectorTranslations {
    pub all: &'static str,
    pub fully_tested: &'static str,
    pub all_tested: &'static str,
    pub international: &'static str,
    pub regional: &'static str,
    pub continents: &'static str,
    pub countries: &'static str,
    pub all_usa: &'static str,
    pub all_usa_tested: &'static str,
    pub all_argentina: &'static str,
    pub all_australia: &'static str,
    pub all_austria: &'static str,
    pub all_azerbaijan: &'static str,
    pub all_belarus: &'static str,
    pub all_belgium: &'static str,
    pub all_ipf_belgium: &'static str,
    pub all_belize: &'static str,
    pub all_bolivia: &'static str,
    pub all_bosniaandherzegovina: &'static str,
    pub all_brazil: &'static str,
    pub all_bulgaria: &'static str,
    pub all_canada: &'static str,
    pub all_chile: &'static str,
    pub all_china: &'static str,
    pub all_colombia: &'static str,
    pub all_croatia: &'static str,
    pub all_cyprus: &'static str,
    pub all_czechia: &'static str,
    pub all_denmark: &'static str,
    pub all_ecuador: &'static str,
    pub all_estonia: &'static str,
    pub all_finland: &'static str,
    pub all_france: &'static str,
    pub all_georgia: &'static str,
    pub all_germany: &'static str,
    pub all_greece: &'static str,
    pub all_grenada: &'static str,
    pub all_guatemala: &'static str,
    pub all_guyana: &'static str,
    pub all_hongkong: &'static str,
    pub all_hungary: &'static str,
    pub all_iceland: &'static str,
    pub all_india: &'static str,
    pub all_indonesia: &'static str,
    pub all_iran: &'static str,
    pub all_ireland: &'static str,
    pub all_israel: &'static str,
    pub all_italy: &'static str,
    pub all_japan: &'static str,
    pub all_kazakhstan: &'static str,
    pub all_kuwait: &'static str,
    pub all_kyrgyzstan: &'static str,
    pub all_latvia: &'static str,
    pub all_lithuania: &'static str,
    pub all_malaysia: &'static str,
    pub all_mexico: &'static str,
    pub all_moldova: &'static str,
    pub all_nauru: &'static str,
    pub all_nepal: &'static str,
    pub all_netherlands: &'static str,
    pub all_newzealand: &'static str,
    pub all_nicaragua: &'static str,
    pub all_niue: &'static str,
    pub all_norway: &'static str,
    pub all_panama: &'static str,
    pub all_papuanewguinea: &'static str,
    pub all_paraguay: &'static str,
    pub all_peru: &'static str,
    pub all_philippines: &'static str,
    pub all_poland: &'static str,
    pub all_portugal: &'static str,
    pub all_romania: &'static str,
    pub all_russia: &'static str,
    pub all_scotland: &'static str,
    pub all_serbia: &'static str,
    pub all_singapore: &'static str,
    pub all_slovakia: &'static str,
    pub all_slovenia: &'static str,
    pub all_spain: &'static str,
    pub all_southafrica: &'static str,
    pub all_southkorea : &'static str,
    pub all_sweden: &'static str,
    pub all_switzerland: &'static str,
    pub all_taiwan: &'static str,
    pub all_thailand: &'static str,
    pub all_turkey: &'static str,
    pub all_uganda: &'static str,
    pub all_uk: &'static str,
    pub all_uk_tested: &'static str,
    pub all_ukraine: &'static str,
    pub all_uruguay: &'static str,
    pub all_usvirginislands: &'static str,
    pub all_venezuela: &'static str,
    pub all_vietnam: &'static str,
    pub all_affiliates: &'static str,
    pub all_internationals: &'static str,
}

#[derive(Serialize)]
pub struct AgeClassSelectorTranslations {
    pub all: &'static str,
    pub youth5_12: &'static str,
    pub teen13_15: &'static str,
    pub teen16_17: &'static str,
    pub teen18_19: &'static str,
    pub juniors20_23: &'static str,
    pub seniors24_34: &'static str,
    pub submasters35_39: &'static str,
    pub masters40_44: &'static str,
    pub masters45_49: &'static str,
    pub masters50_54: &'static str,
    pub masters55_59: &'static str,
    pub masters60_64: &'static str,
    pub masters65_69: &'static str,
    pub masters70_74: &'static str,
    pub masters75_79: &'static str,
    pub masters80p: &'static str,
    pub masters40_49: &'static str,
    pub masters50_59: &'static str,
    pub masters60_69: &'static str,
    pub masters70_79: &'static str,
    pub label_masters_by_5s: &'static str,
    pub label_masters_by_10s: &'static str,
    pub ipf_open: &'static str,
    pub ipf_subjunior: &'static str,
    pub ipf_junior: &'static str,
    pub ipf_senior: &'static str,
    pub ipf_master1: &'static str,
    pub ipf_master2: &'static str,
    pub ipf_master3: &'static str,
    pub ipf_master4: &'static str,
}

#[derive(Serialize)]
pub struct LifterPageTranslations {
    pub personal_bests: &'static str,
    pub competition_results: &'static str,
}

#[derive(Serialize)]
pub struct Translations {
    pub units: UnitsTranslations,
    pub equipment: EquipmentTranslations,
    pub sex: SexTranslations,
    pub page_titles: PageTitleTranslations,
    pub header: HeaderTranslations,
    pub html_header: HtmlHeaderTranslations,
    pub columns: ColumnTranslations,
    pub country: CountryTranslations,
    pub buttons: ButtonTranslations,
    pub labels: LabelTranslations,
    pub selectors: SelectorTranslations,
    pub lifter_page: LifterPageTranslations,
}

/// Owner struct of all translation state.
pub struct LangInfo {
    ar: Translations,
    cs: Translations,
    de: Translations,
    el: Translations,
    en: Translations,
    eo: Translations,
    es: Translations,
    fi: Translations,
    fr: Translations,
    hr: Translations,
    hu: Translations,
    it: Translations,
    ja: Translations,
    ko: Translations,
    lt: Translations,
    nl: Translations,
    pl: Translations,
    pt: Translations,
    sk: Translations,
    sl: Translations,
    sr: Translations,
    sv: Translations,
    ru: Translations,
    tr: Translations,
    uk: Translations,
    vi: Translations,
    zh_hant: Translations,
    zh_hans: Translations,
}

static LANGPACK: LangInfo = LangInfo {
    ar: include!(concat!(env!("OUT_DIR"), "/ar.rs")),
    cs: include!(concat!(env!("OUT_DIR"), "/cs.rs")),
    de: include!(concat!(env!("OUT_DIR"), "/de.rs")),
    el: include!(concat!(env!("OUT_DIR"), "/el.rs")),
    en: include!(concat!(env!("OUT_DIR"), "/en.rs")),
    eo: include!(concat!(env!("OUT_DIR"), "/eo.rs")),
    es: include!(concat!(env!("OUT_DIR"), "/es.rs")),
    fi: include!(concat!(env!("OUT_DIR"), "/fi.rs")),
    fr: include!(concat!(env!("OUT_DIR"), "/fr.rs")),
    hr: include!(concat!(env!("OUT_DIR"), "/hr.rs")),
    hu: include!(concat!(env!("OUT_DIR"), "/hu.rs")),
    it: include!(concat!(env!("OUT_DIR"), "/it.rs")),
    ja: include!(concat!(env!("OUT_DIR"), "/ja.rs")),
    ko: include!(concat!(env!("OUT_DIR"), "/ko.rs")),
    lt: include!(concat!(env!("OUT_DIR"), "/lt.rs")),
    nl: include!(concat!(env!("OUT_DIR"), "/nl.rs")),
    pl: include!(concat!(env!("OUT_DIR"), "/pl.rs")),
    pt: include!(concat!(env!("OUT_DIR"), "/pt.rs")),
    sk: include!(concat!(env!("OUT_DIR"), "/sk.rs")),
    sl: include!(concat!(env!("OUT_DIR"), "/sl.rs")),
    sr: include!(concat!(env!("OUT_DIR"), "/sr.rs")),
    sv: include!(concat!(env!("OUT_DIR"), "/sv.rs")),
    ru: include!(concat!(env!("OUT_DIR"), "/ru.rs")),
    tr: include!(concat!(env!("OUT_DIR"), "/tr.rs")),
    uk: include!(concat!(env!("OUT_DIR"), "/uk.rs")),
    vi: include!(concat!(env!("OUT_DIR"), "/vi.rs")),
    zh_hant: include!(concat!(env!("OUT_DIR"), "/zh-Hant.rs")),
    zh_hans: include!(concat!(env!("OUT_DIR"), "/zh-Hans.rs")),
};

impl LangInfo {
    /// Returns a reference to the LangInfo stored in the binary.
    pub fn from_global() -> &'static LangInfo {
        &LANGPACK
    }

    pub fn translations(&self, language: Language) -> &Translations {
        match language {
            Language::ar => &self.ar,
            Language::cs => &self.cs,
            Language::de => &self.de,
            Language::el => &self.el,
            Language::en => &self.en,
            Language::eo => &self.eo,
            Language::es => &self.es,
            Language::fi => &self.fi,
            Language::fr => &self.fr,
            Language::hr => &self.hr,
            Language::hu => &self.hu,
            Language::it => &self.it,
            Language::ja => &self.ja,
            Language::ko => &self.ko,
            Language::lt => &self.lt,
            Language::nl => &self.nl,
            Language::pl => &self.pl,
            Language::pt => &self.pt,
            Language::sk => &self.sk,
            Language::sl => &self.sl,
            Language::sr => &self.sr,
            Language::sv => &self.sv,
            Language::ru => &self.ru,
            Language::tr => &self.tr,
            Language::uk => &self.uk,
            Language::vi => &self.vi,
            Language::zh_hant => &self.zh_hant,
            Language::zh_hans => &self.zh_hans,
        }
    }
}

impl Translations {
    pub fn translate_equipment(&self, equip: Equipment) -> &str {
        match equip {
            Equipment::Raw => self.equipment.raw,
            Equipment::Wraps => self.equipment.wraps,
            Equipment::Single => self.equipment.single,
            Equipment::Multi => self.equipment.multi,
            Equipment::Unlimited => self.equipment.unlimited,
            Equipment::Straps => self.equipment.straps,
        }
    }

    pub fn translate_sex(&self, sex: Sex) -> &str {
        match sex {
            Sex::M => self.sex.m,
            Sex::F => self.sex.f,
            Sex::Mx => self.sex.mx,
        }
    }

    pub fn translate_country(&self, country: Country) -> &str {
        match country {
            Country::Abkhazia => self.country.abkhazia,
            Country::Afghanistan => self.country.afghanistan,
            Country::Albania => self.country.albania,
            Country::Algeria => self.country.algeria,
            Country::AmericanSamoa => self.country.americansamoa,
            Country::Angola => self.country.angola,
            Country::Argentina => self.country.argentina,
            Country::Aruba => self.country.aruba,
            Country::Armenia => self.country.armenia,
            Country::Austria => self.country.austria,
            Country::Australia => self.country.australia,
            Country::Azerbaijan => self.country.azerbaijan,
            Country::Bahamas => self.country.bahamas,
            Country::Bahrain => self.country.bahrain,
            Country::Bangladesh => self.country.bangladesh,
            Country::Belarus => self.country.belarus,
            Country::Belgium => self.country.belgium,
            Country::Belize => self.country.belize,
            Country::Benin => self.country.benin,
            Country::Bolivia => self.country.bolivia,
            Country::BosniaAndHerzegovina => self.country.bosniaandherzegovina,
            Country::Botswana => self.country.botswana,
            Country::Brazil => self.country.brazil,
            Country::BritishVirginIslands => self.country.britishvirginislands,
            Country::Brunei => self.country.brunei,
            Country::Bulgaria => self.country.bulgaria,
            Country::BurkinaFaso => self.country.burkinafaso,
            Country::CaboVerde => self.country.caboverde,
            Country::Cambodia => self.country.cambodia,
            Country::Cameroon => self.country.cameroon,
            Country::Canada => self.country.canada,
            Country::CaymanIslands => self.country.caymanislands,
            Country::CentralAfricanRepublic => self.country.centralafricanrepublic,
            Country::Chile => self.country.chile,
            Country::China => self.country.china,
            Country::Colombia => self.country.colombia,
            Country::Comoros => self.country.comoros,
            Country::Congo => self.country.congo,
            Country::CookIslands => self.country.cookislands,
            Country::CostaRica => self.country.costarica,
            Country::Croatia => self.country.croatia,
            Country::Cuba => self.country.cuba,
            Country::Cyprus => self.country.cyprus,
            Country::Czechia => self.country.czechia,
            Country::Czechoslovakia => self.country.czechoslovakia,
            Country::Denmark => self.country.denmark,
            Country::Djibouti => self.country.djibouti,
            Country::DominicanRepublic => self.country.dominicanrepublic,
            Country::EastGermany => self.country.eastgermany,
            Country::EastTimor => self.country.easttimor,
            Country::Ecuador => self.country.ecuador,
            Country::Egypt => self.country.egypt,
            Country::ElSalvador => self.country.elsalvador,
            Country::England => self.country.england,
            Country::Estonia => self.country.estonia,
            Country::Eswatini => self.country.eswatini,
            Country::Ethiopia => self.country.ethiopia,
            Country::Fiji => self.country.fiji,
            Country::Finland => self.country.finland,
            Country::France => self.country.france,
            Country::Gabon => self.country.gabon,
            Country::Georgia => self.country.georgia,
            Country::Germany => self.country.germany,
            Country::Ghana => self.country.ghana,
            Country::Gibraltar => self.country.gibraltar,
            Country::Greece => self.country.greece,
            Country::Grenada => self.country.grenada,
            Country::Guatemala => self.country.guatemala,
            Country::Guinea => self.country.guinea,
            Country::GuineaBissau => self.country.guineabissau,
            Country::Guyana => self.country.guyana,
            Country::Haiti => self.country.haiti,
            Country::Honduras => self.country.honduras,
            Country::HongKong => self.country.hongkong,
            Country::Hungary => self.country.hungary,
            Country::Iceland => self.country.iceland,
            Country::India => self.country.india,
            Country::Indonesia => self.country.indonesia,
            Country::Ireland => self.country.ireland,
            Country::Israel => self.country.israel,
            Country::Italy => self.country.italy,
            Country::Iran => self.country.iran,
            Country::Iraq => self.country.iraq,
            Country::IsleOfMan => self.country.isleofman,
            Country::IvoryCoast => self.country.ivorycoast,
            Country::Japan => self.country.japan,
            Country::Jamaica => self.country.jamaica,
            Country::Jordan => self.country.jordan,
            Country::Kazakhstan => self.country.kazakhstan,
            Country::Kenya => self.country.kenya,
            Country::Kiribati => self.country.kiribati,
            Country::Kuwait => self.country.kuwait,
            Country::Kyrgyzstan => self.country.kyrgyzstan,
            Country::Laos => self.country.laos,
            Country::Latvia => self.country.latvia,
            Country::Lebanon => self.country.lebanon,
            Country::Lesotho => self.country.lesotho,
            Country::Liberia => self.country.liberia,
            Country::Libya => self.country.libya,
            Country::Lithuania => self.country.lithuania,
            Country::Luxembourg => self.country.luxembourg,
            Country::Madagascar => self.country.madagascar,
            Country::Malaysia => self.country.malaysia,
            Country::Mali => self.country.mali,
            Country::Malta => self.country.malta,
            Country::MarshallIslands => self.country.marshallislands,
            Country::Mauritania => self.country.mauritania,
            Country::Mauritius => self.country.mauritius,
            Country::Mexico => self.country.mexico,
            Country::Moldova => self.country.moldova,
            Country::Monaco => self.country.monaco,
            Country::Mongolia => self.country.mongolia,
            Country::Montenegro => self.country.montenegro,
            Country::Morocco => self.country.morocco,
            Country::Myanmar => self.country.myanmar,
            Country::Namibia => self.country.namibia,
            Country::Nauru => self.country.nauru,
            Country::Nepal => self.country.nepal,
            Country::Netherlands => self.country.netherlands,
            Country::NetherlandsAntilles => self.country.netherlandsantilles,
            Country::NewCaledonia => self.country.newcaledonia,
            Country::NewZealand => self.country.newzealand,
            Country::Nicaragua => self.country.nicaragua,
            Country::Niger => self.country.niger,
            Country::Nigeria => self.country.nigeria,
            Country::Niue => self.country.niue,
            Country::NorthMacedonia => self.country.northmacedonia,
            Country::Norway => self.country.norway,
            Country::NorthernIreland => self.country.northernireland,
            Country::Oman => self.country.oman,
            Country::Pakistan => self.country.pakistan,
            Country::Palestine => self.country.palestine,
            Country::Panama => self.country.panama,
            Country::PapuaNewGuinea => self.country.papuanewguinea,
            Country::Paraguay => self.country.paraguay,
            Country::Peru => self.country.peru,
            Country::Philippines => self.country.philippines,
            Country::Poland => self.country.poland,
            Country::Portugal => self.country.portugal,
            Country::PuertoRico => self.country.puertorico,
            Country::Qatar => self.country.qatar,
            Country::Rhodesia => self.country.rhodesia,
            Country::Romania => self.country.romania,
            Country::Russia => self.country.russia,
            Country::Rwanda => self.country.rwanda,
            Country::Samoa => self.country.samoa,
            Country::SaudiArabia => self.country.saudiarabia,
            Country::Scotland => self.country.scotland,
            Country::Senegal => self.country.senegal,
            Country::Serbia => self.country.serbia,
            Country::SerbiaAndMontenegro => self.country.serbiaandmontenegro,
            Country::SierraLeone => self.country.sierraleone,
            Country::Singapore => self.country.singapore,
            Country::Slovakia => self.country.slovakia,
            Country::Slovenia => self.country.slovenia,
            Country::SolomonIslands => self.country.solomonislands,
            Country::SouthAfrica => self.country.southafrica,
            Country::SouthKorea => self.country.southkorea,
            Country::Spain => self.country.spain,
            Country::SriLanka => self.country.srilanka,
            Country::Sudan => self.country.sudan,
            Country::Suriname => self.country.suriname,
            Country::Sweden => self.country.sweden,
            Country::Switzerland => self.country.switzerland,
            Country::Syria => self.country.syria,
            Country::Tahiti => self.country.tahiti,
            Country::Taiwan => self.country.taiwan,
            Country::Tajikistan => self.country.tajikistan,
            Country::Tanzania => self.country.tanzania,
            Country::Thailand => self.country.thailand,
            Country::TheGambia => self.country.thegambia,
            Country::Togo => self.country.togo,
            Country::Tonga => self.country.tonga,
            Country::Transnistria => self.country.transnistria,
            Country::TrinidadAndTobago => self.country.trinidadandtobago,
            Country::Tunisia => self.country.tunisia,
            Country::Turkey => self.country.turkey,
            Country::Turkmenistan => self.country.turkmenistan,
            Country::Tuvalu => self.country.tuvalu,
            Country::UAE => self.country.uae,
            Country::Uganda => self.country.uganda,
            Country::UK => self.country.uk,
            Country::Ukraine => self.country.ukraine,
            Country::Uruguay => self.country.uruguay,
            Country::USA => self.country.usa,
            Country::USSR => self.country.ussr,
            Country::USVirginIslands => self.country.usvirginislands,
            Country::Uzbekistan => self.country.uzbekistan,
            Country::Vanuatu => self.country.vanuatu,
            Country::Venezuela => self.country.venezuela,
            Country::Vietnam => self.country.vietnam,
            Country::Wales => self.country.wales,
            Country::WallisAndFutuna => self.country.wallisandfutuna,
            Country::WestGermany => self.country.westgermany,
            Country::Yemen => self.country.yemen,
            Country::Yugoslavia => self.country.yugoslavia,
            Country::Zambia => self.country.zambia,
            Country::Zimbabwe => self.country.zimbabwe,
        }
    }
}

/// Selects the localized format of displayed numbers.
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum NumberFormat {
    /// Arabic numerals with a period as decimal separator, like "1234.5".
    ArabicPeriod,
    /// Arabic numerals with a comma as decimal separator, like "1234,5".
    ArabicComma,
}

impl Language {
    /// Gets the number format for the given language.
    pub fn number_format(self) -> NumberFormat {
        // Taken from the following list:
        // https://en.wikipedia.org/wiki/Decimal_separator
        match self {
            Language::ar => NumberFormat::ArabicPeriod,
            Language::cs => NumberFormat::ArabicComma,
            Language::de => NumberFormat::ArabicComma,
            Language::el => NumberFormat::ArabicComma,
            Language::en => NumberFormat::ArabicPeriod,
            Language::eo => NumberFormat::ArabicComma,
            Language::es => NumberFormat::ArabicPeriod, // TODO: Only Central America.
            Language::fi => NumberFormat::ArabicComma,
            Language::fr => NumberFormat::ArabicComma,
            Language::hr => NumberFormat::ArabicComma,
            Language::hu => NumberFormat::ArabicComma,
            Language::it => NumberFormat::ArabicComma,
            Language::ja => NumberFormat::ArabicPeriod,
            Language::ko => NumberFormat::ArabicPeriod,
            Language::lt => NumberFormat::ArabicComma,
            Language::nl => NumberFormat::ArabicComma,
            Language::pl => NumberFormat::ArabicComma,
            Language::pt => NumberFormat::ArabicComma,
            Language::sk => NumberFormat::ArabicComma,
            Language::sl => NumberFormat::ArabicComma,
            Language::sr => NumberFormat::ArabicComma,
            Language::sv => NumberFormat::ArabicComma,
            Language::ru => NumberFormat::ArabicComma,
            Language::tr => NumberFormat::ArabicComma,
            Language::uk => NumberFormat::ArabicComma,
            Language::vi => NumberFormat::ArabicComma,
            Language::zh_hant => NumberFormat::ArabicPeriod,
            Language::zh_hans => NumberFormat::ArabicPeriod,
        }
    }
}

/// Type that gets serialized into a localized `WeightAny`.
///
/// This is the final weight type that should be stored in the `Context`
/// and passed to the `Template`.
#[derive(Copy, Clone)]
pub struct LocalizedWeightAny {
    pub format: NumberFormat,
    pub weight: WeightAny,
}

impl Serialize for LocalizedWeightAny {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s: String = match self.format {
            NumberFormat::ArabicPeriod => format!("{}", self.weight),
            NumberFormat::ArabicComma => self.weight.format_comma(),
        };
        serializer.serialize_str(&s)
    }
}

/// Type that gets serialized into a localized `Points`.
#[derive(Copy, Clone)]
pub struct LocalizedPoints {
    pub format: NumberFormat,
    pub points: Points,
}

impl Serialize for LocalizedPoints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s: String = match self.format {
            NumberFormat::ArabicPeriod => format!("{}", self.points),
            NumberFormat::ArabicComma => self.points.format_comma(),
        };
        serializer.serialize_str(&s)
    }
}

/// Type that gets serialized into a localized ordinal.
///
/// This is useful for Spanish and Portuguese, which use a special
/// notation for ordinal numbers (dependent on the lifter's `Sex`).
#[derive(Copy, Clone)]
pub enum LocalizedOrdinal {
    /// Renders `1` as `1`.
    NumberOnly(u32),

    /// Renders `1` as `1º`, for men.
    RomanceMasculine(u32),

    /// Renders `1` as `1ª`, for women.
    RomanceFeminine(u32),
}

impl LocalizedOrdinal {
    pub fn from(n: u32, language: Language, sex: Sex) -> LocalizedOrdinal {
        match language {
            Language::es | Language::it | Language::pt => match sex {
                Sex::M | Sex::Mx => LocalizedOrdinal::RomanceMasculine(n),
                Sex::F => LocalizedOrdinal::RomanceFeminine(n),
            },
            _ => LocalizedOrdinal::NumberOnly(n),
        }
    }
}

impl Serialize for LocalizedOrdinal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::NumberOnly(n) => serializer.serialize_u32(n),
            Self::RomanceMasculine(n) => {
                let s = format!("{n}º");
                serializer.serialize_str(&s)
            }
            Self::RomanceFeminine(n) => {
                let s = format!("{n}ª");
                serializer.serialize_str(&s)
            }
        }
    }
}

/// Type that gets serialized into a localized `Place`.
#[derive(Copy, Clone)]
pub enum LocalizedPlace {
    P(LocalizedOrdinal),
    G,
    DQ,
    DD,
    NS,
}

impl LocalizedPlace {
    pub fn from(place: Place, language: Language, sex: Sex) -> LocalizedPlace {
        match place {
            Place::P(n) => {
                let p = u8::from(n) as u32;
                LocalizedPlace::P(LocalizedOrdinal::from(p, language, sex))
            }
            Place::G => LocalizedPlace::G,
            Place::DQ => LocalizedPlace::DQ,
            Place::DD => LocalizedPlace::DD,
            Place::NS => LocalizedPlace::NS,
        }
    }
}

impl Serialize for LocalizedPlace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            LocalizedPlace::P(ord) => ord.serialize(serializer),
            LocalizedPlace::G => serializer.serialize_str("G"),
            LocalizedPlace::DQ => serializer.serialize_str("DQ"),
            LocalizedPlace::DD => serializer.serialize_str("DD"),
            LocalizedPlace::NS => serializer.serialize_str("NS"),
        }
    }
}

/// Type that gets serialized into a localized `WeightClassAny`.
#[derive(Copy, Clone)]
pub struct LocalizedWeightClassAny {
    pub format: NumberFormat,
    pub class: WeightClassAny,
}

impl Serialize for LocalizedWeightClassAny {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s: String = match self.format {
            NumberFormat::ArabicPeriod => format!("{}", self.class),
            NumberFormat::ArabicComma => self.class.format_comma(),
        };
        serializer.serialize_str(&s)
    }
}

impl fmt::Display for LocalizedWeightClassAny {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.format {
            NumberFormat::ArabicPeriod => write!(f, "{}", self.class),
            NumberFormat::ArabicComma => write!(f, "{}", self.class.format_comma()),
        }
    }
}

/// Gets the lifter's name localized into the target language.
pub fn localized_name(lifter: &opldb::Lifter, language: Language) -> &str {
    match language {
        Language::el => lifter.greek_name.as_deref().unwrap_or(&lifter.name),
        Language::ja => lifter.japanese_name.as_deref().unwrap_or(&lifter.name),
        Language::ko => lifter.korean_name.as_deref().unwrap_or(&lifter.name),
        Language::ru | Language::uk => lifter.cyrillic_name.as_deref().unwrap_or(&lifter.name),
        Language::zh_hans | Language::zh_hant => {
            lifter.chinese_name.as_deref().unwrap_or(&lifter.name)
        }

        _ => &lifter.name,
    }
}

/// Localizes the separator between integer and fraction based on
/// `NumberFormat`.
pub trait LocalizeNumber {
    type LocalizedType;

    fn in_format(self, format: NumberFormat) -> Self::LocalizedType;
}

impl LocalizeNumber for WeightAny {
    type LocalizedType = LocalizedWeightAny;

    fn in_format(self, format: NumberFormat) -> LocalizedWeightAny {
        LocalizedWeightAny {
            format,
            weight: self,
        }
    }
}

impl LocalizeNumber for WeightClassAny {
    type LocalizedType = LocalizedWeightClassAny;

    fn in_format(self, format: NumberFormat) -> LocalizedWeightClassAny {
        LocalizedWeightClassAny {
            format,
            class: self,
        }
    }
}

impl LocalizeNumber for Points {
    type LocalizedType = LocalizedPoints;

    fn in_format(self, format: NumberFormat) -> LocalizedPoints {
        LocalizedPoints {
            format,
            points: self,
        }
    }
}
