use super::req_delay;
use crate::{
    provider::{
        def::shares_distribution_provider_def::shorten_address,
        funds_activity_provider::{
            FundsActivityProvider, FundsActivityViewData, LoadFundsActivityParJs,
            LoadFundsActivityResJs,
        },
    },
    service::number_formats::{format_decimal_readable, format_short},
};
use anyhow::Result;
use async_trait::async_trait;

pub struct FundsActivityProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl FundsActivityProvider for FundsActivityProviderMock {
    async fn get(&self, pars: LoadFundsActivityParJs) -> Result<LoadFundsActivityResJs> {
        req_delay().await;

        // note that short_amount will be set at the end, based on amount
        // normally we'd set short_amount manually, but tedious + not bad to ensure correctness to not confuse testers here
        let raw_entries = vec![FundsActivityViewData {
            amount: "123".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "111".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "4VWUNOA5RH5OKMCSGEBETHLSPYQMDN3KBQNCQMQNDZZO7P4VDG3A".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/4VWUNOA5RH5OKMCSGEBETHLSPYQMDN3KBQNCQMQNDZZO7P4VDG3A".to_owned(),
        }, FundsActivityViewData {
            amount: "923".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "111".to_owned(),
            is_income: "false".to_owned(), 
            type_label: "Withdraw".to_owned(),
            description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "IR2PR2LQQZAGN3LC3BURTM5IR2ZVTXM3AVSL2Q5YUKTSNMDEIPXQ".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/IR2PR2LQQZAGN3LC3BURTM5IR2ZVTXM3AVSL2Q5YUKTSNMDEIPXQ".to_owned(),
        }, FundsActivityViewData {
            amount: "10001100".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "112.2".to_owned(),
            amount_without_fee: "11100000".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "I72VSHIMVGG4ETJUFSZ3RZIBW6E7WAFGO3YGEMDSI3WL6F5INUMQ".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/I72VSHIMVGG4ETJUFSZ3RZIBW6E7WAFGO3YGEMDSI3WL6F5INUMQ".to_owned(),
        }, FundsActivityViewData {
            amount: "10".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "0.4".to_owned(),
            amount_without_fee: "9".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "TKGKH7DATOCY4HHQALKMRRZJIH6YVOIQASXSWZIFDHL2KAICWIWA".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/TKGKH7DATOCY4HHQALKMRRZJIH6YVOIQASXSWZIFDHL2KAICWIWA".to_owned(),
        }, FundsActivityViewData {
            amount: "33333".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.23".to_owned(),
            amount_without_fee: "22222".to_owned(),
            is_income: "false".to_owned(), 
            type_label: "Withdraw".to_owned(),
            description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "FEYO3OTTFS56XWHFHMNMKD4BAN2UGB7VZGR5KD2AAUQEHMS5WLHA".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/FEYO3OTTFS56XWHFHMNMKD4BAN2UGB7VZGR5KD2AAUQEHMS5WLHA".to_owned(),
        }, FundsActivityViewData {
            amount: "0.23".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "0.001".to_owned(),
            amount_without_fee: "0.01".to_owned(),
            is_income: "false".to_owned(), 
            type_label: "Withdraw".to_owned(),
            description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "KSEVH6TMZB7EI6WFOHR6ZPOECZQ5ZNZWPDSAS62F77QRHL4RXGDQ".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/KSEVH6TMZB7EI6WFOHR6ZPOECZQ5ZNZWPDSAS62F77QRHL4RXGDQ".to_owned(),
        }, FundsActivityViewData {
            amount: "10".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1".to_owned(),
            amount_without_fee: "0.1".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "3CUYREVXKFMJOSWJRC3GY6UEAJ3BA36RGN4PKSL7CYRLCWZSIT3A".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned(),
        }, FundsActivityViewData {
            amount: "123000".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "111".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "SF3XQB6ABD5R5PBYBBULDZQ5UYSCQHGNGATTXKVUMVERK6AXZC2A".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/SF3XQB6ABD5R5PBYBBULDZQ5UYSCQHGNGATTXKVUMVERK6AXZC2A".to_owned(),
        }, FundsActivityViewData {
            amount: "12".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "111".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "CDLZBTE7EE4LHXDYJ4UPZK6EVHRRZ7IKU3GIWAP6YR7GC27NONIQ".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/CDLZBTE7EE4LHXDYJ4UPZK6EVHRRZ7IKU3GIWAP6YR7GC27NONIQ".to_owned(),
        }, FundsActivityViewData {
            amount: "12123123422324233".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "111".to_owned(),
            is_income: "false".to_owned(), 
            type_label: "Withdraw".to_owned(),
            description: "This is a short fake description".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "ZCP7AHV3F6CJBUCC3RKWSKWJS6MBQJNJJ4B2Q5XLUIEHEH5QXX5A".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/ZCP7AHV3F6CJBUCC3RKWSKWJS6MBQJNJJ4B2Q5XLUIEHEH5QXX5A".to_owned(),
        }, FundsActivityViewData {
            amount: "1000".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "111".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "Maybe income will have descriptions too?".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "KV62JNUZ5SBWZX3ZMWLC26WTH6GMIIDB6OCLMTQRN4AN4EYB5MDQ".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/KV62JNUZ5SBWZX3ZMWLC26WTH6GMIIDB6OCLMTQRN4AN4EYB5MDQ".to_owned(),
        }, FundsActivityViewData {
            amount: "12312312112".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "111".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "2RQL5Z76YH3P3OQVVRRDGH4B7IWSYKQUEPQNGJ34ZPUSVMT466SQ".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/2RQL5Z76YH3P3OQVVRRDGH4B7IWSYKQUEPQNGJ34ZPUSVMT466SQ".to_owned(),
        }, FundsActivityViewData {
            amount: "123".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "111".to_owned(),
            is_income: "false".to_owned(), 
            type_label: "Withdraw".to_owned(),
            description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "CLSUJ42ZP7ACLIPMXQC5UQ6YR5MRPGI7KPVQ7TS47AO2U7AQ2MDA".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/CLSUJ42ZP7ACLIPMXQC5UQ6YR5MRPGI7KPVQ7TS47AO2U7AQ2MDA".to_owned(),
        }, FundsActivityViewData {
            amount: "123000".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "123000".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "UB7MEC6EYAMA7ZHWVKK7NILOBBRP5O2KMKRV23BKGHORDVU74MHQ".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/UB7MEC6EYAMA7ZHWVKK7NILOBBRP5O2KMKRV23BKGHORDVU74MHQ".to_owned(),
        }, FundsActivityViewData {
            amount: "123".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "123".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "G64CSMUHIPJRDDAWAIEHSZ5D3M2IA4PUV7SVC4ZY5E4YKVAECCPA".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/G64CSMUHIPJRDDAWAIEHSZ5D3M2IA4PUV7SVC4ZY5E4YKVAECCPA".to_owned(),
        }, FundsActivityViewData {
            amount: "5".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "4.9".to_owned(),
            is_income: "false".to_owned(), 
            type_label: "Withdraw".to_owned(),
            description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "MZZ4WUZHXAEIMSKJX55ZPZ4MP4WL7HXYZQYT2UL4YUNEHX4EB7BQ".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/MZZ4WUZHXAEIMSKJX55ZPZ4MP4WL7HXYZQYT2UL4YUNEHX4EB7BQ".to_owned(),
        }, FundsActivityViewData {
            amount: "500".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "489".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "UGGBO4ORVXDDLOOUPOOB2UTKKMI3KEMCM33CL24BUJKLSLNIG4ZA".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/UGGBO4ORVXDDLOOUPOOB2UTKKMI3KEMCM33CL24BUJKLSLNIG4ZA".to_owned(),
        }, FundsActivityViewData {
            amount: "1111".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "1110".to_owned(),
            is_income: "false".to_owned(), 
            type_label: "Withdraw".to_owned(),
            description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "5SEN4SFFYTB5Z3IJYHSOQOTKAQPDRLVIQXMI5TYGL5GYY2ZVBXSA".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/5SEN4SFFYTB5Z3IJYHSOQOTKAQPDRLVIQXMI5TYGL5GYY2ZVBXSA".to_owned(),
        }, FundsActivityViewData {
            amount: "123".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "111".to_owned(),
            is_income: "false".to_owned(), 
            type_label: "Withdraw".to_owned(),
            description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "Y3SL4S6K5LKGTHI2QFVZTBAAW75FG3YF3HIPROKZRRF3FVF2RVFQ".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/Y3SL4S6K5LKGTHI2QFVZTBAAW75FG3YF3HIPROKZRRF3FVF2RVFQ".to_owned(),
        }, FundsActivityViewData {
            amount: "550.123".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "548.123".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "GFEXQC3GF7X7LRCURSHXXJPMMMO7MZYU55XAO67KQDYLNLBDZHWA".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/GFEXQC3GF7X7LRCURSHXXJPMMMO7MZYU55XAO67KQDYLNLBDZHWA".to_owned(),
        }, FundsActivityViewData {
            amount: "1212".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "1211".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "CFCHQBSHOPE6A5QTZ7KN3QYGME6WWIYJUBBNFCYE6DDYSJI4SD6A".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/CFCHQBSHOPE6A5QTZ7KN3QYGME6WWIYJUBBNFCYE6DDYSJI4SD6A".to_owned(),
        }, FundsActivityViewData {
            amount: "88.123137899".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "84.123137899".to_owned(),
            is_income: "true".to_owned(), 
            type_label: "Income".to_owned(),
            description: "".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "VPBTLKO2FNBVT5VCL7ST3VIIA3FYNGFLHYMOUOPFBHC4EQK6JCCA".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/VPBTLKO2FNBVT5VCL7ST3VIIA3FYNGFLHYMOUOPFBHC4EQK6JCCA".to_owned(),
        }, FundsActivityViewData {
            amount: "2999".to_owned(),
            short_amount: "".to_owned(),
            short_amount_without_fee: "".to_owned(),
            fee: "1.2".to_owned(),
            amount_without_fee: "111".to_owned(),
            is_income: "false".to_owned(), 
            type_label: "Withdraw".to_owned(),
            description: "Bought supplies and services, Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_owned(),
            date: "Wed, 20 Apr 2022".to_owned(),
            tx_id: "FYIRN74JXW54KHOMNRLM42JKAYVGUA33JKXCGPHQIFWVDBY5SAQA".to_string(), 
            address: shorten_address(&"7XSZQUQ2GJB25W37LVM5R4CMKKVC4VNSMIPCIWJYWM5ORA5VA4JRCNOJ4Y".parse().unwrap())?,
            tx_link: "https://testnet.algoexplorer.io/tx/FYIRN74JXW54KHOMNRLM42JKAYVGUA33JKXCGPHQIFWVDBY5SAQA".to_owned(),
        }];

        let truncated_raw_entries = if let Some(max_results) = pars.max_results {
            let max_results = max_results.parse()?;
            raw_entries.into_iter().take(max_results).collect()
        } else {
            raw_entries
        };

        let mut entries = vec![];

        // set short and readable amount
        for e in truncated_raw_entries {
            let amount = e.amount.parse()?;
            let readable_amount = format_decimal_readable(amount)?;
            let short_amount = format_short(amount)?;
            // log::debug!("{} -> {}", amount, short_amount);

            let amount_without_fee = e.amount_without_fee.parse()?;
            let readable_amount_without_fee = format_decimal_readable(amount_without_fee)?;
            let short_amount_without_fee = format_short(amount_without_fee)?;
            // log::debug!("{} -> {}", amount_without_fee, short_amount_without_fee);

            entries.push(FundsActivityViewData {
                // overwrite the dummy empty with short amounts (note that they can be equal to original, if no need to shorten)
                short_amount,
                short_amount_without_fee,

                // overwrite amount with readable amount
                amount: readable_amount,
                amount_without_fee: readable_amount_without_fee,

                ..e
            });
        }

        Ok(LoadFundsActivityResJs { entries })
    }
}
