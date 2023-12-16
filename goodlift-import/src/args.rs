use color_eyre::eyre::Result;

pub struct Args {
    pub cid: String,
}

impl Args {
    pub fn parse() -> Result<Self> {
        let mut args = pico_args::Arguments::from_env();

        Ok(Args {
            cid: args.value_from_str("--cid")?,
        })
    }
}
