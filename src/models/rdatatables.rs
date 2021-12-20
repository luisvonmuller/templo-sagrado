/*
8888888b.        8888888b.        d8888 88888888888     d8888 888888b.   888      8888888888 .d8888b.
888   Y88b       888  "Y88b      d88888     888        d88888 888  "88b  888      888       d88P  Y88b
888    888       888    888     d88P888     888       d88P888 888  .88P  888      888       Y88b.
888   d88P       888    888    d88P 888     888      d88P 888 8888888K.  888      8888888    "Y888b.
8888888P"        888    888   d88P  888     888     d88P  888 888  "Y88b 888      888           "Y88b.
888 T88b  888888 888    888  d88P   888     888    d88P   888 888    888 888      888             "888
888  T88b        888  .d88P d8888888888     888   d8888888888 888   d88P 888      888       Y88b  d88P
888   T88b       8888888P" d88P     888     888  d88P     888 8888888P"  88888888 8888888888 "Y8888P"
*/

use super::*;

#[allow(non_snake_case)]
#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "sysuser"]
pub struct DataTablesSysUserListing {
    pub user_name: String,
    pub user_balance: f64,
    pub user_bonus: f64,
    pub user_creation: NaiveDateTime,
    pub user_lasttimeonline: Option<NaiveDateTime>,
    pub user_status: Option<bool>,
    pub user_id: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "product"]
pub struct DataTablesProductListing {
    pub product_id: i32,
    pub product_image: String,
    pub product_title: String,
    pub product_is_active: bool,
    pub product_value: f64,
}

#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "chat"]
/* Datatables chats on admin */
pub struct DataTablesChats {
    pub chat_id: i32,
    pub init_time: NaiveDateTime,
    pub client_id: i32,
    pub clerk_id: i32,
}

#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "clerk_info"]
pub struct DataTablesChatsClerk {
    pub clerk_info_exhibition: Option<String>,
}

#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "sysuser"]
pub struct DatatTablesChatUser {
    pub user_name: String,
}

#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "intends"]
pub struct DataTablesIntends {
    pub intend_type: i32,
    pub intend_status: i32,
    pub intend_ask_time: NaiveDateTime,
    pub intend_received_time: Option<NaiveDateTime>,
    pub intend_answer_time: Option<NaiveDateTime>,
}

#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "clerk_info"]
pub struct DataTablesIntendsClerk {
    pub clerk_info_exhibition: Option<String>,
}

#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "sysuser"]
pub struct DataTablesIntendsClient {
    pub user_name: String,
}

#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "call"]
/* Datatables chats on admin */
pub struct DataTablesVoice {
    pub call_id: i32,
    pub call_begin_date: NaiveDateTime,
    pub user_id: i32,
    pub clerk_id: i32,
    pub call_file: Option<String>,
}

#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "clerk_info"]
pub struct DataTablesVoiceClerk {
    pub clerk_info_exhibition: Option<String>,
}

#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "sysuser"]
pub struct DataTablesVoiceUser {
    pub user_name: String,
}

/* ClerksViewListing (Implements joins) */
#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "sysuser"]
pub struct ClerksViewListing {
    pub user_name: String,
    pub user_uni: Option<String>,
    pub user_balance: f64,
    pub user_status: Option<bool>,
    pub user_id: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "status_clerk"]
pub struct ClerksViewListingStatusClerk {
    pub status: i32,
}

#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "clerk_info"]
pub struct ClerksViewListingClerkInfo {
    pub clerk_image: Option<String>,
}

#[derive(Debug, QueryableByName, Serialize, Clone)]
#[table_name = "banners"]
pub struct DataTablesBannerListing {
    pub banner_creation_date: NaiveDateTime,
    pub banner_mobile: String,
    pub banner_desktop: String,
    pub banner_id: i32,
}
