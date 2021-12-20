/* To import all  macros from diesel ... */
use crate::schema::*;

/* For JSON */
use serde::Serialize;

/* Date handling... */
use chrono::{NaiveDate, NaiveDateTime};

use diesel::sql_types::{Date, Double, Int4, Nullable, Text, Timestamp, BigInt};

pub mod enums;
pub mod rdatatables;

/* Struct implementation to a better performance and data orientation guidance */
#[derive(QueryableByName, Serialize)]
pub struct BirthdateListing {
    #[sql_type = "Int4"]
    pub user_id: i32,
    #[sql_type = "Text"]
    pub user_name: String,
    #[sql_type = "Date"]
    pub user_birthdate: NaiveDate,
}

#[derive(QueryableByName, Serialize)]
pub struct Day {
    #[sql_type = "Timestamp"]
    pub day: NaiveDateTime,
}

/* This one stands for the r-datatables counting struct */
#[derive(QueryableByName, Serialize)]
pub struct Sum {
    #[sql_type = "Nullable<Double>"]
    pub sum: Option<f64>,
}

/* This one stands for the r-datatables counting struct */
#[derive(QueryableByName, Serialize)]
pub struct Value {
    #[sql_type = "Nullable<Double>"]
    pub value: Option<f64>,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "sysuser"]
pub struct NewSysUser {
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    pub user_birthdate: NaiveDate,
    pub user_genre: String,
    pub user_alias: Option<String>, //Option for beeing Nullable<Type>
    pub user_newsletter: bool,
    pub user_creation: NaiveDateTime,
    pub user_lasttimeonline: Option<NaiveDateTime>, //Option for beeing Nullable<Type>
    pub user_balance: f64,
    pub user_bonus: f64,
    pub user_type_id: i32,
    pub user_status: Option<bool>,
    pub user_uni: Option<String>,
    pub user_fb_id: Option<String>,
}

#[derive(Debug, Queryable, QueryableByName, Serialize)]
#[table_name = "sysuser"]
pub struct SysUser {
    pub user_id: i32,
    pub user_name: String,
    pub user_email: String,
    pub user_password: String,
    pub user_birthdate: NaiveDate,
    pub user_genre: String,
    pub user_alias: Option<String>, //Option for beeing Nullable<Type>
    pub user_newsletter: bool,
    pub user_creation: NaiveDateTime,
    pub user_lasttimeonline: Option<NaiveDateTime>, //Option for beeing Nullable<Type>
    pub user_balance: f64,
    pub user_bonus: f64,
    pub user_type_id: i32,
    pub user_status: Option<bool>,
    pub user_uni: Option<String>,
    pub user_fb_id: Option<String>,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "clerk_info"]
pub struct NewClerkInfo {
    pub clerk_description: Option<String>, //Short description
    pub clerk_info_long_description: Option<String>, //Longe description
    pub clerk_info_experience: Option<String>, // Experience detailament
    pub clerk_image: Option<String>,       // Profile Picture
    pub clerk_info_cpf: Option<String>,    // UNI
    pub clerk_info_phrase: Option<String>,
    pub clerk_info_comission_rate: Option<String>,
    pub clerk_info_chat: Option<bool>,
    pub clerk_info_mail: Option<bool>,
    pub clerk_info_voice: Option<bool>,
    pub clerk_info_webcam: Option<bool>,
    pub clerk_info_exhibition: Option<String>, // Name to be displayed
    pub clerk_info_priority: Option<i32>,      //Priority selector
    pub user_id: i32,                          // Who referes
}

#[derive(Debug, Queryable, QueryableByName, Serialize)]
#[table_name = "clerk_info"]
pub struct ClerkInfo {
    pub clerk_info_id: i32,
    pub clerk_description: Option<String>, //Short description
    pub clerk_info_long_description: Option<String>, //Longe description
    pub clerk_info_experience: Option<String>, // Experience detailament
    pub clerk_image: Option<String>, // Profile Picture
    pub clerk_info_cpf: Option<String>, // UNI 
    pub clerk_info_phrase: Option<String>,
    pub clerk_info_comission_rate: Option<String>,
    pub clerk_info_chat: Option<bool>,
    pub clerk_info_mail: Option<bool>,
    pub clerk_info_voice: Option<bool>,
    pub clerk_info_webcam: Option<bool>,
    pub clerk_info_exhibition: Option<String>, // Name to be displayed
    pub clerk_info_priority: Option<i32>, //Priority selector
    pub user_id: i32, // Who referes

}
#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "clerk_bank"]
pub struct NewClerkBank {
    pub clerk_id: i32,
    pub clerk_bank_name: String,
    pub clerk_bank_account_type: String,
    pub clerk_bank_agency_number: String,
    pub clerk_bank_acc_number: String,
    pub clerk_bank_cpf: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct ClerkBank {
    pub clerk_bank_id: i32,
    pub clerk_id: i32,
    pub clerk_bank_name: String,
    pub clerk_bank_account_type: String,
    pub clerk_bank_agency_number: String,
    pub clerk_bank_acc_number: String,
    pub clerk_bank_cpf: String,
}

#[derive(Insertable, Debug)]
#[table_name = "user_type"]
pub struct NewUserType {
    pub user_type_title: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct UserType {
    pub user_type_id: i32,
    pub user_type_title: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Product {
    pub product_id: i32,
    pub product_title: String,
    pub product_value: f64,
    pub product_bonus: f64,
    pub product_description: String,
    pub product_image: String,
    pub product_is_active: bool,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "product"]
pub struct NewProduct {
    pub product_title: String,
    pub product_value: f64,
    pub product_bonus: f64,
    pub product_description: String,
    pub product_image: String,
    pub product_is_active: bool,
}

#[derive(Debug, Queryable, Serialize)]
pub struct PaypalPayment { 
    pub paypal_payment_id: i32, 
    pub paypal_payment_source_identifier: String, 
    pub paypal_payment_sale_id: i32
}

#[derive(Debug, Queryable, Serialize, Insertable)]
#[table_name = "paypal_payment"]
pub struct NewPaypalPayment { 
    pub paypal_payment_source_identifier: String,
    pub paypal_payment_sale_id: i32
}

#[derive(Debug, Queryable, Serialize)]
pub struct Address {
    pub address_id: i32,
    pub address_number: Option<String>,
    pub address_street: Option<String>,
    pub address_city: Option<String>,
    pub address_state: Option<String>,
    pub address_country: Option<String>,
    pub address_postalcode: Option<String>,
    pub user_id: i32,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "address"]
pub struct NewAddress {
    pub address_number: String,
    pub address_street: String,
    pub address_city: String,
    pub address_state: String,
    pub address_country: String,
    pub address_postalcode: String,
    pub user_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Phone {
    pub phone_id: i32,
    pub phone_number: String,
    pub user_id: i32,
    pub phone_type_id: i32,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "phone"]
pub struct NewPhone {
    pub phone_number: String,
    pub user_id: i32,
    pub phone_type_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct ProductCategory {
    pub product_category_id: i32,
    pub product_category_title: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Chat {
    pub chat_id: i32,
    pub client_id: i32,
    pub clerk_id: i32,
    pub client_socket: String,
    pub clerk_socket: String,
    pub init_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct ChatMsg {
    pub chat_msg_id: i32,
    pub chat_msg_user_id: i32,
    pub chat_msg_body: Option<String>,
    pub chat_msg_time: NaiveDateTime,
    pub chat_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct ProductList {
    pub product_list_id: i32,
    pub product_list_amount: i32,
    pub product_list_use_points: bool,
    pub product_id: i32,
    pub sale_id: i32,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "product_list"]
pub struct NewProductList {
    pub product_list_amount: i32,
    pub product_list_use_points: bool,
    pub product_id: i32,
    pub sale_id: i32,
}

#[derive(Debug, Queryable, QueryableByName, Serialize, Clone)]
#[table_name = "sale"]
pub struct Sale {
    pub sale_id: i32,
    pub sale_date: NaiveDateTime,
    pub sale_real_value: Option<f64>,
    pub sale_points_value: Option<i32>,
    pub user_id: i32,
    pub sale_status: Option<i32>,
    pub sale_payment_source: Option<String>,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "sale"]
pub struct NewSale {
    pub sale_date: NaiveDateTime,
    pub sale_real_value: Option<f64>,
    pub sale_points_value: Option<i32>,
    pub user_id: i32,
    pub sale_status: Option<i32>,
    pub sale_payment_source: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[table_name = "call"]
pub struct UpdateCallFile {
    pub call_end_date: Option<NaiveDateTime>,
    pub call_file: Option<String>,
}

#[derive(Debug, AsChangeset)]
#[table_name = "syslayout_item"]
pub struct UpdateSysLayoutItem {
    pub syspage_content: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct SysPage {
    pub syspage_id: i32,
    pub syspage_title: String,
    pub syspage_contet: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Config {
    pub config_id: i32,
    pub site_name: String,
    pub site_seo_desc: String,
    pub site_seo_tags: String,
    pub site_mail_val: f64,
    pub site_new_user_bonus: String,
    pub absolute_min_value_chat: f64,
    pub absolute_min_value_voice: f64,
}

#[derive(Debug, Queryable, AsChangeset)]
#[table_name = "config"]
pub struct UpdateConfig {
    pub site_name: String,
    pub site_seo_desc: String,
    pub site_seo_tags: String,
    pub site_mail_val: f64,
    pub site_new_user_bonus: String,
    pub absolute_min_value_chat: f64,
    pub absolute_min_value_voice: f64,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "post"]
pub struct NewPost {
    pub post_title: String,
    pub post_image: String,
    pub post_seo_tags: String,
    pub post_seo_desc: String,
    pub post_content: String,
    pub post_date: NaiveDate,
}

#[derive(Debug, QueryableByName, Queryable, Serialize)]
#[table_name = "post"]
pub struct Post {
    pub post_id: i32,
    pub post_title: String,
    pub post_image: String,
    pub post_seo_tags: String,
    pub post_seo_desc: String,
    pub post_content: String,
    pub post_date: NaiveDate,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "email_notification_list"]
pub struct EmailNotificationListJoin {
    pub client_id: i32,
    pub clerk_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct EmailNotificationList {
    pub email_notification_id: i32,
    pub client_id: i32,
    pub clerk_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Message {
    pub message_id: i32,
    pub clerk_info_id: i32,
    pub user_id: i32,
    pub message_header: String,
    pub message_body: String,
}

#[derive(Insertable, Debug)]
#[table_name = "status_clerk"]
pub struct NewStatusClerk {
    pub clerk_id: i32,
    pub status: i32,
    pub is_available_chat: bool,
    pub is_available_voice: bool,
    pub is_available_video: bool,
    pub is_available_mail: bool,
}

#[derive(Debug, Queryable, Serialize)]
pub struct StatusClerk {
    pub status_clerk_id: i32,
    pub clerk_id: i32,
    pub status: i32,
    pub is_available_chat: bool,
    pub is_available_voice: bool,
    pub is_available_video: bool,
    pub is_available_mail: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "chat_msg"]
pub struct NewChatMsg {
    pub chat_msg_user_id: i32,
    pub chat_msg_body: Option<String>,
    pub chat_msg_time: NaiveDateTime,
    pub chat_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "chat"]
pub struct NewChat {
    pub client_id: i32,
    pub clerk_id: i32,
    pub client_socket: String,
    pub clerk_socket: String,
    pub init_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[table_name = "call"]
pub struct NewCall {
    pub call_begin_date: NaiveDateTime,
    pub call_end_date: Option<NaiveDateTime>,
    pub user_id: i32,
    pub clerk_id: i32,
    pub call_file: Option<String>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Call {
    pub call_id: i32,
    pub call_begin_date: NaiveDateTime,
    pub call_end_date: Option<NaiveDateTime>,
    pub user_id: i32,
    pub clerk_id: i32,
    pub call_file: Option<String>,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "stripe_payment"]
pub struct NewStripePayment {
    pub stripe_payment_source: String,
    pub sale_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct StripePayment {
    pub stripe_payment_id: i32,
    pub stripe_payment_source: String,
    pub sale_id: i32,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "call_email"]
pub struct NewCallEmail {
    pub call_email_request_title: String,
    pub call_email_request_body: String,
    pub call_email_request_date: NaiveDateTime,
    pub call_email_request_to_email: String,
    pub user_id: i32,
    pub clerk_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct CallEmail {
    pub call_email_id: i32,
    pub call_email_request_title: String,
    pub call_email_request_body: String,
    pub call_email_request_date: NaiveDateTime,
    pub call_email_request_to_email: String,
    pub call_email_response_title: Option<String>,
    pub call_email_response_body: Option<String>,
    pub call_email_response_date: Option<NaiveDateTime>,
    pub user_id: i32,
    pub clerk_id: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Testimonials {
    pub testimonials_id: i32,
    pub testimonials_clerk_id: i32,
    pub testimonials_client_id: i32,
    pub testimonials_content: String,
    pub testimonials_value: i32,
    pub testimonials_date: NaiveDate,
    pub testimonials_status: bool,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "testimonials"]
pub struct NewTestimonials {
    pub testimonials_clerk_id: i32,
    pub testimonials_client_id: i32,
    pub testimonials_content: String,
    pub testimonials_value: i32,
    pub testimonials_date: NaiveDate,
    pub testimonials_status: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "text_chat_transaction"]
pub struct NewTextChatTransaction {
    pub text_chat_transaction_value: f64,
    pub text_chat_transaction_paid_balance: Option<f64>, //Some(F64)
    pub text_chat_transaction_paid_bonus: Option<f64>,   // None
    pub text_chat_transaction_value_pay_off: f64,
    pub text_chat_transaction_chat_id: i32,
    pub text_chat_transaction_client_signature: Option<String>,
    pub text_chat_transaction_clerk_signature: Option<String>,
    pub text_chat_transaction_client_id: i32,
    pub text_chat_transaction_clerk_id: i32,
    pub text_chat_transaction_creation: NaiveDateTime,
    pub text_chat_transaction_update_client_signature: Option<NaiveDateTime>,
    pub text_chat_transaction_update_clerk_signature: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[table_name = "voice_chat_transaction"]
pub struct NewVoiceChatTransaction {
    pub voice_chat_transaction_value: f64,
    pub voice_chat_transaction_value_pay_off: f64,
    pub voice_chat_transaction_paid_balance: Option<f64>,
    pub voice_chat_transaction_paid_bonus: Option<f64>,
    pub voice_chat_transaction_chat_id: i32,
    pub voice_chat_transaction_client_signature: Option<String>,
    pub voice_chat_transaction_clerk_signature: Option<String>,
    pub voice_chat_transaction_client_id: i32,
    pub voice_chat_transaction_clerk_id: i32,
    pub voice_chat_transaction_creation: NaiveDateTime,
    pub voice_chat_transaction_update_client_signature: Option<NaiveDateTime>,
    pub voice_chat_transaction_update_clerk_signature: Option<NaiveDateTime>,
}

#[derive(Insertable, Queryable, Debug, Serialize)]
#[table_name = "intends"]
pub struct NewIntend {
    pub intend_client_id: i32,
    pub intend_clerk_id: i32,
    pub intend_type: i32,
    pub intend_status: i32,
    pub intend_ask_time: NaiveDateTime,
    pub intend_received_time: Option<NaiveDateTime>,
    pub intend_answer_time: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct Intend {
    pub intend_id: i32,
    pub intend_client_id: i32,
    pub intend_clerk_id: i32,
    pub intend_type: i32,
    pub intend_status: i32,
    pub intend_ask_time: NaiveDateTime,
    pub intend_received_time: Option<NaiveDateTime>,
    pub intend_answer_time: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, AsChangeset)]
#[table_name = "banners"]
pub struct NewBanner {
    pub banner_creation_date: NaiveDateTime,
    pub banner_desktop: String,
    pub banner_mobile: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Banner {
    pub banner_id: i32,
    pub banner_creation_date: NaiveDateTime,
    pub banner_desktop: String,
    pub banner_mobile: String,
}

#[derive(Debug, Queryable, Serialize, Insertable)]
#[table_name = "syslog"]
pub struct NewSysLog {
    pub syslog_creation: NaiveDateTime,
    pub syslog_content: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct SysLog {
    pub syslog_id: i32,
    pub syslog_creation: NaiveDateTime,
    pub syslog_content: String,
}

#[derive(Debug, Queryable, Serialize, Insertable)]
#[table_name = "clerk_time"]
pub struct NewClerkTime {
    pub clerk_time_clerk_id: i32,
	pub clerk_time_date: NaiveDateTime,
	pub clerk_time_event_type: i32
}

#[derive(Debug, Queryable, QueryableByName, Serialize)]
#[table_name = "clerk_time"]
pub struct ClerkTime {
    pub clerk_time_id: i32,
    pub clerk_time_clerk_id: i32,
    pub clerk_time_date: NaiveDateTime,
    pub clerk_time_event_type: i32,
}



#[derive(Debug, Queryable, QueryableByName, Serialize)]
#[table_name = "attendance_tag"]
pub struct AttendanceTag {
    pub attendance_tag_id: i32,
    pub attendance_tag_name: String,
    pub attendance_tag_slug: String,
}

#[derive(Debug, Insertable)]
#[table_name = "attendance_tag"]
pub struct NewAttendanceTag {
    pub attendance_tag_name: String,
    pub attendance_tag_slug: String,
}

#[derive(Debug, Queryable, QueryableByName, Serialize)]
#[table_name = "clerk_tag"]
pub struct ClerkTag {
    pub clerk_tag_id: i32,
    pub clerk_tag_user_id: i32,
    pub clerk_tag_attendance_tag_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "clerk_tag"]
pub struct NewClerkTag {
    pub clerk_tag_user_id: i32,
    pub clerk_tag_attendance_tag_id: i32,
}


#[derive(Debug, Queryable, QueryableByName, Serialize)]
#[table_name = "clerk_schedule"]
pub struct ClerkSchedule {
    pub clerk_schedule_id: i32,
    pub clerk_schedule_user_id: i32, /* sysuser (user_id) */
    /* Monday */
    pub clerk_schedule_mon: bool,
    pub clerk_schedule_mon_init: String,
    pub clerk_schedule_mon_end: String,
    /* Tuesday */
    pub clerk_schedule_tue: bool,
    pub clerk_schedule_tue_init: String,
    pub clerk_schedule_tue_end: String,
    /* Wednesday */
    pub clerk_schedule_wed: bool,
    pub clerk_schedule_wed_init: String,
    pub clerk_schedule_wed_end: String,
    /* Thursday */
    pub clerk_schedule_thu: bool,
    pub clerk_schedule_thu_init: String,
    pub clerk_schedule_thu_end: String,
    /* Friday */
    pub clerk_schedule_fri: bool,
    pub clerk_schedule_fri_init: String,
    pub clerk_schedule_fri_end: String,
    /* Saturday */
    pub clerk_schedule_sat: bool,
    pub clerk_schedule_sat_init: String,
    pub clerk_schedule_sat_end: String,
    /* Sunday */
    pub clerk_schedule_sun: bool,
    pub clerk_schedule_sun_init: String,
    pub clerk_schedule_sun_end: String,
}

#[derive(Debug, Queryable, Serialize, Insertable)]
#[table_name = "clerk_schedule"]
pub struct NewClerkSchedule {
    pub clerk_schedule_user_id: i32, /* sysuser (user_id) */
    /* Monday */
    pub clerk_schedule_mon: bool,
    pub clerk_schedule_mon_init: String,
    pub clerk_schedule_mon_end: String,
    /* Tuesday */
    pub clerk_schedule_tue: bool,
    pub clerk_schedule_tue_init: String,
    pub clerk_schedule_tue_end: String,
    /* Wednesday */
    pub clerk_schedule_wed: bool,
    pub clerk_schedule_wed_init: String,
    pub clerk_schedule_wed_end: String,
    /* Thursday */
    pub clerk_schedule_thu: bool,
    pub clerk_schedule_thu_init: String,
    pub clerk_schedule_thu_end: String,
    /* Friday */
    pub clerk_schedule_fri: bool,
    pub clerk_schedule_fri_init: String,
    pub clerk_schedule_fri_end: String,
    /* Saturday */
    pub clerk_schedule_sat: bool,
    pub clerk_schedule_sat_init: String,
    pub clerk_schedule_sat_end: String,
    /* Sunday */
    pub clerk_schedule_sun: bool,
    pub clerk_schedule_sun_init: String,
    pub clerk_schedule_sun_end: String,
}

#[derive(Serialize)]
pub struct NewSalesWeeksBeforeData {
    pub sum: Vec<f64>,
    pub count: Vec<i64>,
    pub week_alias: Vec<u32>,
}

#[derive(QueryableByName, Serialize)]
pub struct NewSalesWeeksBeforeDataResult {
    #[sql_type = "Nullable<Double>"]
    pub sum: Option<f64>,
    #[sql_type = "Nullable<BigInt>"]
    pub count: Option<i64>,
}

#[derive(Serialize)]
pub struct NewSalesDaysBeforeData {
    pub sum: Vec<f64>,
    pub count: Vec<i64>,
    pub week_day: Vec<u32>,
}

#[derive(QueryableByName, Serialize)]
pub struct NewSalesDaysBeforeDataResult {
    #[sql_type = "Nullable<Double>"]
    pub sum: Option<f64>,
    #[sql_type = "Nullable<BigInt>"]
    pub count: Option<i64>,
}

#[derive(Serialize)]
pub struct NewSalesMonthsBeforeData {
    pub sum: Vec<f64>,
    pub count: Vec<i64>,
    pub month: Vec<i32>,
}

#[derive(QueryableByName, Serialize)]
pub struct NewSalesMonthsBeforeDataResult {
    #[sql_type = "Nullable<Double>"]
    pub sum: Option<f64>,
    #[sql_type = "Nullable<BigInt>"]
    pub count: Option<i64>,
    #[sql_type = "Nullable<Int4>"]
    pub month_index: Option<i32>,
}