#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


fn main() {
    rocket::ignite().mount("/", routes![routes::users::hello, routes::posts::get_post]).launch();
}