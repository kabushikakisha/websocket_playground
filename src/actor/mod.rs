use actix::{Actor, Addr, AsyncContext, Handler, Message, Running, StreamHandler};
use actix_web_actors::ws;
use crate::session_manager::WsSessionManager;

// -----------------------------------------------------------
// Message Structs for WebSocket Events
// ----------------------------

#[derive(Message)]
#[rtype(result ="()")]
pub struct Connect {
    pub(crate) addr: Addr<WebSocket>,
}

#[derive(Message)]
#[rtype(result ="()")]
pub struct Disconnect {
    pub(crate) addr: Addr<WebSocket>,
}

// message to be broadcast to all connected clients
// msg - content to be sent
// sender - address of actor that sent msg
#[derive(Message, Clone)] // Clone is derived to allow easy duplication of the message
#[rtype(result ="()")]
pub struct BroadcastMessage {
    pub(crate) msg: String,
    pub(crate) sender: Addr<WebSocket>
}

#[derive(Message)]
#[rtype(result ="()")]
pub struct DefaultMessage {
    pub text: String,
}

impl Handler<DefaultMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: DefaultMessage, ctx: &mut Self::Context) {
        ctx.text(msg.text);
    }
}

impl Handler<BroadcastMessage> for WebSocket {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        ctx.text(msg.msg);
    }
}

pub struct WebSocket {
    pub(crate) manager: Addr<WsSessionManager>,
}

// implementing Actor trait, so WS can be used as an Actix Actor
impl Actor for WebSocket {

    type Context = ws::WebsocketContext<Self>; // defines the type of context used by the websocket actor

    fn started(&mut self, ctx: &mut Self::Context) {
        self.manager.do_send(Connect {
            addr: ctx.address(), // send the address of this WS actor to the session manager
        });
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        self.manager.do_send(Disconnect {
            addr: ctx.address(),
        });
        Running::Stop // indicates to the actor that the runtime should stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        //check if the message is a valid TEXT message
        if let Ok(ws::Message::Text(text)) = msg{
            self.manager.do_send(BroadcastMessage {
                msg: text.to_string(),
                sender: ctx.address(),
            });
        }
    }
}