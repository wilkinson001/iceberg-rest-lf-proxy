use aws_config::BehaviorVersion;
use aws_sdk_glue::operation::{get_database::GetDatabaseOutput, get_databases};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, head, post},
};
use serde_json::{Value, json};

use crate::models::ListNamespacesResponse;

mod models;

#[tokio::main]
async fn main() {
    let sdk_config = aws_config::load_defaults(BehaviorVersion::v2026_01_12()).await;
    let clients: AWSClients = AWSClients {
        glue: aws_sdk_glue::Client::new(&sdk_config),
        lf: aws_sdk_lakeformation::Client::new(&sdk_config),
    };
    let app = Router::new()
        .route("/v1/config", get(config))
        .route("/v1/catalogs/{catalog}", get(get_catalog))
        .route("/v1/catalogs/{catalog}/namespaces", get(get_namespaces))
        .route(
            "/v1/catalogs/{catalog}/namespaces/{ns}",
            get(load_namespace_metadata),
        )
        .route(
            "/v1/catalogs/{catalog}/namespaces/{ns}/properties",
            post(update_namespace_properties),
        )
        .route(
            "/v1/catalogs/{catalog}/namespaces/{ns}/tables",
            get(list_tables),
        )
        .route(
            "/v1/catalogs/{catalog}/namespaces/{ns}/tables/{table}",
            get(load_tables),
        )
        .route(
            "/v1/catalogs/{catalog}/namespaces/{ns}/tables/{table}",
            head(table_exists),
        )
        .with_state(clients);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn config() {}
async fn get_catalog(Path(catalog): Path<String>) {}

async fn get_namespaces(
    Path(catalog): Path<String>,
    State(state): State<AWSClients>,
) -> Json<Value> {
    let databases_result = state
        .glue
        .get_databases()
        .catalog_id(catalog)
        .into_paginator()
        .send()
        .next()
        .await
        .unwrap();
    let databases = match databases_result {
        Ok(value) => value.database_list,
        Err(e) => panic!("{}", e),
    };

    let response: models::ListNamespacesResponse = ListNamespacesResponse {
        next_page_token: String::from(""),
        namespaces: Vec::new(),
    };
    Json(serde_json::to_string(&response))
}

async fn load_namespace_metadata(Path((catalog, ns)): Path<(String, String)>) {}
async fn update_namespace_properties(Path((catalog, ns)): Path<(String, String)>) {}
async fn list_tables(Path((catalog, ns)): Path<(String, String)>) {}
async fn load_tables(Path((catalog, ns, table)): Path<(String, String, String)>) {}
async fn table_exists(Path((catalog, ns, table)): Path<(String, String, String)>) {}

#[derive(Clone)]
struct AWSClients {
    glue: aws_sdk_glue::Client,
    lf: aws_sdk_lakeformation::Client,
}
