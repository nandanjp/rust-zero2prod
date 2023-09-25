use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener; //bind a port with TcpListener and hand this over to the HttpServer using listen

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn greet(req: HttpRequest) -> impl Responder {
    //return something implementing Responder trait
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

//mark run as public.
//No longer a binary entrypoint, therefore we can mark it as async (main cannot be set as async since its the main entrypoint of a binary)
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
            .route("/subscriptions", web::post().to(subscribe))
    })
    // .bind(address)?
    .listen(listener)?
    .run();
    Ok(server)
}
