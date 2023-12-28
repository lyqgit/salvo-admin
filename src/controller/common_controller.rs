use salvo::{handler, Request,Response, FlowCtrl, Depot, prelude::StatusCode};
use crate::utils::res::{res_json_custom};
use crate::utils::redis;

#[handler]
pub async fn auth_token(req:&mut Request,res:&mut Response, ctrl: &mut FlowCtrl,depot: &mut Depot){
  if let Some(token) = req.headers().get("Authorization"){
    // 验证token
    match redis::get::<i32,&str>(&token.to_str().unwrap().to_string().replace("Bearer ","")){
      Err(_)=>{
        ctrl.skip_rest();
        res.render(res_json_custom::<()>(401,"token无效".to_string()));
      },
      Ok(user_id)=>{
        // println!("有token---{}",user_id);
        depot.insert("userId",user_id);
      }
    }

  }else{
    ctrl.skip_rest();
    res.render(res_json_custom::<()>(401,"token无效".to_string()));
  }
  
}

#[handler]
pub async fn catcher_err(res: &mut Response, ctrl: &mut FlowCtrl) {
  println!("发生panic");
  if let Some(status_code) = res.status_code {
    match status_code {
      StatusCode::NOT_FOUND=>{
        // ctrl.skip_rest();
        // println!("任意状态,{:#?}",res.body.size());
        res.render(res_json_custom::<()>(404, "没有对应的接口".to_string()));
      },
      StatusCode::INTERNAL_SERVER_ERROR=>{
        println!("500错误");
        ctrl.skip_rest();
        res.body("".into());
        res.render(res_json_custom::<()>(500, "服务器发生错误".to_string()));
      },
      // StatusCode::BAD_REQUEST=>{
      //   println!("BAD_REQUEST");
      //   ctrl.skip_rest();
      //   // res.body(serde_json::to_string(&res_custom::<()>(500, "发生错误".to_string())).unwrap().into());
      //   // res.render(res_json_custom::<()>(500, "发生错误".to_string()));
      // }
      _=>{
        // println!("任意状态,{:#?}",res.body.size());
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        // res.render(res_json_custom::<()>(500, "发生错误".to_string()));
        res.body("".into());
        res.render(res_json_custom::<()>(500, "服务器发生错误".to_string()));

        // println!("任意状态,{:#?}",res.body.size());
        ctrl.skip_rest();
        // res.body(res_json_custom::<()>(500, "发生错误".to_string()));
      }
    }
  }
}