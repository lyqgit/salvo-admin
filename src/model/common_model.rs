use salvo::oapi::ToSchema;
use serde::{Serialize};

#[derive(Debug,Serialize,ToSchema)]
pub struct ResObj<T:ToSchema>{
  pub code:i32,
  pub data:Option<T>,
  pub msg:String
}