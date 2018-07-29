//! Defines the `Country` field for the `meets` table.

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, EnumString)]
pub enum Country {
    Algeria,
    Argentina,
    Aruba,
    Australia,
    Azerbaijan,
    Austria,
    Belarus,
    Belgium,
    Brazil,
    Britain,
    #[serde(rename = "British Virgin Islands")]
    #[strum(to_string = "British Virgin Islands")]
    BritishVirginIslands,
    Bulgaria,
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
    Czechia,
    Denmark,
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
    Japan,
    Kazakhstan,
    Latvia,
    Lithuania,
    Luxembourg,
    Malaysia,
    Mexico,
    Mongolia,
    Morocco,
    Netherlands,
    #[serde(rename = "New Caledonia")]
    #[strum(to_string = "New Caledonia")]
    NewCaledonia,
    #[serde(rename = "New Zealand")]
    #[strum(to_string = "New Zealand")]
    NewZealand,
    Nicaragua,
    Norway,
    #[serde(rename = "N.Ireland")]
    #[strum(to_string = "N.Ireland")]
    NorthernIreland,
    Oman,
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
    Russia,
    Samoa,
    Scotland,
    Serbia,
    Singapore,
    Slovakia,
    Slovenia,
    #[serde(rename = "South Africa")]
    #[strum(to_string = "South Africa")]
    SouthAfrica,
    #[serde(rename = "South Korea")]
    #[strum(to_string = "South Korea")]
    SouthKorea,
    Spain,
    Sweden,
    Switzerland,
    Tahiti,
    Taiwan,
    Turkey,
    UAE,
    UK,
    Ukraine,
    Uruguay,
    USA,
    #[serde(rename = "US Virgin Islands")]
    #[strum(to_string = "US Virgin Islands")]
    USVirginIslands,
    Uzbekistan,
    Venezuela,
    Vietnam,
    Wales,
}
