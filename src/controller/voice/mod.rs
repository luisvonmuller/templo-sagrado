/*
888     888  .d88888b. 8888888 .d8888b.  8888888888       .d8888b.   .d88888b.  888b    888 88888888888 8888888b.   .d88888b.  888      888      8888888888 8888888b.
888     888 d88P" "Y88b  888  d88P  Y88b 888             d88P  Y88b d88P" "Y88b 8888b   888     888     888   Y88b d88P" "Y88b 888      888      888        888   Y88b
888     888 888     888  888  888    888 888             888    888 888     888 88888b  888     888     888    888 888     888 888      888      888        888    888
Y88b   d88P 888     888  888  888        8888888         888        888     888 888Y88b 888     888     888   d88P 888     888 888      888      8888888    888   d88P
 Y88b d88P  888     888  888  888        888             888        888     888 888 Y88b888     888     8888888P"  888     888 888      888      888        8888888P"
  Y88o88P   888     888  888  888    888 888             888    888 888     888 888  Y88888     888     888 T88b   888     888 888      888      888        888 T88b
   Y888P    Y88b. .d88P  888  Y88b  d88P 888             Y88b  d88P Y88b. .d88P 888   Y8888     888     888  T88b  Y88b. .d88P 888      888      888        888  T88b
    Y8P      "Y88888P" 8888888 "Y8888P"  8888888888       "Y8888P"   "Y88888P"  888    Y888     888     888   T88b  "Y88888P"  88888888 88888888 8888888888 888   T88b
*/

/* Form recognition */
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

/* For sure we need to import the macros from diesel */
use diesel::prelude::*;

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Form */
use rocket::request::LenientForm;

/* Template */
use rocket_contrib::templates::Template;

use crate::AdminUser;

/* Since we can have a logged out try, we will import redirect and flash */
use rocket::response::{Flash, Redirect};

#[post("/register-call", data = "<voice_stuff>")]
pub fn register_call(content_type: &ContentType, voice_stuff: Data) {
    /* Importings structs an macros down here */
    use crate::models::UpdateCallFile;
    use crate::schema::call;

    /* File system functions */
    use std::fs;

    /* Pass hashing */
    use crypto::digest::Digest;
    use crypto::sha2::Sha512;

    /* Database connection */
    use crate::establish_connection;

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    /* Allowow parsing text stuff */
    options
        .allowed_fields
        .push(MultipartFormDataField::text("call_id"));

    /* Allow Clerk Profile image parsing */
    options
        .allowed_fields
        .push(MultipartFormDataField::file("call_file"));

    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, voice_stuff, options).unwrap();

    /* Instanciating variables for text fields */
    let call_id: i32 = multipart_form_data.texts.get("call_id").unwrap()[0]
        .text
        .parse()
        .unwrap();

    /* Clerk call audio file assignment */
    let call_file = &multipart_form_data.files.get("call_file").unwrap()[0];

    //Lenient form imports
    use chrono::Utc;
    let current_time = Some(Utc::now().naive_utc());

    let file_field = &call_file; // Because we only put one "photo" field to the allowed_fields, the max length of this file_fields is 1.

    let _content_type = &file_field.content_type;
    let _file_name = &file_field.file_name;
    let _path = &file_field.path;

    /* Hasher for filename */
    let mut hasher = Sha512::new();
    hasher.input_str(&Utc::now().naive_utc().to_string());

    /* Path parsing */
    let absolute_path: String = format!(
        "{}/{}{}",
        crate::base_path(),
        &hasher.result_str(),
        ".ogg"
    );
    
    let new_call_file = format!("/assets/uploads/{}.{}", &hasher.result_str(), ".ogg");
    fs::copy(_path, &absolute_path).unwrap();

    diesel::update(call::table.filter(call::call_id.eq(call_id)))
        .set(UpdateCallFile {
            call_end_date: current_time,
            call_file: Some(new_call_file),
        })
        .execute(&establish_connection())
        .unwrap();
}

#[get("/call-chat")]
pub fn call_chat() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/voice");
    Template::render("home/voice-test", &map)
}

#[get("/")]
pub fn index(_administrative: AdminUser) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/voice");
    Template::render("pages/voice/list", &map)
}

#[get("/", rank = 2)]
pub fn index_no_login() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}

/* Hash map and Datatables */
use rdatatables::*;

/* This one stands for our query data structure */
use crate::models::rdatatables::{DataTablesVoice, DataTablesVoiceClerk, DataTablesVoiceUser};

#[post("/list", data = "<query>")]
pub fn list(
    _adminitrative: AdminUser,
    query: LenientForm<DataTableQuery>,
) -> Json<OutcomeData<(DataTablesVoice, DataTablesVoiceClerk, DataTablesVoiceUser)>> {
    Json(datatables_query::<(
        DataTablesVoice,
        DataTablesVoiceClerk,
        DataTablesVoiceUser,
    )>(
        Tables {
            origin: ("call", "call_id"), /* From */
            fields: vec![
                "call.call_id",
                "call.call_begin_date",
                "call.user_id",
                "call.clerk_id",
                "call.call_file",
                "sysuser.user_id",
                "clerk_info.user_id",
                "clerk_info.clerk_info_exhibition",
                "sysuser.user_name",
            ], /* Fields to seek for */
            join_targets: Some(vec![
                ("inner", ("clerk_info", "user_id"), ("call", "clerk_id")),
                ("inner", ("sysuser", "user_id"), ("call", "user_id")),
            ]), /* Join Targets explained over here */
            datatables_post_query: query.into_inner(), /* Incoming Query */
            query: None,                 /* Our builded query holder */
            condition: None,
        },
        crate::establish_connection(),
    ))
}

#[get("/voice-total-minutes-transacted/<call_id>")]
pub fn voice_total_minutes_transacted(call_id: i32) -> Json<Vec<Count>> {
    use diesel::dsl::sql_query;

    #[allow(non_snake_case)]
    let countUp: Vec<Count> = sql_query(format!(
        "SELECT COUNT(*) FROM voice_chat_transaction WHERE voice_chat_transaction_chat_id={}",
        call_id
    ))
    .load(&crate::establish_connection())
    .expect("We cannot count impossible numbers");

    Json(countUp)
}
