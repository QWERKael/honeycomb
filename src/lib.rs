pub mod manage;
pub mod error;


pub use manage::manage::*;
pub use manage::communication;
pub use error::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
