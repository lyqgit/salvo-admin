use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct SwaggerAuth{
    pub username:String,
    pub password:String,
}
