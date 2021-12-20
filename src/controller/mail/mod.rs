/* For sure we need to import the macros from diesel */
use diesel::prelude::*;

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Template */
use rocket_contrib::templates::Template;

/* Importing User struct of our session handler */
use crate::{AdminUser, User};

pub mod create;
pub mod send;

/* Since we can have a logged out try, we will import redirect and flash */
use rocket::response::{Flash, Redirect};

#[get("/")]
pub fn index(_administrative: AdminUser) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/");
    Template::render("pages/mail/index", &map)
}

#[get("/", rank = 2)]
pub fn index_no_login() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}


#[get("/list")]
pub fn list(_administrative: AdminUser) -> Json<String> {
    use crate::models::CallEmail;
    use crate::schema::call_email;

    /* Database connection */
    use crate::establish_connection;

    let results = call_email::table
        .select(call_email::all_columns)
        .load::<CallEmail>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/single/<mail_id>")]
pub fn single(_administrative: AdminUser, mail_id: i32) -> Json<String> {
    use crate::models::CallEmail;
    use crate::schema::call_email;

    /* Database connection */
    use crate::establish_connection;

    let results = call_email::table
        .select(call_email::all_columns)
        .filter(call_email::call_email_id.eq(mail_id))
        .load::<CallEmail>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    Json(serde_json::to_string(&results[0]).unwrap())
}

#[get("/user-list")]
pub fn user_list(_user: User) -> Json<String> {
    use crate::models::CallEmail;
    use crate::schema::call_email;

    /* Database connection */
    use crate::establish_connection;

    let results = call_email::table
        .select(call_email::all_columns)
        .filter(call_email::clerk_id.eq(_user.user_id as i32))
        .load::<CallEmail>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/clerk-list")]
pub fn clerk_list(_user: User) -> Json<String> {
    use crate::models::CallEmail;
    use crate::schema::call_email;

    /* Database connection */
    use crate::establish_connection;

    let results = call_email::table
        .select(call_email::all_columns)
        .filter(call_email::clerk_id.eq(_user.user_id as i32))
        .load::<CallEmail>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    Json(serde_json::to_string(&results).unwrap())
}
