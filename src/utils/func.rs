use salvo::oapi::ToSchema;
use crate::model::menu_model::{Router, Meta, MenuTree, SysMenuPage};
use crate::entity::sys_menu_entity::SysMenu;
use crate::model::common_model::Page;
use crate::model::dept_model::{DeptList, DeptTree};

// 路由数组转树
pub fn router_arr_to_tree2(ori_arr:Vec<SysMenu>,pid:i64)->Vec<Router>{
  let mut re_list = Vec::<Router>::new();
  for (_,it) in ori_arr.iter().enumerate(){
    if pid == it.parent_id && !it.menu_type.eq("F"){
      let temp_meta = Meta{
        title:it.menu_name.clone(),
        icon:it.icon.clone(),
        link:(||->Option<String>{
          if it.is_frame == 0{
            Some(it.path.clone())
          }else{
            None
          }
        })(),
        no_cache:it.is_cache==1
      };

      let children = router_arr_to_tree2(ori_arr.clone(),it.menu_id);

      let temp_router = Router{
        always_show:(||->Option<bool>{
          if it.visible.eq("0") && !it.menu_type.eq("C") && it.is_frame == 1{
            Some(true)
          }else{
            None
          }
        })(),
        children:(||->Option<Vec<Router>>{
          if children.len()>0{
            Some(children)
          }else{
            None
          }
        })(),
        component:it.component.clone().map_or(String::from("Layout"), |v|{
          if v.is_empty() && it.parent_id == 0 && it.menu_type.eq("M"){
            String::from("Layout")
          }else if it.parent_id != 0 && it.menu_type.eq("M"){
            String::from("ParentView")
          }else{
            v
          }

        }
        ),
        hidden:it.status.eq("1"),
        name:it.path.clone(),
        path:(||->String{
          if it.menu_type.eq("C") || it.is_frame == 0{
            it.path.clone()
          }else if it.menu_type.eq("M") && it.parent_id != 0{
            it.path.clone()
          }else{
            "/".to_owned()+&it.path
          }
        })(),
        redirect:(||->Option<String>{
          if it.is_frame == 1 && it.menu_type.eq("M"){
            Some(String::from("noRedirect"))
          }else{
            None
          }
        })(),
        meta:temp_meta
      };
      re_list.push(temp_router);
    }
  }
  re_list
}

// 路由数组转树
#[allow(dead_code)]
#[allow(unused_must_use)]
pub fn router_arr_to_tree(re_list:&mut Vec<Router>,ori_arr:Vec<SysMenu>,pid:i64){
  for (_,it) in ori_arr.iter().enumerate(){
    if pid == it.parent_id && !it.menu_type.eq("F"){
      let temp_meta = Meta{
        title:it.menu_name.clone(),
        icon:it.icon.clone(),
        link:(||->Option<String>{
          if it.is_frame == 0{
            Some(it.path.clone())
          }else{
            None
          }
        })(),
        no_cache:it.is_cache==1
      };

      let mut children = Vec::<Router>::new();
      router_arr_to_tree(&mut children,ori_arr.clone(),it.menu_id);

      let temp_router = Router{
        always_show:(||->Option<bool>{
          if it.visible.eq("0") && !it.menu_type.eq("C") && it.is_frame == 1{
            Some(true)
          }else{
            None
          }
        })(),
        children:(||->Option<Vec<Router>>{
          if children.len()>0{
            Some(children)
          }else{
            None
          }
        })(),
        component:it.component.clone().map_or(String::from("Layout"), |v|{
            if v.is_empty() && it.parent_id == 0 && it.menu_type.eq("M"){
              String::from("Layout")
            }else if it.parent_id != 0 && it.menu_type.eq("M"){
              String::from("ParentView")
            }else{
              v
            }
          
          }
        ),
        hidden:it.status.eq("1"),
        name:it.path.clone(),
        path:(||->String{
          if it.menu_type.eq("C") || it.is_frame == 0{
            it.path.clone()
          }else if it.menu_type.eq("M") && it.parent_id != 0{
            it.path.clone()
          }else{
            "/".to_owned()+&it.path
          }
        })(),
        redirect:(||->Option<String>{
          if it.is_frame == 1 && it.menu_type.eq("M"){
            Some(String::from("noRedirect"))
          }else{
            None
          }
        })(),
        meta:temp_meta
      };
      re_list.push(temp_router)
    }
  }
}

// 菜单数组转树
pub fn menu_arr_to_tree(re_list:&mut Vec<MenuTree>,ori_arr:Vec<SysMenuPage>,pid:i64){
  for (_,it) in ori_arr.iter().enumerate(){
    if pid == it.parent_id && !it.menu_type.eq("F"){


      let mut children = Vec::<MenuTree>::new();
      menu_arr_to_tree(&mut children,ori_arr.clone(),it.menu_id);

      let temp_router = MenuTree{
        children:(||->Option<Vec<MenuTree>>{
          if children.len()>0{
            Some(children)
          }else{
            None
          }
        })(),
        id:it.menu_id,
        label:it.menu_name.clone()
      };
      re_list.push(temp_router)
    }
  }
}


// 岗位数组转树
pub fn dept_arr_to_tree(re_list:&mut Vec<DeptTree>,ori_arr:Vec<DeptList>,pid:i64){
  for (_,it) in ori_arr.iter().enumerate(){
    if pid == it.parent_id.unwrap(){


      let mut children = Vec::<DeptTree>::new();
      dept_arr_to_tree(&mut children,ori_arr.clone(),it.dept_id.clone().unwrap());

      let temp_router = DeptTree{
        children:(||->Option<Vec<DeptTree>>{
          if children.len()>0{
            Some(children)
          }else{
            None
          }
        })(),
        id:it.dept_id.clone().unwrap(),
        label:it.dept_name.clone().unwrap()
      };
      re_list.push(temp_router)
    }
  }
}

pub fn create_page(page_num:u64,page_size:u64)->(u64,u64){
  
  let mut size = 10;
  if page_size >1{
    size = page_size
  }
  let mut num = 0;
  if page_num >1{
    num = (page_num - 1)*size
  }
  (num,size)
}

pub fn is_modify_ok(affected:u64)->bool{
  if affected>=1{
    true
  }else{
    false
  }
}

pub fn create_page_list<T:ToSchema>(rows:Vec<T>,total:u64)->Page<T>{
  Page{
    rows,
    total
  }
}
