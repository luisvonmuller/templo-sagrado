/* Import estabilish connection from main */
use crate::establish_connection;

/* Template */
use rocket_contrib::templates::Template;

/* Multipart Form */
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Importing User struct of our session handler */
use crate::AdminUser;

/* Since we can have a logged out try, we will import redirect and flash */
use rocket::response::{Flash, Redirect};

#[get("/")]
pub fn index(_administrative: AdminUser) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/");
    Template::render("pages/blog/index", &map)
}

#[get("/", rank = 2)]
pub fn index_no_login() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}

#[get("/delete-post/<post_id>")]
pub fn delete_post(_administrative: AdminUser, post_id: i32) {
    use crate::schema::post;
    use diesel::prelude::*;

    diesel::delete(post::table.filter(post::post_id.eq(post_id)))
        .execute(&crate::establish_connection())
        .expect("Whoops, we can't delete this.");
}

#[post("/new-post", data = "<new_post>")]
pub fn new_post(_administrative: AdminUser, content_type: &ContentType, new_post: Data) {
    /* Database */
    use crate::models::NewPost;
    use crate::schema::post;

    /* File parsing */
    use std::fs;

    /* Date processing */
    use chrono::Utc;
    use diesel::prelude::*;

    /* File name hashing */
    use crypto::digest::Digest;
    use crypto::sha2::Sha512;

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::file("post_image"),
        MultipartFormDataField::text("post_title"),
        MultipartFormDataField::text("post_seo_tags"),
        MultipartFormDataField::text("post_seo_desc"),
        MultipartFormDataField::text("post_content"),
    ];

    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, new_post, options).unwrap();

    /* Each one of them return an option */
    let post_image = multipart_form_data.files.get("post_image");
    let post_title = multipart_form_data.texts.get("post_title").unwrap()[0]
        .text
        .to_string();
    let post_seo_tags = multipart_form_data.texts.get("post_seo_tags").unwrap()[0]
        .text
        .to_string();
    let post_seo_desc = multipart_form_data.texts.get("post_seo_desc").unwrap()[0]
        .text
        .to_string();
    let post_content = multipart_form_data.texts.get("post_content").unwrap()[0]
        .text
        .to_string();

    let post_img = match post_image {
        Some(img) => {
            let file_field = &img[0];
            let _content_type = &file_field.content_type;
            let _file_name = &file_field.file_name;
            let _path = &file_field.path;

            /* Lets split name to get format */
            let format: Vec<&str> = _file_name.as_ref().unwrap().split('.').collect(); /* Reparsing the fileformat */

            /* Hasher for filename */
            let mut hasher = Sha512::new();
            hasher.input_str(&Utc::now().naive_utc().to_string());

            /* Path parsing */
            let absolute_path: String = format!(
                "{}{}.{}",
                crate::base_path(),
                &hasher.result_str(),
                format[1].to_string()
            );
            
            fs::copy(_path, &absolute_path).unwrap();
            Some(format!(
                "/assets/uploads/{}.{}",
                &hasher.result_str(),
                format[1].to_string()
            ))
        }
        None => None,
    };

    diesel::insert_into(post::table)
        .values(NewPost {
            post_title: post_title,
            post_image: post_img.unwrap(),
            post_seo_tags: post_seo_tags,
            post_seo_desc: post_seo_desc,
            post_content: post_content,
            post_date: Utc::now().naive_utc().date(),
        })
        .execute(&establish_connection())
        .unwrap();
}

#[get("/list")]
pub fn list(_administrative: AdminUser) -> Json<String> {
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

#[get("/single-post/<post_id>")]
pub fn single(_administrative: AdminUser, post_id: i32) -> Json<String> {
    use crate::models::Post;
    use crate::schema::post;
    use diesel::prelude::*;

    let results = post::table
        .select(post::all_columns)
        .filter(post::post_id.eq(post_id))
        .load::<Post>(&establish_connection())
        .expect("Some Error occured while parsing cookie absolute value. Registered in logs.");

    Json(serde_json::to_string(&results).unwrap())
}

#[post("/edit-post", data = "<edit_post>")]
pub fn edit_post(_administrative: AdminUser, content_type: &ContentType, edit_post: Data) {
    /* Database */
    use crate::models::NewPost;
    use crate::schema::post;

    /* File parsing */
    use std::fs;

    /* Date processing */
    use chrono::Utc;
    use diesel::prelude::*;

    /* File name hashing */
    use crypto::digest::Digest;
    use crypto::sha2::Sha512;

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::file("post_image"),
        MultipartFormDataField::text("post_title"),
        MultipartFormDataField::text("post_seo_tags"),
        MultipartFormDataField::text("post_seo_desc"),
        MultipartFormDataField::text("post_id"),
        MultipartFormDataField::text("post_content"),
    ];

    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, edit_post, options).unwrap();

    let post_title = multipart_form_data.texts.get("post_title").unwrap()[0]
        .text
        .to_string();

    let post_id = multipart_form_data.texts.get("post_id").unwrap()[0]
        .text
        .to_string();

    let post_seo_tags = multipart_form_data.texts.get("post_seo_tags").unwrap()[0]
        .text
        .to_string();
    let post_seo_desc = multipart_form_data.texts.get("post_seo_desc").unwrap()[0]
        .text
        .to_string();
    let post_content = multipart_form_data.texts.get("post_content").unwrap()[0]
        .text
        .to_string();

    let tmp_abstract_img = &multipart_form_data.files.get("post_image").unwrap();

    let post_img = match &tmp_abstract_img[0].file_name {
        Some(_) => {
            let file_field = &tmp_abstract_img[0]; // Because we only put one "photo" field to the allowed_fields, the max length of this file_fields is 1.

            let _content_type = &file_field.content_type;
            let _file_name = &file_field.file_name;
            let _path = &file_field.path;

            /* Lets split name to get format */
            let format: Vec<&str> = _file_name.as_ref().unwrap().split('.').collect(); /* Reparsing the fileformat */

            /* Hasher for filename */
            let mut hasher = Sha512::new();
            hasher.input_str(&Utc::now().naive_utc().to_string());

            /* Path parsing */
            let absolute_path: String = format!(
                "{}{}.{}",
                crate::base_path(),
                &hasher.result_str(),
                format[1].to_string()
            );

            fs::copy(_path, &absolute_path).unwrap();
            Some(format!(
                "/assets/uploads/{}.{}",
                &hasher.result_str(),
                format[1].to_string()
            ))
        }
        None => Some(
            post::table
                .select(post::post_image)
                .filter(post::post_id.eq(post_id.parse::<i32>().unwrap()))
                .load::<String>(&crate::establish_connection())
                .expect("Shit!!!! mMERDADADADDJIOADIJADOIJAOI")[0]
                .to_owned(),
        ),
    };

    diesel::update(post::table.filter(post::post_id.eq(post_id.parse::<i32>().unwrap())))
        .set(NewPost {
            post_title: post_title,
            post_image: post_img.unwrap(),
            post_seo_tags: post_seo_tags,
            post_seo_desc: post_seo_desc,
            post_content: post_content,
            post_date: Utc::now().naive_utc().date(),
        })
        .execute(&establish_connection())
        .unwrap();
}
