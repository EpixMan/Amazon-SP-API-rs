use std::collections::HashMap;
use reqwest::{Method, Response};
use crate::error_handling::Errors;
use crate::general::{Client, CountryMarketplace};
pub enum CompetitiveSummaryIncludedData {
    FeaturedBuyingOptions,
    ReferencePrices,
    LowestPricedOffers
}
impl CompetitiveSummaryIncludedData {
    fn to_string(&self) -> String {
        match self {
            CompetitiveSummaryIncludedData::FeaturedBuyingOptions => "featuredBuyingOptions".to_string(),
            CompetitiveSummaryIncludedData::ReferencePrices => "referencePrices".to_string(),
            CompetitiveSummaryIncludedData::LowestPricedOffers => "lowestPricedOffers".to_string()
        }
    }
}
pub struct ProductPricing;
impl ProductPricing {
    /// **DEV NOTE:** This will internally use the Vec of sku to make it as a batch
    /// the set of responses for a batch of Featured Offer Expected Price (FOEP) requests.
    ///
    /// Rate (requests per second): 0.033
    /// Burst: 1
    ///
    /// # Parameters
    /// - `client`: Reference to the HTTP client.
    /// - `requests`: A vector of individual FOEP request parameters.
    /// - `uri`: The URI associated with the requests. Default: `/products/pricing/2022-05-01/offer/featuredOfferExpectedPrice`.
    ///
    /// # Responses
    /// - **200 (Success):** Returns a `GetFeaturedOfferExpectedPriceBatchResponse` object.
    pub async fn get_featured_offer_expected_price_batch(
        client: &mut Client,
        //requests: Vec<(String, String)>, // Tuple (marketplace_id, sku)
        uri: Option<String>, // Optional custom URI
        method: String,
        body: Option<String>,
        headers: Option<HashMap<String, String>>,
        market_place: crate::general::CountryMarketplace,
        sku: Vec<String>,
    ) -> Result<Response, Errors> {
        const DEFAULT_URI: &str = "/products/pricing/2022-05-01/offer/featuredOfferExpectedPrice";

        let uri = uri.unwrap_or(DEFAULT_URI.to_string());
        let mut params = vec![("uri".to_string(),uri.clone()), ("method".to_string(), method), ("marketplaceId".to_string(), market_place.details().0.to_string())];
        if let Some(o) = body {
            params.push(("body".to_string(), o));
        }
        if let Some(h) = headers {
            params.push(("headers".to_string(), h.iter().map(|s| serde_json::to_string(&s).unwrap()).collect()));
        }
        let final_params: Vec<Vec<(String,String)>> = sku.iter().map(|small_sku| {
            let mut ff = params.clone();
            ff.push((String::from("sku"), small_sku.clone()));
            ff
        }).collect();

        client
            .make_request(&uri, Method::POST, [("getFeaturedOfferExpectedPriceBatchRequestBody".to_string(), serde_json::to_string(&final_params)?)])
            .await
    }

    /// Returns the competitive summary for a batch of ASIN and marketplaceId combinations.
    /// With number of items between 1 and 20.
    ///
    /// Rate (requests per second): 0.033
    /// Burst: 1
    ///
    /// # Parameters
    /// - `client`: Reference to the HTTP client.
    /// - `requests`: A vector of individual request parameters.
    /// - `uri`: The URI associated with the requests. Default: `/products/pricing/2022-05-01/items/competitiveSummary`.
    ///
    /// # Responses
    /// - **200 (Success):** Returns a `CompetitiveSummaryBatchResponse` object.
    async fn get_competitive_summary(
        client: &mut Client,
        asin: Vec<String>,
        market_place: CountryMarketplace,
        included_data: Vec<CompetitiveSummaryIncludedData>,
        method: String,
        uri: Option<String>, // Optional custom URI
    ) -> Result<Response, Errors> {

        const DEFAULT_URI: &str = "/products/pricing/2022-05-01/items/competitiveSummary";

        let uri = uri.unwrap_or(DEFAULT_URI.to_string());

        let mut params = vec![
            ("uri".to_string(),uri.clone()),
            ("method".to_string(), method),
            ("marketplaceId".to_string(), market_place.details().0.to_string()),
            (String::from("includedData"), included_data.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(",")),
        ];
        let final_params: Vec<_> = asin.iter().map(|asin| {
            let mut ff = params.clone();
            ff.push((String::from("asin"), asin.clone()));
            ff
        }).collect();




        client
            .make_request(&uri, Method::POST, [("requests", serde_json::to_string(&final_params)?)])
            .await
    }


}