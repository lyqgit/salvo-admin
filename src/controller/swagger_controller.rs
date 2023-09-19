use salvo::{handler, Request,Response, FlowCtrl, Depot};
use salvo::prelude::{Redirect, SessionDepotExt, Text};
use salvo::session::Session;

static USER_NAME:&str = "admin";
static PASS_WORD:&str = "salvo-admin2023";

static LOGIN_HTML: &str = r#"<!DOCTYPE html>
<html>
    <head>
        <title>swagger-ui login</title>
    </head>
    <style>
        html,body{
            margin:0;
            padding:0;
            width:100%;
            height:100%;
        }
        .container{
            display:flex;
            align-item:center;
            justify-content:center;
        }
        .form{
            display:flex;
            align-item:center;
            justify-content:center;
            flex-direction:column;
        }
        .mt-20{
            margin-top: 20px;
        }

    </style>
    <body class="container">
        <form class="form" action="/swaggerLogin" method="post">
            <h1>swagger-ui</h1>
            <input type="text" name="username" placeholder="用户名" />
            <input class="mt-20" type="password" name="password" placeholder="密码" />
            <button class="mt-20" type="submit" id="submit">登录</button>
        </form>
    </body>
</html>
"#;

#[handler]
pub async fn auth_token(req:&mut Request,res:&mut Response, ctrl: &mut FlowCtrl,depot: &mut Depot){
    if let Some(session) = depot.session_mut(){
        if let Some(_) = session.get::<String>("swagger-auth"){
            ctrl.call_next(req,depot,res).await;
        }else{
            ctrl.skip_rest();
            res.render(Text::Html(LOGIN_HTML));
        }
    }else{
        ctrl.skip_rest();
        res.render(Text::Html(LOGIN_HTML));
    }

}

#[handler]
pub async fn swagger_login(req:&mut Request,res:&mut Response,depot: &mut Depot){

    let username = req.form::<String>("username").await;
    let password = req.form::<String>("password").await;
    if let (Some(name),Some(pass)) = (username,password){
        if name.eq(USER_NAME) && pass.eq(PASS_WORD){
            let mut session = Session::new();
            session
                .insert("swagger-auth", "salvo-admin")
                .unwrap();
            depot.set_session(session);
            res.render(Redirect::other("/swagger-ui"));
        }
    }
    res.render(Text::Html(LOGIN_HTML));
}