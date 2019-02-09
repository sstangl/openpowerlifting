//! Defines valid entries in the MeetState column.

use strum::ParseError;

use crate::Country;

/// The State column.
#[derive(Debug, PartialEq)]
pub enum State {
    InAustralia(AustraliaState),
    InBrazil(BrazilState),
    InCanada(CanadaState),
    InGermany(GermanyState),
    InIndia(IndiaState),
    InMexico(MexicoState),
    InNetherlands(NetherlandsState),
    InNewZealand(NewZealandState),
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
            Country::Australia => Ok(State::InAustralia(s.parse::<AustraliaState>()?)),
            Country::Brazil => Ok(State::InBrazil(s.parse::<BrazilState>()?)),
            Country::Canada => Ok(State::InCanada(s.parse::<CanadaState>()?)),
            Country::Germany => Ok(State::InGermany(s.parse::<GermanyState>()?)),
            Country::India => Ok(State::InIndia(s.parse::<IndiaState>()?)),
            Country::Mexico => Ok(State::InMexico(s.parse::<MexicoState>()?)),
            Country::Netherlands => Ok(State::InNetherlands(s.parse::<NetherlandsState>()?)),
            Country::NewZealand => Ok(State::InNewZealand(s.parse::<NewZealandState>()?)),
            Country::Russia => Ok(State::InRussia(s.parse::<RussiaState>()?)),
            Country::USA => Ok(State::InUSA(s.parse::<USAState>()?)),
            _ => Err(ParseError::VariantNotFound),
        }
    }
}

/// A state in Australia.
#[rustfmt::skip]
#[derive(Debug, EnumString, PartialEq)]
pub enum AustraliaState {
    ACT, NSW, NT, QLD, SA, TAS, VIC, WA
}

/// A state in Brazil.
#[rustfmt::skip]
#[derive(Debug, EnumString, PartialEq)]
pub enum BrazilState {
    AC, AL, AP, AM, BA, CE, DF, ES, GO, MA, MT, MS, MG, PA,
    PB, PR, PE, PI, RJ, RN, RS, RO, RR, SC, SP, SE, TO
}

/// A state in Canada.
#[rustfmt::skip]
#[derive(Debug, EnumString, PartialEq)]
pub enum CanadaState {
    AB, BC, MB, NB, NL, NT, NS, NU, ON, PE, QC, SK, YT
}

/// A state in Germany.
#[rustfmt::skip]
#[derive(Debug, EnumString, PartialEq)]
pub enum GermanyState {
    BW, BY, BE, BB, HB, HE, HH, MV, NI, NW, RP, SH, SL, SN, ST, TH
}

/// A state in India.
#[rustfmt::skip]
#[derive(Debug, EnumString, PartialEq)]
pub enum IndiaState {
    AP, AR, AS, BR, CG, GA, GJ, HR, HP, JK, JH, KA,
    KL, MP, MH, MN, ML, MZ, NL, OR, PB, RJ, SK, TN,
    TR, UK, UP, WB, AN, CH, DH, DD, DL, LD, PY
}

/// A state in Mexico.
#[rustfmt::skip]
#[derive(Debug, EnumString, PartialEq)]
pub enum MexicoState {
    AG, BC, BS, CM, CS, CH, CO, CL, DF, DG, GT,
    GR, HG, JA, EM, MI, MO, NA, NL, OA, PU, QT,
    QR, SL, SI, SO, TB, TM, TL, VE, YU, ZA
}

/// A state in the Netherlands
#[rustfmt::skip]
#[derive(Debug, EnumString, PartialEq)]
pub enum NetherlandsState {
    DR, FL, FR, GE, GR, LI, NB, NH, OV, UT, ZE,
    ZH
}

/// A state in New Zealand.
#[rustfmt::skip]
#[derive(Debug, EnumString, PartialEq)]
pub enum NewZealandState {
    NTL, AKL, WKO, BOP, GIS, HKB, TKI, MWT, WGN,
    TAS, NSN, MBH, WTC, CAN, OTA, STL
}

/// A state in Russia.
#[rustfmt::skip]
#[derive(Debug, EnumString, PartialEq)]
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
#[derive(Debug, EnumString, PartialEq)]
pub enum USAState {
    AL, AK, AZ, AR, CA, CO, CT, DE, DC, FL, GA, HI, ID, IL, IN, IA, KS,
    KY, LA, ME, MD, MA, MI, MN, MS, MO, MT, NE, NV, NH, NJ, NM, NY, NC,
    ND, OH, OK, OR, PA, RI, SC, SD, TN, TX, UT, VT, VA, WA, WV, WI, WY,

    /// Guam is an unincorporated territory of the USA.
    Guam,
}
