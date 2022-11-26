use crate::{
    error::FrError,
    provider::roadmap_provider::{
        GetRoadmapParJs, GetRoadmapResJs, RoadmapItemJs, RoadmapProvider,
    },
};
use anyhow::Result;
use async_trait::async_trait;

use super::req_delay;

pub struct RoadmapProviderMock {}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl RoadmapProvider for RoadmapProviderMock {
    async fn get(&self, _: GetRoadmapParJs) -> Result<GetRoadmapResJs, FrError> {
        req_delay().await;

        Ok(GetRoadmapResJs {
            items: vec![
                mock_header("Q1 2023"),
                mock_item("Lorem ipsum dolor sit amet, consectetur adipiscing elit"),
                mock_item("Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua"),
                mock_item("Some roadmap item descr"),
                mock_header("Q2 2023"),
                mock_item("Some roadmap item descr"),
                mock_item("Lorem ipsum dolor sit amet, consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua sed do eiusmod tempor incididunt ut labore et dolore magna aliqua sed do eiusmod tempor incididunt ut labore et dolore magna aliqua"),
                mock_header("Q3 2023"),
                mock_item("Some roadmap item descr"),
                mock_item("Some roadmap item descr"),
                mock_item("Lorem ipsum dolor sit amet, consectetur adipiscing elit"),
                mock_item("Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua"),
                mock_item("Some roadmap item descr"),
                mock_header("Q4 2023"),
                mock_item("Some roadmap item descr"),
                mock_item("Lorem ipsum dolor sit amet, consectetur adipiscing elit"),
                mock_item("Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua"),
                mock_item("Lorem ipsum dolor sit amet, consectetur adipiscing elit"),
                mock_item("Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua"),
                mock_item("Lorem ipsum dolor sit amet, consectetur adipiscing elit"),
                mock_item("Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua"),
                mock_header("Q1 2023"),
                mock_item("Some roadmap item descr"),
                mock_item("Some roadmap item descr"),
                mock_item("Some roadmap item descr"),
                mock_item("Some roadmap item descr"),
                mock_item("Some roadmap item descr"),
                mock_item("Some roadmap item descr"),
                mock_item("Some roadmap item descr"),
                mock_item("Some roadmap item descr"),
                mock_item("Some roadmap item descr"),
                mock_header("Q2 2023"),
                mock_item("Some roadmap item descr"),
                mock_header("Q3 2023"),
                mock_item("Some roadmap item descr"),
                mock_header("Q4 2023"),
                mock_item("Some roadmap item descr"),
                mock_item("Lorem ipsum dolor sit amet, consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua sed do eiusmod tempor incididunt ut labore et dolore magna aliqua sed do eiusmod tempor incididunt ut labore et dolore magna aliqua"),
                mock_item("Some roadmap item descr"),
                mock_item("Some roadmap item descr"),
                ],
        })
    }
}

fn mock_header(text: &str) -> RoadmapItemJs {
    RoadmapItemJs {
        item_type: "header".to_owned(),
        tx_id: None,
        tx_link: None,
        date: None,
        text: text.to_owned(),
    }
}

fn mock_item(text: &str) -> RoadmapItemJs {
    RoadmapItemJs {
        item_type: "item".to_owned(),
        tx_id: Some("5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned()),
        tx_link: Some("https://testnet.algoexplorer.io/tx/5ZKN6IXOQOVHWCN2EGKIL3Z4SSW6ZEBPJXDZDLXESCAXMYFBPTZA".to_owned()),
        date: Some("Wed, 20 Apr 2022 12:01:00 +0000".to_owned()),
        text: text.to_owned(),
    }
}
