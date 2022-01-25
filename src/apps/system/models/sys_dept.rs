use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Serialize, Default, Validate)]
pub struct SearchReq {
    pub dept_id: Option<String>,
    #[validate(length(min = 1))]
    pub dept_name: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default, Validate)]
pub struct AddReq {
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: String,
    pub phone: String,
    pub email: String,
    pub status: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct DeleteReq {
    pub dept_id: String,
}

#[derive(Deserialize, Clone, Debug, Serialize, Validate)]
pub struct EditReq {
    pub dept_id: String,
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: String,
    pub phone: String,
    pub email: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult, Default)]
pub struct DeptResp {
    pub dept_id: String,
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: String,
    pub phone: String,
    pub email: String,
    pub status: String,
}

#[derive(Deserialize, Clone, Debug, Serialize, Default)]
pub struct RespTree {
    #[serde(flatten)]
    pub data: DeptResp,
    pub children: Option<Vec<RespTree>>,
}
