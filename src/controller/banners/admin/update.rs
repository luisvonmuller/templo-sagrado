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

use diesel::prelude::*;
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

/* Importing User struct of our session handler */
use crate::AdminUser;

#[post("/update-banner", data = "<income_data>")]
pub fn update_banner(_administrative: AdminUser, content_type: &ContentType, income_data: Data) {
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
        MultipartFormDataField::text("banner_id"),
        MultipartFormDataField::file("banner_mobile"),
        MultipartFormDataField::file("banner_desktop"),
    ];

    let multipart_form_data = MultipartFormData::parse(content_type, income_data, options).unwrap();

    /* Parse the banner id thats beeing updated */
    let banner_id: i32 = multipart_form_data.texts.get("banner_id").unwrap()[0]
        .text
        .parse::<i32>()
        .expect("Not a intenger value");

    /* Image parsing */
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

    diesel::update(banners::table.filter(banners::banner_id.eq(banner_id)))
        .set(NewBanner {
            banner_creation_date: Utc::now().naive_utc(),
            banner_desktop: match banner_desktop {
                Some(value) => value,
                None => retrieve_banner_desktop(banner_id)[0].to_owned(),
            },
            banner_mobile: match banner_mobile {
                Some(value) => value,
                None => retrieve_banner_mobile(banner_id)[0].to_owned(),
            },
        })
        .execute(&crate::establish_connection())
        .unwrap();
}

fn retrieve_banner_desktop(id: i32) -> Vec<String> {
    use crate::schema::banners;

    banners::table
        .select(banners::banner_desktop)
        .filter(banners::banner_id.eq(id))
        .load::<String>(&crate::establish_connection())
        .expect("No image found")
}

fn retrieve_banner_mobile(id: i32) -> Vec<String> {
    use crate::schema::banners;

    banners::table
        .select(banners::banner_mobile)
        .filter(banners::banner_id.eq(id))
        .load::<String>(&crate::establish_connection())
        .expect("No image found")
}
