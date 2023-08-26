use serde::{Deserialize,Serialize};
use salvo::oapi::{ToSchema};

#[derive(Debug,Deserialize)]
pub struct ChatRxMsg{
    pub token:String,
    pub name:String,
    pub msg:String,
}

#[derive(Debug,Serialize,ToSchema)]
pub struct ChatTxMsg{
    pub name:String,
    pub msg:String,
}
