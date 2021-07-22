mod controller;
mod env_var;
mod schema;

use crate::controller::{Controller, InMemController};
use crate::schema::{Mutation, MySchema, MySubscription, QueryRoot};
use actix_cors::Cors;
use actix_web::guard::{Get, Header};
use actix_web::middleware::Compress;
use actix_web::web::{resource, Data, Payload};
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::http::playground_source;
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql_actix_web::{Request, Response, WSSubscription};
use env_var::get_env;
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

#[actix_web::get("/voyager")]
async fn index_voyager() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("voyager/index.html")))
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
    MySchema::build(QueryRoot::default(), Mutation, MySubscription)
        .data(controller)
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = build_schema();
    let env = get_env();
    let _ = get_env();
    let port = if let Some(port) = env.port {
        port
    } else {
        "3000".to_string()
    };
    let bind_ip = format!("0.0.0.0:{}", port);

    println!("Server start on {}", bind_ip);

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin(),
            )
            .wrap(Compress::default())
            .service(index)
            .service(
                resource("/")
                    .guard(Get())
                    .guard(Header("upgrade", "websocket"))
                    .to(index_ws),
            )
            .service(resource("/").guard(Get()).to(index_playground))
            .service(index_voyager)
    })
    .bind(bind_ip)?
    .run()
    .await
}
