use super::*;


/* Hash map and Datatables */
use rdatatables::*;

use rocket::request::LenientForm;

/* This one stands for our query data structure */
use crate::models::rdatatables::{DataTablesIntends, DataTablesIntendsClerk, DataTablesIntendsClient};

#[post("/list", data = "<query>")]
pub fn list(
    _adminitrative: AdminUser,
    query: LenientForm<DataTableQuery>,
) -> Json<OutcomeData<(DataTablesIntends, DataTablesIntendsClerk, DataTablesIntendsClient)>> {
    Json(datatables_query::<(
        DataTablesIntends,
        DataTablesIntendsClerk,
        DataTablesIntendsClient,
    )>(
        Tables {
            origin: ("intends", "intend_id"), /* From */
            fields: vec![
                "user_name",
                "clerk_info_exhibition",
                "intend_type",
                "intend_status",
                "intend_ask_time",
                "intend_received_time",
                "intend_answer_time",
            ], /* Fields to seek for */
            join_targets: Some(vec![
                ("inner", ("clerk_info", "user_id"), ("intends", "intend_clerk_id")),
                ("inner", ("sysuser", "user_id"), ("intends", "intend_client_id")),
            ]), /* Join Targets explained over here */
            datatables_post_query: query.into_inner(), /* Incoming Query */
            query: None,                 /* Our builded query holder */
            condition: None,
        },
        crate::establish_connection(),
    ))
}


#[get("/list/<amount>")]
pub fn list_json(_administrative: AdminUser, amount: i32)  -> Json<Vec<Intend>>{
    use crate::schema::intends;

    let results: Vec<Intend> = intends::table
    .select(intends::all_columns)
    .order_by(intends::intend_ask_time.desc())
    .limit(amount as i64)
    .load::<Intend>(&crate::establish_connection()).expect("No intends found?");

    Json(results)

} 