use rand;
pub fn add_one_fn(x:i32) -> i32{
    x+1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one_fn(2));
    }
}