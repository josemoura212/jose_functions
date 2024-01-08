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
    fn it_works_somar() {
        let result = somar(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_subtrair() {
        let result = subtrair(2, 2);
        assert_eq!(result, 0);
    }
}
