use clickhouse::Row;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct Entity {
    pub uid: i64,
    pub users_id: i64,
    pub send_type_cd: i16,
    pub msg_key: String,
    pub template_code: String,
    pub sender_key: String,
    pub cell: String,
    pub title_param: Option<String>,
    pub kakao_user_param: Option<String>,
    pub message: Option<String>,
    pub attach: Option<String>,
    pub tm_subject: Option<String>,
    pub tm_message: Option<String>,
    pub tm_callback_no: String,
    pub tm_send_yn: String,
    #[serde(with = "clickhouse::serde::time::datetime64::micros")]
    pub reserv_dt: OffsetDateTime,
    #[serde(with = "clickhouse::serde::time::datetime64::micros")]
    pub send_dt: OffsetDateTime,
    #[serde(with = "clickhouse::serde::time::datetime64::micros")]
    pub return_dt: OffsetDateTime,
    #[serde(with = "clickhouse::serde::time::datetime64::micros")]
    pub reg_dt: OffsetDateTime,
    pub return_code: String,
    pub svc_div: String,
    pub currency_type_div: Option<String>,
    pub price: i32,
    pub is_test: bool,
    pub supplement: Option<String>,
    pub cate_cd: i32,
    pub member_msg_group_id: Option<String>,
    pub member_msg_id: Option<String>,
    pub member_mall_no: i32,
    pub server_id: i32,
    pub pay_type_cd: i32,
    pub msg_dt: Option<String>,
    pub kko_msg_type: Option<String>,
    pub mass_send_id: Option<String>,
    pub member_id: String,
    pub custom_key: Option<String>,
    pub origin_return_code: Option<String>,
    pub header: Option<String>,
}