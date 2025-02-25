// API endpoints for the HTTP server
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::application::user_service::UserService;
use crate::domain::user::UserInput;
use crate::infrastructure::db::DbPool;

async fn register(
    pool: web::Data<DbPool>,
    input: web::Json<UserInput>,
) -> impl Responder {
    match UserService::register(&pool, input.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn login(
    pool: web::Data<DbPool>,
    input: web::Json<UserInput>,
) -> impl Responder {
    match UserService::login(&pool, input.into_inner()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().body(e),
    }
}

pub async fn run_server(pool: DbPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}