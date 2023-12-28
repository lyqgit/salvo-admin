use salvo::{Router,Service};
use salvo::serve_static::StaticDir;
use salvo::logging::Logger;
use salvo::catcher::Catcher;
use salvo::oapi::{OpenApi};
use salvo::prelude::{CatchPanic, SessionHandler};
use salvo::oapi::swagger_ui::SwaggerUi;
use salvo::session::CookieStore;
use crate::controller::common_controller;
use crate::controller::swagger_controller;


pub mod user_router;
pub mod dict_router;
pub mod menu_router;
pub mod role_router;
pub mod dept_router;
pub mod post_router;
pub mod monitor_router;
pub mod excel_router;


pub fn init_router()->Router{
    let static_dir = Router::with_path("/static/<*path>").get(
        StaticDir::new([
            "static/",
        ]).auto_list(true),
    );

    let router = Router::new().hoop(Logger::new())
        .hoop(CatchPanic::new())
        .push(static_dir)
        .push(
            user_router::init_router_no_token()
        )
        .push(
            // 需要验证的api
            Router::new().hoop(common_controller::auth_token)
                .push(
                    user_router::init_router()
                )
                .push(
                    dict_router::init_router()
                )
                .push(
                    menu_router::init_router()
                )
                .push(
                role_router::init_router()
                )
                .push(
                    dept_router::init_router()
                )
                .push(
                    post_router::init_router()
                )
                .push(
                    monitor_router::init_router()
                )
                .push(
                    excel_router::init_router()
                )

        );

    let session_handler = SessionHandler::builder(
        CookieStore::new(),
        b"salvo-adminsalvo-adminalvo-adminsalvo-admin2023salvo-admin2023salvo-admin2023",
    )
        .build()
        .unwrap();
    let doc = OpenApi::new("后台接口文档", "0.2.1").tags(["用户","路由","角色","菜单","部门","字典","岗位","系统","文档"]).merge_router(&router);
    let router = router
        .push(
        Router::new().hoop(session_handler).push(
            Router::new().hoop(swagger_controller::auth_token)
                .push(doc.into_router("/api-doc/openapi.json"))
                .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
            )
                .push(
                Router::with_path("/swaggerLogin").post(swagger_controller::swagger_login)
            )
        );

    router
}

pub fn init_service()->Service{
    let router = init_router();
    Service::new(router).catcher(Catcher::default().hoop(common_controller::catcher_err))
}