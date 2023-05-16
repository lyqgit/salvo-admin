use salvo::{handler, Request,Response, FlowCtrl, Depot, prelude::StatusCode};
use crate::utils::res::{res_json_custom,res_custom};

#[handler]
pub async fn auth_token(&self,req:&mut Request,res:&mut Response, ctrl: &mut FlowCtrl){
  println!("走中间件");
  if let Some(_token) = req.headers().get("token"){
    println!("有token");
  }else{
    println!("没有token");
    ctrl.skip_rest();
    res.render(res_json_custom::<()>(401,"token无效".to_string()));
  }
  
}

#[handler]
pub async fn catcher_err(&self, _req: &Request, _depot: &Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
  println!("发生panic");
  if let Some(status_code) = res.status_code {
    match status_code {
      StatusCode::NOT_FOUND=>{
        // ctrl.skip_rest();
        println!("任意状态,{:#?}",res.body.size());
        res.render(res_json_custom::<()>(404, "发生错误".to_string()));
      },
      StatusCode::INTERNAL_SERVER_ERROR=>{
        println!("500错误");
        ctrl.skip_rest();
        res.render(res_json_custom::<()>(500, "发生错误".to_string()));
      },
      // StatusCode::BAD_REQUEST=>{
      //   println!("BAD_REQUEST");
      //   ctrl.skip_rest();
      //   // res.body(serde_json::to_string(&res_custom::<()>(500, "发生错误".to_string())).unwrap().into());
      //   // res.render(res_json_custom::<()>(500, "发生错误".to_string()));
      // }
      _=>{
        println!("任意状态,{:#?}",res.body.size());
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        // res.render(res_json_custom::<()>(500, "发生错误".to_string()));
        res.body("".into());
        res.render(res_json_custom::<()>(500, "发生错误".to_string()));

        println!("任意状态,{:#?}",res.body.size());
        ctrl.skip_rest();
        // res.body(res_json_custom::<()>(500, "发生错误".to_string()));
      }
    }
  }
}