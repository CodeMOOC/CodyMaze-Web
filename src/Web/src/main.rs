#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;

#[get("/")]
fn index(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
        "name": "Handlebars"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

fn landing_mirabilandia(info: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Path is {}!", info))
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
            .service(index)
            .route("/mirabilandia/position/{pos}", web::get().to(landing_mirabilandia))
            .service(fs::Files::new("/", "./static"))
    })
    .bind("localhost:8088")
    .unwrap()
    .run()
    .unwrap();
}
