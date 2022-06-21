pub fn sub(a: u32, b: u32) -> u32 {
    a-b
}

// Units tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
