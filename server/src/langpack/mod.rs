//! Internationalization facilities.

use serde;
use serde::ser::Serialize;
use serde_json;

use std::error::Error;

use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use strum::IntoEnumIterator;

use opldb;
use opldb::fields;

/// List of languages accepted by the project, in ISO 639-1 code.
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, EnumIter, EnumString, Serialize)]
pub enum Language {
    /// German, without regional variance.
    de,
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
    /// Italian.
    it,
    /// Polish.
    pl,
    /// Portuguese.
    pt,
    /// Slovenian.
    sl,
    /// Russian.
    ru,
    /// Turkish.
    tr,
    /// Vietnamese.
    vi,
    /// Chinese, written in Traditional Chinese script.
    #[serde(rename = "zh-Hant")]
    #[strum(to_string = "zh-Hant")]
    zh_hant,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::de => "de",
                Language::en => "en",
                Language::eo => "eo",
                Language::es => "es",
                Language::fi => "fi",
                Language::fr => "fr",
                Language::it => "it",
                Language::pl => "pl",
                Language::pt => "pt",
                Language::sl => "sl",
                Language::ru => "ru",
                Language::tr => "tr",
                Language::vi => "vi",
                Language::zh_hant => "zh-Hant",
            }
        )
    }
}

impl Language {
    /// Returns the units associated with the language.
    pub fn default_units(self) -> opldb::WeightUnits {
        match self {
            Language::en => opldb::WeightUnits::Lbs,
            _ => opldb::WeightUnits::Kg,
        }
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
    pub units: opldb::WeightUnits,
}

impl<'a> Locale<'a> {
    pub fn new(
        langinfo: &'a LangInfo,
        language: Language,
        units: opldb::WeightUnits,
    ) -> Locale<'a> {
        Locale {
            langinfo,
            language,
            strings: langinfo.get_translations(language),
            number_format: language.number_format(),
            units,
        }
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
    pub straps: String,
}

#[derive(Serialize, Deserialize)]
pub struct SexTranslations {
    pub m: String,
    pub f: String,
}

#[derive(Serialize, Deserialize)]
pub struct CountryTranslations {
    pub algeria: String,
    pub argentina: String,
    pub aruba: String,
    pub australia: String,
    pub austria: String,
    pub azerbaijan: String,
    pub belarus: String,
    pub belgium: String,
    pub brazil: String,
    pub britain: String,
    pub britishvirginislands: String,
    pub bulgaria: String,
    pub canada: String,
    pub caymanislands: String,
    pub colombia: String,
    pub costarica: String,
    pub cotedivoire: String,
    pub czechia: String,
    pub denmark: String,
    pub ecuador: String,
    pub egypt: String,
    pub elsalvador: String,
    pub england: String,
    pub estonia: String,
    pub fiji: String,
    pub finland: String,
    pub france: String,
    pub germany: String,
    pub greece: String,
    pub guatemala: String,
    pub guyana: String,
    pub hongkong: String,
    pub hungary: String,
    pub iceland: String,
    pub india: String,
    pub indonesia: String,
    pub ireland: String,
    pub israel: String,
    pub italy: String,
    pub iran: String,
    pub japan: String,
    pub kazakhstan: String,
    pub latvia: String,
    pub lithuania: String,
    pub luxembourg: String,
    pub malaysia: String,
    pub mexico: String,
    pub mongolia: String,
    pub morocco: String,
    pub netherlands: String,
    pub newcaledonia: String,
    pub newzealand: String,
    pub nicaragua: String,
    pub norway: String,
    pub northernireland: String,
    pub oman: String,
    pub papuanewguinea: String,
    pub peru: String,
    pub philippines: String,
    pub poland: String,
    pub portugal: String,
    pub puertorico: String,
    pub russia: String,
    pub samoa: String,
    pub scotland: String,
    pub serbia: String,
    pub singapore: String,
    pub slovakia: String,
    pub slovenia: String,
    pub southafrica: String,
    pub southkorea: String,
    pub spain: String,
    pub sweden: String,
    pub switzerland: String,
    pub tahiti: String,
    pub taiwan: String,
    pub turkey: String,
    pub uae: String,
    pub uk: String,
    pub ukraine: String,
    pub uruguay: String,
    pub usa: String,
    pub usvirginislands: String,
    pub uzbekistan: String,
    pub venezuela: String,
    pub vietnam: String,
    pub wales: String,
}

#[derive(Serialize, Deserialize)]
pub struct HeaderTranslations {
    pub rankings: String,
    pub meets: String,
    pub data: String,
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
    pub mcculloch: String,
    pub num_lifters: String,
}

#[derive(Serialize, Deserialize)]
pub struct ButtonTranslations {
    pub search: String,
}

#[derive(Serialize, Deserialize)]
pub struct SelectorTranslations {
    pub equipment: EquipmentSelectorTranslations,
    pub weightclass: WeightClassSelectorTranslations,
    pub sort: SortSelectorTranslations,
    pub year: YearSelectorTranslations,
    pub sex: SexSelectorTranslations,
    pub fed: FedSelectorTranslations,
}

#[derive(Serialize, Deserialize)]
pub struct EquipmentSelectorTranslations {
    pub raw: String,
    pub wraps: String,
    pub raw_wraps: String,
    pub single: String,
    pub multi: String,
}

#[derive(Serialize, Deserialize)]
pub struct WeightClassSelectorTranslations {
    pub all: String,
    pub traditional: String,
    pub ipfmen: String,
    pub ipfwomen: String,
}

#[derive(Serialize, Deserialize)]
pub struct SortSelectorTranslations {
    pub by_squat: String,
    pub by_bench: String,
    pub by_deadlift: String,
    pub by_total: String,
    pub by_allometric: String,
    pub by_glossbrenner: String,
    pub by_mcculloch: String,
    pub by_wilks: String,
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
pub struct FedSelectorTranslations {
    pub all: String,
    pub all_tested: String,
    pub all_amateur: String,
    pub international: String,
    pub regional: String,
    pub all_usa: String,
    pub all_argentina: String,
    pub all_australia: String,
    pub all_canada: String,
    pub all_czechia: String,
    pub all_finland: String,
    pub all_germany: String,
    pub all_ireland: String,
    pub all_israel: String,
    pub all_russia: String,
    pub all_uk: String,
    pub all_ukraine: String,
}

#[derive(Serialize, Deserialize)]
pub struct LifterPageTranslations {
    pub personal_bests: String,
    pub competition_results: String,
}

#[derive(Serialize, Deserialize)]
pub struct Translations {
    pub units: UnitsTranslations,
    pub equipment: EquipmentTranslations,
    pub sex: SexTranslations,
    pub header: HeaderTranslations,
    pub columns: ColumnTranslations,
    pub country: CountryTranslations,
    pub buttons: ButtonTranslations,
    pub selectors: SelectorTranslations,
    pub lifter_page: LifterPageTranslations,
}

/// Owner struct of all translation state.
#[derive(Default)]
pub struct LangInfo {
    de: Option<Translations>,
    en: Option<Translations>,
    eo: Option<Translations>,
    es: Option<Translations>,
    fi: Option<Translations>,
    fr: Option<Translations>,
    it: Option<Translations>,
    pl: Option<Translations>,
    pt: Option<Translations>,
    sl: Option<Translations>,
    ru: Option<Translations>,
    tr: Option<Translations>,
    vi: Option<Translations>,
    zh_hant: Option<Translations>,
}

impl LangInfo {
    pub fn load_translations(
        &mut self,
        language: Language,
        filename: &str,
    ) -> Result<(), Box<Error>> {
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let trans = serde_json::from_str(&contents)?;

        match language {
            Language::de => self.de = trans,
            Language::en => self.en = trans,
            Language::eo => self.eo = trans,
            Language::es => self.es = trans,
            Language::fi => self.fi = trans,
            Language::fr => self.fr = trans,
            Language::it => self.it = trans,
            Language::pl => self.pl = trans,
            Language::pt => self.pt = trans,
            Language::sl => self.sl = trans,
            Language::ru => self.ru = trans,
            Language::tr => self.tr = trans,
            Language::vi => self.vi = trans,
            Language::zh_hant => self.zh_hant = trans,
        };

        Ok(())
    }

    pub fn get_translations(&self, language: Language) -> &Translations {
        match language {
            Language::de => self.de.as_ref().unwrap(),
            Language::en => self.en.as_ref().unwrap(),
            Language::eo => self.eo.as_ref().unwrap(),
            Language::es => self.es.as_ref().unwrap(),
            Language::fi => self.fi.as_ref().unwrap(),
            Language::fr => self.fr.as_ref().unwrap(),
            Language::it => self.it.as_ref().unwrap(),
            Language::pl => self.pl.as_ref().unwrap(),
            Language::pt => self.pt.as_ref().unwrap(),
            Language::sl => self.sl.as_ref().unwrap(),
            Language::ru => self.ru.as_ref().unwrap(),
            Language::tr => self.tr.as_ref().unwrap(),
            Language::vi => self.vi.as_ref().unwrap(),
            Language::zh_hant => self.zh_hant.as_ref().unwrap(),
        }
    }
}

impl Translations {
    pub fn translate_equipment(&self, equip: fields::Equipment) -> &str {
        match equip {
            fields::Equipment::Raw => &self.equipment.raw,
            fields::Equipment::Wraps => &self.equipment.wraps,
            fields::Equipment::Single => &self.equipment.single,
            fields::Equipment::Multi => &self.equipment.multi,
            fields::Equipment::Straps => &self.equipment.straps,
        }
    }

    pub fn translate_sex(&self, sex: fields::Sex) -> &str {
        match sex {
            fields::Sex::M => &self.sex.m,
            fields::Sex::F => &self.sex.f,
        }
    }

    pub fn translate_country(&self, country: fields::Country) -> &str {
        match country {
            fields::Country::Algeria => &self.country.algeria,
            fields::Country::Argentina => &self.country.argentina,
            fields::Country::Aruba => &self.country.aruba,
            fields::Country::Austria => &self.country.austria,
            fields::Country::Australia => &self.country.australia,
            fields::Country::Azerbaijan => &self.country.azerbaijan,
            fields::Country::Belarus => &self.country.belarus,
            fields::Country::Belgium => &self.country.belgium,
            fields::Country::Brazil => &self.country.brazil,
            fields::Country::Britain => &self.country.britain,
            fields::Country::BritishVirginIslands => &self.country.britishvirginislands,
            fields::Country::Bulgaria => &self.country.bulgaria,
            fields::Country::Canada => &self.country.canada,
            fields::Country::CaymanIslands => &self.country.caymanislands,
            fields::Country::Colombia => &self.country.colombia,
            fields::Country::CostaRica => &self.country.costarica,
            fields::Country::CoteDIvoire => &self.country.cotedivoire,
            fields::Country::Czechia => &self.country.czechia,
            fields::Country::Denmark => &self.country.denmark,
            fields::Country::Ecuador => &self.country.ecuador,
            fields::Country::Egypt => &self.country.egypt,
            fields::Country::ElSalvador => &self.country.elsalvador,
            fields::Country::England => &self.country.england,
            fields::Country::Estonia => &self.country.estonia,
            fields::Country::Fiji => &self.country.fiji,
            fields::Country::Finland => &self.country.finland,
            fields::Country::France => &self.country.france,
            fields::Country::Germany => &self.country.germany,
            fields::Country::Greece => &self.country.greece,
            fields::Country::Guatemala => &self.country.guatemala,
            fields::Country::Guyana => &self.country.guyana,
            fields::Country::HongKong => &self.country.hongkong,
            fields::Country::Hungary => &self.country.hungary,
            fields::Country::Iceland => &self.country.iceland,
            fields::Country::India => &self.country.india,
            fields::Country::Indonesia => &self.country.indonesia,
            fields::Country::Ireland => &self.country.ireland,
            fields::Country::Israel => &self.country.israel,
            fields::Country::Italy => &self.country.italy,
            fields::Country::Iran => &self.country.iran,
            fields::Country::Japan => &self.country.japan,
            fields::Country::Kazakhstan => &self.country.kazakhstan,
            fields::Country::Latvia => &self.country.latvia,
            fields::Country::Lithuania => &self.country.lithuania,
            fields::Country::Luxembourg => &self.country.luxembourg,
            fields::Country::Malaysia => &self.country.malaysia,
            fields::Country::Mexico => &self.country.mexico,
            fields::Country::Mongolia => &self.country.mongolia,
            fields::Country::Morocco => &self.country.morocco,
            fields::Country::Netherlands => &self.country.netherlands,
            fields::Country::NewCaledonia => &self.country.newcaledonia,
            fields::Country::NewZealand => &self.country.newzealand,
            fields::Country::Nicaragua => &self.country.nicaragua,
            fields::Country::Norway => &self.country.norway,
            fields::Country::NorthernIreland => &self.country.northernireland,
            fields::Country::Oman => &self.country.oman,
            fields::Country::PapuaNewGuinea => &self.country.papuanewguinea,
            fields::Country::Peru => &self.country.peru,
            fields::Country::Philippines => &self.country.philippines,
            fields::Country::Poland => &self.country.poland,
            fields::Country::Portugal => &self.country.portugal,
            fields::Country::PuertoRico => &self.country.puertorico,
            fields::Country::Russia => &self.country.russia,
            fields::Country::Samoa => &self.country.samoa,
            fields::Country::Scotland => &self.country.scotland,
            fields::Country::Serbia => &self.country.serbia,
            fields::Country::Singapore => &self.country.singapore,
            fields::Country::Slovakia => &self.country.slovakia,
            fields::Country::Slovenia => &self.country.slovenia,
            fields::Country::SouthAfrica => &self.country.southafrica,
            fields::Country::SouthKorea => &self.country.southkorea,
            fields::Country::Spain => &self.country.spain,
            fields::Country::Sweden => &self.country.sweden,
            fields::Country::Switzerland => &self.country.switzerland,
            fields::Country::Tahiti => &self.country.tahiti,
            fields::Country::Taiwan => &self.country.taiwan,
            fields::Country::Turkey => &self.country.turkey,
            fields::Country::UAE => &self.country.uae,
            fields::Country::UK => &self.country.uk,
            fields::Country::Ukraine => &self.country.ukraine,
            fields::Country::Uruguay => &self.country.uruguay,
            fields::Country::USA => &self.country.usa,
            fields::Country::USVirginIslands => &self.country.usvirginislands,
            fields::Country::Uzbekistan => &self.country.uzbekistan,
            fields::Country::Venezuela => &self.country.venezuela,
            fields::Country::Vietnam => &self.country.vietnam,
            fields::Country::Wales => &self.country.wales,
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
            Language::de => NumberFormat::ArabicComma,
            Language::en => NumberFormat::ArabicPeriod,
            Language::eo => NumberFormat::ArabicComma,
            Language::es => NumberFormat::ArabicPeriod, // TODO: Only Central America.
            Language::fi => NumberFormat::ArabicComma,
            Language::fr => NumberFormat::ArabicComma,
            Language::it => NumberFormat::ArabicComma,
            Language::pl => NumberFormat::ArabicComma,
            Language::pt => NumberFormat::ArabicComma,
            Language::sl => NumberFormat::ArabicComma,
            Language::ru => NumberFormat::ArabicComma,
            Language::tr => NumberFormat::ArabicComma,
            Language::vi => NumberFormat::ArabicComma,
            Language::zh_hant => NumberFormat::ArabicComma,
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
    pub weight: fields::WeightAny,
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
    pub points: fields::Points,
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

/// Type that gets serialized into a localized `WeightClassAny`.
#[derive(Copy, Clone)]
pub struct LocalizedWeightClassAny {
    pub format: NumberFormat,
    pub class: fields::WeightClassAny,
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

/// Gets the appropriat
pub fn get_localized_name(
    lifter: &opldb::Lifter,
    language: Language,
) -> &str {
    match language {
        Language::de => &lifter.name,
        Language::en => &lifter.name,
        Language::eo => &lifter.name,
        Language::es => &lifter.name,
        Language::fi => &lifter.name,
        Language::fr => &lifter.name,
        Language::it => &lifter.name,
        Language::pl => &lifter.name,
        Language::pt => &lifter.name,
        Language::sl => &lifter.name,
        Language::ru => {
            if let Some(ref cyr) = lifter.cyrillic_name {
                cyr
            } else {
                &lifter.name
            }
        }
        Language::tr => &lifter.name,
        Language::vi => &lifter.name,
        Language::zh_hant => &lifter.name,
    }
}
