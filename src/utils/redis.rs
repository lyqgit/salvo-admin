use redis::{Client,Commands,ToRedisArgs,RedisResult,FromRedisValue};
use crate::GLOBAL_REDIS;

#[allow(dead_code)]
pub fn set_ex<K:ToRedisArgs,V:ToRedisArgs>(key:K,value:V,second:usize)->RedisResult<()>{
  let _:() = Client::set_ex(&mut GLOBAL_REDIS.clone(),key,value,second)?;
  Ok(())
}

#[allow(dead_code)]
pub fn get<T:FromRedisValue,K:ToRedisArgs>(key:K)->RedisResult<T>{
  let t:T = Client::get(&mut GLOBAL_REDIS.clone(),key)?;
  Ok(t)
}

#[allow(dead_code)]
#[allow(unused_must_use)]
pub fn del<K:ToRedisArgs>(key:K)->RedisResult<()>{
  let _:() = Client::del(&mut GLOBAL_REDIS.clone(),key)?;
  Ok(())
}