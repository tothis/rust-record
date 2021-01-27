#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket_contrib::serve::StaticFiles;
mod handler;

fn main() {
    rocket::ignite()
        .mount("/user", routes![
            handler::user::index,
            handler::user::get_user_by_id
        ])
        .mount("/", StaticFiles::from("static"))
        .launch();
}