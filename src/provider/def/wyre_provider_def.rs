use crate::provider::wyre_provider::{WyreProvider, WyreReserveParsJs, WyreReserveResJs};
use algonaut::core::Address;
use anyhow::{Error, Result};
use async_trait::async_trait;
use base::reqwest_ext::ResponseExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct WyreProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WyreProvider for WyreProviderDef {
    async fn reserve(&self, pars: WyreReserveParsJs) -> Result<WyreReserveResJs> {
        let api = test_wyre_api()?;

        let address = pars.address.parse().map_err(Error::msg)?;

        let res = api.reserve(&address).await?;

        Ok(WyreReserveResJs {
            url: res.url.clone(),
            reservation: res.reservation.clone(),
        })
    }
}

pub struct WyreApi {
    host: String,
    client: Client,
    account_id: String,
    token: String,
}

impl WyreApi {
    pub fn new(host: &str, account_id: &str, token: &str) -> WyreApi {
        let client = reqwest::Client::new();
        WyreApi {
            host: host.to_owned(),
            client,
            account_id: account_id.to_owned(),
            token: token.to_owned(),
        }
    }

    pub async fn reserve(&self, address: &Address) -> Result<WyreReserveRes> {
        let body = WyreRegistrationBody {
            referrer_account_id: self.account_id.to_owned(),
            amount: "1".to_string(),
            source_currency: "USD".to_string(),
            dest_currency: "ALGO".to_string(),
            dest: format!("algorand:{}", address),

            // prefill (convenience for testing)
            first_name: "firstname".to_owned(),
            last_name: "lastname".to_owned(),
            phone: "000000".to_owned(),
            email: "sfsdf@sdfsf.com".to_owned(),
            country: "DE".to_owned(),
            postal_code: "AAAA".to_owned(), // for non-us addresses
            state: "sdfsfd".to_owned(),
            city: "sdfds".to_owned(),
            street1: "dfsdf".to_owned(),

            // card (can't be passed):
            // VISA:
            // 4111111111111111 or 4444333322221111
            // MASTERCARD (International only) - Use when testing 3D Secure:
            // 5454545454545454 
            // Exp (both): 10/23
            // CVV (both): 555
        };

        let url = format!("{}/orders/reserve", self.host);
        let res = self
            .client
            .post(url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", self.token.clone())
            .json(&body)
            .send()
            .await?
            .to_error_if_http_error()
            .await?
            .json()
            .await?;

        Ok(res)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct WyreRegistrationBody {
    referrer_account_id: String,
    amount: String,
    source_currency: String,
    dest_currency: String,
    dest: String,

    // prefill (convenience for testing) - comment (or enable conditionally)
    first_name: String,
    last_name: String,
    phone: String,
    email: String,
    country: String,
    postal_code: String,
    state: String,
    city: String,
    street1: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WyreReserveRes {
    pub url: String,
    pub reservation: String,
}

fn test_wyre_api() -> Result<WyreApi> {
    Ok(WyreApi::new(
        "https://api.testwyre.com/v3",
        "AC_93XFWQDE78B",
        "Bearer TEST-SK-TE2HEE4Z-ANGQQJTU-EWRYNJ9L-QPQJPWY3",
    ))
}
