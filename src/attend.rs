/* Websocket crate */
use ws::{
    listen, CloseCode, Error, Handler, Handshake, Message, Request, Response, Result, Sender,
};

/* Diesel S10 (*badunts*) */
/* or vin k */
use diesel::*;

/* For JSON parsing and reparsing */
use serde::Deserialize;

/* Stuff that I dont really know whats for but hey, we need it. */
use std::cell::Cell;
use std::rc::Rc;

/* Datetime for loggin (intends) */
use chrono::Utc;

/* Web socket user counter and sender informations */
#[derive(Debug)]
struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
}

/* Import status enum */
use crate::models::enums::Status;
use crate::models::StatusClerk;

/* For better performance on websocket I'll set-up threads */
use std::thread;

/*

Common socket messages structure that will hold every needed event:
-------------------------------------------------------------------
@fields ${action} => What to execute (Match stmt arms...)
        ${clerk_id  [Nullable i32] <OPTIONAL>} stands for desired clerk or when itselfs send something to the thread
        ${client_id [Nullable i32] <OPTIONAL>} stands for the desired client or when itselfs send something to the thread
        ${data  [Nullable i32] <Optional> } Whenever we register and event, like a new chat, here will be the id to be returned to be used on the redirect by JS
        ${from} => Which side is comming the message, is it from a clerk? Is it from a Client?

-------------------------------------------------------------------

*/
#[derive(Debug, Deserialize)]
struct SocketMessage {
    action: String,
    clerk_id: Option<i32>,
    client_id: Option<i32>,
    data: Option<i32>,
    voice: Option<bool>,
    text: Option<bool>,
    video: Option<bool>,
    from: String,
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

        use crate::schema::status_clerk;

        /* NEW CODE BEGIN */
        match status_clerk::table
            .select(status_clerk::all_columns)
            .load::<StatusClerk>(&crate::establish_connection())
        {
            /* Ok(status_clerk) = Vec<StatusClerk> */
            Ok(status_clerk) => {
                /* Returning the clerk status vec */
                return self.out.broadcast(Message::Text(format!(
                    r#"{{"action":"clerks-available", "data":{}}}"#,
                    serde_json::to_string(&status_clerk).unwrap()
                )));
            }
            Err(e) => {
                crate::controller::logs::panics::manual_log(
                    String::from("Err-WebSocket"),
                    String::from("on_open method error."),
                    e.to_string(),
                );
            }
        } /* End match status_clerk.select... */

        /* Default return in error case OBS: "data": {} */
        self.out.broadcast(Message::Text(format!(
            r#"{{"action":"clerks-available", "data":{}}}"#,
            "{}".to_string()
        )))
        /* NEW CODE END */
    }

    fn on_message(&mut self, message: Message) -> Result<()> {
        use diesel::prelude::*;
        /* Parses message into a text string that we could actualy use */
        let raw_message = message.clone().into_text()?;

        /*
            If the code from serde_json matches an Ok result the code of the match pattern will
            be execute, else, we will only broadcast things below it.
        */
        match serde_json::from_str::<SocketMessage>(&raw_message) {
            Ok(parsed_status) => {
                println!("parsed_status: \n{:?}", parsed_status);
                match parsed_status.from.as_str() {
                    "Clerk" => {
                        match parsed_status.action.as_str() {
                            "voice-chat-acc" => {
                                /* [VOICE] Will select the last one that targets this and update its its status and time to be a accepted oportunity */
                                use crate::schema::intends;

                                /* Since we have a boogy statement here, we will need to have a spooky choice over there to retrieve just the ones that we really wanna to */
                                diesel::update(
                                    intends::table
                                        .filter(
                                            intends::intend_clerk_id
                                                .eq(parsed_status.clerk_id.unwrap()),
                                        )
                                        .filter(
                                            intends::intend_client_id
                                                .eq(parsed_status.client_id.unwrap()),
                                        )
                                        .filter(intends::intend_type.eq(0))
                                        .filter(intends::intend_status.eq(1))
                                        .filter(intends::intend_received_time.is_not_null()),
                                )
                                .set((
                                    intends::intend_status.eq(2),
                                    intends::intend_answer_time.eq(Some(Utc::now().naive_utc())),
                                ))
                                .execute(&crate::establish_connection())
                                .expect("No ids provided");
                            }
                            "voice-chat-received" => {
                                /* [VOICE] Will select the last one that targets this and update its its status and time to be a accepted oportunity */
                                use crate::schema::intends;

                                /* Since we have a boogy statement here, we will need to have a spooky choice over there to retrieve just the ones that we really wanna to */
                                diesel::update(
                                    intends::table
                                        .filter(
                                            intends::intend_clerk_id
                                                .eq(parsed_status.clerk_id.unwrap()),
                                        )
                                        .filter(
                                            intends::intend_client_id
                                                .eq(parsed_status.client_id.unwrap()),
                                        )
                                        .filter(intends::intend_type.eq(0))
                                        .filter(intends::intend_status.eq(0)),
                                )
                                .set((
                                    intends::intend_status.eq(1),
                                    intends::intend_received_time.eq(Some(Utc::now().naive_utc())),
                                ))
                                .execute(&crate::establish_connection())
                                .expect("No ids provided");
                            }
                            "voice-chat-ref" => {
                                /* [VOICE] Will select the last one that targets this and update its its status and time to be a refused oportunity */
                                use crate::schema::intends;

                                diesel::update(
                                    intends::table
                                        .filter(
                                            intends::intend_clerk_id
                                                .eq(parsed_status.clerk_id.unwrap()),
                                        )
                                        .filter(
                                            intends::intend_client_id
                                                .eq(parsed_status.client_id.unwrap()),
                                        )
                                        .filter(intends::intend_type.eq(0))
                                        .filter(intends::intend_status.eq(1))
                                        .filter(intends::intend_received_time.is_not_null()),
                                )
                                .set((
                                    intends::intend_status.eq(3),
                                    intends::intend_answer_time.eq(Some(Utc::now().naive_utc())),
                                ))
                                .execute(&crate::establish_connection())
                                .expect("No ids provided");
                            }
                            "text-chat-acc" => {
                                /* [TEXT] Will select the last one that targets this and update its its status and time to be a accepted oportunity */
                                use crate::schema::intends;

                                diesel::update(
                                    intends::table
                                        .filter(
                                            intends::intend_clerk_id
                                                .eq(parsed_status.clerk_id.unwrap()),
                                        )
                                        .filter(
                                            intends::intend_client_id
                                                .eq(parsed_status.client_id.unwrap()),
                                        )
                                        .filter(intends::intend_type.eq(1))
                                        .filter(intends::intend_status.eq(1))
                                        .filter(intends::intend_received_time.is_not_null()),
                                )
                                .set((
                                    intends::intend_status.eq(2),
                                    intends::intend_answer_time.eq(Some(Utc::now().naive_utc())),
                                ))
                                .execute(&crate::establish_connection())
                                .expect("No ids provided");
                            }
                            "text-chat-received" => {
                                /* [VOICE] Will select the last one that targets this and update its its status and time to be a accepted oportunity */
                                use crate::schema::intends;

                                /* Since we have a boogy statement here, we will need to have a spooky choice over there to retrieve just the ones that we really wanna to */
                                diesel::update(
                                    intends::table
                                        .filter(
                                            intends::intend_clerk_id
                                                .eq(parsed_status.clerk_id.unwrap()),
                                        )
                                        .filter(
                                            intends::intend_client_id
                                                .eq(parsed_status.client_id.unwrap()),
                                        )
                                        .filter(intends::intend_type.eq(1))
                                        .filter(intends::intend_status.eq(0)),
                                )
                                .set((
                                    intends::intend_status.eq(1),
                                    intends::intend_received_time.eq(Some(Utc::now().naive_utc())),
                                ))
                                .execute(&crate::establish_connection())
                                .expect("No ids provided");
                            }
                            "text-chat-ref" => {
                                /* [TEXT] Will select the last one that targets this and update its its status and time to be a refused oportunity */
                                use crate::schema::intends;

                                diesel::update(
                                    intends::table
                                        .filter(
                                            intends::intend_clerk_id
                                                .eq(parsed_status.clerk_id.unwrap()),
                                        )
                                        .filter(
                                            intends::intend_client_id
                                                .eq(parsed_status.client_id.unwrap()),
                                        )
                                        .filter(intends::intend_type.eq(1))
                                        .filter(intends::intend_status.eq(1))
                                        .filter(intends::intend_received_time.is_not_null()),
                                )
                                .set((
                                    intends::intend_status.eq(3),
                                    intends::intend_answer_time.eq(Some(Utc::now().naive_utc())),
                                ))
                                .execute(&crate::establish_connection())
                                .expect("No ids provided");
                            }

                            "im-in" => {
                                /*
                                This pattern will make the clerk available on the database, also, it will broadcast all clerks that are available by now
                                on the datatabase table that holds all our status references

                                post-fix: Maybe, in a far future, that will be niecer to implement a RC::cell that holds each clerk id conected, and by so
                                hold a possible disconnect behaviour event.
                                By now, myself, luís, do not know how to do it.
                                */
                                use crate::schema::status_clerk;

                                /* Fast fix */
                                let thread_mail_status = parsed_status.clerk_id.unwrap().to_owned();
                                
                                /* This will trigger the e-mail leads*/
                                let _good_mail = thread::Builder::new()
                                    .name("Mailing soulmates service ".into())
                                    .spawn(move || {
                                        crate::controller::home::notify_my_soul_mates(
                                            thread_mail_status,
                                        );
                                    })
                                    .unwrap();

                                /* Updates status clerk table, who holds information about availables, busy and non-availables clerks */
                                diesel::update(status_clerk::table.filter(
                                    status_clerk::clerk_id.eq(parsed_status.clerk_id.unwrap()),
                                ))
                                .set((
                                    status_clerk::status.eq(Status::Online as i32),
                                    status_clerk::is_available_chat.eq(parsed_status.text.unwrap()),
                                    status_clerk::is_available_voice
                                        .eq(parsed_status.voice.unwrap()),
                                    status_clerk::is_available_video
                                        .eq(parsed_status.video.unwrap()),
                                    status_clerk::is_available_mail.eq(true),
                                ))
                                .execute(&crate::establish_connection())
                                .unwrap();

                                /* Query and Streams back all clerks status */
                                let results: Vec<StatusClerk> = status_clerk::table
                                    .select(status_clerk::all_columns)
                                    .load::<StatusClerk>(&crate::establish_connection())
                                    .expect(
                                        "Some Error occured while parsing cookie absolute value. Registered in logs.",
                                    );

                                self.out
                                    .broadcast(Message::Text(format!(
                                        r#"{{"action":"clerks-available", "data":{}}}"#,
                                        serde_json::to_string(&results).unwrap()
                                    )))
                                    .expect("None message has been transmitted");
                            }
                            "im-busy" => {
                                /*
                                This pattern will be executed as an update for displaying that the clerk is busy (em atendimento on brazilian portuguese)

                                post-fix: Maybe, in a far future, that will be niecer to implement a RC::cell that holds each clerk id conected, and by so
                                hold a possible disconnect behaviour event.
                                By now, myself, luís, do not know how to do it.
                                */
                                use crate::schema::status_clerk;

                                /*
                                Updates the self database reference that will set them as a busy
                                */

                                diesel::update(status_clerk::table.filter(
                                    status_clerk::clerk_id.eq(parsed_status.clerk_id.unwrap()),
                                ))
                                .set(status_clerk::status.eq(Status::Oncall as i32))
                                .execute(&crate::establish_connection())
                                .unwrap();

                                /* Query and Streams back all clerks status */
                                let results: Vec<StatusClerk> = status_clerk::table
                                    .select(status_clerk::all_columns)
                                    .load::<StatusClerk>(&crate::establish_connection())
                                    .expect(
                                        "Some Error occured while parsing cookie absolute value. Registered in logs.",
                                    );

                                self.out
                                    .broadcast(Message::Text(format!(
                                        r#"{{"action":"clerks-available", "data":{}}}"#,
                                        serde_json::to_string(&results).unwrap()
                                    )))
                                    .expect("None message has been transmitted");
                            }
                            "im-out" => {
                                use crate::schema::status_clerk;
                                /*
                                This pattern will be executed as an update for displaying that the clerk is offline (indisponível)

                                post-fix:  Maybe, in a far future, that will be niecer to implement a RC::cell that holds each clerk id conected, and by so
                                hold a possible disconnect behaviour event.
                                By now, myself, luís, do not know how to do it.
                                */
                                diesel::update(status_clerk::table.filter(
                                    status_clerk::clerk_id.eq(parsed_status.clerk_id.unwrap()),
                                ))
                                .set(status_clerk::status.eq(Status::Offline as i32))
                                .execute(&crate::establish_connection())
                                .unwrap();

                                /* Query and Streams back all clerks status */
                                let results: Vec<StatusClerk> = status_clerk::table
                                    .select(status_clerk::all_columns)
                                    .load::<StatusClerk>(&crate::establish_connection())
                                    .expect(
                                        "Some Error occured while parsing cookie absolute value. Registered in logs.",
                                    );

                                self.out
                                    .broadcast(Message::Text(format!(
                                        r#"{{"action":"clerks-available", "data":{}}}"#,
                                        serde_json::to_string(&results).unwrap()
                                    )))
                                    .expect("None message has been transmitted");
                            }
                            &_ => {
                                //Unregistered (and unexpected) pattern
                            }
                        }
                    }
                    "Client" => {
                        match parsed_status.action.as_str() {
                            "new-client-entered-thread" => {
                                /* This patterns executes when a live soul enters the website to give back all clients */
                                use crate::schema::status_clerk;
                                /* Query and Streams back all clerks status */
                                let results: Vec<StatusClerk> = status_clerk::table
                                .select(status_clerk::all_columns)
                                .load::<StatusClerk>(&crate::establish_connection())
                                .expect(
                                    "Some Error occured while parsing cookie absolute value. Registered in logs.",
                                );

                                self.out
                                    .broadcast(Message::Text(format!(
                                        r#"{{"action":"clerks-available", "data":{}}}"#,
                                        serde_json::to_string(&results).unwrap()
                                    )))
                                    .expect("None message has been transmitted");
                            }
                            "voice-chat-intend" => {
                                use crate::models::NewIntend;
                                use crate::schema::intends;

                                diesel::insert_into(intends::table)
                                    .values(NewIntend {
                                        intend_clerk_id: parsed_status.clerk_id.unwrap(),
                                        intend_client_id: parsed_status.client_id.unwrap(),
                                        intend_status: 0,
                                        intend_type: 0,
                                        intend_ask_time: Utc::now().naive_utc(),
                                        intend_received_time: None,
                                        intend_answer_time: None,
                                    })
                                    .execute(&crate::establish_connection())
                                    .expect("No ids provided");
                            }
                            "text-chat-intend" => {
                                use crate::models::NewIntend;
                                use crate::schema::intends;
                                diesel::insert_into(intends::table)
                                    .values(NewIntend {
                                        intend_clerk_id: parsed_status.clerk_id.unwrap(),
                                        intend_client_id: parsed_status.client_id.unwrap(),
                                        intend_status: 0,
                                        intend_type: 1,
                                        intend_ask_time: Utc::now().naive_utc(),
                                        intend_received_time: None,
                                        intend_answer_time: None,
                                    })
                                    .execute(&crate::establish_connection())
                                    .expect("No ids provided");
                            }
                            &_ => {
                                //Unregistered (and unexpected) pattern
                            }
                        }
                    }
                    "Administrative" => {
                        //Nothing by now
                    }
                    &_ => {}
                }
            }
            Err(e) => {
                println!(
                    "{}",
                    format!(
                        "Parsing content into status failed! message: {:?}, error: {:?}",
                        raw_message, e
                    )
                );
                /* Additional logs */
                crate::controller::logs::panics::manual_log(
                    String::from("Err-WebSocket"),
                    format!("Attend on_message raw_message => {:?}", &raw_message),
                    e.to_string(),
                );
            }
        }
        /*
            Down here we will match all possible patterns that incomes
            to the socket, as well the non covered ones.
        */
        self.out.broadcast(message)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        /*
        Magic stuff of websocket implementations, when I coded this only me and god knew what
        I was doing, now, only god knows.
        */
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
        /* This piece here never executes when it shoulds and the thread always panic, this is a fail fail safe method */
        println!("The server encountered an error: {:?}", err);
    }
}

pub fn websocket() -> () {
    let count = Rc::new(Cell::new(0));
    println!("WEBSOCKET de atendimento running at: 127.0.0.1:9001");

    /* Here you change */
    listen("127.0.0.1:9001", |out| Server {
        out: out,
        count: count.clone(),
    })
    .unwrap()
}
