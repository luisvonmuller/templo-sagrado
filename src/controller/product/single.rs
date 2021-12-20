/*
8888888b. 8888888 .d8888b.  8888888b.  888             d8888 Y88b   d88P
888  "Y88b  888  d88P  Y88b 888   Y88b 888            d88888  Y88b d88P
888    888  888  Y88b.      888    888 888           d88P888   Y88o88P
888    888  888   "Y888b.   888   d88P 888          d88P 888    Y888P
888    888  888      "Y88b. 8888888P"  888         d88P  888     888
888    888  888        "888 888        888        d88P   888     888
888  .d88P  888  Y88b  d88P 888        888       d8888888888     888
8888888P" 8888888 "Y8888P"  888        88888888 d88P     888     888
 */

// Common db connection function (dã)
use crate::establish_connection;

//Macros.
use diesel::prelude::*;

//Json type response (must have)
use rocket_contrib::json::Json;

#[get("/single/<product_id>", format = "json", rank = 1)]
pub fn show_product(product_id: i32) -> Json<String> {
    use crate::schema::product;
    use crate::models::Product;
    
	let results = product::table
        .select(product::all_columns)
		.filter(product::product_id.eq(product_id))
		.load::<Product>(&establish_connection())
		.expect("Some shit happned while retrieving the full list of users.!!! <Panic at the Disco> ops, thread!");

	Json(serde_json::to_string(&results).unwrap())
}

/* Every function that handles a rank  it's a func that holds no-allowed users to acess our data */
#[get("/single", format = "json", rank = 2)]
pub fn list_hack() -> Json<&'static str> {
    Json("You are not allowed to reach this content. Please, leave or dont't. You will not access it anyways.")
}

