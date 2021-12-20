#![feature(proc_macro_hygiene, decl_macro, type_ascription)]
/* Provides Date, Time and other structs. */
extern crate chrono;

/* All macros needed from rocket. */
#[macro_use]
extern crate rocket;
extern crate futures;
extern crate handlebars;
extern crate log;
extern crate new_tokio_smtp;
extern crate rand;
extern crate rocket_cors;
extern crate rocket_multipart_form_data;
extern crate roxmltree;
extern crate tokio;

#[macro_use]
extern crate vec1;

/* The same is valid from diesel. */
#[macro_use]
extern crate diesel;

/* Multipart file handling */
extern crate multipart;

/* I'll not comment this, dã */
extern crate dotenv;

/* Datatatbles rust implementations */
extern crate rdatatables;

/* Data structures for any kind of stuff that u want to insert ou read from. */
pub mod models;

/* Importing schema n'declaring (This generates a lot of query builder by macros). */
pub mod schema;

/* Controller module that listen to routes */
pub mod controller;

/* Our own model functions */
pub mod database;

/* Using simple dot env reader and using some of env functions that are owned by std.*/
use dotenv::dotenv;
use std::env;

/* Attend web socket */
pub mod attend;

/* Text chat web bocket */
pub mod textchat;

/* VoIP chat web glub glub */
pub mod voicechat;

/* Handle bar custom helpers */
pub mod helpers;

/* Importing the diesel common types and traits  */
use diesel::pg::PgConnection;
use diesel::prelude::*;

/* For better performance on websocket I'll set-up threads */
use std::thread;

/* Session sutff */
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Request};

/* Template */
use rocket_contrib::templates::Template;

/* Redirect */
use rocket::response::Redirect;

use tokio::prelude::*;
use tokio::timer::Interval;

use std::time::{Duration, Instant};

pub mod sitemap;

#[derive(Debug)]
pub struct User {
    pub user_id: i32,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = rocket::response::Redirect;

    fn from_request(
        request: &'a Request<'r>,
    ) -> rocket::Outcome<User, (rocket::http::Status, rocket::response::Redirect), ()> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| match cookie {
                cookie => cookie.value().parse().ok(),
            })
            .map(|id| User { user_id: id })
            .or_forward(())
    }
}
// new Session();
#[derive(Debug)]
pub struct AdminUser(usize);

impl<'a, 'r> FromRequest<'a, 'r> for AdminUser {
    type Error = rocket::response::Redirect;

    fn from_request(
        request: &'a Request<'r>,
    ) -> rocket::Outcome<AdminUser, (rocket::http::Status, rocket::response::Redirect), ()> {
        request
            .cookies()
            .get_private("easytarot_administrative_user")
            .and_then(|cookie| match cookie {
                cookie => cookie.value().parse().ok(),
            })
            .map(|id| AdminUser(id))
            .or_forward(())
    }
}

#[catch(404)]
fn not_found(_req: &Request) -> Redirect {
    Redirect::to("/login")
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

/* Returns base uri path to uploads and things like that, MUST & MUST use */
pub fn base_path() -> String {
    String::from("/var/www/templo-sagrado.com/assets/uploads/")
}

pub fn get_values() -> (f64, f64, f64) {
    use crate::schema::config;
    use diesel::prelude::*;

    config::table
        .select((
            config::site_mail_val,
            config::absolute_min_value_chat,
            config::absolute_min_value_voice,
        ))
        .load::<(f64, f64, f64)>(&establish_connection())
        .expect("Error parsing values!")[0]
}

/* This functions gets from database getting the current amount voice miuntes and downs it by one */  
pub fn voice_minutes_take_off() { 
    use crate::schema::global_states;
    use diesel::prelude::*;


    let old_value: i32 = global_states::table.select(
        global_states::voice_minutes
    ).load::<i32>(&establish_connection())
    .expect("Nothing wrong happned")[0];

    let new_value : i32 = old_value - 1;

    diesel::update(
        global_states::table.filter(
            global_states::global_states_id.eq(1)
        )
    )
    .set(
        global_states::voice_minutes.eq(new_value)
    )
    .execute(&crate::establish_connection())
    .expect("We cannot update this, dude.");

}

pub fn establish_connection() -> PgConnection {
    /* Checks of dotenv setup is ok :P */
    dotenv().ok();

    /* Instanciate an database_url from the .env */
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    /* On rust you don't use return, just dont put a semi-colon on the end of stmt
    and it will be returned! :O */
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}



fn main() {
    /* This will clear the clerk status on compile time */
    crate::controller::clear_online_status(None);
    /* Initializing panic! hook */
    crate::controller::logs::panics::init();

    //sitemap::sitemap_generate("https://templo-sagrado.com", "PT");

    thread::Builder::new()
        .name("Build sitemap automatically daily".into())
        .spawn(|| {
            // { day: 86400, week: 604800, month: 2592000, }
            let task = Interval::new(Instant::now(), Duration::from_secs(86400))
                .for_each(|_instant| {
                    sitemap::sitemap_generate("https://templo-sagrado.com", "PT");
                    Ok(())
                })
                .map_err(|e| panic!("interval errored; err={:?}", e));
            tokio::run(task);
        })
        .unwrap();

    thread::Builder::new()
        .name("Attend web Socket thread".into())
        .spawn(|| {
            attend::websocket();
        })
        .unwrap();

    thread::Builder::new()
        .name("Text Chat web Socket thread".into())
        .spawn(|| {
            textchat::websocket();
        })
        .unwrap();

    thread::Builder::new()
        .name("Voice Chat web Socket thread".into())
        .spawn(|| {
            voicechat::websocket();
        })
        .unwrap();

    /* Every route that stands for a no_login is repsonsable of handling no valid session template alternative */
    rocket::ignite()
        .register(catchers![internal_error, not_found])
        .mount(
            "/",
            routes![
                /* Paypal New Module */
                controller::sales::paypal::new_paypal_sale,
                controller::sales::paypal::update_paypal_sale,
                /* Novo perfil de atendente */
                controller::home::clerk_attendance_tags,
                controller::home::clerk_schedule,
                controller::home::clerk_ratings,
                controller::home::clerk_testimonials,
                controller::home::clerk_profile_id,
                /* Novos atendimentos */
                controller::home::attendance::clerk_attendance,
                controller::home::attendance::my_last_status,
                controller::home::attendance::my_behaviour_on_a_day,
                controller::home::attendance::register_event_by_clerk,
                controller::home::attendance::voice_transactions_data,
                controller::home::attendance::text_transactions_data,
                controller::home::attendance::earned_since_begun,
                /* Start Automated functions */
                controller::assets,
                controller::robots,
                controller::sitemap,
                /* END automated functions */
                /* Mail Implementations from home */
                controller::home::retrive_whole_chat_user,
                controller::home::get_my_mail,
                controller::home::get_answer_email,
                /* END Mail Implementations from home */
                /* The new credits function */
                controller::home::my_credits,
                /* Meta TEXT chat information */
                controller::home::get_chat_meta_info,
                /* Meta Voice chat information */
                controller::home::get_voip_meta_info,
                /* Meta comission */
                controller::home::get_comission_rate,
                /* EMAIL STUFF */
                controller::user::update::generate_new_pass_by_mail,
                controller::home::user_email,
                controller::home::clerk_email,
                controller::clear_online_status,
                controller::home::mail,
                controller::mail::create::new_email_request,
                controller::mail::create::new_email_response,
                /* END EMAIL STUFF */
                /* Home -> Page <- Privacidade  */
                controller::home::privacidade::privacy_policy,
                controller::home::privacidade::privacy_policy_pt,
                controller::home::privacidade::privacy_policy_no_login,
                controller::home::privacidade::privacy_policy_no_login_pt,
                /* Home -> Page <- Privacidade */
                /* Home -> Json <- Product listing */
                controller::home::product_list,
                /* Home -> Json <- Product listing */
                controller::voice::call_chat,
                controller::voice::register_call,
                controller::home::whats_my_id,
                controller::home::facebook_auth,
                controller::home::my_balance,
                controller::home::minutes_up_voice,
                controller::home::minutes_up,
                controller::home::voip,
                controller::home::register_voice_chat,
                controller::home::index_maintence,
                controller::home::index,
                controller::home::minutes_out_voice,
                controller::home::index_no_login,
                controller::home::privacy,
                controller::home::login,
                controller::home::logout,
                controller::home::logout_pt,
                controller::home::register,
                controller::home::register_pt,
                controller::home::faq,
                controller::home::faq_pt,
                controller::home::faq_no_login,
                controller::home::faq_no_login_pt,
                controller::home::buy_credits,
                controller::home::buy_credits_pt,
                controller::home::buy_credits_no_login,
                controller::home::buy_credits_no_login_pt,
                controller::home::who_clerk,
                controller::home::who_client,
                controller::home::get_all_depos,
                controller::home::tarologist,
                controller::home::tarologist_pt,
                controller::home::tarologist_no_login,
                controller::home::tarologist_no_login_pt,
                controller::home::testimonials,
                controller::home::testimonials_pt,                
                controller::home::testimonials_no_login,
                controller::home::testimonials_no_login_pt,
                controller::home::contact,
                controller::home::contact_pt,
                controller::home::chat,
                controller::home::chat_info,
                controller::home::chat_info_clerk,
                controller::home::client_info,
                controller::home::whats_my_id_no_login,
                controller::home::my_acc_user,
                controller::home::login_screen,
                controller::home::login_screen_pt,
                controller::home::am_i_a_clerk,
                controller::home::tarologist_profile,
                controller::home::tarologist_profile_pt,
                controller::home::tarologist_profile_no_login,
                controller::home::tarologist_profile_no_login_pt,
                controller::home::register_chat,
                controller::home::minutes_out,
                controller::home::end_chat,
                controller::home::page_content,
                controller::home::about_us,
                controller::home::about_us_pt,
                controller::home::about_us_no_login,
                controller::home::about_us_no_login_pt,
                controller::home::check_mail,
                controller::populate_status_clerk,
                controller::home::blog,
                controller::home::blog_pt,
                controller::home::blog_no_login,                
                controller::home::blog_no_login_pt,
                controller::home::all_posts,
                controller::home::notify_me,
                controller::home::single_blog_post,
                controller::home::single_blog_post_pt,
                /* MAIL IMPLEMENTATIONS */
                controller::home::client_mail,
                controller::home::new_client_mail,
                controller::home::get_mail_data,
                controller::home::answer_mail,
                /* END MAIL IMPLEMENTATIONS */
                /* Testimonials implementations */
                controller::testimonials::give_testimonial,
                controller::testimonials::submit_testiomonial,
                /* Home-side Text Chat Transactions implementations */
                controller::transactions::text_chat::register_text_chat_transaction,
                controller::transactions::text_chat::client_sign_text_chat_transaction,
                controller::transactions::text_chat::clerk_sign_text_chat_transaction,
                controller::transactions::text_chat::clerk_text_chat_amount_owned,
                /* Home-side voice Chat Transactions implementations */
                controller::transactions::voice_chat::register_voice_chat_transaction,
                controller::transactions::voice_chat::client_sign_voice_chat_transaction,
                controller::transactions::voice_chat::clerk_sign_voice_chat_transaction,
                controller::transactions::voice_chat::clerk_voice_chat_amount_owned,
                /*  [End] Home-side Voice Chat Transactions implementations [End]*/
            ],
        )
        .mount(
            "/user/",
            routes![
                controller::user::create::new_user,
                controller::user::read::list,
                controller::user::read::client_chat_list,
                controller::user::read::self_data,
                controller::user::read::client_mail_list,
                controller::user::read::client_sales_list,
                controller::user::read::list_hack,
                controller::user::read::get_clerk_name,
                controller::user::update::update_user,
                controller::user::update::generate_new_pass,
            ],
        )
        .mount("/product/", routes![controller::product::read::list,])
        .mount(
            "/admin/",
            routes![
                controller::retrieve_name,
                controller::index,
                controller::index_hack,
                controller::process_login,
                controller::login,
                controller::logout,
                controller::chat::total_minutes_transacted,
                controller::voice::voice_total_minutes_transacted,
                controller::pages,
                controller::register,
                controller::faq,
                controller::buy_minutes,
                controller::clerks,
                controller::products,
                controller::depos,
                controller::contact,
                controller::chat_user,
                controller::chat_clerk,
                controller::my_acc_user,
                controller::reports::generate_payment_report,
                /* Text chat transaction implementations */
                controller::transactions::text_chat::get_paid,
                controller::transactions::text_chat::get_given,
                controller::transactions::text_chat::get_hold,
                controller::transactions::text_chat::get_procesed,
                controller::transactions::text_chat::pay_clerk,
                controller::transactions::text_chat::text_pay_back,
                controller::transactions::text_chat::all_time_paid,
                controller::transactions::text_chat::all_time_given,
                controller::transactions::text_chat::all_time_processed,
                controller::transactions::text_chat::all_time_paid_from_bonus, /* Paid from bonus */
                controller::transactions::text_chat::all_time_paid_from_cash,  /* Paid from balance */
                /* Voice chat transaction implementations */
                controller::transactions::voice_chat::get_paid,
                controller::transactions::voice_chat::get_given,
                controller::transactions::voice_chat::get_hold,
                controller::transactions::voice_chat::get_procesed,
                controller::transactions::voice_chat::pay_clerk,
                controller::transactions::voice_chat::voice_pay_back,
                controller::transactions::voice_chat::all_time_paid,
                controller::transactions::voice_chat::all_time_given,
                controller::transactions::voice_chat::all_time_processed,
                controller::transactions::voice_chat::all_time_paid_from_bonus, /* Paid from bonus */
                controller::transactions::voice_chat::all_time_paid_from_cash,  /* Paid from balance */
            ],
        )
        .mount(
            "/admin/clients/",
            routes![
                controller::clients::index,
                controller::clients::update_balance,
                controller::clients::update_bonus,
                controller::clients::update_client_status,
                controller::clients::index_no_login,
                controller::clients::read::list,
                controller::clients::read::single,
                controller::clients::read::balance,
                controller::clients::read::bonus,
                controller::clients::read::get_birthdates,
            ],
        )
        .mount(
            "/admin/testimonials/",
            routes![
                controller::testimonials::admin::index,
                controller::testimonials::admin::index_no_login,
                controller::testimonials::admin::list,
                controller::testimonials::admin::change_status,
            ],
        )
        .mount(
            "/admin/clerk/schedule",
            routes![
               controller::clerk::schedule::get_clerk_schedule,
               controller::clerk::schedule::set_clerk_schedule,
            ],
        )
        .mount(
            "/admin/attendance-tags/",
            routes![
                controller::config::attendance_tag::index,
                controller::config::attendance_tag::index_no_login,
                controller::config::attendance_tag::list_tags,
                controller::config::attendance_tag::new_tag,
                controller::config::attendance_tag::delete_tag,
                controller::config::attendance_tag::get_clerk_tags,
                controller::config::attendance_tag::relate,
                controller::config::attendance_tag::unrelate,
            ],
        )
        .mount("/admin/intends/",  
            routes![
                controller::intends::index,
                controller::intends::index_no_login,
                controller::intends::read::list_json, /* For admin home, will take just five always */
                controller::intends::read::list,
            ]
        )
        .mount(
            "/admin/product/",
            routes![
                controller::product::index,
                controller::product::index_no_login,
                controller::product::read::list,
                controller::product::read::list_no_login,
                controller::product::read::product_history,
                controller::product::create::new_product,
                controller::product::single::show_product,
                controller::product::update::edit_product,
                controller::product::update::update_product_status,
            ],
        )
        .mount(
            "/admin/clerk/",
            routes![
                controller::clerk::change_status,
                controller::clerk::index,
                controller::clerk::index_no_login,
                controller::clerk::pay_debts,
                controller::clerk::create::new_clerk,
                controller::clerk::update::edit_clerk,
                controller::clerk::read::single,
                controller::clerk::read::list,
            ],
        )
        .mount(
            "/admin/stats/",
            routes![
                controller::stats::minutes_voice,
                controller::stats::total_clients,
                controller::stats::total_active_clients,
                controller::stats::total_clerks,
                controller::stats::total_active_clerks,
                controller::stats::new_clients,
                controller::stats::new_sales_values,
                controller::stats::new_sales,
                controller::stats::new_sales_months,
                controller::stats::days::new_sales_days,
                controller::stats::weeks::new_sales_week,
            ],
        )
        .mount(
            "/admin/attendance-chat/",
            routes![
                controller::chat::index,
                controller::chat::index_no_login,
                controller::chat::user_chats,
                controller::chat::clerk_chats,
                controller::chat::list,
                controller::chat::retrive_whole_chat,
            ],
        )
        .mount(
            "/admin/attendance-voice/",
            routes![
                controller::voice::index,
                controller::voice::index_no_login,
                controller::voice::list,
            ],
        )
        .mount(
            "/admin/attendance-mail/",
            routes![
                controller::mail::index,
                controller::mail::index_no_login,
                controller::mail::list,
                controller::mail::single,
            ],
        )
        .mount(
            "/admin/pages/",
            routes![
                controller::pages::index,
                controller::pages::all_pages,
                controller::pages::update_element,
                controller::pages::retrieve_element_content,
                controller::pages::update_img_element,
            ],
        )
        .mount(
            "/admin/blog/",
            routes![
                controller::blog::index,
                controller::blog::index_no_login,
                controller::blog::edit_post,
                controller::blog::new_post,
                controller::blog::delete_post,
                controller::blog::list,
                controller::blog::single,
            ],
        )
        .mount(
            "/admin/config/",
            routes![
                controller::config::index,
                controller::config::index_no_login,
                controller::config::get_configs,
                controller::config::update_configs,
                controller::config::update_page,
            ],
        )
        .mount(
            "/admin/sales/",
            routes![
                controller::sales::index,
                controller::sales::index_no_login,
                controller::sales::list,
                controller::sales::new_stats,
                controller::sales::create::new_sale,
                controller::sales::create::new_stripe_sale,
                controller::sales::update::update_stripe_sale,
            ],
        )
        .mount(
            "/admin/banners/",
            routes![
                controller::banners::admin::index,
                controller::banners::admin::index_no_login,
                controller::banners::admin::read::list,
                controller::banners::admin::create::new_banner,
                controller::banners::admin::delete::delete_banner,
                controller::banners::admin::update::update_banner,
            ],
        )
        .attach(Template::fairing())
        .launch();
}
