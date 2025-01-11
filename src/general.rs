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
