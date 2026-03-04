use std::fmt;

use serde::{Deserialize, de};

use super::api_response::PaginationMeta;

const DEFAULT_PAGE: usize = 1;
const DEFAULT_LIMIT: usize = 10;
const MAX_LIMIT: usize = 50;

fn default_page() -> usize {
    DEFAULT_PAGE
}

fn default_limit() -> usize {
    DEFAULT_LIMIT
}

fn deserialize_usize_from_string<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct UsizeOrString;

    impl<'de> de::Visitor<'de> for UsizeOrString {
        type Value = usize;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a usize or a string containing a usize")
        }

        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v as usize)
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            v.parse().map_err(de::Error::custom)
        }
    }

    deserializer.deserialize_any(UsizeOrString)
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub struct PaginationParams {
    #[serde(default = "default_page", deserialize_with = "deserialize_usize_from_string")]
    pub page: usize,
    #[serde(default = "default_limit", deserialize_with = "deserialize_usize_from_string")]
    pub limit: usize,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: DEFAULT_PAGE,
            limit: DEFAULT_LIMIT,
        }
    }
}

impl PaginationParams {
    pub fn normalized(self) -> Self {
        let page = if self.page == 0 {
            DEFAULT_PAGE
        } else {
            self.page
        };
        let limit = self.limit.max(1).min(MAX_LIMIT);

        Self { page, limit }
    }

    fn offset(&self) -> usize {
        (self.page - 1) * self.limit
    }

    pub fn apply<T>(self, items: Vec<T>) -> Vec<T> {
        let normalized = self.normalized();
        let offset = normalized.offset();

        if items.is_empty() || offset >= items.len() {
            return Vec::new();
        }

        items
            .into_iter()
            .skip(offset)
            .take(normalized.limit)
            .collect()
    }

    pub fn meta(&self, total: usize) -> PaginationMeta {
        let normalized = self.normalized();

        let total_pages = if total == 0 {
            0
        } else {
            ((total as f64) / (normalized.limit as f64)).ceil() as usize
        };

        PaginationMeta {
            total,
            page: normalized.page,
            limit: normalized.limit,
            total_pages,
            has_next_page: total_pages > 0 && normalized.page < total_pages,
            has_previous_page: total_pages > 0 && normalized.page > 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_zero_page_and_large_limit() {
        let params = PaginationParams {
            page: 0,
            limit: 500,
        };
        let normalized = params.normalized();
        assert_eq!(normalized.page, 1);
        assert_eq!(normalized.limit, MAX_LIMIT);
    }

    #[test]
    fn apply_returns_expected_slice() {
        let params = PaginationParams { page: 2, limit: 3 };
        let items = vec![1, 2, 3, 4, 5, 6, 7];
        let result = params.apply(items);
        assert_eq!(result, vec![4, 5, 6]);
    }

    #[test]
    fn meta_calculates_flags_correctly() {
        let params = PaginationParams { page: 2, limit: 5 };
        let meta = params.meta(17);
        assert_eq!(meta.total, 17);
        assert_eq!(meta.total_pages, 4);
        assert!(meta.has_next_page);
        assert!(meta.has_previous_page);
    }
}
