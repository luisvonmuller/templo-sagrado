/*
  ____ _   _    _  _____   ____ _____ _   _ _____ _____
 / ___| | | |  / \|_   _| / ___|_   _| | | |  ___|  ___|
| |   | |_| | / _ \ | |   \___ \ | | | | | | |_  | |_
| |___|  _  |/ ___ \| |    ___) || | | |_| |  _| |  _|
 \____|_| |_/_/   \_|_|   |____/ |_|  \___/|_|   |_|

*/

/* Common db connection function (dã) */
use crate::establish_connection;

/* Macros. */
use diesel::prelude::*;

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Template */
use rocket_contrib::templates::Template;

/* Form */
use rocket::request::LenientForm;

/* Importing User struct of our session handler */
use crate::{AdminUser, User};

#[get("/user-chats/<user_id>")]
pub fn user_chats(_administrative: AdminUser, user_id: i32) -> Json<String> {
    use crate::models::{Chat, SysUser};
    use crate::schema::{chat, sysuser};

    let results = chat::table
        .inner_join(sysuser::table)
        .select((chat::all_columns, sysuser::all_columns))
        .filter(chat::client_id.eq(user_id))
        .load::<(Chat, SysUser)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/clerk-chats/<user_id>")]
pub fn clerk_chats(_administrative: AdminUser, user_id: i32) -> Json<String> {
    use crate::models::{Chat, SysUser};
    use crate::schema::{chat, sysuser};

    let results = chat::table
        .inner_join(sysuser::table)
        .select((chat::all_columns, sysuser::all_columns))
        .filter(chat::clerk_id.eq(user_id))
        .load::<(Chat, SysUser)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    Json(serde_json::to_string(&results).unwrap())
}

/* Hash map and Datatables */
use rdatatables::*;

/* This one stands for our query data structure */
use crate::models::rdatatables::{DataTablesChats, DataTablesChatsClerk, DatatTablesChatUser};

#[post("/list", data = "<query>")]
pub fn list(
    _adminitrative: AdminUser,
    query: LenientForm<DataTableQuery>,
) -> Json<OutcomeData<(DataTablesChats, DataTablesChatsClerk, DatatTablesChatUser)>> {
    Json(datatables_query::<(
        DataTablesChats,
        DataTablesChatsClerk,
        DatatTablesChatUser,
    )>(
        Tables {
            origin: ("chat", "chat_id"), /* From */
            fields: vec![
                "chat_id",
                "init_time",
                "client_id",
                "clerk_id",
                "clerk_info_exhibition",
                "user_name",
            ], /* Fields to seek for */
            join_targets: Some(vec![
                ("inner", ("clerk_info", "user_id"), ("chat", "clerk_id")),
                ("inner", ("sysuser", "user_id"), ("chat", "client_id")),
            ]), /* Join Targets explained over here */
            datatables_post_query: query.into_inner(), /* Incoming Query */
            query: None,                 /* Our builded query holder */
            condition: None,
        },
        crate::establish_connection(),
    ))
}

#[get("/retrive_whole_chat/<chat_id>")]
pub fn retrive_whole_chat(_administrative: AdminUser, chat_id: i32) -> Json<String> {
    use crate::models::ChatMsg;
    use crate::schema::chat_msg;

    let results = chat_msg::table
        .select(chat_msg::all_columns)
        .filter(chat_msg::chat_id.eq(chat_id))
        .load::<ChatMsg>(&establish_connection())
        .expect("Some error occured when retrieving chat list. It was registered on logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/retrive_whole_chat/<chat_id>", rank = 2)]
pub fn retrive_whole_chat_user(_common_user: User, chat_id: i32) -> Json<String> {
    use crate::models::ChatMsg;
    use crate::schema::chat_msg;

    let results = chat_msg::table
        .select(chat_msg::all_columns)
        .filter(chat_msg::chat_id.eq(chat_id))
        .load::<ChatMsg>(&establish_connection())
        .expect("Some error occured when retrieving chat list. It was registered on logs.");

    Json(serde_json::to_string(&results).unwrap())
}

use rdatatables::Count;

#[get("/total-minutes-transacted/<chat_id>")]
pub fn total_minutes_transacted(chat_id: i32) -> Json<Vec<Count>> {
    use diesel::dsl::sql_query;

    #[allow(non_snake_case)]
    let countUp: Vec<Count> = sql_query(format!(
        "SELECT COUNT(*) FROM text_chat_transaction WHERE text_chat_transaction_chat_id={}",
        chat_id
    ))
    .load(&crate::establish_connection())
    .expect("We cannot count impossible numbers");

    Json(countUp)
}

#[get("/")]
pub fn index(_administrative: AdminUser) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/clerk");
    Template::render("pages/chat/list", &map)
}


/* Since we can have a logged out try, we will import redirect and flash */
use rocket::response::{Flash, Redirect};

#[get("/", rank = 2)]
pub fn index_no_login() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}
