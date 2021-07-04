mod controller;
mod schema;

use crate::controller::{Controller, InMemController};
use crate::schema::{Mutation, MySchema, MySubscription, Query};
use actix_web::guard::{Get, Header};
use actix_web::web::{resource, Data, Payload};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::http::playground_source;
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql_actix_web::{Request, Response, WSSubscription};
use tokio::sync::Mutex;

#[actix_web::post("/")]
async fn index(schema: Data<MySchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

async fn index_ws(
    schema: Data<MySchema>,
    req: HttpRequest,
    payload: Payload,
) -> Result<HttpResponse> {
    WSSubscription::start(MySchema::clone(&*schema), &req, payload)
}

fn build_schema() -> MySchema {
    let controller: Mutex<Box<dyn Controller>> = Mutex::new(Box::new(InMemController::default()));
    MySchema::build(Query::default(), Mutation, MySubscription)
        .data(controller)
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = build_schema();
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(index)
            .service(
                resource("/")
                    .guard(Get())
                    .guard(Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(resource("/").guard(Get()).to(index_playground))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
