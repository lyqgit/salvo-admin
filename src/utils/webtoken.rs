use jsonwebtoken::{encode,Header,EncodingKey,errors::Result,DecodingKey,Validation,decode,TokenData};
use serde::{Serialize,Deserialize};
use fastdate::DateTime;

#[derive(Debug,Serialize,Deserialize)]
pub struct MyClaims{
    pub id:i32,
    pub username:String,
    pub time:i64,
    pub exp:usize
}

pub fn create_token(id:i32,username:String)->Result<String>{
  let time = DateTime::now();
  let my_claims = MyClaims{id,username,time:time.unix_timestamp()+3600*6,exp: (time.unix_timestamp() + 3600 * 6) as usize };
  encode(&Header::default(), &my_claims, &EncodingKey::from_secret("poem_claims".as_ref()))
}

pub fn verify_token(token:&str)->Result<TokenData<MyClaims>>{
  decode::<MyClaims>(token, &DecodingKey::from_secret("poem_claims".as_ref()), &Validation::default())
}