/* Lenient form imports */
use diesel::prelude::*;

/* Import estabilish connection from main */
use crate::establish_connection;

/* Multipart Form */
use rocket::http::ContentType;
use rocket::Data;

use crate::User;

use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

#[post("/new-email-request", data = "<email>")]
pub fn new_email_request(user: User, content_type: &ContentType, email: Data) {
    use crate::models::NewCallEmail;
    use crate::schema::{call_email, sysuser};

    //Lenient form imports
    use chrono::Utc;

    /* ------------------- CREATE A EMAIL ------------------- */
    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::text("email_to_clerk_id"),
        MultipartFormDataField::text("email_title"),
        MultipartFormDataField::text("email_body"),
        MultipartFormDataField::text("email_response_address"),
    ];

    // Fill the options
    let form_data = MultipartFormData::parse(content_type, email, options).unwrap();

    let email_to_clerk = form_data.texts.get("email_to_clerk_id").unwrap()[0]
        .text
        .to_string();
    let email_title = form_data.texts.get("email_title").unwrap()[0]
        .text
        .to_string();
    let email_body = form_data.texts.get("email_body").unwrap()[0]
        .text
        .to_string();
    let email_response_address = form_data.texts.get("email_response_address").unwrap()[0]
        .text
        .to_string();

    /* Insert email request on database */
    diesel::insert_into(call_email::table)
        .values(NewCallEmail {
            call_email_request_title: email_title,
            call_email_request_body: email_body,
            call_email_request_date: Utc::now().naive_utc(),
            call_email_request_to_email: email_response_address,
            user_id: user.user_id as i32,
            clerk_id: email_to_clerk.parse::<i32>().unwrap(),
        })
        .execute(&establish_connection())
        .unwrap();
    /* Update user emails balance on database */
    diesel::update(sysuser::table.filter(sysuser::user_id.eq(user.user_id as i32)))
        .set(
            sysuser::user_balance.eq(
                /* Remove 1 email balance */
                sysuser::user_balance - crate::get_values().0
            )
        )
        .execute(&establish_connection())
        .unwrap();
}

#[post("/new-email-response", data = "<email>")]
pub fn new_email_response(user: User, content_type: &ContentType, email: Data) {
    use crate::schema::{call_email, sysuser};

    //Lenient form imports
    use chrono::Utc;

    /* ------------------- CREATE A EMAIL ------------------- */
    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::text("call_email_id"),
        MultipartFormDataField::text("email_title"),
        MultipartFormDataField::text("email_body"),
    ];

    // Fill the options
    let form_data = MultipartFormData::parse(content_type, email, options).unwrap();

    let call_email_id = form_data.texts.get("call_email_id").unwrap()[0]
        .text
        .to_string()
        .parse::<i32>()
        .unwrap();
    let email_title = form_data.texts.get("email_title").unwrap()[0]
        .text
        .to_string();
    let email_body = form_data.texts.get("email_body").unwrap()[0]
        .text
        .to_string();

    /* Update email request on database */
    diesel::update(call_email::table.filter(call_email::call_email_id.eq(call_email_id)))
        .set((
            call_email::call_email_response_title.eq(email_title),
            call_email::call_email_response_body.eq(email_body),
            call_email::call_email_response_date.eq(Utc::now().naive_utc()),
        ))
        .execute(&establish_connection())
        .unwrap();
    /* Update user emails balance on database */
    diesel::update(sysuser::table.filter(sysuser::user_id.eq(user.user_id as i32)))
        .set(sysuser::user_balance.eq(
            /* Remove 1 email balance */
            sysuser::user_balance - crate::get_values().0,
        ))
        .execute(&establish_connection())
        .unwrap();
}
