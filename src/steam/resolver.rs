//! Steam URL resolver (src/steam/resolver.rs in the original).
//!
//! Turns user input into (workshop_id, app_id):
//! - https://steamcommunity.com/sharedfiles/filedetails/?id=NNNN
//! - raw numeric workshop IDs
//! - collection URLs -> resolved via GetCollectionDetails/v1

/// Parse a workshop ID out of a URL or bare number.
pub fn parse_workshop_id(input: &str) -> Option<i64> {
    todo!("regex: [?&]id=(\\d+)  or  ^\\d+$")
}

/// A Steam collection and its member item IDs.
pub struct Collection {
    pub collection_id: i64,
    pub title: String,
    pub item_ids: Vec<i64>,
}

pub async fn resolve_collection(url: &str) -> anyhow::Result<Collection> {
    todo!("POST GetCollectionDetails/v1 with collectioncount=1")
}
