/*

888     888 8888888b.  8888888b.        d8888 88888888888 8888888888
888     888 888   Y88b 888  "Y88b      d88888     888     888
888     888 888    888 888    888     d88P888     888     888
888     888 888   d88P 888    888    d88P 888     888     8888888
888     888 8888888P"  888    888   d88P  888     888     888
888     888 888        888    888  d88P   888     888     888
Y88b. .d88P 888        888  .d88P d8888888888     888     888
 "Y88888P"  888        8888888P" d88P     888     888     8888888888

*/

/* ORM Macros. */
use diesel::prelude::*;

/* Multipart Form */
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

/* Data imports */
use chrono::{NaiveDate, Utc};

/* Importing User struct of our session handler */
use crate::AdminUser;

pub fn retrieve_image(clerk_info_id: i32) -> String {
    use crate::establish_connection;
    use crate::schema::clerk_info;

    let results = clerk_info::table
        .select(clerk_info::clerk_image)
        .filter(clerk_info::clerk_info_id.eq(clerk_info_id))
        .load::<Option<String>>(&establish_connection())
        .expect("Some shit happned while retrieving the full list of users.!!! <Panic at the Disco> ops, thread!");

    results[0].as_ref().unwrap().to_string()
}

#[post("/update-clerk", data = "<clerk>")]
pub fn edit_clerk(_administrative: AdminUser, content_type: &ContentType, clerk: Data) {
    /* Importings structs an macros down here */
    use crate::models::{NewAddress, NewClerkBank, NewClerkInfo, NewPhone, NewSysUser};
    use crate::schema::{address, clerk_bank, clerk_info, phone, sysuser};

    use std::fs;

    /* Database connection */
    use crate::establish_connection;

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
        .push(MultipartFormDataField::text("clerk_description"));
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
        .push(MultipartFormDataField::text("clerk_info_description"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("user_id"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_id"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_bank_id"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("phone_id"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("address_id"));
    options
        .allowed_fields
        .push(MultipartFormDataField::text("clerk_info_long_description"));

    /* Allow Clerk Profile image parsing */
    options
        .allowed_fields
        .push(MultipartFormDataField::file("clerk_image"));

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
    let user_id = multipart_form_data.texts.get("user_id").unwrap()[0]
        .text
        .to_string();
    let clerk_info_id = multipart_form_data.texts.get("clerk_info_id").unwrap()[0]
        .text
        .to_string();
    let clerk_bank_id = multipart_form_data.texts.get("clerk_bank_id").unwrap()[0]
        .text
        .to_string();
    let phone_id = multipart_form_data.texts.get("phone_id").unwrap()[0]
        .text
        .to_string();
    let address_id = multipart_form_data.texts.get("address_id").unwrap()[0]
        .text
        .to_string();
    let clerk_info_long_description = multipart_form_data
        .texts
        .get("clerk_info_long_description")
        .unwrap()[0]
        .text
        .to_string();

    /* Clerk profile image assignment */
    let tmp_abstract_img = &multipart_form_data.files.get("clerk_image").unwrap();

    match &tmp_abstract_img[0].file_name {
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

            let new_clerk_image = format!(
                "/assets/uploads/{}.{}",
                &hasher.result_str(),
                format[1].to_string()
            );
            fs::copy(_path, &absolute_path).unwrap();
            diesel::update(
                clerk_info::table
                    .filter(clerk_info::clerk_info_id.eq(clerk_info_id.parse::<i32>().unwrap())),
            )
            .set(NewClerkInfo {
                clerk_image: Some(new_clerk_image.to_string()),
                clerk_description: Some(clerk_info_description.to_string()),
                clerk_info_experience: Some(clerk_info_experience.to_string()),
                user_id: user_id.parse::<i32>().unwrap(),
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
        }
        None => {
            diesel::update(
                clerk_info::table
                    .filter(clerk_info::clerk_info_id.eq(clerk_info_id.parse::<i32>().unwrap())),
            )
            .set(NewClerkInfo {
                clerk_image: Some(retrieve_image(clerk_info_id.parse().unwrap())),
                clerk_description: Some(clerk_info_description.to_string()),
                clerk_info_experience: Some(clerk_info_experience.to_string()),
                user_id: user_id.parse::<i32>().unwrap(),
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
        }
    }

    /* Pash hashing by sha512 */
    let mut hasher = Sha512::new();
    hasher.input_str(&user_password);
    let birth_split: Vec<&str> = user_birthdate.split("/").collect();

    let old_data: f64 = retrieve_debts(user_id.parse::<i32>().unwrap())[0];
    let old_bonus: f64 = retrieve_bonus(user_id.parse::<i32>().unwrap())[0];

    diesel::update(sysuser::table.filter(sysuser::user_id.eq(user_id.parse::<i32>().unwrap())))
        .set(NewSysUser {
            user_name: user_name.to_string(),
            user_email: user_email.to_string(),
            user_password: if crate::controller::user::update::check_pass(
                user_id.parse::<i32>().unwrap(),
                user_password.clone(),
            ) {
                user_password.to_string()
            } else {
                hasher.result_str()
            },
            user_birthdate: NaiveDate::from_ymd(
                birth_split[2].parse().unwrap(),
                birth_split[1].parse().unwrap(),
                birth_split[0].parse().unwrap(),
            ),
            user_genre: user_genre.to_string(),
            user_alias: Some("Tarologo".to_owned()),
            user_newsletter: true,
            user_creation: Utc::now().naive_utc(),
            user_lasttimeonline: Some(Utc::now().naive_utc()),
            user_balance: old_data,
            user_bonus: old_bonus,
            user_type_id: 2,
            user_status: Some(true),
            user_uni: Some(clerk_bank_cpf.to_string()),
            user_fb_id: Some("".to_string()),
        })
        .execute(&establish_connection())
        .unwrap();

    diesel::update(
        address::table.filter(address::address_id.eq(address_id.parse::<i32>().unwrap())),
    )
    .set(NewAddress {
        address_number: user_addr_num.to_string(),
        address_street: user_street.to_string(),
        address_city: user_city.to_string(),
        address_state: user_state.to_string(),
        address_country: "Portugal".to_string(),
        address_postalcode: user_zip_code.to_string(),
        user_id: user_id.parse::<i32>().unwrap(),
    })
    .execute(&establish_connection())
    .unwrap();

    diesel::update(phone::table.filter(phone::phone_id.eq(phone_id.parse::<i32>().unwrap())))
        .set(NewPhone {
            phone_number: user_phone_number.to_string(),
            user_id: user_id.parse::<i32>().unwrap(),
            phone_type_id: 1,
        })
        .execute(&establish_connection())
        .unwrap();

    diesel::update(
        clerk_bank::table
            .filter(clerk_bank::clerk_bank_id.eq(clerk_bank_id.parse::<i32>().unwrap())),
    )
    .set(NewClerkBank {
        clerk_id: user_id.parse::<i32>().unwrap(),
        clerk_bank_name: clerk_bank_name,
        clerk_bank_account_type: clerk_bank_account_type,
        clerk_bank_agency_number: clerk_bank_agency_number,
        clerk_bank_acc_number: clerk_bank_acc_number,
        clerk_bank_cpf: clerk_bank_cpf, /* Since bank account can be of other owner */
    })
    .execute(&establish_connection())
    .unwrap();
}

pub fn retrieve_debts(user_id: i32) -> Vec<f64> {
    use crate::schema::sysuser;

    sysuser::table.select(
        sysuser::user_balance,
    )
    .filter(sysuser::user_id.eq(user_id))
    .load::<f64>(&crate::establish_connection())
    .expect("Some shit happned while retrieving the full list of users.!!! <Panic at the Disco> ops, thread!")
}

pub fn retrieve_bonus(user_id: i32) -> Vec<f64> {
    use crate::schema::sysuser;

    sysuser::table.select(
        sysuser::user_bonus,
    )
    .filter(sysuser::user_id.eq(user_id))
    .load::<f64>(&crate::establish_connection())
    .expect("Some shit happned while retrieving the full list of users.!!! <Panic at the Disco> ops, thread!")
}
