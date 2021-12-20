/* Template */
use rocket_contrib::templates::Template;

/* Common db connection function */
use crate::establish_connection;

/* Database Macros. */
use diesel::prelude::*;

/* Json type response  */
use rocket_contrib::json::Json;

/* Importing User struct of our session handler */
use crate::AdminUser;

pub mod create;
pub mod update;
pub mod paypal;

/* Since we can have a logged out try, we will import redirect and flash */
use rocket::response::{Flash, Redirect};

#[get("/")]
pub fn index(_administrative: AdminUser) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", "/voice");
    Template::render("pages/sales/list", &map)
}

#[get("/", rank = 2)]
pub fn index_no_login() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/admin/login"),
        "Ops... parece que sua sessão é inválida. Por favor, refaça o login.",
    )
}

/* Hash map and Datatables */
use rdatatables::*;
use rocket::request::LenientForm;

/* This one stands for our query data structure */
use crate::models::rdatatables::DatatTablesChatUser;
use crate::models::Sale;

#[post("/list", data = "<query>")]
pub fn list(
    _adminitrative: AdminUser,
    query: LenientForm<DataTableQuery>,
) -> Json<OutcomeData<(Sale, DatatTablesChatUser)>> {
    Json(datatables_query::<(Sale, DatatTablesChatUser)>(
        Tables {
            origin: ("sale", "sale_id"), /* From */
            fields: vec![
                "sale.sale_id",
                "sale.sale_date",
                "sale.sale_real_value",
                "sale.sale_status",
                "sale.sale_payment_source",
                "sale.user_id",
                "sysuser.user_name",
                "sale.sale_points_value",
            ], /* Fields to seek for */
            join_targets: Some(vec![("inner", ("sysuser", "user_id"), ("sale", "user_id"))]), /* Join Targets explained over here */
            datatables_post_query: query.into_inner(), /* Incoming Query */
            query: None,                               /* Our builded query holder */
            condition: None,
        },
        crate::establish_connection(),
    ))
}

#[get("/new-stats/<sale_id>/<stat>")]
pub fn new_stats(_administrative: AdminUser, sale_id: i32, stat: i32) {
    use crate::schema::sale;

    diesel::update(sale::table.filter(sale::sale_id.eq(sale_id)))
        .set(sale::sale_status.eq(Some(stat)))
        .execute(&establish_connection())
        .expect("error parsing something");

    match stat {
        1 => {
            let (sale_user_id, sale_value): (i32, Option<f64>) =
                diesel::update(sale::table.filter(sale::sale_id.eq(sale_id)))
                    .set(sale::sale_status.eq(1))
                    .returning((sale::user_id, sale::sale_real_value))
                    .get_result(&crate::establish_connection())
                    .unwrap();

            /* Give Cesar whats belongs to cesar */
            crate::controller::payments::give_cesar_what_belongs_to_cesar(
                sale_user_id,
                sale_value.unwrap(),
            );
        }
        _ => {}
    }
}
