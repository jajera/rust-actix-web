use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::env;
use std::sync::Mutex;
use std::time::{Duration, SystemTime};
use uuid::Uuid;
use lazy_static::lazy_static;

const DEFAULT_PORT: &str = "80";

lazy_static! {
    static ref START_TIME: Mutex<SystemTime> = Mutex::new(SystemTime::now());
}

#[get("/")]
async fn root() -> impl Responder {
    let hostname: String = env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());

    format!(
        "Hello, world from {} port {}!",
        hostname,
        port
    )
}

#[get("/new_uuid")]
async fn new_uuid() -> impl Responder {
    HttpResponse::Ok().body(Uuid::new_v4().to_string())
}

#[get("/myip")]
async fn my_ip(req: HttpRequest) -> impl Responder {
    let client_ip = get_client_ip(&req);
    HttpResponse::Ok().body(client_ip)
}

#[get("/soh")]
async fn soh() -> impl Responder {
    let uptime = get_uptime();
    HttpResponse::Ok().body(format!("Uptime: {}", uptime))
}

async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("404 Not Found")
}

fn get_client_ip(req: &HttpRequest) -> String {
    req.connection_info()
        .peer_addr()
        .map_or("Unknown".to_string(), |addr| addr.to_string())
}

fn get_uptime() -> String {
    let start_time = START_TIME.lock().unwrap();
    let duration = start_time.elapsed().unwrap();
    format_duration(duration)
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting web server...");
    
    let _start_time = *START_TIME.lock().unwrap();

    let port: String = env::var("PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());

    HttpServer::new(move || {
        App::new()
            .service(root)
            .service(new_uuid)
            .service(my_ip)
            .service(soh)
            .default_service(web::get().to(not_found))
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}
