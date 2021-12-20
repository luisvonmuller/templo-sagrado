// Websocket crate
use ws::{
    listen, CloseCode, Error, Handler, Handshake, Message, Request, Response, Result, Sender,
};

//Stuff that I dont really know whats for but hey, we need it. (y)
use std::cell::Cell;
use std::rc::Rc;

// Server web application handler
struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
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

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        self.count.set(self.count.get() + 1);
        let number_of_connection = self.count.get();

        let _open_message = format!(
            "{} entered and the number of live connections is {}",
            &handshake.peer_addr.unwrap(),
            &number_of_connection
        );

        Ok(())
    }

    fn on_message(&mut self, message: Message) -> Result<()> {
        let raw_message = message.into_text()?;
        self.out.broadcast(Message::Text(raw_message.to_string()))
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
    println!("Chat VOICE WEBSOCKET running at: 127.0.0.1:9003");

    listen("127.0.0.1:9003", |out| Server {
        out: out,
        count: count.clone(),
    })
    .unwrap()
}
