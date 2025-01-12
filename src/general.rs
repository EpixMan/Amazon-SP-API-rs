use crate::error_handling::Errors;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::time::{ Instant};
use reqwest::{ Response, Url};

const ENDPOINT_NA: &str = "https://sellingpartnerapi-na.amazon.com";
const ENDPOINT_EU: &str = "https://sellingpartnerapi-eu.amazon.com";
const ENDPOINT_FE: &str = "https://sellingpartnerapi-fe.amazon.com";

enum CountryMarketplace {
    Canada,
    UnitedStates,
    Mexico,
    Brazil,
    Ireland,
    Spain,
    UnitedKingdom,
    France,
    Belgium,
    Netherlands,
    Germany,
    Italy,
    Sweden,
    SouthAfrica,
    Poland,
    Egypt,
    Turkey,
    SaudiArabia,
    UnitedArabEmirates,
    India,
    Singapore,
    Australia,
    Japan,
}
pub struct ClientInformation {
    pub refresh_token: String,
    pub client_id: String,
    pub client_secret: String,
    pub country_marketplace: CountryMarketplace,
}
#[derive(Deserialize)]
struct AccessToken {
    access_token: String,
    expires_in: i64,
    refresh_token: String,
    token_type: String,
}

impl ClientInformation {
    async fn get_access_token(&self, x: &reqwest::Client) -> Result<AccessToken, Errors> {
        let ff = json!({
        "refresh_token": self.refresh_token,
        "client_id": self.client_id,
        "client_secret":self.client_secret,
        "grant_type": "refresh_token"});

        Ok(x.request(
            reqwest::Method::POST,
            "https://api.amazon.com/auth/o2/token",
        )
        .body(ff.to_string())
        .send()
        .await?
        .json::<AccessToken>()
        .await?)
    }
}
impl CountryMarketplace {
    /// Returns the marketplace ID and the endpoint for the given country.
    fn details(&self) -> (&'static str, &'static str) {
        match self {
            CountryMarketplace::Canada => ("A2EUQ1WTGCTBG2", ENDPOINT_NA),
            CountryMarketplace::UnitedStates => ("ATVPDKIKX0DER", ENDPOINT_NA),
            CountryMarketplace::Mexico => ("A1AM78C64UM0Y8", ENDPOINT_NA),
            CountryMarketplace::Brazil => ("A2Q3Y263D00KWC", ENDPOINT_NA),
            CountryMarketplace::Ireland => ("A28R8C7NBKEWEA", ENDPOINT_EU),
            CountryMarketplace::Spain => ("A1RKKUPIHCS9HS", ENDPOINT_EU),
            CountryMarketplace::UnitedKingdom => ("A1F83G8C2ARO7P", ENDPOINT_EU),
            CountryMarketplace::France => ("A13V1IB3VIYZZH", ENDPOINT_EU),
            CountryMarketplace::Belgium => ("AMEN7PMS3EDWL", ENDPOINT_EU),
            CountryMarketplace::Netherlands => ("A1805IZSGTT6HS", ENDPOINT_EU),
            CountryMarketplace::Germany => ("A1PA6795UKMFR9", ENDPOINT_EU),
            CountryMarketplace::Italy => ("APJ6JRA9NG5V4", ENDPOINT_EU),
            CountryMarketplace::Sweden => ("A2NODRKZP88ZB9", ENDPOINT_EU),
            CountryMarketplace::SouthAfrica => ("AE08WJ6YKNBMC", ENDPOINT_EU),
            CountryMarketplace::Poland => ("A1C3SOZRARQ6R3", ENDPOINT_EU),
            CountryMarketplace::Egypt => ("ARBP9OOSHTCHU", ENDPOINT_EU),
            CountryMarketplace::Turkey => ("A33AVAJ2PDY3EV", ENDPOINT_EU),
            CountryMarketplace::SaudiArabia => ("A17E79C6D8DWNP", ENDPOINT_EU),
            CountryMarketplace::UnitedArabEmirates => ("A2VIGQ35RCS4UG", ENDPOINT_EU),
            CountryMarketplace::India => ("A21TJRUUN4KGV", ENDPOINT_EU),
            CountryMarketplace::Singapore => ("A19VAU5U5O7RUS", ENDPOINT_FE),
            CountryMarketplace::Australia => ("A39IBJ37TRP1C6", ENDPOINT_FE),
            CountryMarketplace::Japan => ("A1VC38T7YXB528", ENDPOINT_FE),
        }
    }
}
pub struct Client {
    access_token: AccessToken,
    client_information: ClientInformation,
    last_refresh: Instant,
    reqwest_client: reqwest::Client,
}
impl Client {
    async fn refresh_token(&mut self) {
        match self
            .client_information
            .get_access_token(&self.reqwest_client).await
        {
            Ok(o) => self.access_token = o,
            Err(_) => {}
        }
    }
    pub async fn new(client: ClientInformation) -> Result<Self, Errors> {
        let reqwest_client = reqwest::Client::new();
        match client.get_access_token(&reqwest_client).await {
            Ok(o) => Ok(Client {
                access_token: o,
                client_information: client,
                last_refresh: Instant::now(),
                reqwest_client,
            }),
            Err(e) => Err(e),
        }
    }
    async fn check_validity(&mut self) {
        if Instant::now().duration_since(self.last_refresh).as_secs()
            > (self.access_token.expires_in - 10) as u64
        {
            self.refresh_token().await;
        }
    }
    fn create_header(&mut self) -> HeaderMap {
        let mut header_map = HeaderMap::new();
        header_map.insert("x-amz-access-token", self.access_token.access_token.parse().unwrap());
        header_map.insert("CONTENT_TYPE", "application/json".parse().unwrap());
        header_map.insert("user-agent", "Amazon-SP-API-rs 0.1.0".parse().unwrap());
        header_map
    }
    pub async fn make_request(&mut self, path: &str, method: reqwest::Method, parameters: HashMap<&str, &str>) -> Result<Response, Errors> {
        parameters.push( ("marketplaceIds", self.client_information.country_marketplace.details().0));
        Ok(self.reqwest_client.request(method, Url::parse_with_params(format!("{}{}",self.client_information.country_marketplace.details().1, path).as_str(), parameters)?).headers(self.create_header()).send().await?)
    }
}
