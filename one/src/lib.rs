pub fn add(a: u32, b: u32) -> u64 {
    a as u64 + b as u64
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
