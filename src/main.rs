use actix_web::{HttpRequest, Responder};
use std::net::TcpListener;
//use tokio::net::TcpListener;
use zero2prod::run;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let address = format!("127.0.0.1:{}", 8000);
    let listener = TcpListener::bind(address)?;
    let _response = run(listener)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use zero2prod::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health_check().await;
        assert!(response.status().is_success());
    }
}
