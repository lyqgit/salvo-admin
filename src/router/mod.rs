use salvo::{Router,Service};
use salvo::serve_static::StaticDir;
use salvo::logging::Logger;
use salvo::catcher::Catcher;
use salvo::oapi::{Info, OpenApi};
use salvo::prelude::CatchPanic;
use salvo::oapi::swagger_ui::SwaggerUi;
use crate::controller::common_controller;


pub mod user_router;
pub mod dict_router;
pub mod menu_router;
pub mod role_router;
pub mod dept_router;


pub fn init_router()->Router{
    let static_dir = Router::with_path("/static").get(
        StaticDir::new([
            "static/",
        ])
            .listing(true),
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

        );


    let doc = OpenApi::new(Info::new("后台接口文档", "0.0.1")).merge_router(&router);
    let router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));
    router
}

pub fn init_service()->Service{
    let router = init_router();
    Service::new(router).catcher(Catcher::default().hoop(common_controller::catcher_err))
}