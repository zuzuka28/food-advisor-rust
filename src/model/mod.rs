pub(crate) mod category;
pub(crate) mod recipe;

pub use self::category::*;
pub use self::recipe::*;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SearchResult<T> {
    pub count: i64,
    pub items: Vec<T>,
}
