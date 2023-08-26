use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use futures_util::{FutureExt, StreamExt};
use once_cell::sync::Lazy;
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;

use salvo::prelude::*;
use salvo::websocket::{Message, WebSocket, WebSocketUpgrade};
use crate::model::chat_model::{ChatRxMsg, ChatTxMsg};
use crate::utils::redis;
use crate::utils::res::ResObj;

type Users = RwLock<HashMap<usize, mpsc::UnboundedSender<Result<Message, salvo::Error>>>>;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);
static ONLINE_USERS: Lazy<Users> = Lazy::new(Users::default);


#[handler]
pub async fn user_connected(req: &mut Request, res: &mut Response) -> Result<(), StatusError> {
    WebSocketUpgrade::new().upgrade(req, res, handle_socket).await
}

async fn handle_socket(ws: WebSocket) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);


    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    let fut = rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            println!("websocket send error---{:?}",e);
        }
    });
    tokio::task::spawn(fut);
    let fut = async move {
        ONLINE_USERS.write().await.insert(my_id, tx);

        while let Some(result) = user_ws_rx.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("websocket error(uid={my_id}): {e}");
                    break;
                }
            };
            user_message(my_id, msg).await;
        }

        user_disconnected(my_id).await;
    };
    tokio::task::spawn(fut);
}

async fn user_message(my_id: usize, msg: Message) {
    let rx_str = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };



    let mut code = 0;

    let mut rx_msg = ChatRxMsg{token:String::new(),name:String::new(),msg:String::new()};

    match serde_json::from_str(rx_str) {
        Ok(v)=>{
            rx_msg = v;
        },
        Err(e)=>{
            println!("websocket rx msg---{:?}",e);
            code = 500
        }
    }


    if let Ok(_) = redis::get::<i32,&str>(rx_msg.token.as_str()){
        // token验证通过，可以发送消息
    }else{
        code = 401
    }

    let res:ResObj<ChatTxMsg> = ResObj{
        code,
        data:(||->Option<ChatTxMsg>{
            if code == 0{
                Some(ChatTxMsg{
                    name:rx_msg.name.clone(),
                    msg:rx_msg.msg.clone()
                })
            }else{
                None
            }
        })(),
        msg:(||->String{
            if code == 0{
                String::from("访问成功")
            }else if code == 401{
                String::from("拒绝通信")
            }else{
                String::from("服务器发生错误")
            }
        })()
    };

    let new_msg = serde_json::to_string(&res).unwrap();

    // New message from this user, send it to everyone else (except same uid)...
    for (&uid, tx) in ONLINE_USERS.read().await.iter() {
        if my_id != uid {
            if let Err(_disconnected) = tx.send(Ok(Message::text(new_msg.clone()))) {
                // The tx is disconnected, our `user_disconnected` code
                // should be happening in another task, nothing more to
                // do here.
            }
        }
    }
}

async fn user_disconnected(my_id: usize) {
    eprintln!("good bye user: {my_id}");
    // Stream closed up, so remove from the user list
    ONLINE_USERS.write().await.remove(&my_id);
}
