use crate::generator::toml::write_cargo_toml;
use crate::generator::write_graphql;
use proc_macro2::TokenStream;
use quote::quote;
use seaography_types::schema_meta::SchemaMeta;
use std::path::Path;

pub mod generator;

pub fn write_project<P: AsRef<Path>>(
    path: &P,
    schema: &SchemaMeta,
    crate_name: &String,
) -> std::io::Result<()> {
    std::fs::create_dir_all(path.as_ref().join("src/graphql"))?;

    write_cargo_toml(path, crate_name, &schema.version)?;

    write_graphql(
        &path.as_ref().join("src/graphql"),
        &schema.tables,
        &schema.enums,
    )?;

    let lib_tokens = quote! {
        pub mod orm;
        pub mod graphql;

        pub use graphql::QueryRoot;
        pub use graphql::OrmDataloader;
    };
    std::fs::write(&path.as_ref().join("src/lib.rs"), lib_tokens.to_string())?;

    let crate_name_token: TokenStream = crate_name.parse().unwrap();
    let db_url = &schema.url;

    let main_tokens = quote! {
        use async_graphql::{
            http::{playground_source, GraphQLPlaygroundConfig},
            EmptyMutation, EmptySubscription, Schema, dataloader::DataLoader
        };
        use async_graphql_poem::GraphQL;
        use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
        use sea_orm::Database;

        use #crate_name_token::*;

        #[handler]
        async fn graphql_playground() -> impl IntoResponse {
            Html(playground_source(GraphQLPlaygroundConfig::new("/")))
        }

        #[tokio::main]
        async fn main() {
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .with_test_writer()
                .init();

            // TODO: use .env file to configure url
            let database = Database::connect(#db_url).await.unwrap();

            // TODO use environment variables to configure dataloader batch size
            let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
                OrmDataloader {
                    db: database.clone()
                },
                tokio::spawn
            ) ;

            let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
                .data(database)
                .data(orm_dataloader)
                .finish();

            let app = Route::new().at("/", get(graphql_playground).post(GraphQL::new(schema)));

            println!("Playground: http://localhost:8000");

            Server::new(TcpListener::bind("0.0.0.0:8000"))
                .run(app)
                .await
                .unwrap();
        }

    };

    std::fs::write(&path.as_ref().join("src/main.rs"), main_tokens.to_string())?;

    Ok(())
}
