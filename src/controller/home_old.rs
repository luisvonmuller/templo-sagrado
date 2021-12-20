/* Session Handling Imports */
use rocket::http::{Cookie, Cookies};
use rocket::request::LenientForm;

/* Template */
use rocket_contrib::templates::Template;
use std::collections::HashMap;

/* File fairing */
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

/* Stabilishing connections to db */
use crate::establish_connection;

/* Since we can have a logged out try, we will import redirect and flash */
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};

/* Table macros */
use chrono::{NaiveDate, NaiveDateTime, Utc};

use diesel::prelude::*;

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Pass hashing */
use crypto::digest::Digest;
use crypto::sha2::Sha512;

use crate::User;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

#[get("/mail")]
pub fn mail() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/");
    Template::render("pages/mail/index", &map)
}

#[get("/page-content/<page_name>")]
pub fn page_content(page_name: String) -> Json<String> {
    use crate::schema::syspage;

    let results: Vec<String> = syspage::table
        .select(syspage::syspage_content)
        .filter(syspage::syspage_title.eq(page_name.to_string()))
        .load::<String>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/check-mail/<try_mail>")]
pub fn check_mail(try_mail: String) -> Json<String> {
    use crate::schema::sysuser;

    let results = sysuser::table
        .select(sysuser::user_email)
        .filter(sysuser::user_email.eq(try_mail.to_string()))
        .load::<String>(&crate::establish_connection())
        .expect("Error.");

    if results.len() > 0 {
        Json(serde_json::to_string(&false).unwrap())
    } else {
        Json(serde_json::to_string(&true).unwrap())
    }
}

/* Front-end query function that will help with msging that you dont enough have minutes left to start a chat */
#[get("/my-balance")]
pub fn my_balance(user: User) -> Json<String> {
    use crate::schema::sysuser;

    let _balance: f64 = sysuser::table
        .select(sysuser::user_balance)
        .filter(sysuser::user_id.eq(user.user_id))
        .load::<f64>(&establish_connection())
        .expect("Some error ocurred when trying to parse minutes left")[0];

    Json(serde_json::to_string(&_balance).unwrap())
}

#[post("/facebook-auth", data = "<facebook_data>")]
pub fn facebook_auth(
    mut cookies: Cookies<'_>,
    content_type: &ContentType,
    facebook_data: Data,
) -> Redirect {
    use crate::models::{NewAddress, NewPhone, NewSysUser};
    use crate::schema::*;

    let mut options = MultipartFormDataOptions::new();

    /* Facebook data */
    options
        .allowed_fields
        .push(MultipartFormDataField::text("email"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("name"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("id"));

    /* Personal Info*/
    options
        .allowed_fields
        .push(MultipartFormDataField::text("birthdate"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("gender"));

    let form_data = MultipartFormData::parse(content_type, facebook_data, options).unwrap();

    /* Facebook data */
    let email = form_data.texts.get("email").unwrap()[0].text.to_string();
    let name = form_data.texts.get("name").unwrap()[0].text.to_string();
    let id = form_data.texts.get("id").unwrap()[0].text.to_string();
    let birthdate = form_data.texts.get("birthdate").unwrap()[0]
        .text
        .to_string();

    /* Pash hashing by sha512 */
    let mut hasher = Sha512::new();
    hasher.input_str(&id);
    let birth_split: Vec<&str> = birthdate.split("-").collect();

    let connection = establish_connection();

    /* Verify if a user already have an account or facebook is connected*/
    let user_id = sysuser::table.select(sysuser::user_id)
        .filter(sysuser::user_fb_id.eq(&id))
        .filter(sysuser::user_email.eq(&email))
        .load::<i32>(&establish_connection())
        .expect("Some shit happned while retrieving the full list of users.!!! <Panic at the Disco> ops, thread!");

    // Verifica se o usuário já existe
    if user_id.len() > 0 {
        cookies.add_private(Cookie::new("user_id", user_id[0].to_string()));
        Redirect::to("/my-acc")
    } else {
        /* Creating user */
        let user_inserted_id: i32 = diesel::insert_into(sysuser::table)
            .values(NewSysUser {
                user_name: name,
                user_email: email,
                user_password: hasher.result_str(),
                user_birthdate: NaiveDate::from_ymd(
                    birth_split[0].parse().unwrap(),
                    birth_split[1].parse().unwrap(),
                    birth_split[2].parse().unwrap(),
                ),
                user_genre: "".to_string(),
                user_alias: Some(thread_rng().sample_iter(&Alphanumeric).take(30).collect()),
                user_newsletter: true,
                user_creation: Utc::now().naive_utc(),
                user_lasttimeonline: Some(Utc::now().naive_utc()),
                user_points: 0,
                user_balance: 0.00,
                user_type_id: 1,
                user_status: Some(true),
                user_minutes: Some(0),
                user_min_chat: Some(0),
                user_min_voip: Some(0),
                user_amount_email: Some(0),
                user_fb_id: Some(id.to_string()),
                user_uni: Some(thread_rng().sample_iter(&Alphanumeric).take(10).collect()),
            })
            .returning(sysuser::user_id)
            .get_result(&connection)
            .unwrap();

        /* Respective Adress inserting */
        diesel::insert_into(address::table)
            .values(NewAddress {
                address_number: "".to_string(),
                address_street: "".to_string(),
                address_city: "".to_string(),
                address_state: "".to_string(),
                address_country: "".to_string(),
                address_postalcode: "".to_string(),
                user_id: user_inserted_id,
            })
            .execute(&connection)
            .unwrap();

        /* Respective phone inserting */
        diesel::insert_into(phone::table)
            .values(NewPhone {
                phone_number: "".to_string(),
                user_id: user_inserted_id,
                phone_type_id: 1,
            })
            .execute(&connection)
            .unwrap();

        cookies.add_private(Cookie::new("user_id", user_inserted_id.to_string()));
        Redirect::to("/my-acc")
    }
}

#[get("/facebook1")]
pub fn facebook1() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/paypal");
    Template::render("home/fb1", &map)
}

#[get("/facebook2")]
pub fn facebook2() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/paypal");
    Template::render("home/fb2", &map)
}
#[get("/facebook3")]
pub fn facebook3() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/paypal");
    Template::render("home/fb3", &map)
}

#[get("/privacy")]
pub fn privacy() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/privacy");
    Template::render("pages/privacy", &map)
}

#[get("/privacidade")]
pub fn privacidade(user: User) -> Template {
    use crate::models::{Address, Phone, SysUser};
    use crate::schema::{address, phone, sysuser};

    let mut context = HashMap::new();

    let self_data = sysuser::table
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

    context.insert("self_data", self_data);

    Template::render("home/privacy", context)
}

#[get("/privacidade", rank = 2)]
pub fn privacidade_no_login() -> Template {
    use crate::schema::syspage;
    let mut context = HashMap::new();
    let content: Vec<String> = syspage::table
        .select(syspage::syspage_content)
        .filter(syspage::syspage_title.eq("Privacidade".to_string()))
        .load::<String>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    context.insert("page_data", content);

    Template::render("home/privacy", &context)
}

#[get("/logout")]
pub fn logout(user: User, mut cookies: Cookies) -> Redirect {
    use crate::schema::status_clerk;

    diesel::update(status_clerk::table.filter(status_clerk::clerk_id.eq(user.user_id as i32)))
        .set(status_clerk::is_available.eq(0))
        .execute(&crate::establish_connection())
        .unwrap();

    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/")
}

/* Login form... */
#[derive(FromForm)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[post("/auth", data = "<login>")]
pub fn login(
    mut cookies: Cookies,
    login: LenientForm<LoginForm>,
) -> Result<Redirect, Flash<Redirect>> {
    use crate::schema::*;

    /* Pash hashing by SHA-512 */
    let mut hasher = Sha512::new();

    hasher.input_str(&login.password);

    let user_id = sysuser::table
        .select((
            sysuser::user_id,
            sysuser::user_type_id,
            sysuser::user_min_chat,
            sysuser::user_min_voip,
            sysuser::user_amount_email,
        ))
        .filter(sysuser::user_email.eq(login.email.to_string()))
        .filter(sysuser::user_password.eq(hasher.result_str()))
        .load::<(i32, i32, Option<i32>, Option<i32>, Option<i32>)>(&establish_connection())
        .expect(
            "Some bad shit happned while parsing a login.!!! <Panic at the Disco> ops, thread!",
        );

    if user_id.len() > 0 {
        cookies.add_private(Cookie::new("user_id", user_id[0].0.to_string()));
        cookies.add_private(Cookie::new("user_type_id", user_id[0].1.to_string()));
        cookies.add_private(Cookie::new(
            "user_min_chat",
            user_id[0].2.unwrap().to_string(),
        ));
        cookies.add_private(Cookie::new(
            "user_min_voip",
            user_id[0].3.unwrap().to_string(),
        ));
        cookies.add_private(Cookie::new(
            "user_amount_email",
            user_id[0].4.unwrap().to_string(),
        ));
        Ok(Redirect::to("/my-acc"))
    } else {
        Err(Flash::error(
            Redirect::to("/login"),
            "Ops... Não encontramos seus dados... Por favor, tente novamente ou recupere sua senha abaixo.",
        ))
    }
}

#[get("/chat-info")]
pub fn chat_info(user: User) -> Json<String> {
    use crate::models::Chat;
    use crate::schema::chat;

    let results = chat::table
        .select(chat::all_columns)
        .filter(chat::client_id.eq(user.user_id as i32))
        .order(chat::chat_id.desc())
        .load::<Chat>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/chat-info-clerk")]
pub fn chat_info_clerk(user: User) -> Json<String> {
    use crate::models::Chat;
    use crate::schema::chat;
    let results = chat::table
        .select(chat::all_columns)
        .filter(chat::clerk_id.eq(user.user_id as i32))
        .order(chat::chat_id.desc())
        .load::<Chat>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");
    Json(serde_json::to_string(&results).unwrap())
}

#[derive(FromForm, Debug)]
pub struct RefClient {
    client_id: i32,
}

#[post("/get_client_info", data = "<_client>")]
pub fn client_info(user: User, _client: LenientForm<RefClient>) -> Json<String> {
    use crate::models::{Address, SysUser};
    use crate::schema::{address, sysuser};

    let results = sysuser::table
        .inner_join(address::table)
        .select((sysuser::all_columns, address::all_columns))
        .filter(sysuser::user_id.eq(_client.client_id))
        .load::<(SysUser, Address)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/home")]
pub fn index_maintence() -> Template {
    use crate::models::{ClerkInfo, SysUser, UserType};
    use crate::schema::{clerk_info, status_clerk, sysuser, user_type};

    let mut context = HashMap::new();

    let self_data = sysuser::table
        .inner_join(user_type::table)
        .inner_join(clerk_info::table)
        .inner_join(status_clerk::table)
        .select((
            sysuser::all_columns,
            user_type::all_columns,
            clerk_info::all_columns,
        ))
        .filter(user_type::user_type_id.eq(2))
        .filter(sysuser::user_status.eq(true))
        .limit(8)
        .order(clerk_info::clerk_info_priority.desc())
        .order(status_clerk::is_available.desc())
        .load::<(SysUser, UserType, ClerkInfo)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");
    context.insert("self_data", self_data);

    Template::render("home/undermn", &context)
}

pub fn parse_testimonial(user_id: &i32) -> Vec<crate::models::ClerkInfo> {
    use crate::schema::clerk_info;

    use crate::models::ClerkInfo;

    clerk_info::table
        .select(clerk_info::all_columns)
        .filter(clerk_info::user_id.eq(*user_id))
        .load::<ClerkInfo>(&establish_connection())
        .expect("Shit!")
}

#[get("/")]
pub fn index(_user: User) -> Template {
    use crate::models::{Address, ClerkInfo, Phone, SysUser, Testimonials, UserType};
    use crate::schema::{
        address, clerk_info, phone, status_clerk, sysuser, testimonials, user_type,
    };

    let mut context = HashMap::new();

    let results = sysuser::table
        .inner_join(address::table)
        .inner_join(phone::table)
        .select((
            sysuser::all_columns,
            address::all_columns,
            phone::all_columns,
        ))
        .filter(sysuser::user_id.eq(_user.user_id as i32))
        .load::<(SysUser, Address, Phone)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    let self_data = sysuser::table
        .inner_join(user_type::table)
        .inner_join(clerk_info::table)
        .inner_join(status_clerk::table)
        .select((
            sysuser::all_columns,
            user_type::all_columns,
            clerk_info::all_columns,
        ))
        .filter(user_type::user_type_id.eq(2))
        .filter(sysuser::user_status.eq(true))
        .order(clerk_info::clerk_info_priority.desc())
        .order(status_clerk::is_available.desc())
        .limit(8)
        .load::<(SysUser, UserType, ClerkInfo)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    let depos_data = testimonials::table
        .select(testimonials::all_columns)
        .filter(testimonials::testimonials_status.eq(true))
        .load::<Testimonials>(&establish_connection())
        .expect("Can't retrieve data");

    let mut parsed: Vec<(Vec<ClerkInfo>, Testimonials)> = Vec::new();

    for data in depos_data {
        parsed.append(&mut vec![(
            parse_testimonial(&data.testimonials_clerk_id),
            data,
        )]);
    }

    context.insert("self_data", (&results[0], self_data, parsed));

    Template::render("home/index", &context)
   // Template::render("home/undermn", &context)
}

#[get("/whats-my-id")]
pub fn whats_my_id(user: User) -> Json<String> {
    Json(serde_json::to_string(&(user.user_id as i32)).unwrap())
}

#[get("/whats-my-id", rank = 2)]
pub fn whats_my_id_no_login() -> Json<String> {
    Json(serde_json::to_string(&"no-login".to_string()).unwrap())
}

#[get("/blog")]
pub fn blog(user: User) -> Template {
    use crate::models::{Post, SysUser};
    use crate::schema::{post, sysuser};

    let mut context = HashMap::new();

    let self_data = sysuser::table
        .select(sysuser::all_columns)
        .filter(sysuser::user_id.eq(user.user_id as i32))
        .load::<SysUser>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    let blog_data = post::table
        .select(post::all_columns)
        .load::<Post>(&establish_connection())
        .expect("Erro retrieving");

    context.insert("self_data", (self_data, blog_data));

    Template::render("home/blog", context)
}

#[get("/blog", rank = 2)]
pub fn blog_no_login() -> Template {
    use crate::models::Post;
    use crate::schema::post;

    let mut context = HashMap::new();

    let blog_data = post::table
        .select(post::all_columns)
        .load::<Post>(&establish_connection())
        .expect("Erro retrieving");

    context.insert("self_data", ((), blog_data));

    Template::render("home/blog", context)
}

#[get("/notify-me/<clerk_id>")]
pub fn notify_me(user: User, clerk_id: i32) {
    use crate::models::EmailNotificationListJoin;
    use crate::schema::email_notification_list;

    let stuff = diesel::insert_into(email_notification_list::table)
        .values(EmailNotificationListJoin {
            client_id: user.user_id as i32, /* Parses the client session and retrieves the id */
            clerk_id: clerk_id,             /* When who gets online trigger */
        })
        .execute(&crate::establish_connection());

    match stuff {
        Ok(_) => println!("User has joined mail list"),
        Err(_) => println!("User tryed to rejoin"),
    }
}

#[get("/posts/all")]
pub fn all_posts() -> Json<String> {
    use crate::models::Post;
    use crate::schema::post;
    use diesel::prelude::*;

    let results = post::table
        .select(post::all_columns)
        .order(post::post_id.desc())
        .load::<Post>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/blog-post/<post_slug>")]
pub fn single_blog_post(post_slug: String) -> Template {
    use crate::models::Post;

    use diesel::dsl::sql_query;
    use diesel::prelude::*;

    let results: Vec<Post> = sql_query(format!(
        "{}='{}'",
        "SELECT * FROM public.post WHERE REPLACE(LOWER(post.post_title), ' ', '-' )", post_slug
    ))
    .load(&crate::establish_connection())
    .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    let mut context = HashMap::new();

    context.insert("post", results);
    Template::render("home/post", context)
}

#[get("/", rank = 2)]
pub fn index_no_login() -> Template {
    use crate::models::{Address, ClerkInfo, Phone, SysUser, Testimonials, UserType};
    use crate::schema::{
        address, clerk_info, phone, status_clerk, sysuser, testimonials, user_type,
    };

    let mut context = HashMap::new();

    let self_data = sysuser::table
        .inner_join(user_type::table)
        .inner_join(clerk_info::table)
        .inner_join(status_clerk::table)
        .select((
            sysuser::all_columns,
            user_type::all_columns,
            clerk_info::all_columns,
        ))
        .filter(user_type::user_type_id.eq(2))
        .filter(sysuser::user_status.eq(true))
        .order(clerk_info::clerk_info_priority.desc())
        .order(status_clerk::is_available.desc())
        .limit(8)
        .load::<(SysUser, UserType, ClerkInfo)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    let depos_data = testimonials::table
        .select(testimonials::all_columns)
        .filter(testimonials::testimonials_status.eq(true))
        .load::<Testimonials>(&establish_connection())
        .expect("Can't retrieve data");

    let mut parsed: Vec<(Vec<ClerkInfo>, Testimonials)> = Vec::new();

    for data in depos_data {
        parsed.append(&mut vec![(
            parse_testimonial(&data.testimonials_clerk_id),
            data,
        )]);
    }

    context.insert("clerk_data", ((), self_data, parsed));
    Template::render("home/index", &context)
    //Template::render("home/undermn", &context)
}

//Static files handling (Assets, Images, Js scripts, and other cool things.)
#[get("/assets/<file..>")]
pub fn assets(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("assets/").join(file)).ok()
}

#[get("/register")]
pub fn register() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/new-user");
    Template::render("home/register", &map)
}

#[get("/faq")]
pub fn faq(user: User) -> Template {
    use crate::models::{Address, Phone, SysUser};
    use crate::schema::{address, phone, sysuser};

    let mut context = HashMap::new();

    let self_data = sysuser::table
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

    context.insert("self_data", self_data);

    Template::render("home/faq", context)
}

#[get("/faq", rank = 2)]
pub fn faq_no_login() -> Template {
    use crate::schema::syspage;
    let mut context = HashMap::new();
    let content: Vec<String> = syspage::table
        .select(syspage::syspage_content)
        .filter(syspage::syspage_title.eq("FAQ".to_string()))
        .load::<String>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    context.insert("page_data", content);

    Template::render("home/faq", &context)
}

#[get("/buy-minutes")]
pub fn buy_minutes(user: User) -> Template {
    use crate::models::{Address, Phone, SysUser};
    use crate::schema::address;
    use crate::schema::phone;
    use crate::schema::sysuser;

    let mut context = HashMap::new();

    let self_data = sysuser::table
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

    context.insert("self_data", self_data);
    Template::render("home/buy-minutes", &context)
}

#[get("/buy-minutes", rank = 2)]
pub fn buy_minutes_no_login() -> Template {
    let mut context = HashMap::new();
    context.insert("path", "/faq");
    Template::render("home/buy-minutes", &context)
}

#[get("/clerks")]
pub fn clerks() -> Template {
    use crate::models::{ClerkInfo, SysUser, UserType};
    use crate::schema::{clerk_info, status_clerk, sysuser, user_type};

    let mut context = HashMap::new();
    let self_data = sysuser::table
        .inner_join(user_type::table)
        .inner_join(clerk_info::table)
        .inner_join(status_clerk::table)
        .select((
            sysuser::all_columns,
            user_type::all_columns,
            clerk_info::all_columns,
        ))
        .filter(user_type::user_type_id.eq(2))
        .filter(sysuser::user_status.eq(true))
        .order(clerk_info::clerk_info_priority.desc())
        .order(status_clerk::is_available.desc())
        .load::<(SysUser, UserType, ClerkInfo)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    context.insert("self_data", self_data);

    Template::render("home/clerks", &context)
}

#[get("/clerks", rank = 2)]
pub fn clerks_no_login() -> Template {
    use crate::models::{ClerkInfo, SysUser, UserType};
    use crate::schema::clerk_info;
    use crate::schema::sysuser;
    use crate::schema::user_type;

    let mut context = HashMap::new();

    let self_data = sysuser::table
        .inner_join(user_type::table)
        .inner_join(clerk_info::table)
        .select((
            sysuser::all_columns,
            user_type::all_columns,
            clerk_info::all_columns,
        ))
        .filter(user_type::user_type_id.eq(2))
        .filter(sysuser::user_status.eq(true))
        .order(clerk_info::clerk_info_priority.desc())
        .load::<(SysUser, UserType, ClerkInfo)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    context.insert("self_data", self_data);
    Template::render("home/clerks", &context)
}

#[get("/all-depos")]
pub fn get_all_depos() -> Json<String> {
    use crate::models::Message;
    use crate::schema::message;

    let results = message::table
        .select(message::all_columns)
        .load::<Message>(&establish_connection())
        .expect("Some shit happned while retrieving the full list of CLERK users.!!! <Panic at the Disco> ops, thread!");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/depos")]
pub fn depos(cookies: Cookies<'_>) -> Template {
    use crate::models::{Address, Phone, SysUser};
    use crate::schema::{address, phone, sysuser};

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
    Template::render("home/depos", &context)
}

#[get("/login")]
pub fn login_screen(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }
    Template::render("home/login-src", &context)
}

#[get("/wait-for-client")]
pub fn wait_for_client(user: User) -> Template {
    use crate::models::{Address, CallEmail, Phone, SysUser};
    use crate::schema::{address, call_email, phone, sysuser};

    let mut context = HashMap::new();
    /* Profile data */
    let self_data = sysuser::table
        .select(sysuser::all_columns)
        .filter(sysuser::user_id.eq(user.user_id as i32))
        .load::<SysUser>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    /* Non answered on outer attend mail data */
    let mail_data: Vec<(i32, String)> = call_email::table
        .inner_join(sysuser::table)
        .select((call_email::call_email_id, sysuser::user_name))
        .filter(call_email::clerk_id.eq(user.user_id as i32))
        .filter(call_email::call_email_response_body.is_null())
        .load::<(i32, String)>(&establish_connection())
        .expect("Shit happens");

    context.insert("self_data", (self_data, mail_data));

    Template::render("home/wait-for-client", &context)
}

#[get("/chat/<_chat_id>")]
pub fn chat(user: User, _chat_id: i32) -> Result<Template, Redirect> {
    use crate::models::{Address, Phone, SysUser};
    use crate::schema::{address, phone, sysuser};

    let mut context = HashMap::new();

    let self_data = sysuser::table
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

    context.insert("self_data", &self_data);

    if self_data[0].0.user_type_id > 1 {
        Ok(Template::render("home/chat_clerk", &context))
    } else {
        Ok(Template::render("home/chat_user", &context))
    }
}

#[get("/contact")]
pub fn contact() -> Template {
    let mut context = HashMap::new();
    context.insert("path", "/contact");
    Template::render("home/contact", &context)
}

#[get("/sobre")]
pub fn sobre(user: User) -> Template {
    use crate::models::{Address, Phone, SysUser};
    use crate::schema::{address, phone, sysuser};

    let mut context = HashMap::new();

    let self_data = sysuser::table
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

    context.insert("self_data", self_data);

    Template::render("home/about", context)
}

/* Sobre when no session is informed */
#[get("/sobre", rank = 2)]
pub fn sobre_no_login() -> Template {
    use crate::schema::syspage;

    let mut context = HashMap::new();

    let content: Vec<String> = syspage::table
        .select(syspage::syspage_content)
        .filter(syspage::syspage_title.eq("Sobre".to_string()))
        .load::<String>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    context.insert("page_data", content);

    Template::render("home/about", &context)
}

#[get("/paypal")]
pub fn paypal(cookies: Cookies<'_>) -> Template {
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
    Template::render("pages/paypal", &context)
}

/* [Client] User session */
#[get("/my-acc")]
pub fn my_acc_user(user: User) -> Template {
    use crate::models::SysUser;
    use crate::schema::sysuser;

    let mut context = HashMap::new();

    let self_data = sysuser::table
        .select(sysuser::all_columns)
        .filter(sysuser::user_id.eq(user.user_id as i32))
        .load::<SysUser>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    use crate::models::{ClerkInfo, StatusClerk, UserType};
    use crate::schema::{clerk_info, status_clerk, user_type};

    let clerk_data = sysuser::table
        .inner_join(user_type::table)
        .inner_join(clerk_info::table)
        .inner_join(status_clerk::table)
        .select((
            sysuser::all_columns,
            user_type::all_columns,
            clerk_info::all_columns,
            status_clerk::all_columns,
        ))
        .filter(user_type::user_type_id.eq(2))
        .filter(sysuser::user_status.eq(true))
        .filter(status_clerk::is_available.eq(Some(1)))
        .order(clerk_info::clerk_info_priority.desc())
        .load::<(SysUser, UserType, ClerkInfo, StatusClerk)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    context.insert("self_data", (self_data, clerk_data));

    Template::render("home/my-acc-user", &context)
}

#[get("/whoClerk/<user_id>")]
pub fn who_clerk(user_id: i32) -> Json<String> {
    use crate::schema::{clerk_info, sysuser};

    let results = sysuser::table
        .inner_join(clerk_info::table)
        .select((clerk_info::clerk_image, clerk_info::clerk_info_exhibition))
        .filter(sysuser::user_id.eq(user_id))
        .load::<(Option<String>, Option<String>)>(&establish_connection())
        .expect("Something bad happned");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/whoClient/<user_id>")]
pub fn who_client(user_id: i32) -> Json<String> {
    use crate::schema::sysuser;

    let results = sysuser::table
        .select(sysuser::user_name)
        .filter(sysuser::user_id.eq(user_id))
        .load::<String>(&establish_connection())
        .expect("Fail");
    println!("{:?}", results);
    Json(serde_json::to_string(&results).unwrap())
}

#[get("/register-chat/<client_id>")]
pub fn register_chat(user: User, client_id: i32) -> Json<String> {
    use crate::models::NewChat;
    use crate::schema::chat;

    /* Database connection */
    use crate::establish_connection;

    /* Lenient form imports */
    use chrono::Utc;

    let results: i32 = diesel::insert_into(chat::table)
        .values(NewChat {
            client_id: client_id,
            clerk_id: user.user_id as i32,
            client_socket: "void".to_string(),
            clerk_socket: "void".to_string(),
            init_time: Utc::now().naive_utc(),
            end_time: None,
        })
        .returning(chat::chat_id)
        .get_result(&establish_connection())
        .unwrap();

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/register-voice-chat/<client_id>")]
pub fn register_voice_chat(user: User, client_id: i32) -> Json<String> {
    use crate::models::NewCall;
    use crate::schema::call;

    /* Database connection */
    use crate::establish_connection;

    /* Lenient form imports */
    use chrono::Utc;

    let results: i32 = diesel::insert_into(call::table)
        .values(NewCall {
            call_begin_date: Utc::now().naive_utc(),
            call_end_date: None,
            user_id: client_id,
            clerk_id: user.user_id as i32,
            call_file: None,
        })
        .returning(call::call_id)
        .get_result(&establish_connection())
        .unwrap();

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/minutes-out")]
pub fn minutes_out(user: User) -> Json<String> {
    use crate::schema::sysuser;

    let minutes_left: f64 =
        diesel::update(sysuser::table.filter(sysuser::user_id.eq(user.user_id as i32)))
            .set(sysuser::user_balance.eq(sysuser::user_balance - &crate::get_values().1))
            .returning(sysuser::user_balance)
            .get_result(&establish_connection())
            .unwrap();

    Json(serde_json::to_string(&minutes_left).unwrap())
}

/* Count and update the credits for a clerk */
#[get("/minutes-up")]
pub fn minutes_up(user: User) -> Json<String> {
    use crate::schema::sysuser;

    let minutes_left: f64 = diesel::update(
        sysuser::table
            .filter(sysuser::user_id.eq(user.user_id as i32))
            .filter(sysuser::user_type_id.eq(2)),
    )
    .set(
        sysuser::user_balance
            .eq(sysuser::user_balance
                + (&crate::get_values().1 * comission_rate(user.user_id as i32))),
    )
    .returning(sysuser::user_balance)
    .get_result(&establish_connection())
    .unwrap();

    Json(
        serde_json::to_string(&(
            &minutes_left,
            &crate::get_values().1 * comission_rate(user.user_id as i32),
        ))
        .unwrap(),
    )
}

#[get("/minutes-out-voice")]
pub fn minutes_out_voice(user: User) -> Json<String> {
    use crate::schema::sysuser;

    let minutes_left: f64 =
        diesel::update(sysuser::table.filter(sysuser::user_id.eq(user.user_id as i32)))
            .set(sysuser::user_balance.eq(sysuser::user_balance - &crate::get_values().2))
            .returning(sysuser::user_balance)
            .get_result(&establish_connection())
            .unwrap();

    Json(serde_json::to_string(&minutes_left).unwrap())
}

#[get("/minutes-up-voice")]
pub fn minutes_up_voice(user: User) -> Json<String> {
    use crate::schema::sysuser;

    let minutes_left: f64 = diesel::update(
        sysuser::table
            .filter(sysuser::user_id.eq(user.user_id as i32))
            .filter(sysuser::user_type_id.eq(2)),
    )
    .set(
        sysuser::user_balance
            .eq(sysuser::user_balance
                + (&crate::get_values().2 * comission_rate(user.user_id as i32))),
    )
    .returning(sysuser::user_balance)
    .get_result(&establish_connection())
    .unwrap();

    Json(
        serde_json::to_string(&(
            &minutes_left,
            &crate::get_values().2 * comission_rate(user.user_id as i32),
        ))
        .unwrap(),
    )
}

#[get("/end-chat/<chat_id>")]
pub fn end_chat(_user: User, chat_id: i32) -> Json<String> {
    use crate::schema::chat;

    let end_chat_time: Option<NaiveDateTime> =
        diesel::update(chat::table.filter(chat::chat_id.eq(chat_id)))
            .set(chat::end_time.eq(Some(Utc::now().naive_utc())))
            .returning(chat::end_time)
            .get_result(&establish_connection())
            .unwrap();

    Json(serde_json::to_string(&end_chat_time).unwrap())
}

#[get("/am-i-a-clerk")]
pub fn am_i_a_clerk(user: User) -> Json<String> {
    use crate::schema::{clerk_info, sysuser};

    let results = sysuser::table
        .inner_join(clerk_info::table)
        .select((clerk_info::clerk_image, clerk_info::clerk_info_exhibition))
        .filter(sysuser::user_id.eq(user.user_id as i32))
        .load::<(Option<String>, Option<String>)>(&establish_connection())
        .expect("Something bad happned");

    Json(serde_json::to_string(&results).unwrap())
}

#[get("/clerk-profile/<clerk_slug>")]
pub fn clerk_profile(_user: User, clerk_slug: String) -> Template {
    use crate::models::{Address, ClerkInfo, Phone, SysUser, Testimonials};
    use crate::schema::{address, clerk_info, phone, sysuser, testimonials};

    let mut context = HashMap::new();
    use diesel::sql_query;
    /* Self data */
    let self_data = sysuser::table
        .inner_join(address::table)
        .inner_join(phone::table)
        .select((
            sysuser::all_columns,
            address::all_columns,
            phone::all_columns,
        ))
        .filter(sysuser::user_id.eq(_user.user_id as i32))
        .load::<(SysUser, Address, Phone)>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    /* Profile data */
    let clerk_data: Vec<(SysUser, ClerkInfo)> = sql_query(format!("{}='{}'", "select * from public.sysuser 
    inner join clerk_info on clerk_info.user_id = sysuser.user_id WHERE REPLACE(LOWER(clerk_info.clerk_info_exhibition ), ' ', '-' )", clerk_slug))
        .load(&establish_connection())
        .expect(
            "Some Error ocurred when retrieving a clerk profile :( - This was Registered in logs.",
        );

    let depos_data = testimonials::table
        .select(testimonials::all_columns)
        .filter(testimonials::testimonials_status.eq(true))
        .filter(testimonials::testimonials_clerk_id.eq(&clerk_data[0].0.user_id))
        .load::<Testimonials>(&establish_connection())
        .expect("Can't retrieve data");

    context.insert("self_data", (&self_data[0], &clerk_data[0], &depos_data));

    Template::render("home/clerk-profile", &context)
}

#[get("/clerk-profile/<clerk_slug>", rank = 2)]
pub fn clerk_profile_no_login(clerk_slug: String) -> Template {
    use crate::models::{Address, ClerkInfo, Phone, SysUser, Testimonials};
    use crate::schema::{address, clerk_info, phone, sysuser, testimonials};

    let mut context = HashMap::new();
    use diesel::sql_query;

    /* Profile data */
    let clerk_data: Vec<(SysUser, ClerkInfo)> = sql_query(format!("{}='{}'", "select * from public.sysuser 
    inner join clerk_info on clerk_info.user_id = sysuser.user_id WHERE REPLACE(LOWER(clerk_info.clerk_info_exhibition ), ' ', '-' )", clerk_slug))
        .load(&establish_connection())
        .expect(
            "Some Error ocurred when retrieving a clerk profile :( - This was Registered in logs.",
        );

    let depos_data = testimonials::table
        .select(testimonials::all_columns)
        .filter(testimonials::testimonials_status.eq(true))
        .filter(testimonials::testimonials_clerk_id.eq(&clerk_data[0].0.user_id))
        .load::<Testimonials>(&establish_connection())
        .expect("Can't retrieve data");

    context.insert("self_data", (&(), &clerk_data[0], &depos_data));

    Template::render("home/clerk-profile", &context)
}

#[get("/voip/<call_id>")]
pub fn voip(_user: User, call_id: i32) -> Template {
    use crate::models::{Call, SysUser};
    use crate::schema::{call, sysuser};

    let mut context = HashMap::new();

    let am_i_what: Vec<i32> = sysuser::table
        .select(sysuser::user_type_id)
        .filter(sysuser::user_id.eq(_user.user_id as i32))
        .load::<i32>(&crate::establish_connection())
        .expect("shit happnd");

    let self_data = sysuser::table
        .select(sysuser::all_columns)
        .filter(sysuser::user_id.eq(_user.user_id))
        .load::<SysUser>(&crate::establish_connection())
        .expect("Error.");

    let call_data = call::table
        .select(call::all_columns)
        .filter(call::call_id.eq(call_id))
        .load::<Call>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    context.insert("self_data", (self_data, call_data));

    if am_i_what[0] > 1 {
        Template::render("home/voip-clerk", &context)
    } else {
        Template::render("home/voip-user", &context)
    }
}

/* Front-end query function that will help with msging that you dont enough have minutes left to start a chat */
#[get("/chat-mins")]
pub fn how_many_chat_mins(user: User) -> Json<String> {
    use crate::schema::sysuser;

    let minutes_left: i32 = sysuser::table
        .select(sysuser::user_min_chat)
        .filter(sysuser::user_id.eq(user.user_id))
        .load::<Option<i32>>(&establish_connection())
        .expect("Some error ocurred when trying to parse minutes left")[0]
        .unwrap();

    Json(serde_json::to_string(&minutes_left).unwrap())
}

#[get("/voip-mins")]
pub fn how_many_voip_mins(user: User) -> Json<String> {
    use crate::schema::sysuser;

    let minutes_left: i32 = sysuser::table
        .select(sysuser::user_min_voip)
        .filter(sysuser::user_id.eq(user.user_id))
        .load::<Option<i32>>(&establish_connection())
        .expect("Some error ocurred when trying to parse minutes left")[0]
        .unwrap();

    Json(serde_json::to_string(&minutes_left).unwrap())
}

#[get("/user-email")]
pub fn user_email(user: User) -> Template {
    use crate::models::ClerkInfo;
    use crate::schema::clerk_info;

    let mut context = HashMap::new();

    let self_data = clerk_info::table
        .select(clerk_info::all_columns)
        .load::<ClerkInfo>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    context.insert("self_data", self_data);

    Template::render("home/email_user", context)
}

#[get("/clerk-email")]
pub fn clerk_email(user: User) -> Template {
    use crate::models::{Address, Phone, SysUser};
    use crate::schema::{address, phone, sysuser};

    let mut context = HashMap::new();

    let self_data = sysuser::table
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

    context.insert("self_data", self_data);

    Template::render("home/email_clerk", context)
}

#[post("/get-my-mail/<clerk_id>/<mail_id>")]
pub fn get_clerk_mail(user: User, clerk_id: i32, mail_id: i32) -> Json<String> {
    use crate::schema::call_email;

    let mail_data = call_email::table
        .select((
            call_email::call_email_request_to_email,
            call_email::call_email_request_title,
            call_email::call_email_request_body,
        ))
        .filter(call_email::user_id.eq(user.user_id as i32))
        .filter(call_email::clerk_id.eq(clerk_id))
        .filter(call_email::call_email_id.eq(mail_id))
        .load::<(String, String, String)>(&crate::establish_connection())
        .expect("Shit happens.");

    Json(serde_json::to_string(&mail_data).unwrap())
}

#[get("/get-mail-data/<mail_id>")]
pub fn get_mail_data(_user: User, mail_id: i32) -> Json<String> {
    use crate::models::CallEmail;
    use crate::schema::{call_email, sysuser};

    let mail_data = call_email::table
        .inner_join(sysuser::table)
        .select((sysuser::user_name, call_email::all_columns))
        .filter(call_email::call_email_id.eq(mail_id))
        .load::<(String, CallEmail)>(&crate::establish_connection())
        .expect("Shit happens.");

    Json(serde_json::to_string(&mail_data).unwrap())
}

/* Login form... */
#[derive(FromForm)]
pub struct ClientMailForm {
    pub clerk_id: i32,
    pub user_id: i32,
    pub call_email_request_to_email: String,
    pub call_email_request_title: String,
    pub call_email_request_body: String,
}

pub fn get_mail_value() -> f64 {
    use crate::schema::config;

    config::table
        .select(config::site_mail_val)
        .load::<f64>(&crate::establish_connection())
        .expect("Bad Things happend while retrieving mail value.")[0]
}

#[get("/atendimento-email/<_clerk_id>")]
pub fn client_mail(user: User, _clerk_id: i32) -> Template {
    use crate::models::SysUser;
    use crate::schema::sysuser;

    let mut context = HashMap::new();

    let self_data = sysuser::table
        .select(sysuser::all_columns)
        .filter(sysuser::user_id.eq(user.user_id as i32))
        .load::<SysUser>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    context.insert("self_data", vec![self_data]);

    Template::render("home/client-mail", &context)
}

/* Saves post and bla bla bla */
#[post("/client-mail-post", data = "<email_data>")]
pub fn new_client_mail(user: User, email_data: LenientForm<ClientMailForm>) -> Json<String> {
    use crate::models::NewCallEmail;
    use crate::schema::call_email;

    /* Time to populate this mail table for the f1 */
    let mail_req = diesel::insert_into(call_email::table)
        .values(NewCallEmail {
            call_email_request_title: email_data.call_email_request_title.to_string(),
            call_email_request_body: email_data.call_email_request_body.to_string(),
            call_email_request_date: Utc::now().naive_utc(),
            call_email_request_to_email: email_data.call_email_request_to_email.to_string(),
            user_id: user.user_id as i32,
            clerk_id: email_data.clerk_id,
        })
        .execute(&crate::establish_connection());

    /* Take out value of the mail from the client */
    use crate::schema::sysuser;

    diesel::update(sysuser::table.filter(sysuser::user_id.eq(user.user_id as i32)))
        .set(sysuser::user_balance.eq(sysuser::user_balance - get_mail_value()))
        .execute(&crate::establish_connection())
        .unwrap();

    match mail_req {
        Ok(_) => Json(serde_json::to_string(&true).unwrap()),
        Err(_) => Json(serde_json::to_string(&true).unwrap()),
    }
}

#[derive(FromForm, Debug)]
pub struct RespMail {
    pub call_email_id: i32,
    pub call_email_request_to_email: String,
    pub call_email_response_body: String,
    pub call_email_response_title: String,
}

pub fn comission_rate(user_id: i32) -> f64 {
    use crate::schema::clerk_info;

    (clerk_info::table
        .select(clerk_info::clerk_info_comission_rate)
        .filter(clerk_info::user_id.eq(user_id))
        .load::<Option<String>>(&crate::establish_connection())
        .expect("Shit hpnd")[0]
        .as_ref()
        .unwrap()
        .parse::<f64>()
        .unwrap())
        / 100.00
}

#[post("/answer-mail", data = "<mail_data>")]
pub fn answer_mail(_user: User, mail_data: LenientForm<RespMail>) {
    let my_data = generate_my_slug(_user.user_id as i32);

    /* First we send content */
    crate::controller::mail::send::answer_mail(
        mail_data.call_email_response_title.clone(),
        mail_data.call_email_response_body.clone(),
        mail_data.call_email_request_to_email.clone(),
        String::from(my_data.0),
        String::from(my_data.1),
    );

    /* Give comission to the Clerk */
    use crate::schema::sysuser;

    diesel::update(sysuser::table.filter(sysuser::user_id.eq(_user.user_id as i32)))
        .set(
            sysuser::user_balance
                .eq(sysuser::user_balance
                    + (get_mail_value() * comission_rate(_user.user_id as i32))),
        )
        .execute(&crate::establish_connection())
        .unwrap();

    use crate::schema::call_email;

    /* Update e-mail inside our database */
    diesel::update(call_email::table.filter(call_email::call_email_id.eq(mail_data.call_email_id)))
        .set((
            call_email::call_email_response_title
                .eq(Some(mail_data.call_email_response_title.to_string())),
            call_email::call_email_response_body
                .eq(Some(mail_data.call_email_response_body.to_string())),
            call_email::call_email_response_date.eq(Some(Utc::now().naive_utc())),
        ))
        .execute(&crate::establish_connection())
        .unwrap();
}

fn generate_my_slug(clerk_id: i32) -> (String, String) {
    use crate::schema::clerk_info;

    let leticia = clerk_info::table
        .select(clerk_info::clerk_info_exhibition)
        .filter(clerk_info::user_id.eq(clerk_id))
        .load::<Option<String>>(&establish_connection())
        .expect("PUTS!");

    (
        leticia[0].as_ref().unwrap().clone(),
        leticia[0]
            .as_ref()
            .unwrap()
            .to_lowercase()
            .replace(' ', "-")
            .to_string(),
    )
}

pub fn notify_my_soul_mates(clerk_id: i32) {
    use crate::models::EmailNotificationList;
    use crate::schema::{email_notification_list, sysuser};

    let list_data: Vec<EmailNotificationList> = email_notification_list::table
        .select(email_notification_list::all_columns)
        .filter(email_notification_list::clerk_id.eq(clerk_id))
        .load::<EmailNotificationList>(&crate::establish_connection())
        .expect("Error ocorred while parsing list of intersted users of a clerk id: {}");

    let my_data = generate_my_slug(clerk_id);

    for list_item in list_data {
        let a_mail_data = sysuser::table
            .select((sysuser::user_name, sysuser::user_email))
            .filter(sysuser::user_id.eq(list_item.client_id))
            .load::<(String, String)>(&crate::establish_connection())
            .expect("Error ocorred while parsing list of intersted users of a clerk id: {}");

        crate::controller::mail::send::notify_user_online(
            String::from(&a_mail_data[0].0),
            String::from(&a_mail_data[0].1),
            String::from(&my_data.0),
            String::from(&my_data.1),
        );
    }

    diesel::delete(
        email_notification_list::table.filter(email_notification_list::clerk_id.eq(clerk_id)),
    )
    .execute(&crate::establish_connection())
    .expect("Whoops, we can't delete this.");
}
