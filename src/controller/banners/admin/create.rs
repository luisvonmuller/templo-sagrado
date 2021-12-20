/*
8888888b.  8888888888 .d8888b.  8888888 .d8888b. 88888888888 8888888888 8888888b.  8888888 888b    888  .d8888b.
888   Y88b 888       d88P  Y88b   888  d88P  Y88b    888     888        888   Y88b   888   8888b   888 d88P  Y88b
888    888 888       888    888   888  Y88b.         888     888        888    888   888   88888b  888 888    888
888   d88P 8888888   888          888   "Y888b.      888     8888888    888   d88P   888   888Y88b 888 888
8888888P"  888       888  88888   888      "Y88b.    888     888        8888888P"    888   888 Y88b888 888  88888
888 T88b   888       888    888   888        "888    888     888        888 T88b     888   888  Y88888 888    888
888  T88b  888       Y88b  d88P   888  Y88b  d88P    888     888        888  T88b    888   888   Y8888 Y88b  d88P
888   T88b 8888888888 "Y8888P88 8888888 "Y8888P"     888     8888888888 888   T88b 8888888 888    Y888  "Y8888P88
*/

use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

/* Lenient form imports */
use diesel::prelude::*;

/* Importing User struct of our session handler */
use crate::AdminUser;

#[post("/new-banner", data = "<income_data>")]
pub fn new_banner(_administrative: AdminUser, content_type: &ContentType, income_data: Data) {
    /* Database */
    use crate::models::NewBanner;
    use crate::schema::banners;

    /* File parsing */
    use std::fs;

    /* Date processing */
    use chrono::Utc;
    /* File name hashing */
    use crypto::digest::Digest;
    use crypto::sha2::Sha512;

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::file("banner_mobile"),
        MultipartFormDataField::file("banner_desktop"),
    ];

    let multipart_form_data = MultipartFormData::parse(content_type, income_data, options).unwrap();

    let incoming_banner_desktop = multipart_form_data.files.get("banner_desktop");
    let incoming_banner_mobile = multipart_form_data.files.get("banner_mobile");

    let banner_desktop = match incoming_banner_desktop {
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

    let banner_mobile = match incoming_banner_mobile {
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

    /* Till now everything must be great, so lets insert into the database */

    diesel::insert_into(banners::table)
        .values(NewBanner {
            banner_creation_date: Utc::now().naive_utc(),
            banner_desktop: banner_desktop.unwrap().to_owned(),
            banner_mobile: banner_mobile.unwrap().to_owned(),
        })
        .execute(&crate::establish_connection())
        .unwrap();
}
