use crate::provider::wyre_provider::{WyreProvider, WyreReserveResJs};
use anyhow::Result;
use async_trait::async_trait;
use base::reqwest_ext::ResponseExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct WyreProviderDef {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WyreProvider for WyreProviderDef {
    async fn reserve(&self) -> Result<WyreReserveResJs> {
        let api = test_wyre_api()?;

        let res = api.reserve().await?;

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

    pub async fn reserve(&self) -> Result<WyreReserveRes> {
        let body = WyreRegistrationBody {
            referrer_account_id: self.account_id.to_owned(),
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
