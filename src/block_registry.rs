use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BlockInfo {
    pub name: String,
    pub front: String,
    pub back: String,
    pub left: String,
    pub right: String,
    pub top: String,
    pub bottom: String,
}

#[derive(Deserialize, Debug)]
pub struct BlockInfoRegistry {
    pub blocks: Vec<BlockInfo>,
}
