use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct BlockInfo {
    name: String,
    front: String,
    back: String,
    left: String,
    right: String,
    top: String,
    bottom: String,
}

#[derive(Deserialize, Debug)]
pub struct BlockInfoRegistry {
    pub blocks: Vec<BlockInfo>,
}
