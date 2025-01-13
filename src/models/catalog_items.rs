use reqwest::{Method, Response};
use crate::error_handling::Errors;
use crate::general::{Client, CountryMarketplace};
use serde_json;

pub struct CatalogItems;
impl CatalogItems {

    /// Search for and return a list of Amazon catalog items and associated information.
    ///
    /// Rate (requests per second): 2
    /// Burst: 2
    ///
    /// # Parameters
    /// - client: Reference to the HTTP client
    /// - marketplace_ids: Required list of Amazon marketplace identifiers
    /// - identifiers: Optional list of product identifiers to search for
    /// - identifiers_type: Required when identifiers are provided
    /// - included_data: Optional list of data sets to include (default: summaries)
    /// - locale: Optional locale for localized summaries
    /// - seller_id: Required when identifiers_type is SKU
    /// - keywords: Optional list of search keywords (cannot be used with identifiers)
    /// - brand_names: Optional list of brand names for keyword searches
    /// - classification_ids: Optional list of classification IDs for keyword searches
    /// - page_size: Optional number of results per page (max: 20, default: 10)
    /// - page_token: Optional token for pagination
    /// - keywords_locale: Optional language of the keywords
    pub async fn search_catalog_items(
        client: &mut Client,
        marketplace_ids: Vec<CountryMarketplace>,
        identifiers: Option<Vec<String>>,
        identifiers_type: Option<String>,
        included_data: Option<Vec<String>>,
        locale: Option<String>,
        seller_id: Option<String>,
        keywords: Option<Vec<String>>,
        brand_names: Option<Vec<String>>,
        classification_ids: Option<Vec<String>>,
        page_size: Option<i32>,
        page_token: Option<String>,
        keywords_locale: Option<String>,
    ) -> Result<Response, Errors> {
        const URI: &str = "/catalog/2022-04-01/items";

        let mut params: Vec<(String, String)> = Vec::new();

        // Add required parameters
        params.push(("marketplaceIds".to_string(), marketplace_ids.iter().map(|b|b.details().0.to_string()).collect::<Vec<String>>().join(",")));

        // Add optional parameters
        if let Some(ids) = identifiers {
            params.push(("identifiers".to_string(), ids.join(",")));
        }
        if let Some(id_type) = identifiers_type {
            params.push(("identifiersType".to_string(), id_type));
        }
        if let Some(data) = included_data {
            params.push(("includedData".to_string(), data.join(",")));
        }
        if let Some(loc) = locale {
            params.push(("locale".to_string(), loc));
        }
        if let Some(sid) = seller_id {
            params.push(("sellerId".to_string(), sid));
        }
        if let Some(kw) = keywords {
            params.push(("keywords".to_string(), kw.join(",")));
        }
        if let Some(brands) = brand_names {
            params.push(("brandNames".to_string(), brands.join(",")));
        }
        if let Some(class_ids) = classification_ids {
            params.push(("classificationIds".to_string(), class_ids.join(",")));
        }
        if let Some(size) = page_size {
            params.push(("pageSize".to_string(), size.to_string()));
        }
        if let Some(token) = page_token {
            params.push(("pageToken".to_string(), token));
        }
        if let Some(kw_locale) = keywords_locale {
            params.push(("keywordsLocale".to_string(), kw_locale));
        }

        client.make_request(URI, Method::GET, params).await
    }

    /// Retrieves details for an item in the Amazon catalog by ASIN.
    ///
    /// Rate (requests per second): 2
    /// Burst: 2
    ///
    /// # Parameters
    /// - client: Reference to the HTTP client
    /// - asin: The Amazon Standard Identification Number of the item
    /// - marketplace_ids: Required list of Amazon marketplace identifiers
    /// - included_data: Optional list of data sets to include (default: summaries)
    /// - locale: Optional locale for localized summaries
    pub async fn get_catalog_item(
        client: &mut Client,
        asin: String,
        marketplace_ids: Vec<CountryMarketplace>,
        included_data: Option<Vec<String>>,
        locale: Option<String>,
    ) -> Result<Response, Errors> {
        let uri = format!("/catalog/2022-04-01/items/{}", asin);

        let mut params: Vec<(String, String)> = Vec::new();

        // Add required parameters
        params.push(("marketplaceIds".to_string(), marketplace_ids.iter().map(|b|b.details().0.to_string()).collect::<Vec<String>>().join(",")));

        // Add optional parameters
        if let Some(data) = included_data {
            params.push(("includedData".to_string(), data.join(",")));
        }
        if let Some(loc) = locale {
            params.push(("locale".to_string(), loc));
        }

        client.make_request(&uri, Method::GET, params).await
    }
    }