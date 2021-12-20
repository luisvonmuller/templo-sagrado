/*
888      8888888 .d8888b. 88888888888             d8888 888b    888 8888888b.       8888888b.  8888888888        d8888 8888888b.
888        888  d88P  Y88b    888                d88888 8888b   888 888  "Y88b      888   Y88b 888              d88888 888  "Y88b
888        888  Y88b.         888               d88P888 88888b  888 888    888      888    888 888             d88P888 888    888
888        888   "Y888b.      888              d88P 888 888Y88b 888 888    888      888   d88P 8888888        d88P 888 888    888
888        888      "Y88b.    888             d88P  888 888 Y88b888 888    888      8888888P"  888           d88P  888 888    888
888        888        "888    888            d88P   888 888  Y88888 888    888      888 T88b   888          d88P   888 888    888
888        888  Y88b  d88P    888           d8888888888 888   Y8888 888  .d88P      888  T88b  888         d8888888888 888  .d88P
88888888 8888888 "Y8888P"     888          d88P     888 888    Y888 8888888P"       888   T88b 8888888888 d88P     888 8888888P"

*/

// Common db connection function (dã)
use crate::establish_connection;

//Struct
use crate::models::SysUser;

//Macros.
use diesel::prelude::*;

//Json type response (must have)
use rocket_contrib::json::Json;

/* SESSION HANDLING */
use crate::User;

#[get("/list", format = "json", rank = 1)]
pub fn list() -> Json<String> {
    use crate::schema::sysuser;
    use crate::schema::user_type;

    let results = sysuser::table
        .inner_join(user_type::table)
        .select((sysuser::all_columns, user_type::user_type_title))
        .load::<(SysUser, String)>(&establish_connection())
        .expect("Some shit happned while retrieving the full list of users.!!! <Panic at the Disco> ops, thread!");

    Json(serde_json::to_string(&results).unwrap())
}

/* Every function that handles a rank  it's a func that holds no-allowed users to acess our data */
#[get("/list", format = "json", rank = 2)]
pub fn list_hack() -> Json<&'static str> {
    Json("You are not allowed to reach this content. Please, leave or dont't. You will not access it anyways.")
}

#[get("/self_data", format = "json")]
pub fn self_data(user: User) -> Json<String> {
    use crate::models::{Address, Phone};
    use crate::schema::{address, phone, sysuser};

    let results = sysuser::table
        .inner_join(address::table)
        .inner_join(phone::table)
        .select((
            sysuser::all_columns,
            address::all_columns,
            phone::all_columns,
        ))
        .filter(sysuser::user_id.eq(user.user_id as i32))
        .load::<(SysUser, Address, Phone)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/client_chat_list")]
pub fn client_chat_list(user: User) -> Json<String> {
    use crate::models::Chat;
    use crate::schema::chat;
    let results = chat::table
        .select(chat::all_columns)
        .filter(chat::client_id.eq(user.user_id as i32))
        .load::<Chat>(&establish_connection())
        .expect("Some error occured when retrieving chat list. It was registered on logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/client-mail-list")]
pub fn client_mail_list(user: User) -> Json<String> {
    use crate::models::CallEmail;
    use crate::schema::call_email;
    let results = call_email::table
        .select(call_email::all_columns)
        .filter(call_email::user_id.eq(user.user_id as i32))
        .load::<CallEmail>(&establish_connection())
        .expect("Some error occured when retrieving chat list. It was registered on logs.");

    Json(serde_json::to_string(&results).unwrap())
}


#[get("/client-transaction-list")]
pub fn client_sales_list(user: User) -> Json<String> {
    use crate::models::Sale;
    use crate::schema::sale;
    let results = sale::table
        .select(sale::all_columns)
        .filter(sale::user_id.eq(user.user_id as i32))
        .load::<Sale>(&establish_connection())
        .expect("Some error occured when retrieving chat list. It was registered on logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/get_clerk_name/<clerk_id>")]
pub fn get_clerk_name(clerk_id: i32, _user: User) -> Json<String> {
    use crate::schema::sysuser;

    let results = sysuser::table
        .select(sysuser::user_name)
        .filter(sysuser::user_id.eq(clerk_id))
        .load::<String>(&establish_connection())
        .expect("Some error occured when retrieving chat list. It was registered on logs.");

    Json(serde_json::to_string(&results).unwrap())
}
