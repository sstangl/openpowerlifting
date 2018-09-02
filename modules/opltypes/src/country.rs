//! Defines the `Country` field for the `meets` table.

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, EnumString)]
pub enum Country {
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
    Belarus,
    Belgium,
    Benin,
    Bolivia,
    #[serde(rename = "Bosnia and Herzegovina")]
    #[strum(to_string = "Bosnia and Herzegovina")]
    BosniaAndHerzegovina,
    Brazil,
    Britain,
    #[serde(rename = "British Virgin Islands")]
    #[strum(to_string = "British Virgin Islands")]
    BritishVirginIslands,
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
    #[serde(rename = "Costa Rica")]
    #[strum(to_string = "Costa Rica")]
    CostaRica,
    Croatia,
    Cuba,
    Cyprus,
    Czechia,
    Denmark,
    Djibouti,
    #[serde(rename = "Dominican Republic")]
    #[strum(to_string = "Dominican Republic")]
    DominicanRepublic,
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
    Ethiopia,
    Fiji,
    Finland,
    France,
    Georgia,
    Ghana,
    Germany,
    Greece,
    Guatemala,
    Guyana,
    Honduras,
    #[serde(rename = "Hong Kong")]
    #[strum(to_string = "Hong Kong")]
    HongKong,
    Hungary,
    Iceland,
    India,
    Indonesia,
    Ireland,
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
    Liberia,
    Libya,
    Lithuania,
    Luxembourg,
    Macedonia,
    Malaysia,
    Mali,
    #[serde(rename = "Marshall Islands")]
    #[strum(to_string = "Marshall Islands")]
    MarshallIslands,
    Mauritania,
    Mexico,
    Moldova,
    Mongolia,
    Montenegro,
    Morocco,
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
    Oman,
    Pakistan,
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
    Romania,
    Russia,
    Samoa,
    #[serde(rename = "Saudi Arabia")]
    #[strum(to_string = "Saudi Arabia")]
    SaudiArabia,
    Scotland,
    Serbia,
    /// Existed from 1992-2006.
    #[serde(rename = "Serbia and Montenegro")]
    #[strum(to_string = "Serbia and Montenegro")]
    SerbiaAndMontenegro,
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
    Sweden,
    Syria,
    Switzerland,
    Tahiti,
    Taiwan,
    Tajikistan,
    Thailand,
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
    /// Existed from 1945-1992.
    Yugoslavia,
    Zambia,
    Zimbabwe,
}
