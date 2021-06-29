use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    Order,
    Title,
    Created,
    Updated,
}

impl Default for SortBy {
    fn default() -> SortBy {
        SortBy::Order
    }
}

// Comment this out
impl From<&str> for SortBy {
    fn from(mode: &str) -> Self {
        match mode {
            "order" => Self::Order,
            "title" => Self::Title,
            "created" => Self::Created,
            "updated" => Self::Updated,
            _ => Self::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let x: SortBy = dbg!(serde_json::from_str("\"order\"")).unwrap();
        assert_eq!(x, SortBy::Order);
    }
}
