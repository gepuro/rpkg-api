#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use rocket_contrib::json::Json;
use rpkg_api::models::PkgInfo;
// use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection};

// fn type_of<T>(_: T) -> String {
//     let a = std::any::type_name::<T>();
//     return a.to_string();
// }

// fn build_params(query: Option<&str>) -> params {
//     let res = match query {
//         Some(_query) => params![query],
//         None => params![],
//     };
//     res
// }

fn select_rpkg(query: Option<String>) -> Vec<PkgInfo> {
    let conn = Connection::open("data/pkg.db").unwrap();
    let sql = match query {
        Some(ref _query) => "SELECT pkg_name, title, url FROM rpkg WHERE pkg_name LIKE ?",
        None => "SELECT pkg_name, title, url FROM rpkg",
    };
    println!("{}", sql);
    // let query_with_percent = "%".to_string() + query + "%";
    let params_query = params![query];
    // println!("{}", type_of(q));
    let sql_params = match query {
        Some(ref _query) => params_query,
        None => params![],
    };
    let mut stmt = conn.prepare(sql).unwrap();
    let rpkg_iter = stmt
        .query_map(sql_params, |row| {
            Ok(PkgInfo {
                pkg_name: row.get(0)?,
                title: row.get(1)?,
                url: row.get(2)?,
            })
        })
        .unwrap();
    let mut rpkgs = Vec::new();
    for rpkg in rpkg_iter {
        rpkgs.push(rpkg.unwrap());
        // println!("Found rpkg {:?}", rpkg.unwrap().pkg_name);
    }
    rpkgs
}

#[get("/?<q>")]
fn index(q: Option<String>) -> Json<Vec<PkgInfo>> {
    // let hello = "Hello, ";
    // let name = "Atsushi";
    // let new = hello.to_string() + name;
    // let pkg_name = "00mathieu/FarsExample";
    // let pkg_name = name
    //     .map(|name| format!("{}/{}", pkg_name, name))
    //     .unwrap_or_else(|| pkg_name.to_string());
    println!("{:?}", q);
    Json(select_rpkg(q))
    // Json(vec![PkgInfo {
    //     pkg_name: pkg_name.into(),
    //     title: "Read Rocket tutorial".into(),
    //     url: "https://rocket.rs/guide/quickstart/".into(),
    // }])
}

fn main() {
    // let query = Some("%00mathieu%");
    // let query = None;
    // select_rpkg(query);
    rocket::ignite().mount("/", routes![index]).launch();
}
