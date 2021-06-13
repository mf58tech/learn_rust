#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let x = 1;
        assert_eq!(x, 1);
        {
            let x = x+1;
            assert_eq!(x, 2);
        }
        assert_eq!(x, 1);
    }
}
