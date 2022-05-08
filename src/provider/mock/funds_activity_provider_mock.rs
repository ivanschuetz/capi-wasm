use super::{mock_tx_id, req_delay};
use crate::provider::{
    def::shares_distribution_provider_def::shorten_address,
    funds_activity_provider::{
        FundsActivityProvider, FundsActivityViewData, LoadFundsActivityParJs,
        LoadFundsActivityResJs,
    },
};
use anyhow::Result;
use async_trait::async_trait;

pub struct FundsActivityProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl FundsActivityProvider for FundsActivityProviderMock {
    async fn get(&self, _: LoadFundsActivityParJs) -> Result<LoadFundsActivityResJs> {
        req_delay().await;

        Ok(LoadFundsActivityResJs {
            entries: vec![FundsActivityViewData {
                amount: "123".to_owned(),
                is_income: "true".to_owned(), 
                type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "923".to_owned(),
                is_income: "false".to_owned(), type_label: "Withdraw".to_owned(),
                description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "10001100".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "10".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "33333".to_owned(),
                is_income: "false".to_owned(), type_label: "Withdraw".to_owned(),
                description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "0.23".to_owned(),
                is_income: "false".to_owned(), type_label: "Withdraw".to_owned(),
                description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "10".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "123000".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "12".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "12123123422324233".to_owned(),
                is_income: "false".to_owned(), type_label: "Withdraw".to_owned(),
                description: "This is a short fake description".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "1000".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "Maybe income will have descriptions too?".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "12312312112".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "123".to_owned(),
                is_income: "false".to_owned(), type_label: "Withdraw".to_owned(),
                description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "123000".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "123".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "5".to_owned(),
                is_income: "false".to_owned(), type_label: "Withdraw".to_owned(),
                description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "500".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "1111".to_owned(),
                is_income: "false".to_owned(), type_label: "Withdraw".to_owned(),
                description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "123".to_owned(),
                is_income: "false".to_owned(), type_label: "Withdraw".to_owned(),
                description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "550".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "1212".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "88".to_owned(),
                is_income: "true".to_owned(), type_label: "Income".to_owned(),
                description: "".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }, FundsActivityViewData {
                amount: "2999".to_owned(),
                is_income: "false".to_owned(), type_label: "Withdraw".to_owned(),
                description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
                date: "Wed, 20 Apr 2022".to_owned(),
                tx_id: mock_tx_id(), address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
                tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
            }],
        })
    }
}
