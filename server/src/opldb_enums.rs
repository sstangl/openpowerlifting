//! A collection of enums used by the database.

#[derive(Deserialize)]
pub enum Sex {
    M,
    F,
}

#[derive(Deserialize)]
pub enum Equipment {
    Raw,
    Wraps,
    #[serde(rename = "Single-ply")]
    Single,
    #[serde(rename = "Multi-ply")]
    Multi,
    Straps,
}

#[derive(Deserialize)]
pub enum Federation {
    #[serde(rename = "365Strong")]
    _365Strong,
    AAPF,
    AAU,
    ADFPA,
    APA,
    APC,
    APF,
    AsianPF,
    BB,
    BPU,
    BP,
    CAPO,
    CommonwealthPF,
    CPF,
    CPL,
    CPU,
    EPA,
    EPF,
    FESUPO,
    FFForce,
    FPO,
    GBPF,
    GPA,
    GPC,
    #[serde(rename = "GPC-GB")]
    GPCGB,
    #[serde(rename = "GPC-AUS")]
    GPCAUS,
    HERC,
    IPA,
    IPF,
    IPL,
    IrishPF,
    MHP,
    MM,
    NAPF,
    NASA,
    NIPF,
    NPA,
    NSF,
    NZPF,
    OceaniaPF,
    ProRaw,
    PA,
    RAW,
    RPS,
    RUPC,
    ScottishPL,
    SCT,
    SPF,
    THSPA,
    UPA,
    USAPL,
    USPF,
    USPA,
    WelshPA,
    WPC,
    WNPF,
    WRPF,
    #[serde(rename = "WRPF-AUS")]
    WRPFAUS,
    #[serde(rename = "WRPF-CAN")]
    WRPFCAN,
    WUAP,
    XPC,
}
