pub struct Paged<E> {
    endpoint: E,
    pagination: Pagination,
}
pub enum Pagination {
    All,
    Limit(usize),
}

pub trait Pageable {}

impl Default for Pagination {
    fn default() -> Self {
        Pagination::All
    }
}
