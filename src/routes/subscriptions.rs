use actix_web::{body::BoxBody, web, HttpResponse};
//use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

/// Extract form data using serde.
/// This handler get called only if content type is *x-www-form-urlencoded*
/// and content of the request could be deserialized to a `FormData` struct
fn index(form: web::Form<FormData>) -> String {
    format!("Welcome {}!", form.0.name)
    //HttpResponse::with_body(
    //    actix_web::http::StatusCode::OK,
    //    BoxBody::new(format!("Welcome {}!", form.0.name)),
    //)
}
pub async fn subscribe(_form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Ok().finish()
    //match sqlx::query!(
    //    r#"
    //INSERT INTO subscriptions (id, email, name, subscribed_at)
    //VALUES ($1, $2, $3, $4)
    //    "#,
    //    Uuid::new_v4(),
    //    form.email,
    //    form.name,
    //    chrono::Utc::now()
    //)
    //.execute(pool.as_ref())
    //.await
    //{
    //    Ok(_) => HttpResponse::Ok().finish(),
    //    Err(e) => {
    //        println!("Failed to execute query: {}", e);
    //        HttpResponse::InternalServerError().finish()
    //    }
    //}
}
