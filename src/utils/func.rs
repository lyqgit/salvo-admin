use crate::model::menu_model::{Router,Meta};
use crate::entity::sys_menu_entity::SysMenu;

pub fn router_arr_to_tree(mut re_list:Vec<Router>,ori_arr:Vec<SysMenu>,pid:i64){
  for (_,it) ori_arr.iter().enumerate(){
    if pid == it.parent_id{
      let tempMeta = Meta{
        
      }

      let tempRouter = Router{
        always_show:it.visible.eq("0"),
        children:Some(Vec::<Router>::new()),
        component:it.component.map_or(String::from("ParentView"), |v|{
            if v.is_empty(){
              String::from("Layout")
            }else{
              v
            }
          
          }
        ),
        hidden:it.status.eq("1"),
        name:it.path,
        path:it.path,
        redirect:(||->Option<String>{
          if it.is_frame.eq(other)
          Some(String::from("value"))
        })()
      };
    }
  }
}