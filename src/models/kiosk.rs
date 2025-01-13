use reqwest::{Method, Response};
use crate::error_handling::Errors;
use crate::general::Client;

pub struct Kiosk;
pub enum ProcessingStatuses {
    CANCELLED,
    DONE,
    FATAL,
    InProgress,
    InQueue,
}
impl ProcessingStatuses {
    pub fn to_string(&self) -> String {
        match self {
            ProcessingStatuses::CANCELLED => "CANCELLED".to_string(),
            ProcessingStatuses::DONE => "DONE".to_string(),
            ProcessingStatuses::FATAL => "FATAL".to_string(),
            ProcessingStatuses::InProgress => "InProgress".to_string(),
            ProcessingStatuses::InQueue => "InQueue".to_string(),
        }
    }
}

impl Kiosk {
    ///Returns details for the Data Kiosk queries that match the specified filters. See the createQuery operation for details about query retention.
    ///
    /// Rate (requests per second): 0.0222
    ///
    /// Burst: 10
    ///
    /// # Parameters
    ///
    /// - `processing_statuses` (optional):
    ///   A list of processing statuses used to filter queries.
    ///   Minimum count: 1.
    ///   Type: `Vec<ProcessingStatuses>`.
    ///
    /// - `page_size` (optional):
    ///   The maximum number of queries to return in a single call.
    ///   Minimum: 1, Maximum: 100.
    ///   Default: 10.
    ///   Type: `u32`.
    ///
    /// - `created_since` (optional):
    ///   The earliest query creation date and time to include in the response, in ISO 8601 date-time format.
    ///   Default: 90 days ago.
    ///   Type: `String`.
    ///
    /// - `created_until` (optional):
    ///   The latest query creation date and time to include in the response, in ISO 8601 date-time format.
    ///   Default: the time of the `get_queries` request.
    ///   Type: `String`.
    ///
    /// - `pagination_token` (optional):
    ///   A token to fetch a specific page of results when multiple pages are available.
    ///   This token is fetched from the `pagination.nextToken` field in the `GetQueriesResponse` object.
    ///   If absent, the first page of results is returned.
    ///   Type: `String`.
    ///
    /// # Responses
    ///
    /// - **200 (Success):**
    ///   Returns the fetched queries in a `GetQueriesResponse` object.
    ///   - Headers:
    ///     - `x-amzn-RateLimit-Limit` (`String`): Your rate limit (requests per second) for this operation.
    ///     - `x-amzn-RequestId` (`String`): Unique request reference identifier.
    async fn get_queries(client: &mut Client, processing_status: Option<Vec<ProcessingStatuses>>, page_size: Option<u64>, created_since: Option<String>, created_until: Option<String>, pagination_token: Option<String> ) -> Result<Response, Errors> {
        let mut parameters = vec![];
        if let Some(processing_statuses) = processing_status {
            parameters.push(("processingStatuses", processing_statuses.iter().map(|b| b.to_string()).collect()))
        }
        if let Some(page_size) = page_size {
            parameters.push(("pageSize", page_size.to_string()))
        }
        if let Some(created_since) = created_since {
            parameters.push(("createdSince", created_since))
        }
        if let Some(created_until) = created_until {
            parameters.push(("createdUntil", created_until))
        }
        if let Some(pagination_token) = pagination_token {
            parameters.push(("paginationToken", pagination_token))
        }

        client.make_request( "/dataKiosk/2023-11-15/queries", Method::GET, parameters).await
    }



    /// Creates a Data Kiosk query request.
    ///
    /// Rate (requests per second): 0.0167
    ///
    /// Burst: 15
    ///
    /// # Parameters
    /// - `body`: The body of the request. Type: `CreateQuerySpecification`.
    ///
    /// # Responses
    /// - **202 (Success):** Returns a `CreateQueryResponse` object.
    async fn create_query(
        client: &mut Client,
        body: String,
    ) -> Result<Response, Errors> {
        client
            .make_request("/dataKiosk/2023-11-15/queries", Method::POST, [("body", body)])
            .await
    }



    /// Returns query details for the specified queryId.
    ///
    /// Rate (requests per second): 2.0
    ///
    /// Burst: 15
    ///
    /// # Parameters
    /// - `query_id`: The query identifier. Type: `String`.
    ///
    /// # Responses
    /// - **200 (Success):** Returns query details in a `Query` object.
    async fn get_query(client: &mut Client, query_id: String) -> Result<Response, Errors> {
        let endpoint = format!("/dataKiosk/2023-11-15/queries/{}", query_id);

        client
            .make_request(&endpoint, Method::GET, None::<(String, String)>)
            .await
    }

    /// Cancels the query specified by the queryId parameter.
    ///
    /// Rate (requests per second): 0.0222
    ///
    /// Burst: 10
    ///
    /// # Parameters
    /// - `query_id`: The query identifier. Type: `String`.
    ///
    /// # Responses
    /// - **204 (Success):** Indicates successful cancellation with no content.
    async fn cancel_query(client: &mut Client, query_id: String) -> Result<Response, Errors> {
        let endpoint = format!("/dataKiosk/2023-11-15/queries/{}", query_id);

        client
            .make_request(&endpoint, Method::DELETE, None::<(String, String)>)
            .await
    }

    /// Returns the information required for retrieving a Data Kiosk document's contents.
    ///
    /// Rate (requests per second): 0.0167
    ///
    /// Burst: 15
    ///
    /// # Parameters
    /// - `document_id`: The identifier for the Data Kiosk document. Type: `String`.
    ///
    /// # Responses
    /// - **200 (Success):** Returns document details in a `GetDocumentResponse` object.
    async fn get_document(client: &mut Client, document_id: String) -> Result<Response, Errors> {
        let endpoint = format!("/dataKiosk/2023-11-15/documents/{}", document_id);

        client
            .make_request(&endpoint, Method::GET, None::<(String, String)>)
            .await
    }
}
