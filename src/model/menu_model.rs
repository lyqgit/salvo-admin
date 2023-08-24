use salvo::oapi::{ToParameters, ToSchema};
use serde::{Serialize,Deserialize};
use rbatis::rbdc::datetime::DateTime;
use crate::entity::sys_menu_entity::SysMenu;

#[derive(Debug,Serialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
pub struct Router{
  #[serde(rename(serialize = "alwaysShow"))]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub always_show:Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[salvo(schema(value_type=Option<Vec<Object>>))]
  pub children:Option<Vec<Router>>,
  pub component:String,
  pub hidden:bool,
  pub meta:Meta,
  pub name:String,
  pub path:String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub redirect:Option<String>,
}

#[derive(Debug,Serialize,ToSchema,Clone)]
#[salvo(schema(rename_all="camelCase"))]
pub struct Meta{
  pub icon:String,
  pub link:Option<String>,
  #[serde(rename(serialize = "noCache"))]
  pub no_cache:bool,
  pub title:String,
}


#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(serialize="camelCase"))]
pub struct SysMenuPage{
  pub menu_id:i64,
  pub menu_name:String,
  pub parent_id:i64,
  pub order_num:i8,
  pub path:String,
  pub component:Option<String>,
  pub query:Option<String>,
  pub is_frame:i8,
  pub is_cache:i8,
  pub menu_type:String,
  pub visible:String,
  pub status:String,
  pub perms:Option<String>,
  pub icon:String,
  pub create_by:String,
  pub create_time:DateTime,
  pub update_by:String,
  pub update_time:Option<DateTime>,
  pub remark:String,
}

#[derive(Debug,Serialize,Deserialize,Clone,ToParameters)]
#[salvo(parameters(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
#[salvo(parameters(parameter_in = Query))]
pub struct SysMenuPagePayload{
  pub menu_name:Option<String>,
  pub status:Option<String>,
}

#[derive(Debug,Serialize,Deserialize,Clone,ToSchema)]
#[salvo(schema(rename_all="camelCase"))]
#[serde(rename_all(deserialize="camelCase"))]
pub struct SysMenuModifyPayload{
  pub component:Option<String>,
  pub query:Option<String>,
  pub perms:Option<String>,
  pub icon:String,
  pub is_cache:String,
  pub is_frame:String,
  pub menu_name:String,
  pub menu_id:Option<i64>,
  pub menu_type:String,
  pub order_num:i64,
  pub parent_id:i64,
  pub path:Option<String>,
  pub status:String,
  pub visible:String,
  pub create_by:Option<String>,
  pub create_time:Option<DateTime>,
}

impl From<SysMenuModifyPayload> for SysMenu{
  fn from(p: SysMenuModifyPayload) -> Self {
    if p.menu_type.eq("F"){
      Self{
        menu_id:p.menu_id.map_or(0,|v|v),
        menu_name:p.menu_name,
        menu_type:p.menu_type,
        parent_id:p.parent_id,
        order_num:p.order_num,
        path:String::from(""),
        component:None,
        query:None,
        is_frame:p.is_frame.parse::<i8>().unwrap(),
        visible:p.visible,
        status:p.status,
        perms:p.perms,
        icon:p.icon,
        create_by:p.create_by.map_or(String::from(""),|v|v),
        create_time:p.create_time.map_or(DateTime::now(),|v|v),
        update_by:String::from(""),
        update_time:None,
        remark:String::from(""),
        is_cache: p.is_cache.parse::<i8>().unwrap(),
      }
    }else if p.menu_type.eq("C"){
      Self{
        menu_id:p.menu_id.map_or(0,|v|v),
        menu_name:p.menu_name,
        menu_type:p.menu_type,
        parent_id:p.parent_id,
        order_num:p.order_num,
        path:p.path.map_or(String::from(""),|v|v),
        component:p.component,
        query:p.query,
        is_frame:p.is_frame.parse::<i8>().unwrap(),
        visible:p.visible,
        status:p.status,
        perms:p.perms,
        icon:p.icon,
        create_by:p.create_by.map_or(String::from(""),|v|v),
        create_time:p.create_time.map_or(DateTime::now(),|v|v),
        update_by:String::from(""),
        update_time:None,
        remark:String::from(""),
        is_cache: p.is_cache.parse::<i8>().unwrap(),
      }
    }else if p.menu_type.eq("M"){
      Self{
        menu_id:p.menu_id.map_or(0,|v|v),
        menu_name:p.menu_name,
        menu_type:p.menu_type,
        parent_id:p.parent_id,
        order_num:p.order_num,
        path:p.path.map_or(String::from(""),|v|v),
        component:None,
        query:p.query,
        is_frame:p.is_frame.parse::<i8>().unwrap(),
        visible:p.visible,
        status:p.status,
        perms:None,
        icon:p.icon,
        create_by:p.create_by.map_or(String::from(""),|v|v),
        create_time:p.create_time.map_or(DateTime::now(),|v|v),
        update_by:String::from(""),
        update_time:None,
        remark:String::from(""),
        is_cache: p.is_cache.parse::<i8>().unwrap(),
      }
    }else{
      Self{
        menu_id:p.menu_id.map_or(0,|v|v),
        menu_name:p.menu_name,
        menu_type:p.menu_type,
        parent_id:p.parent_id,
        order_num:p.order_num,
        path:p.path.map_or(String::from(""),|v|v),
        component:p.component,
        query:p.query,
        is_frame:p.is_frame.parse::<i8>().unwrap(),
        visible:p.visible,
        status:p.status,
        perms:p.perms,
        icon:p.icon,
        create_by:p.create_by.map_or(String::from(""),|v|v),
        create_time:p.create_time.map_or(DateTime::now(),|v|v),
        update_by:String::from(""),
        update_time:None,
        remark:String::from(""),
        is_cache: p.is_cache.parse::<i8>().unwrap(),
      }
    }

  }
}


#[derive(Debug,Serialize,ToSchema,Clone)]
pub struct MenuTree{
  #[serde(skip_serializing_if = "Option::is_none")]
  #[salvo(schema(value_type=Option<Vec<Object>>))]
  pub children:Option<Vec<MenuTree>>,
  pub id:i64,
  pub label:String,
}

#[derive(Debug,Serialize,ToSchema,Clone)]
#[serde(rename_all(serialize="camelCase"))]
#[salvo(schema(rename_all="camelCase"))]
pub struct RoleMenuTree{
  pub checked_keys:Vec<i64>,
  pub menus:Vec<MenuTree>,
}


