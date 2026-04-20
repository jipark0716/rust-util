use comfy_table::Table;
use util::byte::FromBase64;
use util::encrypt::EncryptKey;
use crate::command::{encrypt, CBT_KEY_NAME, DEV_KEY_NAME, PROD_KEY_NAME};

#[derive(clap::Args, Debug)]
pub struct Args {
    input: String,
}

struct Auth {
    user: String,
    password: String,
}

impl Args {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut table = Table::new();
        table.set_header(vec!["ENV", "USER", "PASSWORD"]);

        if let Ok(user) = self.decrypt(PROD_KEY_NAME)
        {
            table.add_row(vec!["prod", user.user.as_str(), user.password.as_str()]);
            encrypt::Args::new(user.user, user.password).run()?;
        }

        if let Ok(user) = self.decrypt(CBT_KEY_NAME)
        {
            table.add_row(vec!["cbt", user.user.as_str(), user.password.as_str()]);
            encrypt::Args::new(user.user, user.password).run()?;
        }

        if let Ok(user) = self.decrypt(DEV_KEY_NAME)
        {
            table.add_row(vec!["dev", user.user.as_str(), user.password.as_str()]);
            encrypt::Args::new(user.user, user.password).run()?;
        }

        println!("{table}");

        Ok(())
    }

    fn decrypt(&self, env: &str) -> anyhow::Result<Auth> {
        let key = EncryptKey::new_env(env)?;
        let payload = self.input.from_base64()?;
        let dec = key.decrypt_cbc_pkcs7(&*payload)?;
        let mut spl = dec.split("\t").map(|s| s.to_string());

        Ok(Auth {
            user: spl.next().unwrap_or("no user".to_string()),
            password: spl.next().unwrap_or("no password".to_string()),
        })
    }
}