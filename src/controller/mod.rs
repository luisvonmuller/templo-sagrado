pub mod admin;
pub mod banners; /* Banners */
pub mod blog;
pub mod chat; /* Static messaging chat */
pub mod clerk;
pub mod clients;
pub mod config;
pub mod home;
pub mod intends;
pub mod logs;
pub mod mail;
pub mod pages;
pub mod payments;
pub mod product;
pub mod reports;
pub mod sales;
pub mod stats; /* statistics */
pub mod testimonials;
pub mod transactions;
pub mod user;
pub mod voice;

/* Session Handling Imports */
use rocket::http::{Cookie, Cookies};

/* Template */
use rocket_contrib::templates::Template;

/* Hash map */
use std::collections::HashMap;

/* File fairing */
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

//Json type response (must have)
use rocket_contrib::json::Json;

/* Stabilishing connections to db */
use crate::establish_connection;

/* Table macros */
use diesel::prelude::*;

/* Struct for session handling */
use crate::AdminUser;

/* Redirects, flash messages and other stuffs */
use rocket::request::FlashMessage;
use rocket::request::Form;
use rocket::response::{Flash, Redirect};

/* Functions on the main controller down here */
#[get("/logout")]
pub fn logout(mut cookies: Cookies) {
    cookies.remove_private(Cookie::named("signed-in-key"));
}

use crate::models::enums::Status;

/* On recompile time this will run and reset every clerk status */
#[get("/admin/reparse-status")]
pub fn clear_online_status(_administrative: Option<AdminUser>) {
    use crate::schema::status_clerk;

    diesel::update(status_clerk::table)
        .set(status_clerk::status.eq(Status::Offline as i32))
        .execute(&crate::establish_connection())
        .expect("We cannot reparse clerks status, please, check it out.");
}

#[get("/query-for-user-name/<user_id>")]
pub fn retrieve_name(user_id: i32) -> Json<String> {
    use crate::schema::sysuser;

    let results = sysuser::table
        .select(sysuser::user_name)
        .filter(sysuser::user_id.eq(user_id))
        .load::<String>(&establish_connection())
        .expect("Some error occured when retrieving chat list. It was registered on logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/login")]
pub fn login(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();

    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("pages/login", &context)
}

#[get("/sitemap.xml")]
pub fn sitemap() -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/sitemap.xml")).ok()
}

#[get("/robots.txt")]
pub fn robots() -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/robots.txt")).ok()
}

#[derive(FromForm)]
pub struct Login {
    user_email: String,
    user_password: String,
}

#[post("/login", data = "<login>")]
pub fn process_login(
    mut cookies: Cookies<'_>,
    login: Form<Login>,
) -> Result<Redirect, Flash<Redirect>> {
    if login.user_email == "atendimento@templo-sagrado.com"
        && login.user_password == "templo-vonmuller-2021"
    {
        cookies.add_private(Cookie::new("easytarot_administrative_user", 1.to_string()));
        Ok(Redirect::to("/admin"))
    } else {
        Err(Flash::error(
            Redirect::to("/admin/login"),
            "Senha/usuário não confere.",
        ))
    }
}
#[get("/")]
pub fn index(_adminstrative: AdminUser) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", '/');
    Template::render("pages/home/index", &map)
}

#[get("/", rank = 2)]
pub fn index_hack() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}

//Static files handling (Assets, Images, Js scripts, and other cool things.)
#[get("/assets/<file..>")]
pub fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

#[get("/page/<page>/<action>")]
pub fn pages(page: String, action: String) -> Option<NamedFile> {
    NamedFile::open(
        Path::new("templates/pages/")
            .join(page)
            .join(format!("{}.html.hbs", action)),
    )
    .ok()
}

#[get("/products")]
pub fn products() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/products");
    Template::render("pages/home/products", &map)
}

#[get("/register")]
pub fn register() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/new-user");
    Template::render("pages/home/register", &map)
}

#[get("/faq")]
pub fn faq() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/faq");
    Template::render("pages/home/faq", &map)
}

#[get("/buy-minutes")]
pub fn buy_minutes() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/buy-minutes");
    Template::render("pages/home/buy-minutes", &map)
}

#[get("/clerks")]
pub fn clerks() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/clerks");
    Template::render("pages/home/clerks", &map)
}

#[get("/depos")]
pub fn depos() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/depos");
    Template::render("pages/home/depos", &map)
}

#[get("/xxxxx")]
pub fn chat_user() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/chat");
    Template::render("pages/chat_user", &map)
}

#[get("/answer")]
pub fn chat_clerk() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/answer");
    Template::render("pages/chat_clerk", &map)
}

#[get("/contact")]
pub fn contact() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/contact");
    Template::render("pages/home/contact", &map)
}


#[get("/privacy")]
pub fn privacy() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/privacy");
    Template::render("pages/privacy", &map)
}

/* [Client] User session */
#[get("/my-acc")]
pub fn my_acc_user(cookies: Cookies<'_>) -> Template {
    use crate::models::{Address, Phone, SysUser};
    use crate::schema::address;
    use crate::schema::phone;
    use crate::schema::sysuser;

    let user_id: std::option::Option<i32> =
        cookies.get("user_id").map(|c| c.value().parse().unwrap());

    let mut context = HashMap::new();

    match user_id {
        Some(cookie_id) => {
            let self_data = sysuser::table
                .inner_join(address::table)
                .inner_join(phone::table)
                .select((
                    sysuser::all_columns,
                    address::all_columns,
                    phone::all_columns,
                ))
                .filter(sysuser::user_id.eq(cookie_id))
                .load::<(SysUser, Address, Phone)>(&establish_connection())
                .expect(
                    "Some Error occured while parsing cookie absolute value. Registered in logs.",
                );
            context.insert("self_data", self_data);
        }
        None => println!("Please, log-in first."),
    }

    Template::render("pages/home/my-acc-user", &context)
}

#[get("/hand-on-wheel/popoulate_status_clerk")]
pub fn populate_status_clerk() {
    use crate::schema::sysuser;

    let clerks_ids = sysuser::table
        .select(sysuser::user_id)
        .filter(sysuser::user_type_id.eq(2))
        .load::<i32>(&crate::establish_connection())
        .expect("Shit happnd");

    for id in clerks_ids {
        use crate::models::NewStatusClerk;
        use crate::schema::status_clerk;

        diesel::insert_into(status_clerk::table)
            .values(NewStatusClerk {
                clerk_id: id,
                status: Status::Offline as i32,
                is_available_chat: false,
                is_available_voice: false,
                is_available_video: false,
                is_available_mail: false,
            })
            .execute(&crate::establish_connection())
            .unwrap();
    }
}
