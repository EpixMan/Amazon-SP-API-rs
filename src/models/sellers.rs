use reqwest::{Method, Response};
use crate::error_handling::Errors;
use crate::general::Client;

pub struct Sellers;
impl Sellers {
    pub async fn get_marketplace_participations(client: &mut Client) -> Result<Response, Errors> {
        const URL: &str = "/sellers/v1/marketplaceParticipations";

        client.make_request(URL, Method::GET, None::<Vec<(String, String)>> ).await
    }
    pub async fn get_account(client: &mut Client) -> Result<Response, Errors> {
        const URL: &str = "/sellers/v1/account";
        client.make_request(URL, Method::GET, None::<Vec<(String, String)>> ).await
    }
}