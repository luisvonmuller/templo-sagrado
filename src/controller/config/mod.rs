/* Template */
use rocket_contrib::templates::Template;

/* Json type response (must have) */
use rocket_contrib::json::Json;

/* Multipart Form */
use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

/* Importing User struct of our session handler */
use crate::AdminUser;

/* Since we can have a logged out try, we will import redirect and flash */
use rocket::response::{Flash, Redirect};

/* Return Types */
use crate::models::Config;


pub mod attendance_tag;

/* Retrieves configs as desired also its value */
#[get("/get-configs")]
pub fn get_configs() -> Json<Vec<Config>> {
    use crate::establish_connection;
    use crate::models::Config;
    use crate::schema::config;

    /* Diesel macros */
    use diesel::prelude::*;

    let results = config::table
        .select(config::all_columns)
		.filter(config::config_id.eq(1))
		.load::<Config>(&establish_connection())
		.expect("Some shit happned while retrieving the full list of users.!!! <Panic at the Disco> ops, thread!");

    Json(results)
}

#[post("/update-configs", data = "<config_data>")]
pub fn update_configs(_administrative: AdminUser, content_type: &ContentType, config_data: Data) {
    use crate::establish_connection;
    use crate::models::UpdateConfig;
    use crate::schema::config;

    /* Diesel macros */
    use diesel::prelude::*;

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields = vec![
        MultipartFormDataField::text("site_mail_val"),
        MultipartFormDataField::text("site_name"),
        MultipartFormDataField::text("site_new_user_bonus"),
        MultipartFormDataField::text("site_seo_desc"),
        MultipartFormDataField::text("site_seo_tags"),
        MultipartFormDataField::text("absolute_min_value_voice"),
        MultipartFormDataField::text("absolute_min_value_chat"),
    ];
    /* If stuff matches, do stuff */
    let multipart_form_data = MultipartFormData::parse(content_type, config_data, options).unwrap();

    let site_mail_val = multipart_form_data.texts.get("site_mail_val").unwrap()[0]
        .text
        .to_string();
    let site_name = multipart_form_data.texts.get("site_name").unwrap()[0]
        .text
        .to_string();
    let site_new_user_bonus = multipart_form_data
        .texts
        .get("site_new_user_bonus")
        .unwrap()[0]
        .text
        .to_string();
    let site_seo_desc = multipart_form_data.texts.get("site_seo_desc").unwrap()[0]
        .text
        .to_string();
    let site_seo_tags = multipart_form_data.texts.get("site_seo_tags").unwrap()[0]
        .text
        .to_string();
    let absolute_min_value_voice = multipart_form_data
        .texts
        .get("absolute_min_value_voice")
        .unwrap()[0]
        .text
        .to_string();
    let absolute_min_value_chat = multipart_form_data
        .texts
        .get("absolute_min_value_chat")
        .unwrap()[0]
        .text
        .to_string();

    diesel::update(config::table.filter(config::config_id.eq(1)))
        .set(UpdateConfig {
            site_name: site_name,
            site_seo_desc: site_seo_desc,
            site_seo_tags: site_seo_tags,
            site_mail_val: site_mail_val.parse::<f64>().unwrap(),
            site_new_user_bonus: site_new_user_bonus,
            absolute_min_value_chat: absolute_min_value_chat.parse::<f64>().unwrap(),
            absolute_min_value_voice: absolute_min_value_voice.parse::<f64>().unwrap(),
        })
        .execute(&establish_connection())
        .unwrap();
}

/* Shows the Config template */
#[get("/")]
pub fn index(_administrative: AdminUser) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/admin/config/");
    Template::render("pages/config/index", &map)
}

#[get("/", rank = 2)]
pub fn index_no_login() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}

/* Register user input & parses it into the config table */

#[post("/update-page", data = "<page_data>")]
pub fn update_page( _administrative: AdminUser, content_type: &ContentType, page_data: Data) {
    /* Diesel macros */
    use crate::establish_connection;
    use diesel::prelude::*;

    use crate::schema::syspage;

    /* First we declare what we will be accepting on this form */
    let mut options = MultipartFormDataOptions::new();

    options.allowed_fields = vec![
        MultipartFormDataField::text("content"),
        MultipartFormDataField::text("wich"),
    ];

    let multipart_form_data = MultipartFormData::parse(content_type, page_data, options).unwrap();

    let content = multipart_form_data.texts.get("content");
    let page_title = multipart_form_data.texts.get("wich").unwrap()[0]
        .text
        .to_string();

    match content {
        Some(stuff) => {
            diesel::update(syspage::table.filter(syspage::syspage_title.eq(page_title)))
                .set(syspage::syspage_content.eq(stuff[0].text.to_string()))
                .execute(&establish_connection())
                .unwrap();
        }
        None => {
            diesel::update(syspage::table.filter(syspage::syspage_title.eq(page_title)))
                .set(syspage::syspage_content.eq("".to_string()))
                .execute(&establish_connection())
                .unwrap();
        }
    };
}
