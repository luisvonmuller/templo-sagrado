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

/* Import estabilish connection from main */
use crate::establish_connection;

/* Lenient form imports */
use diesel::prelude::*;

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

#[get("/retrieve_categories")]
pub fn retrieve_categories(_user: AdminUser) -> Json<String> {
    use crate::models::ProductCategory;
    use crate::schema::product_category;
    let results = product_category::table
		.select(product_category::all_columns)
		.load::<ProductCategory>(&establish_connection())
		.expect("Some shit happned while retrieving the full list of the products categories.! <Panic at the Disco> ops, <Panic at the thread!>");

    Json(serde_json::to_string(&results).unwrap())
}

#[post("/new-product", data = "<product>")]
pub fn new_product(_user: AdminUser, content_type: &ContentType, product: Data) {
    /* Database */
    use crate::models::NewProduct;
    use crate::schema::product;
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
        MultipartFormDataField::file("product_image"),
        MultipartFormDataField::text("product_name"),
        MultipartFormDataField::text("product_description"),
        MultipartFormDataField::text("product_points_value"),
        MultipartFormDataField::text("product_real_value"),
        MultipartFormDataField::text("product_category_id"),
    ];

    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, product, options).unwrap();

    /* Each one of them return an option */
    let product_image = multipart_form_data.files.get("product_image");
    let product_name = multipart_form_data.texts.get("product_name");
    let product_category_id = multipart_form_data
        .texts
        .get("product_category_id")
        .unwrap()[0]
        .text
        .to_string();
    let product_description = multipart_form_data.texts.get("product_description");
    let product_points_value = multipart_form_data.texts.get("product_points_value");
    let product_real_value = multipart_form_data.texts.get("product_real_value");

    let product_img = match product_image {
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

    diesel::insert_into(product::table)
        .values(NewProduct {
            /* Match user input for product_name */
            product_title: match product_name {
                Some(name) => name[0].text.to_string(),
                None => "NADA INFORMADO".to_string(),
            },
            /* Match user input for product Real Val */
            product_real_value: match product_real_value {
                Some(value) => value[0].text.parse::<f64>().unwrap(),
                None => 0.0,
            },
            /* Match user input for points */
            product_points_value: match product_points_value {
                Some(value) => Some(value[0].text.parse::<i32>().unwrap()),
                None => None,
            },
            /* Match user input for product_name */
            product_description: match product_description {
                Some(desc) => Some(desc[0].text.to_string()),
                None => None,
            },
            product_category_id: product_category_id.parse::<i32>().unwrap(),
            product_image: product_img,
        })
        .execute(&establish_connection())
        .unwrap();
}
