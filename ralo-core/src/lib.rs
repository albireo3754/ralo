pub mod entity;
pub mod log;
pub mod service;
pub mod utils;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
