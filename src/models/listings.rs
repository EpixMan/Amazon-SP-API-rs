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

        client.make_request(&url, Method::GET, Some(parameters)).await
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
        /// Search for and return a list of selling partner listings items
        ///
        /// Rate (requests per second): 5
        /// Burst: 5
        ///
        /// # Parameters
        /// - client: Reference to the HTTP client
        /// - seller_id: Required selling partner identifier
        /// - marketplace: The marketplace to search in (max count: 1)
        /// - issue_locale: Optional locale for issue localization (e.g., "en_US", "fr_CA")
        /// - included_data: Optional data sets to include (default: summaries)
        /// - identifiers: Optional list of product identifiers (max: 20)
        /// - identifiers_type: Required when identifiers are provided
        /// - variation_parent_sku: Optional SKU to filter variation children
        /// - package_hierarchy_sku: Optional SKU to filter package hierarchy
        /// - created_after: Optional filter for items created after date
        /// - created_before: Optional filter for items created before date
        /// - last_updated_after: Optional filter for items updated after date
        /// - last_updated_before: Optional filter for items updated before date
        /// - with_issue_severity: Optional filter by issue severity levels
        /// - with_status: Optional filter by status
        /// - without_status: Optional filter to exclude status
        /// - sort_by: Optional attribute to sort by (default: lastUpdatedDate)
        /// - sort_order: Optional sort direction (default: DESC)
        /// - page_size: Optional results per page (max: 20, default: 10)
        /// - page_token: Optional pagination token
        pub async fn search_listings_items(
            client: &mut Client,
            seller_id: String,
            marketplace: CountryMarketplace,
            issue_locale: Option<String>,
            included_data: Option<Vec<String>>,
            identifiers: Option<Vec<String>>,
            identifiers_type: Option<String>,
            variation_parent_sku: Option<String>,
            package_hierarchy_sku: Option<String>,
            created_after: Option<String>,
            created_before: Option<String>,
            last_updated_after: Option<String>,
            last_updated_before: Option<String>,
            with_issue_severity: Option<Vec<String>>,
            with_status: Option<Vec<String>>,
            without_status: Option<Vec<String>>,
            sort_by: Option<String>,
            sort_order: Option<String>,
            page_size: Option<u32>,
            page_token: Option<String>,
        ) -> Result<Response, Errors> {
            let uri = format!("/listings/2021-08-01/items/{}", seller_id);

            // Validate mutually exclusive parameters
            if identifiers.is_some() && (variation_parent_sku.is_some() || package_hierarchy_sku.is_some()) {
                return Err(Errors::CustomError(
                    "Cannot use identifiers with variationParentSku or packageHierarchySku".to_string()
                ));
            }

            if variation_parent_sku.is_some() && package_hierarchy_sku.is_some() {
                return Err(Errors::CustomError(
                    "Cannot use both variationParentSku and packageHierarchySku".to_string()
                ));
            }

            // Validate array sizes
            if let Some(ids) = &identifiers {
                if ids.len() > 20 {
                    return Err(Errors::CustomError("Maximum 20 identifiers allowed".to_string()));
                }
            }

            // Validate page size
            if let Some(size) = page_size {
                if size > 20 {
                    return Err(Errors::CustomError("Maximum page size is 20".to_string()));
                }
            }

            let mut params: Vec<(String, String)> = Vec::new();

            // Add required marketplace ID
            params.push(("marketplaceIds".to_string(), marketplace.details().0.to_string()));
            params.push(("sellerId".to_string(), seller_id));

            // Add optional parameters
            if let Some(locale) = issue_locale {
                params.push(("issueLocale".to_string(), locale));
            }
            if let Some(data) = included_data {
                params.push(("includedData".to_string(), data.join(",")));
            }
            if let Some(ids) = identifiers {
                params.push(("identifiers".to_string(), ids.join(",")));
            }
            if let Some(id_type) = identifiers_type {
                params.push(("identifiersType".to_string(), format!("{:?}", id_type)));
            }
            if let Some(sku) = variation_parent_sku {
                params.push(("variationParentSku".to_string(), sku));
            }
            if let Some(sku) = package_hierarchy_sku {
                params.push(("packageHierarchySku".to_string(), sku));
            }
            if let Some(date) = created_after {
                params.push(("createdAfter".to_string(), date));
            }
            if let Some(date) = created_before {
                params.push(("createdBefore".to_string(), date));
            }
            if let Some(date) = last_updated_after {
                params.push(("lastUpdatedAfter".to_string(), date));
            }
            if let Some(date) = last_updated_before {
                params.push(("lastUpdatedBefore".to_string(), date));
            }
            if let Some(severities) = with_issue_severity {
                params.push(("withIssueSeverity".to_string(),
                             severities.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(",")));
            }
            if let Some(statuses) = with_status {
                params.push(("withStatus".to_string(),
                             statuses.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(",")));
            }
            if let Some(statuses) = without_status {
                params.push(("withoutStatus".to_string(),
                             statuses.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>().join(",")));
            }
            if let Some(sort) = sort_by {
                params.push(("sortBy".to_string(), format!("{:?}", sort)));
            }
            if let Some(order) = sort_order {
                params.push(("sortOrder".to_string(), format!("{:?}", order)));
            }
            if let Some(size) = page_size {
                params.push(("pageSize".to_string(), size.to_string()));
            }
            if let Some(token) = page_token {
                params.push(("pageToken".to_string(), token));
            }
            println!("{:#?}", params);

            client.make_request(&uri, Method::GET, Some(params)).await
        }
}
