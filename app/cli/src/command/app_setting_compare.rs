use once_cell::sync::Lazy;
use regex::Regex;

static AppSettingRegex: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"appsettings[.]?[A-z]*\.yml").unwrap());

#[derive(clap::Args, Debug)]
pub struct Args {
}

impl Args {
    pub async fn run(&self) {
        if let Err(e) = self.run_e().await {
            println!("fail run {:?}", e);
        }
    }

    pub async fn run_e(&self) -> anyhow::Result<()> {
        
    }
}
