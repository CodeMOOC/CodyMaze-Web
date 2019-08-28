#[macro_use]
extern crate serde_json;

use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;

fn landing_mirabilandia(hb: web::Data<Handlebars>, info: web::Path<(String)>) -> HttpResponse {
    let data = json!({
        "code": info.to_string()
    });
    let body = hb.render("position", &data).unwrap();

    HttpResponse::Ok().body(body)
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
    .bind("localhost:8088")
    .unwrap()
    .run()
    .unwrap();
}
