//! Defines the `Country` field for the `meets` table.

/// The Country column.
#[derive(Copy, Clone, Debug, Deserialize, Display, Serialize, PartialEq, Eq, EnumString)]
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
}
