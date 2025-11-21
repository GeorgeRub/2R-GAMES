use lambda_http::{Request, RequestExt};

#[derive(Debug)]
pub struct Pagination {
    pub offset: i64,
    pub size: i64,
    pub(crate) sort: String,
    pub(crate) sort_direction: String,
}

impl Pagination {
    pub fn new(event: Request) -> Self {
        let queries = event.query_string_parameters();
        let direction = match queries.first("sortDirection").unwrap_or(&"asc") {
            "asc" => "ASC",
            "desc" => "DESC",
            _ => "ASC",
        };

        let column = match queries.first("sort").unwrap_or(&"user_email") {
            "user_id" => "user_id",
            "user_email" => "user_email",
            "user_first_name" => "user_first_name",
            "user_last_name" => "user_last_name",
            "user_active" => "user_active",
            _ => "user_id",
        };

        let limit = queries
            .first("size")
            .unwrap_or(&"10")
            .parse::<i64>()
            .unwrap_or(10);
        let page = queries
            .first("page")
            .unwrap_or(&"0")
            .parse::<i64>()
            .unwrap_or(0);
        let offset = page * limit;

        Self {
            offset,
            size: limit,
            sort: column.to_string(),
            sort_direction: direction.to_string(),
        }
    }
}
