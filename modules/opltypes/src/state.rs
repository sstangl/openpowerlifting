//! Defines valid entries in the MeetState column.

use serde::ser::Serialize;
use strum::ParseError;

use crate::Country;

/// The State column.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    InArgentina(ArgentinaState),
    InAustralia(AustraliaState),
    InBrazil(BrazilState),
    InCanada(CanadaState),
    InGermany(GermanyState),
    InIndia(IndiaState),
    InMexico(MexicoState),
    InNetherlands(NetherlandsState),
    InNewZealand(NewZealandState),
    InRomania(RomaniaState),
    InRussia(RussiaState),
    InUSA(USAState),
}

impl State {
    /// Constructs a State for a specific Country.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::{Country, State, USAState};
    /// let state = State::from_str_and_country("NY", Country::USA).unwrap();
    /// assert_eq!(state, State::InUSA(USAState::NY));
    /// ```
    pub fn from_str_and_country(s: &str, country: Country) -> Result<State, ParseError> {
        match country {
            Country::Argentina => Ok(State::InArgentina(s.parse::<ArgentinaState>()?)),
            Country::Australia => Ok(State::InAustralia(s.parse::<AustraliaState>()?)),
            Country::Brazil => Ok(State::InBrazil(s.parse::<BrazilState>()?)),
            Country::Canada => Ok(State::InCanada(s.parse::<CanadaState>()?)),
            Country::Germany => Ok(State::InGermany(s.parse::<GermanyState>()?)),
            Country::India => Ok(State::InIndia(s.parse::<IndiaState>()?)),
            Country::Mexico => Ok(State::InMexico(s.parse::<MexicoState>()?)),
            Country::Netherlands => Ok(State::InNetherlands(s.parse::<NetherlandsState>()?)),
            Country::NewZealand => Ok(State::InNewZealand(s.parse::<NewZealandState>()?)),
            Country::Romania => Ok(State::InRomania(s.parse::<RomaniaState>()?)),
            Country::Russia => Ok(State::InRussia(s.parse::<RussiaState>()?)),
            Country::USA => Ok(State::InUSA(s.parse::<USAState>()?)),
            _ => Err(ParseError::VariantNotFound),
        }
    }
}

impl Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            State::InArgentina(s) => s.serialize(serializer),
            State::InAustralia(s) => s.serialize(serializer),
            State::InBrazil(s) => s.serialize(serializer),
            State::InCanada(s) => s.serialize(serializer),
            State::InGermany(s) => s.serialize(serializer),
            State::InIndia(s) => s.serialize(serializer),
            State::InMexico(s) => s.serialize(serializer),
            State::InNetherlands(s) => s.serialize(serializer),
            State::InNewZealand(s) => s.serialize(serializer),
            State::InRomania(s) => s.serialize(serializer),
            State::InRussia(s) => s.serialize(serializer),
            State::InUSA(s) => s.serialize(serializer),
        }
    }
}

/// A state in Argentina.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize)]
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
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize)]
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
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize)]
pub enum BrazilState {
    AC, AL, AP, AM, BA, CE, DF, ES, GO, MA, MT, MS, MG, PA,
    PB, PR, PE, PI, RJ, RN, RS, RO, RR, SC, SP, SE, TO
}

/// A state in Canada.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize)]
pub enum CanadaState {
    AB, BC, MB, NB, NL, NT, NS, NU, ON, PE, QC, SK, YT
}

/// A state in Germany.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize)]
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

/// A state in India.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize)]
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
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize)]
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
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize)]
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

/// A state in New Zealand.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize)]
pub enum NewZealandState {
    NTL, AKL, WKO, BOP, GIS, HKB, TKI, MWT, WGN,
    TAS, NSN, MBH, WTC, CAN, OTA, STL
}

/// A county in Romania.
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize)]
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

/// A state in Russia.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize)]
pub enum RussiaState {
    AD, AL, BA, BU, CE, CU, DA, IN, KB, KL, KC, KR, KK, KO, ME, MO, SA,
    SE, TA, TY, UD, ALT, KAM, KHA, KDA, KYA, PER, PRO, STA, ZAB, AMU, ARK,
    AST, BEL, BRY, CHE, IRK, IVA, KGD, KLU, KEM, KIR, KOS, KGN, KRS, LEN,
    LIP, MAG, MOS, MUR, NIZ, NGR, NVS, OMS, ORE, ORL, PNZ, PSK, ROS, RYA, 
    SAK, SAM, SAR, SMO, SVE, TAM, TOM, TUL, TVE, TYE, TYU, ULY, VLA, VGG,
    VLG, VOR, YAR, MOW, SPE, YEV, CHU, KHM, NEN, YAN
}

/// A state in the USA.
#[rustfmt::skip]
#[derive(Copy, Clone, Debug, EnumString, PartialEq, Serialize)]
pub enum USAState {
    AL, AK, AZ, AR, CA, CO, CT, DE, DC, FL, GA, HI, ID, IL, IN, IA, KS,
    KY, LA, ME, MD, MA, MI, MN, MS, MO, MT, NE, NV, NH, NJ, NM, NY, NC,
    ND, OH, OK, OR, PA, RI, SC, SD, TN, TX, UT, VT, VA, WA, WV, WI, WY,

    /// Guam is an unincorporated territory of the USA.
    Guam,
}
