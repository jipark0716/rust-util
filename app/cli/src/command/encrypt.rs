use comfy_table::Table;
use util::byte::ToBase64;
use util::encrypt::EncryptKey;
use crate::command::{CBT_KEY_NAME, DEV_KEY_NAME, PROD_KEY_NAME};

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(long)]
    user: String,
    #[arg(long)]
    password: String,
}

impl Args {
    pub fn new(user: String, password: String) -> Self {
        Self { user, password }
    }

    pub fn run(&self) -> anyhow::Result<()> {
        let mut table = Table::new();
        table.set_header(vec!["ENV", "CONTENT"]);

        table.add_row(vec![
            "prod",
            EncryptKey::new_env(PROD_KEY_NAME)
                .unwrap()
                .encrypt_cbc_pkcs7(format!("{}\t{}", self.user, self.password))
                .to_base64()
                .as_str()
        ]);

        table.add_row(vec![
            "cbt",
            EncryptKey::new_env(CBT_KEY_NAME)
              .unwrap()
              .encrypt_cbc_pkcs7(format!("{}\t{}", self.user, self.password))
              .to_base64()
              .as_str()
        ]);

        table.add_row(vec![
            "dev",
            EncryptKey::new_env(DEV_KEY_NAME)
              .unwrap()
              .encrypt_cbc_pkcs7(format!("{}\t{}", self.user, self.password))
              .to_base64()
              .as_str()
        ]);


        println!("{table}");

        Ok(())
    }
}