pub mod complex_type;
pub mod element;
pub mod errors;
pub mod import;
pub mod include;
pub mod parser;
pub mod restriction;
pub mod schema;
pub mod shared;
pub mod simple_type;
mod traits;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
