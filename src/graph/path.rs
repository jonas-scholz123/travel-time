#[derive(Debug)]
pub struct Path<'a, T> {
    pub score: u16,
    pub val: T,
    pub path: Vec<&'a T>,
}
