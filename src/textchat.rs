/* Websocket crate */
use ws::{
    listen, CloseCode, Error, Handler, Handshake, Message, Request, Response, Result, Sender,
};

/* Diesel S10 (*badunts*) ou será que é o vin? ( ͡° ͜ʖ ͡°) */
use diesel::prelude::*;

/* For JSON parsing and reparsing */
use serde::Deserialize;

/* Stuff that I dont really know whats for but hey, we need it. */
use std::cell::Cell;
use std::rc::Rc;

/* Date time for loggin messagins up */
use chrono::Utc;

/* Web socket user counter and sender informations */
#[derive(Debug)]
struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
}

/*
*******************************************************************
-------------------------------------------------------------------
Common socket messages structure that will hold every needed event:
-------------------------------------------------------------------
@fields ${from} => who send
        ${to} => Who will behave on the message
        ${chat_id} => Chat to where to log the message into 
        ${action} => Transaction / Message / leave / balance-update (disconnect)
        ${Optional<message>} => The message if the action is actually a message
        ${transaction} => i32 (The transaction that has been generated and shout into the server)
-------------------------------------------------------------------
*******************************************************************
*/
#[derive(Debug, Deserialize)]
struct SocketMessage {
    from: i32,
    to: i32,
    chat_id: i32,
    action: String,
    message: Option<String>,
    transaction: Option<i32>,
}

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        match req.resource() {
            "/ws" => {
                println!("Browser Request from {:?}", req.origin().unwrap().unwrap());
                let resp = Response::from_request(req);
                resp
            }

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_open(&mut self, _handshake: Handshake) -> Result<()> {
        self.count.set(self.count.get() + 1);
        Ok(())
    }

    fn on_message(&mut self, message: Message) -> Result<()> {
        /* Parses message into a text string that we could actualy use */
        let raw_message = message.clone().into_text()?;
        /*
            If the code from serde_json matches an Ok result the code of the match pattern will
            be execute, else, we will only broadcast things below it.
        */
        println!("{:?}", &raw_message);      
        match serde_json::from_str::<SocketMessage>(&raw_message) {
            Ok(parsed_message) => {
                if parsed_message.action == String::from("message") {
                    use crate::models::NewChatMsg;
                    use crate::schema::chat_msg;
                    match diesel::insert_into(chat_msg::table)
                        .values(NewChatMsg {
                            chat_msg_user_id: parsed_message.from,
                            chat_msg_body: parsed_message.message,
                            chat_msg_time: Utc::now().naive_utc(),
                            chat_id: parsed_message.chat_id,
                        })
                        .execute(&crate::establish_connection()) { 
                        Ok(_) => {}
                        Err(e) => {
                            /* Additional logs */
                            crate::controller::logs::panics::manual_log(
                                String::from("Err-WebSocket"), 
                                String::from("chat_message insertion error."), 
                                e.to_string()
                            );
                        }
                    }
                }
            }
            Err(e) => {
                println!("{}", format!("Parsing content into message failed! message: {:?}, error: {:?}", raw_message, e));                
                /* Additional logs */
                crate::controller::logs::panics::manual_log(
                    String::from("Err-WebSocket"), 
                    format!("Attend on_message raw_message => {:?}", &raw_message), 
                    e.to_string()
                );
            }
        }
        self.out.broadcast(message)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Abnormal => {
                println!("Closing handshake failed! Unable to obtain closing status from client.")
            }
            _ => println!("The client encountered an error: {}", reason),
        }

        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}

pub fn websocket() -> () {
    let count = Rc::new(Cell::new(0));
    println!("Chat TEXT WEBSOCKET running at: 127.0.0.1:9002");

    listen("127.0.0.1:9002", |out| Server {
        out: out,
        count: count.clone(),
    })
    .unwrap()
}
