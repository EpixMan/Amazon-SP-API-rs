use reqwest::{Method, Response};
use serde_json::{json, Value};
use crate::general::{Client, CountryMarketplace};
use strum_macros;
use crate::error_handling::Errors;

pub struct Listings;
#[allow(non_camel_case_types)]
#[derive(strum_macros::Display)]
pub enum IncludedData {
    summaries,
    issues,
    attributes,
    offers,
    fulfillmentAvailability,
    procurement,
    relationships,
    productTypes,
}
impl Listings {
    pub async fn get_listings_item(
        client: &mut Client,
        seller_id: &str,
        sku: &str,
        country_marketplace: CountryMarketplace,
        included_data: Vec<IncludedData>,

    ) -> Result<Response, Errors> {
        let url = format!("/listings/2021-08-01/items/{}/{}", seller_id, sku);
        let parameters = vec![("marketplaceIds", country_marketplace.details().0.to_string()), ("includedData", included_data.iter().map(|b| b.to_string()).collect::<Vec<String>>().join(","))];

        client.make_request(&url, Method::GET, parameters).await
    }
    pub async fn put_listings_item() {}

    pub async fn patch_listings_item(
        client: &mut Client,
        seller_id: &str,
        sku: &str,
        country_marketplace: CountryMarketplace,
        validation_preview: bool,
        body: &str
    ) -> Result<Response, Errors> {
        let url = format!("/listings/2021-08-01/items/{}/{}", seller_id, sku);
        let mut parameters = vec![("marketplaceIds", country_marketplace.details().0)];
        if validation_preview {
            parameters.push(("mode", "VALIDATION_PREVIEW"))
        }

        client.make_request_w_body(&url, Method::PATCH, Some(parameters), body.parse().unwrap()).await

    }
    ///This is a simple way to change prices
    pub async fn wrapped_patch_listings_item(
        client: &mut Client,
        seller_id: &str,
        sku: &str,
        country_marketplace: CountryMarketplace,
        validation_preview: bool,
        new_price: f64

    ) -> Result<Response, Errors> {
        let item_data: Value = Self::get_listings_item(client, seller_id, sku ,CountryMarketplace::SaudiArabia, vec![IncludedData::productTypes, IncludedData::attributes]).await?.json().await?;
        let item_type = &item_data["productTypes"][0]["productType"];
        let mut offers = item_data["attributes"]["purchasable_offer"].clone();

        offers[0]["our_price"][0]["schedule"][0]["value_with_tax"] = Value::from(new_price);
        let final_body = json!({
            "productType": item_type,
            "patches": [{
                    "op":"replace",
                  "path":"/attributes/purchasable_offer",
                  "value":offers
            }]
        });
        Self::patch_listings_item(client, seller_id, sku, country_marketplace, validation_preview, &*final_body.to_string()).await

    }
}
