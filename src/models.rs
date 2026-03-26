use serde::Serialize;

#[derive(Serialize)]
pub struct ListNamespacesResponse {
    pub next_page_token: String,
    pub namespaces: Vec<Vec<String>>,
}
