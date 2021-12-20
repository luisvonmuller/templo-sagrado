/*
888     888 8888888b.  8888888b.        d8888 88888888888 8888888888
888     888 888   Y88b 888  "Y88b      d88888     888     888
888     888 888    888 888    888     d88P888     888     888
888     888 888   d88P 888    888    d88P 888     888     8888888
888     888 8888888P"  888    888   d88P  888     888     888
888     888 888        888    888  d88P   888     888     888
Y88b. .d88P 888        888  .d88P d8888888888     888     888
"Y88888P"  888        8888888P" d88P     888     888     8888888888

* To acchieve an update for the product table we need to display the current
 * information in the specific product to be edited and submitted by POST.
 * Whenever we may do this, we uncomment this section and the one on the
 * diesel::update so that we can fill the fields with the submitted contents
*/
use crate::establish_connection;

/* Macros. */
use diesel::prelude::*;

/* Importing User struct of our session handler */
use crate::AdminUser;

/* Multipart Form */
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

/* Json type response (must have) */
use rocket_contrib::json::Json;

pub fn retrieve_image(product_id: i32) -> String {
    use crate::schema::product;
    product::table
    .select(product::product_image)
    .filter(product::product_id.eq(product_id))
    .load::<Option<String>>(&establish_connection())
    .expect("Some shit happned while retrieving the full list of users.!!! <Panic at the Disco> ops, thread!")[0].as_ref().unwrap().to_string()
}

#[post("/update-product", data = "<product>")]
pub fn edit_product(_user: AdminUser, content_type: &ContentType, product: Data) {
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
    /* Image */
    options.allowed_fields = vec![
        MultipartFormDataField::file("product_image"),
        MultipartFormDataField::text("product_name"),
        MultipartFormDataField::text("edit_product_category_id"),
        MultipartFormDataField::text("product_description"),
        MultipartFormDataField::text("product_points_value"),
        MultipartFormDataField::text("product_real_value"),
        MultipartFormDataField::text("edit_product_id"),
    ];

    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, product, options).unwrap();

    /* Each one of them return an option */
    let product_image = multipart_form_data.files.get("product_image");
    let product_name = multipart_form_data.texts.get("product_name");
    let product_id = multipart_form_data.texts.get("edit_product_id").unwrap()[0]
        .text
        .to_string(); /* This one is obligatory so if panic no tread */
    let _product_category_id = multipart_form_data
        .texts
        .get("edit_product_category_id")
        .unwrap()[0]
        .text
        .to_string(); /* This one is obligatory so if panic no tread */
    let product_description = multipart_form_data.texts.get("product_description");
    let product_points_value = multipart_form_data.texts.get("product_points_value");
    let product_real_value = multipart_form_data.texts.get("product_real_value");
    let product_img = match product_image {
        Some(img) => {
            if img[0].file_name != None {
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
            } else {
                Some(retrieve_image(product_id.parse::<i32>().unwrap()))
            }
        }
        None => Some(retrieve_image(product_id.parse::<i32>().unwrap())),
    };

    diesel::update(
        product::table.filter(product::product_id.eq(product_id.parse::<i32>().unwrap())),
    )
    .set(NewProduct {
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
        product_category_id: _product_category_id.parse::<i32>().unwrap(),
        product_image: product_img,
    })
    .execute(&establish_connection())
    .unwrap();
}

#[get("/update-status/<id>/<status>")]
pub fn update_product_status(_user: AdminUser, id: i32, status: bool) -> Json<bool> {
    use crate::schema::product;
    use diesel::prelude::*;

    diesel::update(product::table.filter(product::product_id.eq(id)))
        .set(product::product_is_active.eq(!status))
        .execute(&establish_connection())
        .expect("Shit happn");

    Json(true)
}
