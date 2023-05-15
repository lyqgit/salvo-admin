use salvo::{handler, Request,Response, FlowCtrl};
use crate::utils::res;

#[handler]
pub async fn auth_token(&self,req:&mut Request,res:&mut Response, ctrl: &mut FlowCtrl){
  println!("走中间件");
  if let Some(_token) = req.headers().get("token"){
    println!("有token");
  }else{
    println!("没有token");
    ctrl.skip_rest();
    res.render(res::res_json_custom::<()>(401,"token无效".to_string()));
  }
  
}