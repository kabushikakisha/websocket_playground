use std::collections::HashSet;
use actix::{Actor, Addr, Context, Handler};
use crate::actor::{Connect, Disconnect, WebSocket, BroadcastMessage};

pub struct WsSessionManager {
    // A set to track the addresses of connected WebSocket actors
    sessions: HashSet<Addr<WebSocket>>,

    // Stores the last text message broadcast to clients
    pub(crate) last_text: String,
}

impl WsSessionManager {
    pub(crate)  fn new() -> Self {
        let default_str = "{Hello, there}";

        Self {
            sessions: HashSet::new(),
            last_text: String::from(default_str),
        }
    }
}

impl Actor for WsSessionManager {
    type Context = Context<Self>;
}

impl Handler<Connect> for WsSessionManager {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        println!("New Client Connected");

        let addr = msg.addr;

        self.sessions.insert(addr.clone());
    }
}

impl Handler <Disconnect> for WsSessionManager {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("Client disconnected");

        self.sessions.remove(&msg.addr);
    }
}

impl Handler<BroadcastMessage> for WsSessionManager {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Context<Self>) {
        self.last_text = msg.msg.clone();

        for addr in &self.sessions {
            if *addr != msg.sender {
                let msg_clone = msg.clone();

                addr.do_send(msg_clone);
            }
        }
    }
}
