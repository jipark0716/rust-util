use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Attach {
    button: Vec<Button>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Button {
    name: String,
    #[serde(rename = "type")]
    button_type: String,
    url_mobile: Option<String>,
    url_pc: Option<String>,
}

const EMPTY_ATTACH: &str = r#"{"button":[]}"#;

pub(crate) fn format(input: String) -> anyhow::Result<String> {
    let mut attach: Attach = match serde_json::from_str(&input) {
        Ok(a) => a,
        Err(_) => return Ok(EMPTY_ATTACH.to_string()),
    };

    for button in attach.button.iter_mut() {
        if button.name == "채널 추가" {
            button.name = "채널추가".to_string();
        }

        if button.url_pc == None && button.url_mobile != None {
            button.url_pc = button.url_mobile.clone();
        }
    }

    Ok(serde_json::to_string(&attach)?)
}