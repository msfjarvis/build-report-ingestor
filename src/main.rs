#![allow(clippy::no_effect_underscore_binding)]
mod metric;
mod metric_conv;
mod metric_name;
mod model;

use crate::model::CompileStatisticsData;
use rocket::serde::json::Json;
use rocket::{launch, post, routes};

#[post("/", data = "<data>", format = "json")]
fn submit_report(data: Json<CompileStatisticsData>) -> String {
    let data = data.into_inner();
    data.build_uuid.to_owned()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![submit_report])
}
