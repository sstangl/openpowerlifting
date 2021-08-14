//! Internationalization facilities.

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate strum_macros;

extern crate serde_json as json;

use opltypes::*;
use serde::ser::Serialize;
use strum::IntoEnumIterator;

use std::fmt;

/// List of languages accepted by the project, in ISO 639-1 code.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, EnumIter, EnumString, PartialEq, Serialize, Deserialize)]
pub enum Language {
    /// Czech.
    cz,
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
        let s = match self {
            Language::cz => "cz",
            Language::de => "de",
            Language::el => "el",
            Language::en => "en",
            Language::eo => "eo",
            Language::es => "es",
            Language::fi => "fi",
            Language::fr => "fr",
            Language::hr => "hr",
            Language::hu => "hu",
            Language::it => "it",
            Language::ja => "ja",
            Language::ko => "ko",
            Language::lt => "lt",
            Language::nl => "nl",
            Language::pl => "pl",
            Language::pt => "pt",
            Language::sk => "sk",
            Language::sl => "sl",
            Language::sr => "sr",
            Language::sv => "sv",
            Language::ru => "ru",
            Language::tr => "tr",
            Language::uk => "uk",
            Language::vi => "vi",
            Language::zh_hant => "zh-Hant",
            Language::zh_hans => "zh-Hans",
        };
        write!(f, "{}", s)
    }
}

impl Language {
    /// Returns the units associated with the language.
    pub fn default_units(self) -> WeightUnits {
        // English variants are decided by common::select_weight_units().
        WeightUnits::Kg
    }

    /// Returns a list of available languages as strings.
    pub fn string_list() -> Vec<String> {
        Language::iter().map(|lang| lang.to_string()).collect()
    }
}

/// Helper struct to pass around language information.
pub struct Locale<'a> {
    pub langinfo: &'a LangInfo,
    pub language: Language,
    pub strings: &'a Translations,
    pub number_format: NumberFormat,
    pub units: WeightUnits,
}

impl<'a> Locale<'a> {
    pub fn new(langinfo: &'a LangInfo, language: Language, units: WeightUnits) -> Locale<'a> {
        Locale {
            langinfo,
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

#[derive(Serialize, Deserialize)]
pub struct UnitsTranslations {
    pub lbs: String,
    pub kg: String,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentTranslations {
    pub raw: String,
    pub wraps: String,
    pub single: String,
    pub multi: String,
    pub unlimited: String,
    pub straps: String,

    /// Terminology for OpenIPF, meaning "Raw".
    pub classic: String,
    /// Terminology for OpenIPF, meaning "Single-ply".
    pub equipped: String,
}

#[derive(Serialize, Deserialize)]
pub struct SexTranslations {
    pub m: String,
    pub f: String,
    pub mx: String,
}

#[derive(Serialize, Deserialize)]
pub struct CountryTranslations {
    // Continents are placed here rather than make a new struct.
    // We realize these aren't countries.
    pub africa: String,
    pub antarctica: String,
    pub asia: String,
    pub europe: String,
    pub south_america: String,
    pub north_america: String,
    pub oceania: String,

    pub abkhazia: String,
    pub afghanistan: String,
    pub albania: String,
    pub algeria: String,
    pub americansamoa: String,
    pub angola: String,
    pub argentina: String,
    pub armenia: String,
    pub aruba: String,
    pub australia: String,
    pub austria: String,
    pub azerbaijan: String,
    pub bahamas: String,
    pub bahrain: String,
    pub bangladesh: String,
    pub belarus: String,
    pub belgium: String,
    pub belize: String,
    pub benin: String,
    pub bolivia: String,
    pub bosniaandherzegovina: String,
    pub botswana: String,
    pub brazil: String,
    pub britishvirginislands: String,
    pub brunei: String,
    pub bulgaria: String,
    pub burkinafaso: String,
    pub caboverde: String,
    pub cambodia: String,
    pub cameroon: String,
    pub canada: String,
    pub caymanislands: String,
    pub centralafricanrepublic: String,
    pub chile: String,
    pub china: String,
    pub colombia: String,
    pub comoros: String,
    pub congo: String,
    pub cookislands: String,
    pub costarica: String,
    pub croatia: String,
    pub cuba: String,
    pub cyprus: String,
    pub czechia: String,
    pub czechoslovakia: String,
    pub denmark: String,
    pub djibouti: String,
    pub dominicanrepublic: String,
    pub eastgermany: String,
    pub easttimor: String,
    pub ecuador: String,
    pub egypt: String,
    pub elsalvador: String,
    pub england: String,
    pub estonia: String,
    pub eswatini: String,
    pub ethiopia: String,
    pub fiji: String,
    pub finland: String,
    pub france: String,
    pub gabon: String,
    pub georgia: String,
    pub germany: String,
    pub ghana: String,
    pub gibraltar: String,
    pub greece: String,
    pub guatemala: String,
    pub guinea: String,
    pub guineabissau: String,
    pub guyana: String,
    pub haiti: String,
    pub honduras: String,
    pub hongkong: String,
    pub hungary: String,
    pub iceland: String,
    pub india: String,
    pub indonesia: String,
    pub ireland: String,
    pub israel: String,
    pub italy: String,
    pub iran: String,
    pub iraq: String,
    pub ivorycoast: String,
    pub jamaica: String,
    pub japan: String,
    pub jordan: String,
    pub kazakhstan: String,
    pub kenya: String,
    pub kiribati: String,
    pub kuwait: String,
    pub kyrgyzstan: String,
    pub laos: String,
    pub latvia: String,
    pub lebanon: String,
    pub lesotho: String,
    pub liberia: String,
    pub libya: String,
    pub lithuania: String,
    pub luxembourg: String,
    pub malaysia: String,
    pub mali: String,
    pub malta: String,
    pub marshallislands: String,
    pub mauritania: String,
    pub mauritius: String,
    pub mexico: String,
    pub moldova: String,
    pub monaco: String,
    pub mongolia: String,
    pub montenegro: String,
    pub morocco: String,
    pub myanmar: String,
    pub namibia: String,
    pub nauru: String,
    pub nepal: String,
    pub netherlands: String,
    pub netherlandsantilles: String,
    pub newcaledonia: String,
    pub newzealand: String,
    pub nicaragua: String,
    pub niger: String,
    pub nigeria: String,
    pub niue: String,
    pub northmacedonia: String,
    pub norway: String,
    pub northernireland: String,
    pub oman: String,
    pub pakistan: String,
    pub palestine: String,
    pub panama: String,
    pub papuanewguinea: String,
    pub paraguay: String,
    pub peru: String,
    pub philippines: String,
    pub poland: String,
    pub portugal: String,
    pub puertorico: String,
    pub qatar: String,
    pub rhodesia: String,
    pub romania: String,
    pub russia: String,
    pub rwanda: String,
    pub samoa: String,
    pub saudiarabia: String,
    pub scotland: String,
    pub senegal: String,
    pub serbia: String,
    pub serbiaandmontenegro: String,
    pub sierraleone: String,
    pub singapore: String,
    pub slovakia: String,
    pub slovenia: String,
    pub solomonislands: String,
    pub southafrica: String,
    pub southkorea: String,
    pub spain: String,
    pub srilanka: String,
    pub sudan: String,
    pub sweden: String,
    pub switzerland: String,
    pub syria: String,
    pub tahiti: String,
    pub taiwan: String,
    pub tajikistan: String,
    pub tanzania: String,
    pub thailand: String,
    pub thegambia: String,
    pub togo: String,
    pub tonga: String,
    pub transnistria: String,
    pub trinidadandtobago: String,
    pub tunisia: String,
    pub turkey: String,
    pub turkmenistan: String,
    pub tuvalu: String,
    pub uae: String,
    pub uk: String,
    pub ukraine: String,
    pub uganda: String,
    pub uruguay: String,
    pub usa: String,
    pub ussr: String,
    pub usvirginislands: String,
    pub uzbekistan: String,
    pub vanuatu: String,
    pub venezuela: String,
    pub vietnam: String,
    pub wales: String,
    pub wallisandfutuna: String,
    pub westgermany: String,
    pub yemen: String,
    pub yugoslavia: String,
    pub zambia: String,
    pub zimbabwe: String,
}

#[derive(Serialize, Deserialize)]
pub struct PageTitleTranslations {
    pub rankings: String,
    pub records: String,
    pub meets: String,
}

#[derive(Serialize, Deserialize)]
pub struct HTMLHeaderTranslations {
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct HeaderTranslations {
    pub rankings: String,
    pub records: String,
    pub meets: String,
    pub data: String,
    pub apps: String,
    pub status: String,
    pub faq: String,
    pub contact: String,
    pub shop: String,
    pub supportus: String,
}

#[derive(Serialize, Deserialize)]
pub struct ColumnTranslations {
    pub place: String,
    pub formulaplace: String,
    pub liftername: String,
    pub federation: String,
    pub date: String,
    pub location: String,
    pub home: String,
    pub meetname: String,
    pub division: String,
    pub sex: String,
    pub age: String,
    pub equipment: String,
    pub weightclass: String,
    pub bodyweight: String,
    pub squat: String,
    pub bench: String,
    pub deadlift: String,
    pub total: String,
    pub wilks: String,
    pub wilks2020: String,
    pub mcculloch: String,
    pub glossbrenner: String,
    pub ipfpoints: String,
    pub dots: String,
    pub goodlift: String,
    pub num_lifters: String,
}

#[derive(Serialize, Deserialize)]
pub struct ButtonTranslations {
    pub search: String,
}

#[derive(Serialize, Deserialize)]
pub struct LabelTranslations {
    pub sort: String,
    pub category: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct EquipmentSelectorTranslations {
    pub raw: String,
    pub wraps: String,
    pub raw_wraps: String,
    pub single: String,
    pub multi: String,
    pub unlimited: String,
}

#[derive(Serialize, Deserialize)]
pub struct WeightClassSelectorTranslations {
    pub all: String,
    pub traditional: String,
    pub expanded: String,
    pub ipfmen: String,
    pub ipfwomen: String,
    pub para_men: String,
    pub para_women: String,
    pub wp_men: String,
    pub wp_women: String,
}

#[derive(Serialize, Deserialize)]
pub struct SortSelectorTranslations {
    pub by_squat: String,
    pub by_bench: String,
    pub by_deadlift: String,
    pub by_total: String,
    pub by_ah: String,
    pub by_allometric: String,
    pub by_dots: String,
    pub by_glossbrenner: String,
    pub by_goodlift: String,
    pub by_ipfpoints: String,
    pub by_mcculloch: String,
    pub by_nasa: String,
    pub by_reshel: String,
    pub by_schwartzmalone: String,
    pub by_wilks: String,
    pub by_wilks2020: String,
    pub by_division: String,
    pub weight: String,
    pub points: String,
}

#[derive(Serialize, Deserialize)]
pub struct YearSelectorTranslations {
    pub all: String,
}

#[derive(Serialize, Deserialize)]
pub struct SexSelectorTranslations {
    pub all: String,
    pub m: String,
    pub f: String,
}

#[derive(Serialize, Deserialize)]
pub struct EventSelectorTranslations {
    pub all: String,
    pub full_power: String,
    pub push_pull: String,
    pub squat_only: String,
    pub bench_only: String,
    pub deadlift_only: String,
}

#[derive(Serialize, Deserialize)]
pub struct FedSelectorTranslations {
    pub all: String,
    pub fully_tested: String,
    pub all_tested: String,
    pub international: String,
    pub regional: String,
    pub continents: String,
    pub countries: String,
    pub all_usa: String,
    pub all_argentina: String,
    pub all_australia: String,
    pub all_austria: String,
    pub all_azerbaijan: String,
    pub all_belarus: String,
    pub all_belgium: String,
    pub all_bosniaandherzegovina: String,
    pub all_brazil: String,
    pub all_canada: String,
    pub all_chile: String,
    pub all_china: String,
    pub all_colombia: String,
    pub all_croatia: String,
    pub all_czechia: String,
    pub all_denmark: String,
    pub all_estonia: String,
    pub all_finland: String,
    pub all_france: String,
    pub all_georgia: String,
    pub all_germany: String,
    pub all_greece: String,
    pub all_hongkong: String,
    pub all_hungary: String,
    pub all_iceland: String,
    pub all_india: String,
    pub all_indonesia: String,
    pub all_iran: String,
    pub all_ireland: String,
    pub all_israel: String,
    pub all_italy: String,
    pub all_japan: String,
    pub all_kazakhstan: String,
    pub all_kuwait: String,
    pub all_kyrgyzstan: String,
    pub all_latvia: String,
    pub all_lithuania: String,
    pub all_malaysia: String,
    pub all_mexico: String,
    pub all_moldova: String,
    pub all_nauru: String,
    pub all_netherlands: String,
    pub all_newzealand: String,
    pub all_niue: String,
    pub all_norway: String,
    pub all_papuanewguinea: String,
    pub all_paraguay: String,
    pub all_philippines: String,
    pub all_poland: String,
    pub all_portugal: String,
    pub all_romania: String,
    pub all_russia: String,
    pub all_scotland: String,
    pub all_serbia: String,
    pub all_singapore: String,
    pub all_slovakia: String,
    pub all_slovenia: String,
    pub all_spain: String,
    pub all_southafrica: String,
    pub all_sweden: String,
    pub all_switzerland: String,
    pub all_thailand: String,
    pub all_turkey: String,
    pub all_uganda: String,
    pub all_uk: String,
    pub all_uk_tested: String,
    pub all_ukraine: String,
    pub all_usvirginislands: String,
    pub all_vietnam: String,
    pub all_affiliates: String,
    pub all_internationals: String,
}

#[derive(Serialize, Deserialize)]
pub struct AgeClassSelectorTranslations {
    pub all: String,
    pub youth5_12: String,
    pub teen13_15: String,
    pub teen16_17: String,
    pub teen18_19: String,
    pub juniors20_23: String,
    pub seniors24_34: String,
    pub submasters35_39: String,
    pub masters40_44: String,
    pub masters45_49: String,
    pub masters50_54: String,
    pub masters55_59: String,
    pub masters60_64: String,
    pub masters65_69: String,
    pub masters70_74: String,
    pub masters75_79: String,
    pub masters80p: String,
    pub masters40_49: String,
    pub masters50_59: String,
    pub masters60_69: String,
    pub masters70_79: String,
    pub label_masters_by_5s: String,
    pub label_masters_by_10s: String,
    pub ipf_open: String,
    pub ipf_subjunior: String,
    pub ipf_junior: String,
    pub ipf_senior: String,
    pub ipf_master1: String,
    pub ipf_master2: String,
    pub ipf_master3: String,
    pub ipf_master4: String,
}

#[derive(Serialize, Deserialize)]
pub struct LifterPageTranslations {
    pub personal_bests: String,
    pub competition_results: String,
    pub download_as_csv: String,
}

#[derive(Serialize, Deserialize)]
pub struct Translations {
    pub units: UnitsTranslations,
    pub equipment: EquipmentTranslations,
    pub sex: SexTranslations,
    pub page_titles: PageTitleTranslations,
    pub header: HeaderTranslations,
    pub html_header: HTMLHeaderTranslations,
    pub columns: ColumnTranslations,
    pub country: CountryTranslations,
    pub buttons: ButtonTranslations,
    pub labels: LabelTranslations,
    pub selectors: SelectorTranslations,
    pub lifter_page: LifterPageTranslations,
}

/// Owner struct of all translation state.
pub struct LangInfo {
    cz: Translations,
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

impl Default for LangInfo {
    /// Constructs a new [LangInfo].
    ///
    /// The translations are hardcoded as strings at compile time, but parsing the strings
    /// happens at runtime. A test ensures this succeeds.
    ///
    /// TODO: Use a build.rs to also parse at compile time.
    fn default() -> LangInfo {
        LangInfo {
            cz: json::from_str(include_str!("../translations/cz.json")).expect("cz"),
            de: json::from_str(include_str!("../translations/de.json")).expect("de"),
            el: json::from_str(include_str!("../translations/el.json")).expect("el"),
            en: json::from_str(include_str!("../translations/en.json")).expect("en"),
            eo: json::from_str(include_str!("../translations/eo.json")).expect("eo"),
            es: json::from_str(include_str!("../translations/es.json")).expect("es"),
            fi: json::from_str(include_str!("../translations/fi.json")).expect("fi"),
            fr: json::from_str(include_str!("../translations/fr.json")).expect("fr"),
            hr: json::from_str(include_str!("../translations/hr.json")).expect("hr"),
            hu: json::from_str(include_str!("../translations/hu.json")).expect("hu"),
            it: json::from_str(include_str!("../translations/it.json")).expect("it"),
            ja: json::from_str(include_str!("../translations/ja.json")).expect("ja"),
            ko: json::from_str(include_str!("../translations/ko.json")).expect("ko"),
            lt: json::from_str(include_str!("../translations/lt.json")).expect("lt"),
            nl: json::from_str(include_str!("../translations/nl.json")).expect("nl"),
            pl: json::from_str(include_str!("../translations/pl.json")).expect("pl"),
            pt: json::from_str(include_str!("../translations/pt.json")).expect("pt"),
            sk: json::from_str(include_str!("../translations/sk.json")).expect("sk"),
            sl: json::from_str(include_str!("../translations/sl.json")).expect("sl"),
            sr: json::from_str(include_str!("../translations/sr.json")).expect("sr"),
            sv: json::from_str(include_str!("../translations/sv.json")).expect("sv"),
            ru: json::from_str(include_str!("../translations/ru.json")).expect("ru"),
            tr: json::from_str(include_str!("../translations/tr.json")).expect("tr"),
            uk: json::from_str(include_str!("../translations/uk.json")).expect("uk"),
            vi: json::from_str(include_str!("../translations/vi.json")).expect("vi"),
            zh_hant: json::from_str(include_str!("../translations/zh-Hant.json")).expect("zh_hant"),
            zh_hans: json::from_str(include_str!("../translations/zh-Hans.json")).expect("zh_hans"),
        }
    }
}

impl LangInfo {
    pub fn translations(&self, language: Language) -> &Translations {
        match language {
            Language::cz => &self.cz,
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
            Equipment::Raw => &self.equipment.raw,
            Equipment::Wraps => &self.equipment.wraps,
            Equipment::Single => &self.equipment.single,
            Equipment::Multi => &self.equipment.multi,
            Equipment::Unlimited => &self.equipment.unlimited,
            Equipment::Straps => &self.equipment.straps,
        }
    }

    pub fn translate_sex(&self, sex: Sex) -> &str {
        match sex {
            Sex::M => &self.sex.m,
            Sex::F => &self.sex.f,
            Sex::Mx => &self.sex.mx,
        }
    }

    pub fn translate_country(&self, country: Country) -> &str {
        match country {
            Country::Abkhazia => &self.country.abkhazia,
            Country::Afghanistan => &self.country.afghanistan,
            Country::Albania => &self.country.albania,
            Country::Algeria => &self.country.algeria,
            Country::AmericanSamoa => &self.country.americansamoa,
            Country::Angola => &self.country.angola,
            Country::Argentina => &self.country.argentina,
            Country::Aruba => &self.country.aruba,
            Country::Armenia => &self.country.armenia,
            Country::Austria => &self.country.austria,
            Country::Australia => &self.country.australia,
            Country::Azerbaijan => &self.country.azerbaijan,
            Country::Bahamas => &self.country.bahamas,
            Country::Bahrain => &self.country.bahrain,
            Country::Bangladesh => &self.country.bangladesh,
            Country::Belarus => &self.country.belarus,
            Country::Belgium => &self.country.belgium,
            Country::Belize => &self.country.belize,
            Country::Benin => &self.country.benin,
            Country::Bolivia => &self.country.bolivia,
            Country::BosniaAndHerzegovina => &self.country.bosniaandherzegovina,
            Country::Botswana => &self.country.botswana,
            Country::Brazil => &self.country.brazil,
            Country::BritishVirginIslands => &self.country.britishvirginislands,
            Country::Brunei => &self.country.brunei,
            Country::Bulgaria => &self.country.bulgaria,
            Country::BurkinaFaso => &self.country.burkinafaso,
            Country::CaboVerde => &self.country.caboverde,
            Country::Cambodia => &self.country.cambodia,
            Country::Cameroon => &self.country.cameroon,
            Country::Canada => &self.country.canada,
            Country::CaymanIslands => &self.country.caymanislands,
            Country::CentralAfricanRepublic => &self.country.centralafricanrepublic,
            Country::Chile => &self.country.chile,
            Country::China => &self.country.china,
            Country::Colombia => &self.country.colombia,
            Country::Comoros => &self.country.comoros,
            Country::Congo => &self.country.congo,
            Country::CookIslands => &self.country.cookislands,
            Country::CostaRica => &self.country.costarica,
            Country::Croatia => &self.country.croatia,
            Country::Cuba => &self.country.cuba,
            Country::Cyprus => &self.country.cyprus,
            Country::Czechia => &self.country.czechia,
            Country::Czechoslovakia => &self.country.czechoslovakia,
            Country::Denmark => &self.country.denmark,
            Country::Djibouti => &self.country.djibouti,
            Country::DominicanRepublic => &self.country.dominicanrepublic,
            Country::EastGermany => &self.country.eastgermany,
            Country::EastTimor => &self.country.easttimor,
            Country::Ecuador => &self.country.ecuador,
            Country::Egypt => &self.country.egypt,
            Country::ElSalvador => &self.country.elsalvador,
            Country::England => &self.country.england,
            Country::Estonia => &self.country.estonia,
            Country::Eswatini => &self.country.eswatini,
            Country::Ethiopia => &self.country.ethiopia,
            Country::Fiji => &self.country.fiji,
            Country::Finland => &self.country.finland,
            Country::France => &self.country.france,
            Country::Gabon => &self.country.gabon,
            Country::Georgia => &self.country.georgia,
            Country::Germany => &self.country.germany,
            Country::Ghana => &self.country.ghana,
            Country::Gibraltar => &self.country.gibraltar,
            Country::Greece => &self.country.greece,
            Country::Guatemala => &self.country.guatemala,
            Country::Guinea => &self.country.guinea,
            Country::GuineaBissau => &self.country.guineabissau,
            Country::Guyana => &self.country.guyana,
            Country::Haiti => &self.country.haiti,
            Country::Honduras => &self.country.honduras,
            Country::HongKong => &self.country.hongkong,
            Country::Hungary => &self.country.hungary,
            Country::Iceland => &self.country.iceland,
            Country::India => &self.country.india,
            Country::Indonesia => &self.country.indonesia,
            Country::Ireland => &self.country.ireland,
            Country::Israel => &self.country.israel,
            Country::Italy => &self.country.italy,
            Country::Iran => &self.country.iran,
            Country::Iraq => &self.country.iraq,
            Country::IvoryCoast => &self.country.ivorycoast,
            Country::Japan => &self.country.japan,
            Country::Jamaica => &self.country.jamaica,
            Country::Jordan => &self.country.jordan,
            Country::Kazakhstan => &self.country.kazakhstan,
            Country::Kenya => &self.country.kenya,
            Country::Kiribati => &self.country.kiribati,
            Country::Kuwait => &self.country.kuwait,
            Country::Kyrgyzstan => &self.country.kyrgyzstan,
            Country::Laos => &self.country.laos,
            Country::Latvia => &self.country.latvia,
            Country::Lebanon => &self.country.lebanon,
            Country::Lesotho => &self.country.lesotho,
            Country::Liberia => &self.country.liberia,
            Country::Libya => &self.country.libya,
            Country::Lithuania => &self.country.lithuania,
            Country::Luxembourg => &self.country.luxembourg,
            Country::Malaysia => &self.country.malaysia,
            Country::Mali => &self.country.mali,
            Country::Malta => &self.country.malta,
            Country::MarshallIslands => &self.country.marshallislands,
            Country::Mauritania => &self.country.mauritania,
            Country::Mauritius => &self.country.mauritius,
            Country::Mexico => &self.country.mexico,
            Country::Moldova => &self.country.moldova,
            Country::Monaco => &self.country.monaco,
            Country::Mongolia => &self.country.mongolia,
            Country::Montenegro => &self.country.montenegro,
            Country::Morocco => &self.country.morocco,
            Country::Myanmar => &self.country.myanmar,
            Country::Namibia => &self.country.namibia,
            Country::Nauru => &self.country.nauru,
            Country::Nepal => &self.country.nepal,
            Country::Netherlands => &self.country.netherlands,
            Country::NetherlandsAntilles => &self.country.netherlandsantilles,
            Country::NewCaledonia => &self.country.newcaledonia,
            Country::NewZealand => &self.country.newzealand,
            Country::Nicaragua => &self.country.nicaragua,
            Country::Niger => &self.country.niger,
            Country::Nigeria => &self.country.nigeria,
            Country::Niue => &self.country.niue,
            Country::NorthMacedonia => &self.country.northmacedonia,
            Country::Norway => &self.country.norway,
            Country::NorthernIreland => &self.country.northernireland,
            Country::Oman => &self.country.oman,
            Country::Pakistan => &self.country.pakistan,
            Country::Palestine => &self.country.palestine,
            Country::Panama => &self.country.panama,
            Country::PapuaNewGuinea => &self.country.papuanewguinea,
            Country::Paraguay => &self.country.paraguay,
            Country::Peru => &self.country.peru,
            Country::Philippines => &self.country.philippines,
            Country::Poland => &self.country.poland,
            Country::Portugal => &self.country.portugal,
            Country::PuertoRico => &self.country.puertorico,
            Country::Qatar => &self.country.qatar,
            Country::Rhodesia => &self.country.rhodesia,
            Country::Romania => &self.country.romania,
            Country::Russia => &self.country.russia,
            Country::Rwanda => &self.country.rwanda,
            Country::Samoa => &self.country.samoa,
            Country::SaudiArabia => &self.country.saudiarabia,
            Country::Scotland => &self.country.scotland,
            Country::Senegal => &self.country.senegal,
            Country::Serbia => &self.country.serbia,
            Country::SerbiaAndMontenegro => &self.country.serbiaandmontenegro,
            Country::SierraLeone => &self.country.sierraleone,
            Country::Singapore => &self.country.singapore,
            Country::Slovakia => &self.country.slovakia,
            Country::Slovenia => &self.country.slovenia,
            Country::SolomonIslands => &self.country.solomonislands,
            Country::SouthAfrica => &self.country.southafrica,
            Country::SouthKorea => &self.country.southkorea,
            Country::Spain => &self.country.spain,
            Country::SriLanka => &self.country.srilanka,
            Country::Sudan => &self.country.sudan,
            Country::Sweden => &self.country.sweden,
            Country::Switzerland => &self.country.switzerland,
            Country::Syria => &self.country.syria,
            Country::Tahiti => &self.country.tahiti,
            Country::Taiwan => &self.country.taiwan,
            Country::Tajikistan => &self.country.tajikistan,
            Country::Tanzania => &self.country.tanzania,
            Country::Thailand => &self.country.thailand,
            Country::TheGambia => &self.country.thegambia,
            Country::Togo => &self.country.togo,
            Country::Tonga => &self.country.tonga,
            Country::Transnistria => &self.country.transnistria,
            Country::TrinidadAndTobago => &self.country.trinidadandtobago,
            Country::Tunisia => &self.country.tunisia,
            Country::Turkey => &self.country.turkey,
            Country::Turkmenistan => &self.country.turkmenistan,
            Country::Tuvalu => &self.country.tuvalu,
            Country::UAE => &self.country.uae,
            Country::Uganda => &self.country.uganda,
            Country::UK => &self.country.uk,
            Country::Ukraine => &self.country.ukraine,
            Country::Uruguay => &self.country.uruguay,
            Country::USA => &self.country.usa,
            Country::USSR => &self.country.ussr,
            Country::USVirginIslands => &self.country.usvirginislands,
            Country::Uzbekistan => &self.country.uzbekistan,
            Country::Vanuatu => &self.country.vanuatu,
            Country::Venezuela => &self.country.venezuela,
            Country::Vietnam => &self.country.vietnam,
            Country::Wales => &self.country.wales,
            Country::WallisAndFutuna => &self.country.wallisandfutuna,
            Country::WestGermany => &self.country.westgermany,
            Country::Yemen => &self.country.yemen,
            Country::Yugoslavia => &self.country.yugoslavia,
            Country::Zambia => &self.country.zambia,
            Country::Zimbabwe => &self.country.zimbabwe,
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
            Language::cz => NumberFormat::ArabicComma,
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
                let s = format!("{}º", n);
                serializer.serialize_str(&s)
            }
            Self::RomanceFeminine(n) => {
                let s = format!("{}ª", n);
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
        Language::el => lifter.greek_name.as_ref().unwrap_or(&lifter.name),
        Language::ja => lifter.japanese_name.as_ref().unwrap_or(&lifter.name),
        Language::ko => lifter.korean_name.as_ref().unwrap_or(&lifter.name),
        Language::ru | Language::uk => lifter.cyrillic_name.as_ref().unwrap_or(&lifter.name),

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translation_validity() {
        // This will panic if the translation files fail parsing.
        LangInfo::default();
    }
}
