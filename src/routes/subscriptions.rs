#![allow(unused_imports)]
use actix_web::{
    web::{self, UrlEncoded},
    HttpResponse,
};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
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
#[allow(clippy::async_yields_async)]
#[tracing::instrument(
    name = "Adding a new subscriber", 
    skip(form, pool),
    fields(
        subscriber_email = &form.email,
        subscriber_name = &form.name,
    )
    )]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(pool, form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
#[tracing::instrument(
    name = "Saving new subscriber details to the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(
    pool: web::Data<PgPool>,
    form: web::Form<FormData>,
) -> Result<(), HttpResponse> {
    // Let's generate a random unique identifier.
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _request_span_guard = request_span.enter();

    // We don't call `.enter()` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime
    let query_span = tracing::info_span!("Saving new subscriber details to the database.",);

    // tracing::info!(
    //     "request_id {} - Saving new subscriber details to the database.",
    //     request_id
    // );
    match sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use `get_ref` to get an immutable reference to the `PgPool` wrapped by `web::Data`.
    .execute(pool.get_ref())
    // First we attach the instrumentation, then we `.await` it
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            // tracing::info!(
            //     "request_id {} - New subscriber details have been saved.",
            //     request_id
            // );
            HttpResponse::Ok().finish();
        }
        Err(e) => {
            // Using `println!` to capture information about the error
            // in case things don't work out as expected.
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );

            HttpResponse::InternalServerError().finish();
        }
    }

    Ok(())
}
