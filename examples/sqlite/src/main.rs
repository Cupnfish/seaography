use async_graphql::{
    dataloader::DataLoader,
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_poem::GraphQL;
use dotenv::dotenv;
use lazy_static::lazy_static;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
use sea_orm::Database;
use seaography_sqlite_example::*;
use std::env;

lazy_static! {
    static ref URL: String = env::var("URL").unwrap_or("0.0.0.0:8000".into());
    static ref ENDPOINT: String = env::var("ENDPOINT").unwrap_or("/".into());
    static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    static ref DEPTH_LIMIT: Option<usize> = env::var("DEPTH_LIMIT").map_or(None, |data| Some(
        data.parse().expect("DEPTH_LIMIT is not a number")
    ));
    static ref COMPLEXITY_LIMIT: Option<usize> = env::var("COMPLEXITY_LIMIT")
        .map_or(None, |data| {
            Some(data.parse().expect("COMPLEXITY_LIMIT is not a number"))
        });
}

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(&*ENDPOINT)))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();
    let database = Database::connect(&*DATABASE_URL)
        .await
        .expect("Fail to initialize database connection");
    let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
        OrmDataloader {
            db: database.clone(),
        },
        tokio::spawn,
    );
    let mut schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(database)
        .data(orm_dataloader);
    if let Some(depth) = *DEPTH_LIMIT {
        schema = schema.limit_depth(depth);
    }
    if let Some(complexity) = *COMPLEXITY_LIMIT {
        schema = schema.limit_complexity(complexity);
    }
    let schema = schema.finish();
    let app = Route::new().at(
        &*ENDPOINT,
        get(graphql_playground).post(GraphQL::new(schema)),
    );
    println!("Visit GraphQL Playground at http://{}", *URL);
    Server::new(TcpListener::bind(&*URL))
        .run(app)
        .await
        .expect("Fail to start web server");
}
