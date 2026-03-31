use comfy_table::Table;
use util::byte::ToBase64;
use util::encrypt::EncryptKey;

#[derive(clap::Args, Debug)]
pub struct Args {
    #[arg(short, long)]
    user: String,
    #[arg(short, long)]
    password: String,
}

const PROD_KEY_NAME: &str = "aes_bzmoffice_prod";
const DEV_KEY_NAME: &str = "aes_bzmoffice_dev";

impl Args {
    pub fn run(&self) {
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
            "dev",
            EncryptKey::new_env(DEV_KEY_NAME)
                .unwrap()
                .encrypt_cbc_pkcs7(format!("{}\t{}", self.user, self.password))
                .to_base64()
                .as_str()
        ]);


        println!("{table}");
    }
}