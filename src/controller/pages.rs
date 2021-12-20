/* Template */
use rocket_contrib::templates::Template;

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Stabilishing connections to db */
use crate::establish_connection;

/* Table macros */
use diesel::prelude::*;

/* Post stuff */
use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormData, MultipartFormDataField};
use rocket::Data;
use rocket::http::ContentType;

#[get("/all_pages")]
pub fn all_pages() -> Json<String> {
    use crate::schema::{syspage};
    use crate::models::{SysPage};

    let results = syspage::table
        .select(syspage::all_columns)
        .load::<SysPage>(&establish_connection())
        .expect("Some error occured when retrieving chat list. It was registered on logs.");

    Json(serde_json::to_string(&results).unwrap())
   
}

#[get("/")]
pub fn index() -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", '/');
    Template::render("pages/pages/index", &map)
}

#[post("/update-element", data="<element_data>")]
pub fn update_element(content_type: &ContentType, element_data: Data) {
    use crate::schema::{syslayout_item};
    use crate::models::{UpdateSysLayoutItem};

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();
          
    options.allowed_fields.push(MultipartFormDataField::text("syspage_content"));
    options.allowed_fields.push(MultipartFormDataField::text("syslayout_item"));

    let multipart_form_data = MultipartFormData::parse(content_type, element_data, options).unwrap();

    let syspage_content: String = multipart_form_data.texts.get("syspage_content").unwrap()[0].text.to_string();
    let syslayout_item_id: i32 = multipart_form_data.texts.get("syslayout_item").unwrap()[0].text.to_string().parse().unwrap();

    diesel::update(syslayout_item::table.filter(syslayout_item::syslayout_item_id.eq(syslayout_item_id)))
    .set(UpdateSysLayoutItem {
        syspage_content: syspage_content
    })
    .execute(&establish_connection())
    .unwrap();

}


#[post("/update-img-element", data="<element_data>")]
pub fn update_img_element(content_type: &ContentType, element_data: Data) {
    use crate::schema::{syslayout_item};
    use crate::models::{UpdateSysLayoutItem};

    /* File system functions */
    use std::fs;

    /* Pass hashing */
    use crypto::digest::Digest;
    use crypto::sha2::Sha512;
        
    /* Hasher for filename */
    let mut hasher = Sha512::new();
    hasher.input_str(&Utc::now().naive_utc().to_string());
  
    //Lenient form imports
    use chrono::{Utc};

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();
          
    options.allowed_fields.push(MultipartFormDataField::file("syspage_content"));
    options.allowed_fields.push(MultipartFormDataField::text("syslayout_item"));

    let multipart_form_data = MultipartFormData::parse(content_type, element_data, options).unwrap();

    let tmp_abstract_img = &multipart_form_data.files.get("syspage_content").unwrap()[0];

    let syslayout_item_id: i32 = multipart_form_data.texts.get("syslayout_item").unwrap()[0].text.to_string().parse().unwrap();

    let file_field = &tmp_abstract_img; // Because we only put one "photo" field to the allowed_fields, the max length of this file_fields is 1.

    let _content_type = &file_field.content_type;
    let _file_name = &file_field.file_name;
    let _path = &file_field.path;

    let format: Vec<&str> = _file_name.as_ref().unwrap().split('.').collect(); /* Reparsing the fileformat */

     /* Path parsing */
     let absolute_path: String = format!("/var/www/templo-sagrado.com/assets/home/img/{}.{}", &hasher.result_str(), format[1].to_string()); 
     let new_layout_image = format!("/assets/home/img/{}.{}", &hasher.result_str(), format[1].to_string()); 
     fs::copy(_path, &absolute_path).unwrap();

    diesel::update(syslayout_item::table.filter(syslayout_item::syslayout_item_id.eq(syslayout_item_id)))
    .set(UpdateSysLayoutItem {
        syspage_content: new_layout_image
    })
    .execute(&establish_connection())
    .unwrap();

    
}

#[get("/retrive-element-content/<syslayout_item_id>")]
pub fn retrieve_element_content(syslayout_item_id: i32) -> Json<String> {
    use crate::schema::{syslayout_item};

    let results = syslayout_item::table
        .select(syslayout_item::syspage_content)
        .filter(syslayout_item::syslayout_item_id.eq(syslayout_item_id))
        .load::<String>(&establish_connection())
        .expect("Some error occured when retrieving chat list. It was registered on logs.");

    Json(serde_json::to_string(&results).unwrap())
}