#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;
use rpkg_api::models::PkgInfo;

#[get("/?<name>")]
fn index(name: Option<String>) -> Json<Vec<PkgInfo>> {
    // let hello = "Hello, ";
    // let name = "Atsushi";
    // let new = hello.to_string() + name;
    let pkg_name = "00mathieu/FarsExample";
    let pkg_name = name.map(|name| format!("{}/{}", pkg_name, name))
        .unwrap_or_else(|| pkg_name.to_string());
    Json(vec![PkgInfo {
        pkg_name: pkg_name.into(),
        title: "Read Rocket tutorial".into(),
        url: "https://rocket.rs/guide/quickstart/".into(),
    }])
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}