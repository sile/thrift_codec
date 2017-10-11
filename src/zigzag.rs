pub fn from_i32(n: i32) -> u32 {
    ((n << 1) ^ (n >> 31)) as u32
}

pub fn from_i64(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

pub fn to_i32(n: u32) -> i32 {
    (n >> 1) as i32 ^ -(n as i32 & 1)
}

pub fn to_i64(n: u64) -> i64 {
    (n >> 1) as i64 ^ -(n as i64 & 1)
}

#[cfg(test)]
mod test {
    use zigzag;

    #[test]
    fn zigzag_works() {
        assert_eq!(zigzag::from_i32(0), 0);
        assert_eq!(zigzag::from_i32(-1), 1);
        assert_eq!(zigzag::from_i32(1), 2);
        assert_eq!(zigzag::from_i32(-2), 3);
        assert_eq!(zigzag::from_i32(2), 4);

        assert_eq!(zigzag::to_i32(0), 0);
        assert_eq!(zigzag::to_i32(1), -1);
        assert_eq!(zigzag::to_i32(2), 1);
        assert_eq!(zigzag::to_i32(3), -2);
        assert_eq!(zigzag::to_i32(4), 2);
    }
}
