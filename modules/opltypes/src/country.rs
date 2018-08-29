//! Defines the `Country` field for the `meets` table.

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, EnumString)]
pub enum Country {
    Albania,
    Algeria,
    Argentina,
    Armenia,
    Aruba,
    Australia,
    Azerbaijan,
    Austria,
    Bahamas,
    Belarus,
    Belgium,
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
    Cameroon,
    Canada,
    #[serde(rename = "Cayman Islands")]
    #[strum(to_string = "Cayman Islands")]
    CaymanIslands,
    China,
    Colombia,
    #[serde(rename = "Costa Rica")]
    #[strum(to_string = "Costa Rica")]
    CostaRica,
    #[serde(rename = "Côte d’Ivoire")]
    #[strum(to_string = "Côte d’Ivoire")]
    CoteDIvoire,
    Croatia,
    Cyprus,
    Czechia,
    Denmark,
    Djibouti,
    Ecuador,
    Egypt,
    #[serde(rename = "El Salvador")]
    #[strum(to_string = "El Salvador")]
    ElSalvador,
    England,
    Estonia,
    Fiji,
    Finland,
    France,
    Georgia,
    Germany,
    Greece,
    Guatemala,
    Guyana,
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
    Jamaica,
    Japan,
    Kazakhstan,
    Kenya,
    Kiribati,
    Kyrgyzstan,
    Latvia,
    Lebanon,
    Libya,
    Lithuania,
    Luxembourg,
    Malaysia,
    Mexico,
    Mongolia,
    Montenegro,
    Morocco,
    Nauru,
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
    Nigeria,
    Norway,
    #[serde(rename = "N.Ireland")]
    #[strum(to_string = "N.Ireland")]
    NorthernIreland,
    Oman,
    Pakistan,
    #[serde(rename = "Papua New Guinea")]
    #[strum(to_string = "Papua New Guinea")]
    PapuaNewGuinea,
    Peru,
    Philippines,
    Poland,
    Portugal,
    #[serde(rename = "Puerto Rico")]
    #[strum(to_string = "Puerto Rico")]
    PuertoRico,
    Romania,
    Russia,
    Samoa,
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
    Sweden,
    Switzerland,
    Tahiti,
    Taiwan,
    Tajikistan,
    Togo,
    Tonga,
    #[serde(rename = "Trinidad and Tobago")]
    #[strum(to_string = "Trinidad and Tobago")]
    TrinidadAndTobago,
    Turkey,
    Turkmenistan,
    UAE,
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
    /// Existed from 1949-1990.
    #[serde(rename = "West Germany")]
    #[strum(to_string = "West Germany")]
    WestGermany,
    /// Existed from 1945-1992.
    Yugoslavia,
    Zambia,
    Zimbabwe,
}
