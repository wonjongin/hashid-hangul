#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod hangul;
mod tools;

use crate::hangul::ksx1001::int2han;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/dec2han/<integer>")]
fn dec2han(integer: usize) -> String {
  int2han(integer)
}

fn main() {
  rocket::ignite()
    .mount("/", routes![index, dec2han])
    .launch();
}
