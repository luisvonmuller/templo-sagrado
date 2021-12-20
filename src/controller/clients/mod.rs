/*
    ____ _     ___ _____ _   _ _____    ____ ___  _   _ _____ ____   ___  _     _     _____ ____
  / ___| |   |_ _| ____| \ | |_   _|  / ___/ _ \| \ | |_   _|  _ \ / _ \| |   | |   | ____|  _ \
 | |   | |    | ||  _| |  \| | | |   | |  | | | |  \| | | | | |_) | | | | |   | |   |  _| | |_) |
 | |___| |___ | || |___| |\  | | |   | |__| |_| | |\  | | | |  _ <| |_| | |___| |___| |___|  _ <
 \____|_____|___|_____|_| \_| |_|    \____\___/|_| \_| |_| |_| \_\\___/|_____|_____|_____|_| \_\

*/

pub mod read;

/* Template */
use rocket_contrib::templates::Template;

/* Form */
use rocket::request::LenientForm;

/* Since we can have a logged out try, we will import redirect and flash */
use rocket::response::{Flash, Redirect};

/* Session Struct parser */
use crate::AdminUser;

/* Json type response (must have) */
use rocket_contrib::json::Json;

#[get("/")]
pub fn index(_administrative: AdminUser) -> Template {
    /* Hashmap that will store users our user list information */
    let mut context = std::collections::HashMap::new();
    context.insert("path", "/clients");
    Template::render("pages/client/list", &context)
}

#[get("/", rank = 2)]
pub fn index_no_login() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}

#[derive(Debug, FromForm)]
pub struct UserMinutes {
    edit_user_min_user_id: i32,
    edit_user_balance: f64,
}

#[post("/update-client-balance", data = "<form_data>")]
pub fn update_balance(_administrative: AdminUser, form_data: LenientForm<UserMinutes>) {
    use crate::schema::sysuser;
    use diesel::prelude::*;

    diesel::update(sysuser::table.filter(sysuser::user_id.eq(form_data.edit_user_min_user_id)))
        .set(sysuser::user_balance.eq(form_data.edit_user_balance))
        .execute(&crate::establish_connection())
        .unwrap();
}

#[derive(Debug, FromForm)]
pub struct UserBonus {
    edit_user_min_user_id: i32,
    edit_user_bonus: f64,
}

#[post("/update-client-bonus", data = "<form_data>")]
pub fn update_bonus(_administrative: AdminUser, form_data: LenientForm<UserBonus>) {
    use crate::schema::sysuser;
    use diesel::prelude::*;

    diesel::update(sysuser::table.filter(sysuser::user_id.eq(form_data.edit_user_min_user_id)))
        .set(sysuser::user_bonus.eq(form_data.edit_user_bonus))
        .execute(&crate::establish_connection())
        .unwrap();
}


#[get("/update-client-status/<user_id>/<status>")]
pub fn update_client_status(_administrative: AdminUser, user_id: i32, status: bool) -> Json<bool> {
    use crate::schema::sysuser;
    use diesel::prelude::*;

    diesel::update(sysuser::table.filter(sysuser::user_id.eq(user_id)))
        .set(sysuser::user_status.eq(!status))
        .execute(&crate::establish_connection())
        .expect("Shit happn");

    Json(true)
}