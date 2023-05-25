use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct SysCaptcha{
  pub id:i64,
  pub code:String,
  pub create_time:DateTime,
  pub expire_time:i128,
}