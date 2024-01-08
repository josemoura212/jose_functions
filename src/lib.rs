pub fn somar(left: usize, right: usize) -> usize {
    left + right
}
pub fn subtrair(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = somar(2, 2);
        assert_eq!(result, 4);
    }
}
