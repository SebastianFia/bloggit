use actix_web::Responder;

pub async fn home() -> impl Responder {
    "hello from home"
}
