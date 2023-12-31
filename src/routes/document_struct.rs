use serde::Deserialize;
use serde_json::Value;


/// Used for Post: Search
#[derive(Deserialize)]
pub struct DocumentSearch {
    pub index: String,
    pub search_term: Option<String>,
    pub search_in: Option<String>,
    pub return_fields: Option<String>,
    pub from: Option<i64>,
    pub count: Option<i64>
}

/// Used for Get: Search
#[derive(Deserialize)]
pub struct GetDocumentSearchIndex {
    pub index: String
}

/// Used for Get: Search
#[derive(Deserialize)]
pub struct GetDocumentSearchQuery {
    pub search_term: Option<String>,
    pub search_in: Option<String>,
    pub return_fields: Option<String>,
    pub from: Option<i64>,
    pub count: Option<i64>
}

/// Used for Get: Document
#[derive(Deserialize)]
pub struct DocById{
    pub index: String,
    pub document_id: String
}

/// Used for Get: Document
#[derive(Deserialize)]
pub struct ReturnFields{
    pub return_fields: Option<String>
}

/// Used for Post: Document
#[derive(Deserialize)]
pub struct DocumentCreate{
    pub index: String,
    pub data: Value,
    pub dynamic_mode: Option<String>
}

/// Used for Post: Document_multiple
#[derive(Deserialize)]
pub struct MultipleDocumentCreate{
    pub index: String,
    pub data: Vec<Value>,
    pub dynamic_mode: Option<String>
}

/// Used for Put: Document
#[derive(Deserialize)]
pub struct DocumentUpdate {
    pub index: String,
    pub document_id: String,
    pub data: Value
}

/// Used for Delete: Document
#[derive(Deserialize)]
pub struct DocumentDelete {
    pub index: String,
    pub document_id: String
}