use crate::error::FrError;
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
use base::queries::shares_distribution::ShareHoldingPercentage;
use mbase::models::share_amount::ShareAmount;
use mbase::util::decimal_util::DecimalExt;
use rust_decimal::Decimal;

pub struct SharesDistributionProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl SharesDistributionProvider for SharesDistributionProviderMock {
    async fn get(&self, _: SharedDistributionParJs) -> Result<SharedDistributionResJs, FrError> {
        req_delay().await;

        let fake_supply = 10_000_000;

        let mut holders = vec![
            mock_holding(
                "FTPBN666KYZVB5YYYLRZ6GXWBKWLJJSXQ3N753USSWS2WIAYK7WJTTYRPI",
                100_000,
                fake_supply,
            )?,
            mock_holding(
                "NT4TNO4NXGI46MBS6T5HDI25XKO5GESRSMRAATNTAYE6YKUE7N34TONJBA",
                600_123,
                fake_supply,
            )?,
            mock_holding(
                "Y6O4BH3SUBLIHDU33XTLCGTS7SFBRZWFVBUA5IOKCOO2Z4SBVY6XXT6ZUQ",
                2_000,
                fake_supply,
            )?,
            mock_holding(
                "2QFRITR4DMZCISHVXLQROMMSY6AE4L3GUEWHURMU7PX4BUGK5JMXBO4RP4",
                80_934,
                fake_supply,
            )?,
            mock_holding(
                "XJ3MFB3OXKGE75527WJIRW3AN7CGN37M3P722U2JL4ZVHDLZDO2NUE5X6U",
                20,
                fake_supply,
            )?,
            mock_holding(
                "FQ5YHU7EAGRE26CAXZZKL6V7M7DEZCLVS7WLPHL2XZJPHNTMW7AE4CIN3Q",
                25_000,
                fake_supply,
            )?,
            mock_holding(
                "3C42TKTDQ34LECWORTVWFEGE5NJCTCRBF65DBR4ALRRRDTW4IMH6UEZGO4",
                11_000,
                fake_supply,
            )?,
            mock_holding(
                "WJ22SNKZWIDTHIL4MFVOEXKUCKWBQGBPAUFBZHVA7UV2PB6BS4YQKR3EA4",
                3_300,
                fake_supply,
            )?,
            mock_holding(
                "67KZZOEPAKCIDYKA2L36ACH6H7HN3MSJYHRODB4NEZ7W6GDRYTA7S44F3M",
                5_121,
                fake_supply,
            )?,
            mock_holding(
                "FKY3WF4F4DEVPOFIUF2PFXAMZMSLBZZQAISM2N7UOMUVCXCC7BYC3ECSAY",
                100_000,
                fake_supply,
            )?,
            mock_holding(
                "3I2M6J7PAE7AJVZK7CZPNYW6ALXYPW5BXP5NEDNAK6UXYQ633U3HBSQ734",
                95_000,
                fake_supply,
            )?,
            mock_holding(
                "LJLXMEFKOCUESFVBXTWXTJBHOKS3HEKEKIJUGOYW4KQVCBNWZUJO5STBO4",
                1_000_000,
                fake_supply,
            )?,
            mock_holding(
                "Q2SNH74U2ELDOESMMR2ORKM3AGIUFJ53BB4MDFNCQXO32J3YNGANSFSMMI",
                400_000,
                fake_supply,
            )?,
            mock_holding(
                "QKKOGQIODTJOSEHE73HDFDQDTEMN6CI3Q3D5HGWJMVJ2LTOUCRSGAUL6V4",
                2_100_000,
                fake_supply,
            )?,
            mock_holding(
                "BPZI3XJUIKFUSNSREZA4I55AMDCDBKUQQC7CSMJDOS45X2B6XSBZ6WNOXM",
                60_222,
                fake_supply,
            )?,
            mock_holding(
                "WKA6WHSHXB6S2FE3T4ZXTF3D4TVQ363YENQJNODHAUDXUHZRWS4XVI5GCM",
                240_000,
                fake_supply,
            )?,
            mock_holding(
                "SYIP4Q2XZRGQI4OKLUCTOMEDU27USWFXJACUXTWELU3BK2JMLULLZGCNR4",
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
            return Err(FrError::Internal(format!(
                "Invalid mock data: amount sum ({amount_sum}) >= supply ({fake_supply})"
            )));
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
            not_owned_shares: "555".to_owned(),
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
