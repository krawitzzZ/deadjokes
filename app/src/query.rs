#[derive(Debug, Clone, PartialEq)]
pub struct QueryResult<T> {
    pub current_page: u64,
    pub total_pages: u64,
    pub total_count: u64,
    pub items: Vec<T>,
}

impl<T> QueryResult<T> {
    pub fn new(current_page: u64, total_pages: u64, total_count: u64, items: Vec<T>) -> Self {
        QueryResult {
            current_page,
            total_pages,
            total_count,
            items,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct JokeQuery {
    pub page: u64,
    pub body: Option<String>,
}
