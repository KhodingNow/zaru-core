pub mod amount;
pub mod transaction;
pub mod wallet;

pub mod settlement;
pub mod ledger;
pub mod crypto;

pub mod prelude; 


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
