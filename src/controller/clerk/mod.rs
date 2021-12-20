pub mod create;
pub mod read;
pub mod update;
pub mod schedule;

/* Common db connection function */
use crate::establish_connection;

/* Database Macros. */
use diesel::prelude::*;

/* Json type response  */
use rocket_contrib::json::Json;

/* Template */
use rocket_contrib::templates::Template;

/* Importing User struct of our session handler */
use crate::AdminUser;

/* Since we can have a logged out try, we will import redirect and flash */
use rocket::response::{Flash, Redirect};

#[get("/")]
pub fn index(_adminstrative: AdminUser) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/clerk");
    Template::render("pages/clerk/list", &map)
}

#[get("/", rank = 2)]
pub fn index_no_login() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}

#[get("/change-status/<user_id>/<status>")]
pub fn change_status(_adminstrative: AdminUser, user_id: i32, status: bool) -> Json<String> {
    use crate::schema::sysuser;

    diesel::update(sysuser::table.filter(sysuser::user_id.eq(user_id)))
        .set(sysuser::user_status.eq(!status))
        .execute(&establish_connection())
        .unwrap();

    Json(serde_json::to_string(&!status).unwrap())
}

#[get("/payDebts/<user_id>")]
pub fn pay_debts(_adminstrative: AdminUser, user_id: i32) {
    use crate::schema::sysuser;

    diesel::update(sysuser::table.filter(sysuser::user_id.eq(user_id)))
        .set(sysuser::user_balance.eq(0.0))
        .execute(&crate::establish_connection())
        .unwrap();
}

#[derive(Debug, FromForm)]
pub struct UserMinutes {
    edit_user_min_chat: i32,
    edit_user_min_voip: i32,
    edit_user_amount_email: i32,
    edit_user_min_user_id: i32,
}
