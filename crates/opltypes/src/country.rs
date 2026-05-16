//! Defines the `Country` field for the `meets` table.

use phf::phf_map;

/// The Country column.
#[derive(
    Copy, Clone, Debug, Deserialize, Display, Serialize, PartialEq, Eq, EnumString, EnumCount,
)]
pub enum Country {
    Abkhazia,
    Afghanistan,
    Albania,
    Algeria,
    #[serde(rename = "American Samoa")]
    #[strum(to_string = "American Samoa")]
    AmericanSamoa,
    Angola,
    Argentina,
    Armenia,
    Aruba,
    Australia,
    Azerbaijan,
    Austria,
    Bahamas,
    Bahrain,
    Bangladesh,
    Belarus,
    Belgium,
    Belize,
    Benin,
    Bolivia,
    #[serde(rename = "Bosnia and Herzegovina")]
    #[strum(to_string = "Bosnia and Herzegovina")]
    BosniaAndHerzegovina,
    Botswana,
    Brazil,
    #[serde(rename = "British Virgin Islands")]
    #[strum(to_string = "British Virgin Islands")]
    BritishVirginIslands,
    Brunei,
    Bulgaria,
    #[serde(rename = "Burkina Faso")]
    #[strum(to_string = "Burkina Faso")]
    BurkinaFaso,
    #[serde(rename = "Cabo Verde")]
    #[strum(to_string = "Cabo Verde")]
    CaboVerde,
    Cambodia,
    Cameroon,
    Canada,
    #[serde(rename = "Cayman Islands")]
    #[strum(to_string = "Cayman Islands")]
    CaymanIslands,
    #[serde(rename = "Central African Republic")]
    #[strum(to_string = "Central African Republic")]
    CentralAfricanRepublic,
    Chile,
    China,
    Colombia,
    /// Union of the Comoros.
    Comoros,
    Congo,
    #[serde(rename = "Cook Islands")]
    #[strum(to_string = "Cook Islands")]
    CookIslands,
    #[serde(rename = "Costa Rica")]
    #[strum(to_string = "Costa Rica")]
    CostaRica,
    Croatia,
    Cuba,
    Cyprus,
    Czechia,
    /// Existed from 1918-1993.
    Czechoslovakia,
    Denmark,
    Djibouti,
    #[serde(rename = "Dominican Republic")]
    #[strum(to_string = "Dominican Republic")]
    DominicanRepublic,
    /// Existed from 1949-1990.
    #[serde(rename = "East Germany")]
    #[strum(to_string = "East Germany")]
    EastGermany,
    #[serde(rename = "East Timor")]
    #[strum(to_string = "East Timor")]
    EastTimor,
    Ecuador,
    Egypt,
    #[serde(rename = "El Salvador")]
    #[strum(to_string = "El Salvador")]
    ElSalvador,
    England,
    Estonia,
    /// Previously Swaziland: renamed itself in 2018.
    Eswatini,
    Ethiopia,
    Fiji,
    Finland,
    France,
    /// Gained autonomy and country status in 2004.
    //
    // Commonly referred to as Tahiti,
    // the most populous island, but
    // comprises 121 islands and atolls.
    #[serde(rename = "French Polynesia")]
    #[strum(to_string = "French Polynesia")]
    FrenchPolynesia,
    Gabon,
    Georgia,
    Germany,
    Ghana,
    Gibraltar,
    Greece,
    Grenada,
    Guatemala,
    Guinea,
    #[serde(rename = "Guinea-Bissau")]
    #[strum(to_string = "Guinea-Bissau")]
    GuineaBissau,
    Guyana,
    Haiti,
    Honduras,
    #[serde(rename = "Hong Kong")]
    #[strum(to_string = "Hong Kong")]
    HongKong,
    Hungary,
    Iceland,
    India,
    Indonesia,
    Ireland,
    #[serde(rename = "Isle of Man")]
    #[strum(to_string = "Isle of Man")]
    IsleOfMan,
    Israel,
    Italy,
    Iran,
    Iraq,
    #[serde(rename = "Ivory Coast")]
    #[strum(to_string = "Ivory Coast")]
    IvoryCoast,
    Jamaica,
    Japan,
    Jordan,
    Kazakhstan,
    Kenya,
    Kiribati,
    Kuwait,
    Kyrgyzstan,
    Laos,
    Latvia,
    Lebanon,
    Lesotho,
    Liberia,
    Libya,
    Lithuania,
    Luxembourg,
    Madagascar,
    Malaysia,
    Mali,
    Malta,
    #[serde(rename = "Marshall Islands")]
    #[strum(to_string = "Marshall Islands")]
    MarshallIslands,
    Mauritania,
    Mauritius,
    Mexico,
    Moldova,
    Monaco,
    Mongolia,
    Montenegro,
    Morocco,
    Myanmar,
    Namibia,
    Nauru,
    Nepal,
    Netherlands,
    /// Existed from 1954-2010.
    #[serde(rename = "Netherlands Antilles")]
    #[strum(to_string = "Netherlands Antilles")]
    NetherlandsAntilles,
    #[serde(rename = "New Caledonia")]
    #[strum(to_string = "New Caledonia")]
    NewCaledonia,
    #[serde(rename = "New Zealand")]
    #[strum(to_string = "New Zealand")]
    NewZealand,
    Nicaragua,
    Niger,
    Nigeria,
    Niue,
    Norway,
    #[serde(rename = "N.Ireland")]
    #[strum(to_string = "N.Ireland")]
    NorthernIreland,
    // Renamed in Feb 2019
    #[serde(rename = "North Macedonia")]
    #[strum(to_string = "North Macedonia")]
    NorthMacedonia,
    Oman,
    Pakistan,
    Palestine,
    Panama,
    #[serde(rename = "Papua New Guinea")]
    #[strum(to_string = "Papua New Guinea")]
    PapuaNewGuinea,
    Paraguay,
    Peru,
    Philippines,
    Poland,
    Portugal,
    #[serde(rename = "Puerto Rico")]
    #[strum(to_string = "Puerto Rico")]
    PuertoRico,
    Qatar,
    /// Existed from 1965-1979.
    ///
    /// Preceded by the British colony of Southern Rhodesia.
    /// Succeeded by Zimbabwe.
    Rhodesia,
    Romania,
    Russia,
    Rwanda,
    Samoa,
    #[serde(rename = "Saudi Arabia")]
    #[strum(to_string = "Saudi Arabia")]
    SaudiArabia,
    Scotland,
    Senegal,
    Serbia,
    /// Existed from 1992-2006.
    #[serde(rename = "Serbia and Montenegro")]
    #[strum(to_string = "Serbia and Montenegro")]
    SerbiaAndMontenegro,
    #[serde(rename = "Sierra Leone")]
    #[strum(to_string = "Sierra Leone")]
    SierraLeone,
    Singapore,
    Slovakia,
    Slovenia,
    #[serde(rename = "Solomon Islands")]
    #[strum(to_string = "Solomon Islands")]
    SolomonIslands,
    #[serde(rename = "South Africa")]
    #[strum(to_string = "South Africa")]
    SouthAfrica,
    #[serde(rename = "South Korea")]
    #[strum(to_string = "South Korea")]
    SouthKorea,
    Spain,
    #[serde(rename = "Sri Lanka")]
    #[strum(to_string = "Sri Lanka")]
    SriLanka,
    Sudan,
    Suriname,
    Sweden,
    Syria,
    Switzerland,
    /// Recognised as major island of French Polynesia.
    Tahiti,
    Taiwan,
    Tajikistan,
    Tanzania,
    Thailand,
    #[serde(rename = "The Gambia")]
    #[strum(to_string = "The Gambia")]
    TheGambia,
    Togo,
    Tonga,
    /// Unrecognized state. UN recognizes as part of Moldova.
    Transnistria,
    #[serde(rename = "Trinidad and Tobago")]
    #[strum(to_string = "Trinidad and Tobago")]
    TrinidadAndTobago,
    Tunisia,
    Turkey,
    Turkmenistan,
    Tuvalu,
    UAE,
    Uganda,
    UK,
    Ukraine,
    Uruguay,
    USA,
    /// Existed from 1922-1991.
    USSR,
    #[serde(rename = "US Virgin Islands")]
    #[strum(to_string = "US Virgin Islands")]
    USVirginIslands,
    Uzbekistan,
    Vanuatu,
    Venezuela,
    Vietnam,
    Wales,
    #[serde(rename = "Wallis and Futuna")]
    #[strum(to_string = "Wallis and Futuna")]
    WallisAndFutuna,
    /// Existed from 1949-1990.
    #[serde(rename = "West Germany")]
    #[strum(to_string = "West Germany")]
    WestGermany,
    Yemen,
    /// Existed from 1945-1992.
    Yugoslavia,
    Zambia,
    Zimbabwe,
}

/// Serialization of the countries when expressed as a URL segment.
///
/// All letters are lowercase ASCII and spaces are replaced with dashes.
static COUNTRY_AS_URL_SEGMENT: phf::Map<&'static str, Country> = phf_map! {
    "abkhazia" => Country::Abkhazia,
    "afghanistan" => Country::Afghanistan,
    "albania" => Country::Albania,
    "algeria" => Country::Algeria,
    "american-samoa" => Country::AmericanSamoa,
    "angola" => Country::Angola,
    "argentina" => Country::Argentina,
    "armenia" => Country::Armenia,
    "aruba" => Country::Aruba,
    "australia" => Country::Australia,
    "azerbaijan" => Country::Azerbaijan,
    "austria" => Country::Austria,
    "bahamas" => Country::Bahamas,
    "bahrain" => Country::Bahrain,
    "bangladesh" => Country::Bangladesh,
    "belarus" => Country::Belarus,
    "belgium" => Country::Belgium,
    "belize" => Country::Belize,
    "benin" => Country::Benin,
    "bolivia" => Country::Bolivia,
    "bosnia-and-herzegovina" => Country::BosniaAndHerzegovina,
    "botswana" => Country::Botswana,
    "brazil" => Country::Brazil,
    "british-virgin-islands" => Country::BritishVirginIslands,
    "brunei" => Country::Brunei,
    "bulgaria" => Country::Bulgaria,
    "burkina-faso" => Country::BurkinaFaso,
    "cabo-verde" => Country::CaboVerde,
    "cambodia" => Country::Cambodia,
    "cameroon" => Country::Cameroon,
    "canada" => Country::Canada,
    "cayman-islands" => Country::CaymanIslands,
    "central-african-republic" => Country::CentralAfricanRepublic,
    "chile" => Country::Chile,
    "china" => Country::China,
    "colombia" => Country::Colombia,
    "comoros" => Country::Comoros,
    "congo" => Country::Congo,
    "cook-islands" => Country::CookIslands,
    "costa-rica" => Country::CostaRica,
    "croatia" => Country::Croatia,
    "cuba" => Country::Cuba,
    "cyprus" => Country::Cyprus,
    "czechia" => Country::Czechia,
    "czechoslovakia" => Country::Czechoslovakia,
    "denmark" => Country::Denmark,
    "djibouti" => Country::Djibouti,
    "dominican-republic" => Country::DominicanRepublic,
    "east-germany" => Country::EastGermany,
    "east-timor" => Country::EastTimor,
    "ecuador" => Country::Ecuador,
    "egypt" => Country::Egypt,
    "el-salvador" => Country::ElSalvador,
    "england" => Country::England,
    "estonia" => Country::Estonia,
    "eswatini" => Country::Eswatini,
    "ethiopia" => Country::Ethiopia,
    "fiji" => Country::Fiji,
    "finland" => Country::Finland,
    "france" => Country::France,
    "french-polynesia" => Country::FrenchPolynesia,
    "gabon" => Country::Gabon,
    "georgia" => Country::Georgia,
    "germany" => Country::Germany,
    "ghana" => Country::Ghana,
    "gibraltar" => Country::Gibraltar,
    "greece" => Country::Greece,
    "grenada" => Country::Grenada,
    "guatemala" => Country::Guatemala,
    "guinea" => Country::Guinea,
    "guinea-bissau" => Country::GuineaBissau,
    "guyana" => Country::Guyana,
    "haiti" => Country::Haiti,
    "honduras" => Country::Honduras,
    "hong-kong" => Country::HongKong,
    "hungary" => Country::Hungary,
    "iceland" => Country::Iceland,
    "india" => Country::India,
    "indonesia" => Country::Indonesia,
    "ireland" => Country::Ireland,
    "isle-of-man" => Country::IsleOfMan,
    "israel" => Country::Israel,
    "italy" => Country::Italy,
    "iran" => Country::Iran,
    "iraq" => Country::Iraq,
    "ivory-coast" => Country::IvoryCoast,
    "jamaica" => Country::Jamaica,
    "japan" => Country::Japan,
    "jordan" => Country::Jordan,
    "kazakhstan" => Country::Kazakhstan,
    "kenya" => Country::Kenya,
    "kiribati" => Country::Kiribati,
    "kuwait" => Country::Kuwait,
    "kyrgyzstan" => Country::Kyrgyzstan,
    "laos" => Country::Laos,
    "latvia" => Country::Latvia,
    "lebanon" => Country::Lebanon,
    "lesotho" => Country::Lesotho,
    "liberia" => Country::Liberia,
    "libya" => Country::Libya,
    "lithuania" => Country::Lithuania,
    "luxembourg" => Country::Luxembourg,
    "madagascar" => Country::Madagascar,
    "malaysia" => Country::Malaysia,
    "mali" => Country::Mali,
    "malta" => Country::Malta,
    "marshall-islands" => Country::MarshallIslands,
    "mauritania" => Country::Mauritania,
    "mauritius" => Country::Mauritius,
    "mexico" => Country::Mexico,
    "moldova" => Country::Moldova,
    "monaco" => Country::Monaco,
    "mongolia" => Country::Mongolia,
    "montenegro" => Country::Montenegro,
    "morocco" => Country::Morocco,
    "myanmar" => Country::Myanmar,
    "namibia" => Country::Namibia,
    "nauru" => Country::Nauru,
    "nepal" => Country::Nepal,
    "netherlands" => Country::Netherlands,
    "netherlands-antilles" => Country::NetherlandsAntilles,
    "new-caledonia" => Country::NewCaledonia,
    "new-zealand" => Country::NewZealand,
    "nicaragua" => Country::Nicaragua,
    "niger" => Country::Niger,
    "nigeria" => Country::Nigeria,
    "niue" => Country::Niue,
    "norway" => Country::Norway,
    "northern-ireland" => Country::NorthernIreland,
    "north-macedonia" => Country::NorthMacedonia,
    "oman" => Country::Oman,
    "pakistan" => Country::Pakistan,
    "palestine" => Country::Palestine,
    "panama" => Country::Panama,
    "papua-new-guinea" => Country::PapuaNewGuinea,
    "paraguay" => Country::Paraguay,
    "peru" => Country::Peru,
    "philippines" => Country::Philippines,
    "poland" => Country::Poland,
    "portugal" => Country::Portugal,
    "puerto-rico" => Country::PuertoRico,
    "qatar" => Country::Qatar,
    "rhodesia" => Country::Rhodesia,
    "romania" => Country::Romania,
    "russia" => Country::Russia,
    "rwanda" => Country::Rwanda,
    "samoa" => Country::Samoa,
    "saudi-arabia" => Country::SaudiArabia,
    "scotland" => Country::Scotland,
    "senegal" => Country::Senegal,
    "serbia" => Country::Serbia,
    "serbia-and-montenegro" => Country::SerbiaAndMontenegro,
    "sierra-leone" => Country::SierraLeone,
    "singapore" => Country::Singapore,
    "slovakia" => Country::Slovakia,
    "slovenia" => Country::Slovenia,
    "solomon-islands" => Country::SolomonIslands,
    "south-africa" => Country::SouthAfrica,
    "south-korea" => Country::SouthKorea,
    "spain" => Country::Spain,
    "sri-lanka" => Country::SriLanka,
    "sudan" => Country::Sudan,
    "suriname" => Country::Suriname,
    "sweden" => Country::Sweden,
    "syria" => Country::Syria,
    "switzerland" => Country::Switzerland,
    "tahiti" => Country::Tahiti,
    "taiwan" => Country::Taiwan,
    "tajikistan" => Country::Tajikistan,
    "tanzania" => Country::Tanzania,
    "thailand" => Country::Thailand,
    "the-gambia" => Country::TheGambia,
    "togo" => Country::Togo,
    "tonga" => Country::Tonga,
    "transnistria" => Country::Transnistria,
    "trinidad-and-tobago" => Country::TrinidadAndTobago,
    "tunisia" => Country::Tunisia,
    "turkey" => Country::Turkey,
    "turkmenistan" => Country::Turkmenistan,
    "tuvalu" => Country::Tuvalu,
    "uae" => Country::UAE,
    "uganda" => Country::Uganda,
    "uk" => Country::UK,
    "ukraine" => Country::Ukraine,
    "uruguay" => Country::Uruguay,
    "usa" => Country::USA,
    "ussr" => Country::USSR,
    "us-virgin-islands" => Country::USVirginIslands,
    "uzbekistan" => Country::Uzbekistan,
    "vanuatu" => Country::Vanuatu,
    "venezuela" => Country::Venezuela,
    "vietnam" => Country::Vietnam,
    "wales" => Country::Wales,
    "wallis-and-futuna" => Country::WallisAndFutuna,
    "west-germany" => Country::WestGermany,
    "yemen" => Country::Yemen,
    "yugoslavia" => Country::Yugoslavia,
    "zambia" => Country::Zambia,
    "zimbabwe" => Country::Zimbabwe,
};

impl Country {
    /// Whether this Country contains the other.
    #[inline]
    pub fn contains(self, other: Country) -> bool {
        match self {
            Country::UK => other.is_in_uk(),
            _ => false,
        }
    }

    /// Whether the country is in the UK.
    #[inline]
    pub fn is_in_uk(self) -> bool {
        matches!(
            self,
            Country::England
                | Country::Gibraltar
                | Country::NorthernIreland
                | Country::UK
                | Country::Scotland
                | Country::Wales
        )
    }

    /// Matches a Country formatted for a URL.
    ///
    /// Within URLs, the input string is always lowercase, and spaces are replaced with dashes.
    pub fn from_url_segment(s: &str) -> Option<Country> {
        COUNTRY_AS_URL_SEGMENT.get(s).copied()
    }

    /// Returns a Country formatted for inclusion in a URL.
    pub fn as_url_segment(self) -> &'static str {
        // This is an inefficient implementation, but it saves some repetition in the code.
        COUNTRY_AS_URL_SEGMENT
            .entries()
            .find(|(_url_segment, country)| **country == self)
            .map(|(url_segment, _country)| *url_segment)
            .unwrap_or("unknown-country") // Asserted impossible by tests.
    }

    /// Serializes as optional country for inclusion in URLs.
    ///
    /// Helper function for use with the `#serde[serialize_with]` attribute.
    pub fn serialize_opt_as_url_segment<S>(v: &Option<Self>, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match v {
            None => s.serialize_none(),
            Some(country) => s.serialize_str(country.as_url_segment()),
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::EnumCount;

    use super::*;

    /// Tests parsing Country as it appears in CSV files.
    #[test]
    fn parses_country() {
        // Success cases.
        assert_eq!("USA".parse::<Country>().unwrap(), Country::USA);
        assert_eq!("Costa Rica".parse::<Country>().unwrap(), Country::CostaRica);

        // Failure cases.
        assert!("usa".parse::<Country>().is_err());
        assert!("usa ".parse::<Country>().is_err());
        assert!("".parse::<Country>().is_err());
        assert!("canada".parse::<Country>().is_err());
        assert!("CostaRica".parse::<Country>().is_err());
    }

    /// Tests parsing Country as it appears in URLs.
    #[test]
    fn parses_url_segment() {
        // Success cases.
        assert_eq!(Country::from_url_segment("usa"), Some(Country::USA));
        assert_eq!(
            Country::from_url_segment("costa-rica"),
            Some(Country::CostaRica)
        );

        // Failure cases.
        assert_eq!(Country::from_url_segment("USA"), None);
        assert_eq!(Country::from_url_segment("Costa Rica"), None);
    }

    /// Asserts that every country is supported by `Country::from_url_segment`.
    #[test]
    fn url_segment_coverage() {
        assert_eq!(
            Country::COUNT,
            COUNTRY_AS_URL_SEGMENT.len(),
            "COUNTRY_AS_URL_SEGMENT needs updating"
        );
    }
}
