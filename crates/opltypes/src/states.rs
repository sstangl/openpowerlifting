//! Data types for the MeetState column.

use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;
use strum::{IntoEnumIterator, ParseError};

use std::fmt;

use crate::Country;

/// The State column.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    InArgentina(ArgentinaState),
    InAustralia(AustraliaState),
    InBrazil(BrazilState),
    InCanada(CanadaState),
    InChile(ChileState),
    InChina(ChinaState),
    InEngland(EnglandState),
    InGermany(GermanyState),
    InGreece(GreeceState),
    InIndia(IndiaState),
    InMexico(MexicoState),
    InNetherlands(NetherlandsState),
    InNewZealand(NewZealandState),
    InRomania(RomaniaState),
    InRussia(RussiaState),
    InSouthAfrica(SouthAfricaState),
    InUAE(UAEState),
    InUSA(USAState),
}

impl State {
    /// Constructs a State for a specific Country.
    ///
    /// This is how the checker interprets the State column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Country;
    /// # use opltypes::states::{State, USAState};
    /// let state = State::from_str_and_country("NY", Country::USA).unwrap();
    /// assert_eq!(state, State::InUSA(USAState::NY));
    /// ```
    pub fn from_str_and_country(s: &str, country: Country) -> Result<State, ParseError> {
        match country {
            Country::Argentina => Ok(State::InArgentina(s.parse::<ArgentinaState>()?)),
            Country::Australia => Ok(State::InAustralia(s.parse::<AustraliaState>()?)),
            Country::Brazil => Ok(State::InBrazil(s.parse::<BrazilState>()?)),
            Country::Canada => Ok(State::InCanada(s.parse::<CanadaState>()?)),
            Country::Chile => Ok(State::InChile(s.parse::<ChileState>()?)),
            Country::China => Ok(State::InChina(s.parse::<ChinaState>()?)),
            Country::England => Ok(State::InEngland(s.parse::<EnglandState>()?)),
            Country::Germany => Ok(State::InGermany(s.parse::<GermanyState>()?)),
            Country::Greece => Ok(State::InGreece(s.parse::<GreeceState>()?)),
            Country::India => Ok(State::InIndia(s.parse::<IndiaState>()?)),
            Country::Mexico => Ok(State::InMexico(s.parse::<MexicoState>()?)),
            Country::Netherlands => Ok(State::InNetherlands(s.parse::<NetherlandsState>()?)),
            Country::NewZealand => Ok(State::InNewZealand(s.parse::<NewZealandState>()?)),
            Country::Romania => Ok(State::InRomania(s.parse::<RomaniaState>()?)),
            Country::Russia => Ok(State::InRussia(s.parse::<RussiaState>()?)),
            Country::SouthAfrica => Ok(State::InSouthAfrica(s.parse::<SouthAfricaState>()?)),
            Country::UAE => Ok(State::InUAE(s.parse::<UAEState>()?)),
            Country::USA => Ok(State::InUSA(s.parse::<USAState>()?)),
            _ => Err(ParseError::VariantNotFound),
        }
    }

    /// Computes the available states for a given country.
    ///
    /// Useful for debugging information.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Country;
    /// # use opltypes::states::State;
    /// let available = State::get_available_for_country(Country::England).unwrap();
    /// assert!(available.contains(&"GL".to_string()));
    /// ```
    pub fn get_available_for_country(country: Country) -> Option<Vec<String>> {
        fn inner<T>() -> Option<Vec<String>>
        where
            T: IntoEnumIterator,
            <<T as IntoEnumIterator>::Iterator as Iterator>::Item: ToString,
        {
            Some(T::iter().map(|value| value.to_string()).collect())
        }

        match country {
            Country::Argentina => inner::<ArgentinaState>(),
            Country::Australia => inner::<AustraliaState>(),
            Country::Brazil => inner::<BrazilState>(),
            Country::Canada => inner::<CanadaState>(),
            Country::Chile => inner::<ChileState>(),
            Country::China => inner::<ChinaState>(),
            Country::England => inner::<EnglandState>(),
            Country::Germany => inner::<GermanyState>(),
            Country::Greece => inner::<GreeceState>(),
            Country::India => inner::<IndiaState>(),
            Country::Mexico => inner::<MexicoState>(),
            Country::Netherlands => inner::<NetherlandsState>(),
            Country::NewZealand => inner::<NewZealandState>(),
            Country::Romania => inner::<RomaniaState>(),
            Country::Russia => inner::<RussiaState>(),
            Country::SouthAfrica => inner::<SouthAfricaState>(),
            Country::UAE => inner::<UAEState>(),
            Country::USA => inner::<USAState>(),
            _ => None,
        }
    }

    /// Constructs a State given a full, unambiguous code like "USA-NY".
    ///
    /// This is how the server interprets the State column.
    /// Codes of this format are the result of serializing a State value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Country;
    /// # use opltypes::states::{State, USAState};
    /// let state = State::from_full_code("USA-NY").unwrap();
    /// assert_eq!(state, State::InUSA(USAState::NY));
    /// ```
    pub fn from_full_code(s: &str) -> Result<State, ParseError> {
        // The codes are of the form "{Country}-{State}".
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(ParseError::VariantNotFound);
        }

        let country: Country = parts[0].parse::<Country>()?;
        Self::from_str_and_country(parts[1], country)
    }

    /// Returns the Country for the given State.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Country;
    /// # use opltypes::states::{State, USAState};
    /// let state = State::from_full_code("USA-NY").unwrap();
    /// assert_eq!(state.to_country(), Country::USA);
    /// ```
    pub fn to_country(self) -> Country {
        match self {
            State::InArgentina(_) => Country::Argentina,
            State::InAustralia(_) => Country::Australia,
            State::InBrazil(_) => Country::Brazil,
            State::InCanada(_) => Country::Canada,
            State::InChile(_) => Country::Chile,
            State::InChina(_) => Country::China,
            State::InEngland(_) => Country::England,
            State::InGermany(_) => Country::Germany,
            State::InGreece(_) => Country::Greece,
            State::InIndia(_) => Country::India,
            State::InMexico(_) => Country::Mexico,
            State::InNetherlands(_) => Country::Netherlands,
            State::InNewZealand(_) => Country::NewZealand,
            State::InRomania(_) => Country::Romania,
            State::InRussia(_) => Country::Russia,
            State::InSouthAfrica(_) => Country::SouthAfrica,
            State::InUAE(_) => Country::UAE,
            State::InUSA(_) => Country::USA,
        }
    }

    /// Returns a String describing just the given State (no Country).
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Country;
    /// # use opltypes::states::{State, USAState};
    /// let state = State::from_full_code("USA-NY").unwrap();
    /// assert_eq!(state.to_state_string(), "NY");
    /// ```
    pub fn to_state_string(self) -> String {
        match self {
            State::InArgentina(s) => s.to_string(),
            State::InAustralia(s) => s.to_string(),
            State::InBrazil(s) => s.to_string(),
            State::InCanada(s) => s.to_string(),
            State::InChile(s) => s.to_string(),
            State::InChina(s) => s.to_string(),
            State::InEngland(s) => s.to_string(),
            State::InGermany(s) => s.to_string(),
            State::InGreece(s) => s.to_string(),
            State::InIndia(s) => s.to_string(),
            State::InMexico(s) => s.to_string(),
            State::InNetherlands(s) => s.to_string(),
            State::InNewZealand(s) => s.to_string(),
            State::InRomania(s) => s.to_string(),
            State::InRussia(s) => s.to_string(),
            State::InSouthAfrica(s) => s.to_string(),
            State::InUAE(s) => s.to_string(),
            State::InUSA(s) => s.to_string(),
        }
    }
}

impl Serialize for State {
    /// Serialization for the server. The checker uses from_str_and_country().
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let country = self.to_country().to_string();
        let state = self.to_state_string();
        format!("{country}-{state}").serialize(serializer)
    }
}

/// Helper struct for State deserialization.
///
/// This is only used by the server, not by the checker.
/// The checker uses from_str_and_country().
struct StateVisitor;

impl Visitor<'_> for StateVisitor {
    type Value = State;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A Country-State code like USA-NY")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<State, E> {
        State::from_full_code(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for State {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<State, D::Error> {
        deserializer.deserialize_str(StateVisitor)
    }
}

/// A state in Argentina.
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum ArgentinaState {
    /// Ciudad Autónoma de Buenos Aires.
    CA,
    /// Buenos Aires.
    BA,
    /// Catamarca.
    CT,
    /// Chaco.
    CC,
    /// Chubut.
    CH,
    /// Córdoba.
    CB,
    /// Corrientes.
    CN,
    /// Entre Ríos.
    ER,
    /// Formosa.
    FM,
    /// Jujuy.
    JY,
    /// La Pampa.
    LP,
    /// La Rioja.
    LR,
    /// Mendoza.
    MZ,
    /// Misiones.
    MN,
    /// Neuquén.
    NQ,
    /// Río Negro.
    RN,
    /// Salta.
    SA,
    /// San Juan.
    SJ,
    /// San Luis.
    SL,
    /// Santa Cruz.
    SC,
    /// Santa Fe.
    SF,
    /// Santiago del Estero.
    SE,
    /// Tierra del Fuego.
    TF,
    /// Tucumán.
    TM,
}

/// A state in Australia.
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum AustraliaState {
    /// Australian Capital Territory.
    ACT,
    /// Jervis Bay Territory.
    JBT,
    /// New South Wales.
    NSW,
    /// Northern Territory.
    NT,
    /// Queensland.
    QLD,
    /// South Australia.
    SA,
    /// Tasmania.
    TAS,
    /// Victoria.
    VIC,
    /// Western Australia.
    WA,
}

/// A state in Brazil.
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum BrazilState {
    /// Acre.
    AC,
    /// Alagoas.
    AL,
    /// Amapá.
    AP,
    /// Amazonas.
    AM,
    /// Bahia.
    BA,
    /// Ceará.
    CE,
    /// Distrito Federal.
    DF,
    /// Espírito Santo.
    ES,
    /// Goiás.
    GO,
    /// Maranhão.
    MA,
    /// Mato Grosso.
    MT,
    /// Mato Grosso do Sul.
    MS,
    /// Minas Gerais.
    MG,
    /// Pará.
    PA,
    /// Paraíba.
    PB,
    /// Paraná.
    PR,
    /// Pernambuco.
    PE,
    /// Piauí.
    PI,
    /// Rio de Janeiro.
    RJ,
    /// Rio Grande do Norte.
    RN,
    /// Rio Grande do Sul.
    RS,
    /// Rondônia.
    RO,
    /// Roraima.
    RR,
    /// Santa Catarina.
    SC,
    /// São Paulo.
    SP,
    /// Sergipe.
    SE,
    /// Tocantins.
    TO,
}

/// A state in Canada.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum CanadaState {
    AB, BC, MB, NB, NL, NT, NS, NU, ON, PE, QC, SK, YT
}

/// A state in Chile.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum ChileState {
    AI, AN, AP, AT, BI, CO, AR, LI, LL, LR, MA, ML, NB, RM, TA, VS
}

/// A province in China.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum ChinaState {
    /// Anhui Province (安徽省, Ānhuī Shěng).
    AH,
    /// Beijing Municipality (北京市, Běijīng Shì).
    BJ,
    /// Chongqing Municipality (重庆市, Chóngqìng Shì).
    CQ,
    /// Fujian Province (福建省, Fújiàn Shěng).
    FJ,
    /// Guangdong Province (广东省, Guǎngdōng Shěng).
    GD,
    /// Gansu Province (甘肃省, Gānsù Shěng).
    GS,
    /// Guangxi Zhuang Autonomous Region (广西壮族自治区, Guǎngxī Zhuàngzú Zìzhìqū).
    GX,
    /// Guizhou Province (贵州省, Guìzhōu Shěng).
    GZ,
    /// Henan Province (河南省, Hénán Shěng).
    HEN,
    /// Hubei Province (湖北省, Húběi Shěng).
    HUB,
    /// Hebei Province (河北省, Héběi Shěng).
    HEB,
    /// Hainan Province (海南省, Hǎinán Shěng).
    HI,
    /// Hong Kong Special Administrative Region (香港特别行政区, Xiānggǎng Tèbié Xíngzhèngqū).
    ///
    /// We usually treat Hong Kong as a separate country. This is here for completeness.
    HK,
    /// Heilongjiang Province (黑龙江省, Hēilóngjiāng Shěng).
    HL,
    /// Hunan Province (湖南省, Húnán Shěng).
    HUN,
    /// Jilin Province (吉林省, Jílín Shěng).
    JL,
    /// Jiangsu Province (江苏省, Jiāngsū Shěng).
    JS,
    /// Jiangxi Province (江西省, Jiāngxī Shěng).
    JX,
    /// Liaoning Province (辽宁省, Liáoníng Shěng).
    LN,
    /// Macau Special Administrative Region (澳门特别行政区, Àomén Tèbié Xíngzhèngqū).
    MO,
    /// Inner Mongolia Autonomous Region (內蒙古自治区, Nèi Měnggǔ Zìzhìqū).
    NM,
    /// Ningxia Hui Autonomous Region (宁夏回族自治区, Níngxià Huízú Zìzhìqū).
    NX,
    /// Qinghai Province (青海省, Qīnghǎi Shěng).
    QH,
    /// Sichuan Province (四川省, Sìchuān Shěng).
    SC,
    /// Shandong Province (山东省, Shāndōng Shěng).
    SD,
    /// Shanghai Municipality (上海市, Shànghǎi Shì).
    SH,
    /// Shaanxi Province (陕西省, Shǎnxī Shěng).
    SAA,
    /// Shanxi Province (山西省, Shānxī Shěng).
    SAX,
    /// Tianjin Municipality (天津市, Tiānjīn Shì).
    TJ,
    /// Xinjiang Uyghur Autonomous Region (新疆维吾尔自治区, Xīnjiāng Wéiwú'ěr Zìzhìqū).
    XJ,
    /// Tibet Autonomous Region (西藏自治区, Xīzàng Zìzhìqū).
    XZ,
    /// Yunnan Province (云南省, Yúnnán Shěng).
    YN,
    /// Zhejiang Province (浙江省, Zhèjiāng Shěng).
    ZJ,
}

/// A region in England, ill-defined and used only by BP.
///
/// This omits other divisions not in England: Scotland, N.Ireland, and Wales.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum EnglandState {
    /// East Midlands.
    EM,
    /// Greater London.
    GL,
    /// North Midlands.
    NM,
    /// North West.
    NW,
    /// South East.
    SE,
    /// South Midlands.
    SM,
    /// South West.
    SW,
    /// West Midlands.
    WM,
    /// Yorkshire North East.
    YNE,
}

/// A state in Germany.
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum GermanyState {
    /// Baden-Württemberg.
    BW,
    /// Bavaria.
    BY,
    /// Berlin.
    BE,
    /// Brandenburg.
    BB,
    /// Bremen.
    HB,
    /// Hesse.
    HE,
    /// Hamburg.
    HH,
    /// Mecklenburg-Vorpommern.
    MV,
    /// Lower Saxony.
    NI,
    /// North Rhine-Westphalia.
    NRW,
    /// Rhineland-Palatinate.
    RP,
    /// Schleswig-Holstein.
    SH,
    /// Saarland.
    SL,
    /// Saxony.
    SN,
    /// Saxony-Anhalt.
    ST,
    /// Thuringia.
    TH,
}

/// A state in Greece.
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum GreeceState {
    /// Attica.
    ATT,
    /// Central Greece
    CG,
    /// Central Macedonia
    CM,
    /// Crete
    CRE,
    /// Eastern Macedonia and Thrace
    EMT,
    /// Epirus
    EPI,
    /// Ionian Islands
    ION,
    /// North Aegean
    NA,
    /// Peloponnese
    PEL,
    /// South Aegean
    SA,
    /// Thessaly
    THE,
    /// Western Greece
    WG,
    /// Western Macedonia
    WM,
    /// Mount Athos
    MA,
}

/// A state in India.
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum IndiaState {
    /// Andaman and Nicobar Islands.
    AN,
    /// Andhra Pradesh.
    AP,
    /// Arunachal Pradesh.
    AR,
    /// Assam.
    AS,
    /// Bihar.
    BR,
    /// Chhattisgarh.
    CG,
    /// Chandigarh.
    CH,
    /// Daman and Diu.
    DD,
    /// Dadra and Nagar Haveli.
    DH,
    /// Delhi.
    DL,
    /// Goa.
    GA,
    /// Gujarat.
    GJ,
    /// Haryana.
    HR,
    /// Himachal Pradesh.
    HP,
    /// Jammu and Kashmir.
    JK,
    /// Jharkhand.
    JH,
    /// Karnataka.
    KA,
    /// Kerala.
    KL,
    /// Lakshadweep.
    LD,
    /// Madhya Pradesh.
    MP,
    /// Maharashtra.
    MH,
    /// Manipur.
    MN,
    /// Meghalaya.
    ML,
    /// Mizoram.
    MZ,
    /// Nagaland.
    NL,
    /// Orissa.
    OR,
    /// Punjab.
    PB,
    /// Pondicherry / Puducherry.
    PY,
    /// Rajasthan.
    RJ,
    /// Sikkim.
    SK,
    /// Tamil Nadu.
    TN,
    /// Tripura.
    TR,
    /// Uttarakhand.
    UK,
    /// Uttar Pradesh.
    UP,
    /// West Bengal.
    WB,
}

/// A state in Mexico.
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum MexicoState {
    /// Aguascalientes.
    AG,
    /// Baja California.
    BC,
    /// Baja California Sur.
    BS,
    /// Campeche.
    CM,
    /// Chiapas.
    CS,
    /// Chihuahua.
    CH,
    /// Coahuila.
    CO,
    /// Colima.
    CL,
    /// Mexico City.
    DF,
    /// Durango.
    DG,
    /// Guanajuato.
    GT,
    /// Guerrero.
    GR,
    /// Hidalgo.
    HG,
    /// Jalisco.
    JA,
    /// México.
    EM,
    /// Michoacán.
    MI,
    /// Morelos.
    MO,
    /// Nayarit.
    NA,
    /// Nuevo León.
    NL,
    /// Oaxaca.
    OA,
    /// Puebla.
    PU,
    /// Querétaro.
    QT,
    /// Quintana Roo.
    QR,
    /// San Luis Potosí.
    SL,
    /// Sinaloa.
    SI,
    /// Sonora.
    SO,
    /// Tabasco.
    TB,
    /// Tamaulipas.
    TM,
    /// Tlaxcala.
    TL,
    /// Veracruz.
    VE,
    /// Yucatán.
    YU,
    /// Zacatecas.
    ZA,
}

/// A state in the Netherlands.
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum NetherlandsState {
    /// Drenthe.
    DR,
    /// Flevoland.
    FL,
    /// Friesland / Fryslân.
    FR,
    /// Gelderland.
    GE,
    /// Groningen.
    GR,
    /// Limburg.
    LI,
    /// North Brabant / Noord-Brabant.
    NB,
    /// North Holland / Noord-Holland.
    NH,
    /// Overijssel / Overissel.
    OV,
    /// Utrecht.
    UT,
    /// Zeeland.
    ZE,
    /// South Holland / Zuid-Holland.
    ZH,
}

/// A region in New Zealand.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum NewZealandState {
    /// Northland.
    NTL,
    /// Auckland.
    AKL,
    /// Waikato.
    WKO,
    /// Bay of Plenty.
    BOP,
    /// Gisborne (East Coast).
    GIS,
    /// Hawke's Bay.
    HKB,
    /// Taranaki.
    TKI,
    /// Manawatu-Whanganui.
    MWT,
    /// Wellington.
    WGN,
    /// Tasman.
    TAS,
    /// Nelson.
    NSN,
    /// Marlborough.
    MBH,
    /// West Coast.
    WTC,
    /// Canterbury.
    CAN,
    /// Otago.
    OTA,
    /// Southland.
    STL,
}

/// A county in Romania.
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum RomaniaState {
    /// Alba.
    AB,
    /// Argeș.
    AG,
    /// Arad.
    AR,
    /// Bucharest.
    B,
    /// Bacău.
    BC,
    /// Bihor.
    BH,
    /// Bistrița-Năsăud.
    BN,
    /// Brăila.
    BR,
    /// Botoșani.
    BT,
    /// Brașov.
    BV,
    /// Buzău.
    BZ,
    /// Cluj.
    CJ,
    /// Călărași.
    CL,
    /// Caraș-Severin.
    CS,
    /// Constanța.
    CT,
    /// Covasna.
    CV,
    /// Dâmbovița.
    DB,
    /// Dolj.
    DJ,
    /// Gorj.
    GJ,
    /// Galați.
    GL,
    /// Giurgiu.
    GR,
    /// Hunedoara.
    HD,
    /// Harghita.
    HR,
    /// Ilfov.
    IF,
    /// Ialomița.
    IL,
    /// Iași.
    IS,
    /// Mehedinți.
    MH,
    /// Maramureș.
    MM,
    /// Mureș.
    MS,
    /// Neamț.
    NT,
    /// Olt.
    OT,
    /// Prahova.
    PH,
    /// Sibiu.
    SB,
    /// Sălaj.
    SJ,
    /// Satu Mare.
    SM,
    /// Suceava.
    SV,
    /// Tulcea.
    TL,
    /// Timiș.
    TM,
    /// Teleorman.
    TR,
    /// Vâlcea.
    VL,
    /// Vrancea.
    VN,
    /// Vaslui.
    VS,
}

/// An oblast in Russia.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum RussiaState {
    // Republics.
    /// Adygea.
    AD,
    /// Altai Republic.
    AL,
    /// Bashkortostan.
    BA,
    /// Buryatia.
    BU,
    /// Chechnya.
    CE,
    /// Chuvashia.
    CU,
    /// Dagestan.
    DA,
    /// Ingushetia.
    IN,
    /// Kabardino-Balkaria.
    KB,
    /// Khakassia.
    KK,
    /// Kalmykia.
    KL,
    /// Karachay-Cherkessia.
    KC,
    /// Karelia.
    KR,
    /// Komi.
    KO,
    /// Mari El.
    ME,
    /// Mordovia.
    MO,
    /// Sakha o Yakutia.
    SA,
    /// North Ossetia-Alania.
    SE,
    /// Tatarstan.
    TA,
    /// Tyva.
    TY,
    /// Udmurtia.
    UD,

    // Krais.
    /// Altai Krai.
    ALT,
    /// Kamchatka.
    KAM,
    /// Khabarovsk.
    KHA,
    /// Krasnodar.
    KDA,
    /// Krasnoyarsk.
    KYA,
    /// Perm.
    PER,
    /// Primorsky.
    PRI,
    /// Stavropol.
    STA,
    /// Zabaykalsky Krai.
    ZAB,

    // Oblasts.
    /// Amur.
    AMU,
    /// Arkhangelsk.
    ARK,
    /// Astrakhan.
    AST,
    /// Belgorod.
    BEL,
    /// Bryansk.
    BRY,
    /// Chelyabinsk.
    CHE,
    /// Chita (defunct since 2008).
    ///
    /// In 2008, Chita Oblast merged with Agin-Buryat Autonomous Okrug, becoming Zabaykalsky Krai.
    CHI,
    /// Irkutsk.
    IRK,
    /// Ivanovo.
    IVA,
    /// Kaliningrad.
    KGD,
    /// Kaluga.
    KLU,
    /// Kemerovo.
    KEM,
    /// Kirov.
    KIR,
    /// Kostroma.
    KOS,
    /// Kurgan.
    KGN,
    /// Kursk.
    KRS,
    /// Leningrad.
    LEN,
    /// Lipetsk.
    LIP,
    /// Magadan.
    MAG,
    /// Moscow (oblast).
    MOS,
    /// Murmansk.
    MUR,
    /// Nizhny Novgorod.
    NIZ,
    /// Novgorod.
    NGR,
    /// Novosibirsk.
    NVS,
    /// Omsk.
    OMS,
    /// Orenburg.
    ORE,
    /// Oryol.
    ORL,
    /// Penza.
    PNZ,
    /// Pskov.
    PSK,
    /// Rostov.
    ROS,
    /// Ryazan.
    RYA, 
    /// Sakhalin.
    SAK,
    /// Samara.
    SAM,
    /// Saratov.
    SAR,
    /// Smolensk.
    SMO,
    /// Sverdlovsk.
    SVE,
    /// Tambov.
    TAM,
    /// Tomsk.
    TOM,
    /// Tula.
    TUL,
    /// Tver.
    TVE,
    /// Tyumen.
    TYU,
    /// Ulyanovsk.
    ULY,
    /// Vladimir.
    VLA,
    /// Volgograd.
    VGG,
    /// Vologda.
    VLG,
    /// Voronezh.
    VOR,
    /// Yaroslavl.
    YAR,

    // Okrugs.
    /// Aga Buryatia.
    AGB,
    /// Nenetsia,
    NEN,
    /// Ust-Orda Buryatia.
    UOB,
    /// Khantia-Mansia.
    KHM,
    /// Chukotka.
    CHU,
    /// Yamalia.
    YAN,

    // Federal cities.
    /// St. Petersburg.
    SPE,
    /// Moscow (city).
    MOW,

    /// Autonomous Jewish Province.
    YEV,
}

/// A province in South Africa, using conventional acronyms (non-ISO).
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum SouthAfricaState {
    /// Eastern Cape.
    EC,
    /// Free State.
    FS,
    /// Gauteng.
    GT,
    /// KwaZulu-Natal (ISO: NL).
    KZN,
    /// Limpopo.
    LP,
    /// Mpumalanga.
    MP,
    /// Northern Cape.
    NC,
    /// North-West.
    NW,
    /// Western Cape.
    WC,
}

/// AN Emirate in the UAE (non-ISO).
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum UAEState {
    /// Abu Dhabi.
    AD,
    /// Dubai.
    DXB,
    /// Sharjah.
    SHJ,
    /// Ajman.
    AJM,
    /// Umm Al Quwain.
    UAQ,
    /// Ras Al Khaima.
    RAK,
    /// Fujairah.
    FUJ,
}

/// A state in the USA.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, Display, EnumString, EnumIter, PartialEq, Eq, Serialize)]
pub enum USAState {
    AL, AK, AZ, AR, CA, CO, CT, DE, DC, FL, GA, HI, ID, IL, IN, IA, KS,
    KY, LA, ME, MD, MA, MI, MN, MS, MO, MT, NE, NV, NH, NJ, NM, NY, NC,
    ND, OH, OK, OR, PA, RI, SC, SD, TN, TX, UT, VT, VA, WA, WV, WI, WY,

    /// Guam is an unincorporated territory of the USA.
    Guam,
}
