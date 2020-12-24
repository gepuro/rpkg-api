#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::response::content;
use rocket_contrib::json::Json;
use rpkg_api::models::PkgInfo;
use rusqlite::{params, Connection};

fn select_rpkg(query: Option<String>) -> Vec<PkgInfo> {
    let conn = Connection::open("data/pkg.db").unwrap();
    let sql = match query {
        Some(ref _query) => "SELECT pkg_name, title, url FROM rpkg WHERE pkg_name LIKE ? or title LIKE ? or url LIKE ? ORDER BY pkg_name LIMIT 100",
        None => "SELECT pkg_name, title, url FROM rpkg ORDER BY pkg_name LIMIT 100",
    };
    let query_with_percent = query
        .map(|query| format!("%{}%", query))
        .unwrap_or_else(|| format!(""));

    let query_with_percent2: &str = &query_with_percent;
    let params_query = params![
        query_with_percent2,
        query_with_percent2,
        query_with_percent2
    ];
    let sql_params = match query_with_percent2 {
        "" => params![],
        _ => params_query,
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
    }
    rpkgs
}

#[get("/rpkg?<q>")]
fn rpkg(q: Option<String>) -> Json<Vec<PkgInfo>> {
    Json(select_rpkg(q))
}

#[get("/system/health")]
fn health_check() -> &'static str {
    "ok"
}

#[get("/")]
fn index() -> rocket::response::content::Html<&'static str> {
    let html = "<html>
    <body>
    <a href='http://rpkg-api.gepuro.net/rpkg?q=gepuro'>Search R Packages</a> <br>
    How to use this API: <a href='https://github.com/gepuro/rpkg-api/blob/master/README.md'>README</a>
    </body>
    </html>";
    let response = content::Html(html);
    response
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, rpkg, health_check])
        .launch();
}
