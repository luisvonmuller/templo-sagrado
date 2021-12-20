/*
TODO:
     ->  List ( X ) - > Insert ( _ HALF WAY DONE _ Shitty lenient form lol ) - > Delete  (  )
*/

use super::*;

use crate::models::{AttendanceTag, ClerkTag, NewAttendanceTag, NewClerkTag};
use crate::schema::{attendance_tag, clerk_tag};

use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

/* Orm Stuff */
use diesel::prelude::*;

/* Shows the Config template */
#[get("/")]
pub fn index(_administrative: AdminUser) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/admin/attendance-tags");
    Template::render("pages/config/attendance-tags", &map)
}

#[get("/", rank = 2)]
pub fn index_no_login() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}

#[post("/new-tag", data = "<form_data>")]
pub fn new_tag(
    _administrative: AdminUser,
    content_type: &ContentType,
    form_data: Data,
) -> Json<bool> {
    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::text("tag_name"),
        MultipartFormDataField::text("tag_slug"),
    ];

    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, form_data, options).unwrap();

    /* Each one of them return an option */
    let tag_name = &multipart_form_data.texts.get("tag_name").unwrap()[0].text;
    let tag_slug = &multipart_form_data.texts.get("tag_slug").unwrap()[0].text;

    /* parse each into a named correspondent variable */
    diesel::insert_into(attendance_tag::table)
        .values(NewAttendanceTag {
            attendance_tag_name: tag_name.to_string(),
            attendance_tag_slug: tag_slug.to_string(),
        })
        .execute(&crate::establish_connection())
        .expect("Shit happens all the time");

    Json(true)
}

#[get("/list")]
pub fn list_tags(_administrative: AdminUser) -> Json<Vec<AttendanceTag>> {
    let results: Vec<AttendanceTag> = attendance_tag::table
        .select(attendance_tag::all_columns)
        .load::<AttendanceTag>(&crate::establish_connection())
        .expect("We cannot do this.");

    Json(results)
}

#[get("/delete-tag/<tag_id>")]
pub fn delete_tag(_administrative: AdminUser, tag_id: i32) -> Json<bool> {
    /* 1st, DELETE any REFERENCE to the tag that are being deleted */
    diesel::delete(clerk_tag::table.filter(clerk_tag::clerk_tag_attendance_tag_id.eq(tag_id)))
        .execute(&crate::establish_connection())
        .expect("Whoops, we can't delete this.");

    /* Then delete the correspondent tag */
    diesel::delete(attendance_tag::table.filter(attendance_tag::attendance_tag_id.eq(tag_id)))
        .execute(&crate::establish_connection())
        .expect("Whoops, we can't delete this.");

    Json(true)
}

#[get("/get-clerk-tags/<clerk_id>")]
pub fn get_clerk_tags(
    _administrative: AdminUser,
    clerk_id: i32,
) -> Json<Vec<(ClerkTag, AttendanceTag)>> {
    /* We opted to send ClerkTag tupling with the attendancee tag cuz its cool! Nah just
    kidding there is a true meaning on doing this, but for sure I dont know which is! */
    let clerk_tags: Vec<(ClerkTag, AttendanceTag)> = clerk_tag::table
        .inner_join(attendance_tag::table)
        .select((clerk_tag::all_columns, attendance_tag::all_columns))
        .filter(clerk_tag::clerk_tag_user_id.eq(clerk_id))
        .load::<(ClerkTag, AttendanceTag)>(&crate::establish_connection())
        .expect("We cannot do this");

    Json(clerk_tags)
}

#[post("/relate", data = "<form_data>")]
pub fn relate(
    _administrative: AdminUser,
    content_type: &ContentType,
    form_data: Data,
) -> Json<bool> {
    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::text("user_id"),
        MultipartFormDataField::text("tags"),
    ];

    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, form_data, options).unwrap();

    /* Each one of them return an option */
    let user_id: i32 = multipart_form_data.texts.get("user_id").unwrap()[0]
        .text
        .parse::<i32>()
        .unwrap();

    let tags = &multipart_form_data.texts.get("tags").unwrap()[0]
        .text
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|tag| tag.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    for tag_id in tags {
        diesel::insert_into(clerk_tag::table)
            .values(NewClerkTag {
                clerk_tag_user_id: user_id,
                clerk_tag_attendance_tag_id: tag_id.to_owned(),
            })
            .execute(&crate::establish_connection())
            .expect("Whops, we cannot RELATE tags.");
    }

    Json(true)
}

#[get("/unrelate/<clerk_tag_id>")]
pub fn unrelate(_administrative: AdminUser, clerk_tag_id: i32) -> Json<bool> {
    diesel::delete(clerk_tag::table.filter(clerk_tag::clerk_tag_id.eq(clerk_tag_id)))
        .execute(&crate::establish_connection())
        .expect("Whoops, we can't delete this.");

    Json(true)
}
