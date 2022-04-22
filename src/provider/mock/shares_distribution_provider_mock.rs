use crate::js::explorer_links::explorer_address_link_env;
use crate::provider::def::shares_distribution_provider_def::{
    not_owned_shares_holdings, shorten_address,
};
use crate::provider::mock::req_delay;
use crate::provider::shares_distribution_provider::{
    ShareHoldingPercentageJs, SharedDistributionParJs, SharedDistributionResJs,
    SharesDistributionProvider,
};
use anyhow::{anyhow, Error, Result};
use async_trait::async_trait;
use base::flows::create_dao::share_amount::ShareAmount;
use base::{decimal_util::DecimalExt, queries::shares_distribution::ShareHoldingPercentage};
use rust_decimal::Decimal;

pub struct SharesDistributionProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl SharesDistributionProvider for SharesDistributionProviderMock {
    async fn get(&self, _: SharedDistributionParJs) -> Result<SharedDistributionResJs> {
        req_delay().await;

        let fake_supply = 10_000_000;

        let mut holders = vec![
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                10_000,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                200_000,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                2_000,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                80_934,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                20,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                25_000,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                11_000,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                3_300,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                5_121,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                100_000,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                95_000,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                500_000,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                400_000,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                700_123,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                60_222,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                240_000,
                fake_supply,
            )?,
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                777_002,
                fake_supply,
            )?,
        ];

        // sort descendingly by amount
        holders.sort_by(|h1, h2| h2.amount.val().cmp(&h1.amount.val()));

        // verify that the sum of the amounts is the total supply
        // this is mock data, but it needs to make sense for the chart to work properly
        let amount_sum: u64 = holders.iter().map(|h| h.amount.val()).sum();
        if amount_sum >= fake_supply {
            return Err(anyhow!(
                "Invalid mock data: amount sum ({amount_sum}) >= supply ({fake_supply})"
            ));
        }

        let mut holders_js = vec![];
        for h in &holders {
            holders_js.push(ShareHoldingPercentageJs {
                address: h.address.to_string(),
                label: shorten_address(&h.address)?,
                address_browser_link: explorer_address_link_env(&h.address),
                amount: h.amount.to_string(),
                percentage_formatted: h.percentage.format_percentage(),
                percentage_number: h.percentage.to_string(),
                type_: "holder".to_owned(),
            });
        }

        let not_owned = not_owned_shares_holdings(&holders, fake_supply)?;
        log::info!(
            "mock data: not owned shares: amount: {}, percentage: {}",
            not_owned.amount,
            not_owned.percentage_formatted
        );

        holders_js.push(not_owned);

        Ok(SharedDistributionResJs {
            holders: holders_js,
        })
    }
}

fn mock_holding(address_str: &str, amount: u64, supply: u64) -> Result<ShareHoldingPercentage> {
    let amount_decimal: Decimal = amount.into();
    let asset_supply_decimal: Decimal = supply.into();

    Ok(ShareHoldingPercentage {
        address: address_str.parse().map_err(Error::msg)?,
        amount: ShareAmount::new(amount),
        percentage: amount_decimal
            .checked_div(asset_supply_decimal)
            .ok_or_else(|| {
                anyhow!(
                    "Unexpected: division: {} by {} returned an error",
                    amount_decimal,
                    asset_supply_decimal
                )
            })?,
    })
}
