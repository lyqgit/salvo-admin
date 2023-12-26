use salvo::{oapi::{ToSchema}};
use salvo::prelude::Json;

pub use crate::model::common_model::ResObj;


impl<T:ToSchema> ResObj<T> {
    pub fn ok(data: Option<T>)->Self{
      Self {
          code: 0,
          msg: "访问成功".to_string(),
          data,
      }
    }
    pub fn custom_code(code:i32,msg:String) -> Self {
        Self {
            code,
            msg,
            data: None,
        }
    }

    pub fn err(err:String)->Self{
        Self {
          code: 500,
          msg: err,
          data: None,
      }
    }
}

#[allow(dead_code)]
pub fn res_ok<T:ToSchema>(data:Option<T>)->ResObj<T>{
  ResObj::ok(data)
}

#[allow(dead_code)]
pub fn res_json_ok<T:ToSchema>(data:Option<T>)->Json<ResObj<T>>{
  Json(ResObj::ok(data))
}

#[allow(dead_code)]
pub fn res_err<T:ToSchema>(msg:String)->ResObj<T>{
  ResObj::err(msg)
}

#[allow(dead_code)]
pub fn res_json_err<T:ToSchema>(msg:String)->Json<ResObj<T>>{
  Json(ResObj::err(msg))
}

#[allow(dead_code)]
pub fn res_custom<T:ToSchema>(code:i32,msg:String)->ResObj<T>{
  ResObj::custom_code(code,msg)
}

#[allow(dead_code)]
pub fn res_json_custom<T:ToSchema>(code:i32,msg:String)->Json<ResObj<T>>{
  Json(ResObj::custom_code(code,msg))
}

#[allow(dead_code)]
pub type Res<T> = Result<Json<ResObj<T>>,Json<ResObj<()>>>;

#[allow(dead_code)]
pub fn match_ok<T:ToSchema>(res:rbatis::Result<T>)->Res<T>{
    match res {
        Ok(v)=>{
            Ok(res_json_ok(Some(v)))
        },
        Err(err)=>{
            Err(res_json_custom(400,err.to_string()))
        }
    }
}

#[allow(dead_code)]
pub fn match_ok_common_result_no_error<T:ToSchema>(res:Result<T,()>)->Res<T>{
    match res {
        Ok(v)=>{
            Ok(res_json_ok(Some(v)))
        },
        Err(_)=>{
            Err(res_json_custom(400,"服务器发生错误".to_string()))
        }
    }
}


#[allow(dead_code)]
pub fn match_custom_ok<T:ToSchema>(res:rbatis::Result<T>, resolve: Box<dyn FnOnce(T) -> Json<ResObj<T>>>)->Res<T>{
    match res {
        Ok(v)=>{
            Ok(resolve(v))
        },
        Err(err)=>{
            Err(res_json_custom(400,err.to_string()))
        }
    }
}

#[allow(dead_code)]
pub fn match_no_res_ok(res:rbatis::Result<bool>)->Res<()>{
    match res {
        Ok(v)=>{
            if v{
                Ok(res_json_ok(Some(())))
            }else{
                Err(res_json_custom(400,"发生错误".to_string()))
            }
        },
        Err(err)=>{
            Err(res_json_custom(400,err.to_string()))
        }
    }
}