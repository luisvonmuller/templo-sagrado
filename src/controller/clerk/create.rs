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
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

/* Lenient form imports */
use chrono::{NaiveDate, Utc};
use diesel::prelude::*;

/* Importing User struct of our session handler */
use crate::AdminUser;

#[post("/new-clerk", data = "<clerk>")]
pub fn new_clerk(_administrative: AdminUser, content_type: &ContentType, clerk: Data) {
    /* Importings structs an macros down here */

    use crate::models::{
        NewAddress, NewClerkBank, NewClerkInfo, NewPhone, NewStatusClerk, NewSysUser,
    };
    use crate::schema::{address, clerk_bank, clerk_info, phone, status_clerk, sysuser};

    use std::fs;

    /* Database connection */
    use crate::establish_connection;

    /* */
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    /* Pass hashing */
    use crypto::digest::Digest;
    use crypto::sha2::Sha512;

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    /* Allowow parsing text stuff */

    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_name"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_phone_number"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_genre"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_birthdate"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_cpf"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_comission_rate"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_zip_code"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_city"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_state"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_street"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_addr_num"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_email"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_password"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_uni"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_experience"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_phrase"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_oracles"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_description"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_bank_name"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_bank_account_type"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_bank_agency_number"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_bank_acc_number"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_bank_cpf"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_chat"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_mail"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_voice"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_webcam"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_exhibition"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_priority"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_long_description"));

    /* Allow Clerk Profile image parsing */
    options.allowed_fields.push(
        MultipartFormDataField::file("clerk_image")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
    );

    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, clerk, options).unwrap();

    /* Instanciating variables for text fields */
    let user_name = multipart_form_data.texts.get("user_name").unwrap()[0]
        .text
        .to_string();
    let user_phone_number = multipart_form_data.texts.get("user_phone_number").unwrap()[0]
        .text
        .to_string();
    let user_genre = multipart_form_data.texts.get("user_genre").unwrap()[0]
        .text
        .to_string();
    let user_birthdate = multipart_form_data.texts.get("user_birthdate").unwrap()[0]
        .text
        .to_string();
    let clerk_info_cpf = multipart_form_data.texts.get("clerk_info_cpf").unwrap()[0]
        .text
        .to_string();
    let clerk_info_comission_rate = multipart_form_data
        .texts
        .get("clerk_info_comission_rate")
        .unwrap()[0]
        .text
        .to_string();
    let user_zip_code = multipart_form_data.texts.get("user_zip_code").unwrap()[0]
        .text
        .to_string();
    let user_city = multipart_form_data.texts.get("user_city").unwrap()[0]
        .text
        .to_string();
    let user_state = multipart_form_data.texts.get("user_state").unwrap()[0]
        .text
        .to_string();
    let user_street = multipart_form_data.texts.get("user_street").unwrap()[0]
        .text
        .to_string();
    let user_addr_num = multipart_form_data.texts.get("user_addr_num").unwrap()[0]
        .text
        .to_string();
    let user_email = multipart_form_data.texts.get("user_email").unwrap()[0]
        .text
        .to_string();
    let user_password = multipart_form_data.texts.get("user_password").unwrap()[0]
        .text
        .to_string();
    let clerk_info_experience = multipart_form_data
        .texts
        .get("clerk_info_experience")
        .unwrap()[0]
        .text
        .to_string();
    let clerk_info_phrase = multipart_form_data.texts.get("clerk_info_phrase").unwrap()[0]
        .text
        .to_string();
    let clerk_bank_name = multipart_form_data.texts.get("clerk_bank_name").unwrap()[0]
        .text
        .to_string();
    let clerk_bank_account_type = multipart_form_data
        .texts
        .get("clerk_bank_account_type")
        .unwrap()[0]
        .text
        .to_string();
    let clerk_bank_agency_number = multipart_form_data
        .texts
        .get("clerk_bank_agency_number")
        .unwrap()[0]
        .text
        .to_string();
    let clerk_bank_acc_number = multipart_form_data
        .texts
        .get("clerk_bank_acc_number")
        .unwrap()[0]
        .text
        .to_string();
    let clerk_bank_cpf = multipart_form_data.texts.get("clerk_bank_cpf").unwrap()[0]
        .text
        .to_string();
    let clerk_info_chat = multipart_form_data.texts.get("clerk_info_chat").unwrap()[0]
        .text
        .to_string();
    let clerk_info_mail = multipart_form_data.texts.get("clerk_info_mail").unwrap()[0]
        .text
        .to_string();
    let clerk_info_voice = multipart_form_data.texts.get("clerk_info_voice").unwrap()[0]
        .text
        .to_string();
    let clerk_info_webcam = multipart_form_data.texts.get("clerk_info_webcam").unwrap()[0]
        .text
        .to_string();
    let clerk_info_priority = multipart_form_data
        .texts
        .get("clerk_info_priority")
        .unwrap()[0]
        .text
        .to_string();
    let clerk_info_exhibition = multipart_form_data
        .texts
        .get("clerk_info_exhibition")
        .unwrap()[0]
        .text
        .to_string();
    let clerk_info_description = multipart_form_data
        .texts
        .get("clerk_info_description")
        .unwrap()[0]
        .text
        .to_string();
    let clerk_info_long_description = multipart_form_data
        .texts
        .get("clerk_info_long_description")
        .unwrap()[0]
        .text
        .to_string();

    // let user_uni = multipart_form_data.texts.get("user_uni").unwrap()[0].text.to_string();

    /* Clerk profile image assignment */
    let clerk_image = &multipart_form_data.files.get("clerk_image").unwrap();

    /* Pash hashing by sha512 */
    let mut hasher = Sha512::new();
    hasher.input_str(&user_password);
    let birth_split: Vec<&str> = user_birthdate.split("/").collect();

    /* Inserting Sys User */
    let user_inserted_id: i32 = diesel::insert_into(sysuser::table)
        .values(NewSysUser {
            user_name: user_name.to_string(),
            user_email: user_email.to_string(),
            user_password: hasher.result_str(),
            user_birthdate: NaiveDate::from_ymd(
                birth_split[2].parse().unwrap(),
                birth_split[1].parse().unwrap(),
                birth_split[0].parse().unwrap(),
            ),
            user_genre: user_genre.to_string(),
            user_alias: Some(thread_rng().sample_iter(&Alphanumeric).take(30).collect()),
            user_newsletter: true,
            user_creation: Utc::now().naive_utc(),
            user_lasttimeonline: Some(Utc::now().naive_utc()),
            user_balance: 0.00,
            user_bonus: 0.00,
            user_type_id: 2,
            user_status: Some(true),
            user_uni: Some(clerk_info_cpf.to_string()),
            user_fb_id: Some("".to_string()),
        })
        .returning(sysuser::user_id)
        .get_result(&establish_connection())
        .unwrap();

    /* Respective Adress inserting */
    diesel::insert_into(address::table)
        .values(NewAddress {
            address_number: user_addr_num.to_string(),
            address_street: user_street.to_string(),
            address_city: user_city.to_string(),
            address_state: user_state.to_string(),
            address_country: "Europa".to_string(),
            address_postalcode: user_zip_code.to_string(),
            user_id: user_inserted_id,
        })
        .execute(&establish_connection())
        .unwrap();

    use crate::models::enums::Status;

    /* Insert a unique statement that refers to self state */
    diesel::insert_into(status_clerk::table)
        .values(NewStatusClerk {
            clerk_id: user_inserted_id,
            status: Status::Offline as i32,
            is_available_chat: false,
            is_available_voice: false,
            is_available_video: false,
            is_available_mail: false,
        })
        .execute(&crate::establish_connection())
        .unwrap();

    /* Respective phone inserting */
    diesel::insert_into(phone::table)
        .values(NewPhone {
            phone_number: user_phone_number.to_string(),
            user_id: user_inserted_id,
            phone_type_id: 1,
        })
        .execute(&establish_connection())
        .unwrap();

    let file_field = &clerk_image[0]; // Because we only put one "photo" field to the allowed_fields, the max length of this file_fields is 1.

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

    let new_clerk_image = format!(
        "/assets/uploads/{}.{}",
        &hasher.result_str(),
        format[1].to_string()
    );
    fs::copy(_path, &absolute_path).unwrap();

    /* Erros stand for the cpf not beeing sended as just numbers */
    diesel::insert_into(clerk_info::table)
        .values(NewClerkInfo {
            clerk_image: Some(new_clerk_image.to_string()),
            clerk_description: Some(clerk_info_description.to_string()),
            clerk_info_experience: Some(clerk_info_experience.to_string()),
            user_id: user_inserted_id,
            clerk_info_cpf: Some(clerk_info_cpf.to_string()),
            clerk_info_phrase: Some(clerk_info_phrase.to_string()),
            clerk_info_comission_rate: Some(clerk_info_comission_rate.to_string()),
            clerk_info_chat: Some(clerk_info_chat.parse().unwrap()),
            clerk_info_mail: Some(clerk_info_mail.parse().unwrap()),
            clerk_info_voice: Some(clerk_info_voice.parse().unwrap()),
            clerk_info_webcam: Some(clerk_info_webcam.parse().unwrap()),
            clerk_info_exhibition: Some(clerk_info_exhibition.to_string()),
            clerk_info_priority: Some(clerk_info_priority.parse().unwrap_or_else(|_| 0)),
            clerk_info_long_description: Some(clerk_info_long_description.to_string()),
        })
        .execute(&establish_connection())
        .unwrap();

    /* Clerk Bank account information */
    diesel::insert_into(clerk_bank::table)
        .values(NewClerkBank {
            clerk_id: user_inserted_id,
            clerk_bank_name: clerk_bank_name,
            clerk_bank_account_type: clerk_bank_account_type,
            clerk_bank_agency_number: clerk_bank_agency_number,
            clerk_bank_acc_number: clerk_bank_acc_number,
            clerk_bank_cpf: clerk_bank_cpf, /* Since bank account can be of other owner */
        })
        .execute(&establish_connection())
        .unwrap();
}
