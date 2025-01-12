use std::collections::HashMap;
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
    fn to_string(&self) -> String {
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
    fn get_queries(client: &mut Client, parameters: HashMap<&str, &str> ) -> Result<Response, Errors>{
        client.make_request( "/dataKiosk/2023-11-15/queries", Method::GET, parameters)
    }
    ///Creates a Data Kiosk query request.
    ///
    /// Note: The retention of a query varies based on the fields requested. Each field within a schema is annotated with a @resultRetention directive that defines how long a query containing that field will be retained. When a query contains multiple fields with different retentions, the shortest (minimum) retention is applied. The retention of a query's resulting documents always matches the retention of the query.
    fn create_query(client: &mut Client, parameters: HashMap<&str, &str> ) -> Result<Response, Errors>{
        client.make_request( "/dataKiosk/2023-11-15/queries", Method::POST, parameters)
    }
    ///Returns query details for the query specified by the queryId parameter. See the createQuery operation for details about query retention.
    fn get_query(client: &mut Client, parameters: HashMap<&str, &str> ) -> Result<Response, Errors>{
        client.make_request( "/dataKiosk/2023-11-15/queries/", Method::GET, parameters)
    }
    fn cancel_query(client: &mut Client, parameters: HashMap<&str, &str> ) -> Result<Response, Errors>{
        client.make_request( "/dataKiosk/2023-11-15/queries/", Method::DELETE, parameters)
    }
    fn get_document(client: &mut Client, parameters: HashMap<&str, &str> ) -> Result<Response, Errors>{
        client.make_request( "/dataKiosk/2023-11-15/documents/", Method::GET, parameters)
    }
}
