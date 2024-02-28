use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub struct Config {
    pub to_email: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let to_email = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a to email argument"),
        };

        Ok(Config { to_email })
    }
}
