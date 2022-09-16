#[macro_use] extern crate rocket;
mod metric;
mod metric_conv;
mod metric_name;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
