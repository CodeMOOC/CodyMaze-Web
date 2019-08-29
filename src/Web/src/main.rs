#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use regex::Regex;

fn landing_mirabilandia(hb: web::Data<Handlebars>, info: web::Path<(String)>) -> HttpResponse {
    lazy_static! {
        static ref MATCHER: Regex = Regex::new(r"(?i)[abcde][1-5][nesw]?").unwrap();
    }

    if !MATCHER.is_match(&info) {
        HttpResponse::NotFound().body("Invalid position!")
    }
    else {
        let data = json!({
            "code": info.to_string().to_uppercase()
        });
        let body = hb.render("position", &data).unwrap();

        HttpResponse::Ok().body(body)
    }
}

fn main() {
    println!("Starting up Web server...");

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .register_data(handlebars_ref.clone())
            .service(
                web::resource("/mirabilandia/position/{code}").route(web::get().to(landing_mirabilandia))
            )
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:8088")
    .unwrap()
    .run()
    .unwrap();
}
