use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    //HttpResponse::Ok().finish()
    HttpResponse::Ok().body("Server: I am still alive!")
}
