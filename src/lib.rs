use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {} !", &name)
// }

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
        // .route("/", web::get().to(greet))
        // .route("/{name}", web::get().to(greet))
    })
    .listen(listener)?
    // .bind(address)?
    .run();
    Ok(server)
}
