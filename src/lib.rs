static FNV_OFFSET_BASIS_32: u32 = 0x811c9dc5;
static FNV_PRIME_32: u32 = 0x01000193;

static FNV_OFFSET_BASIS_64: u64 = 0xcbf29ce484222325;
static FNV_PRIME_64: u64 = 0x100000001b3;

pub fn fnv1a32(bytes: &[u8]) -> u32 {
    let mut hash = FNV_OFFSET_BASIS_32;

    for byte in bytes.iter() {
        hash = hash ^ (*byte as u32);
        hash = hash.wrapping_mul(FNV_PRIME_32);
    }

    hash
}

pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = FNV_OFFSET_BASIS_64;

    for byte in bytes.iter() {
        hash = hash ^ (*byte as u64);
        hash = hash.wrapping_mul(FNV_PRIME_64);
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnv1a32() {
        let result = fnv1a32("test".as_bytes());
        assert_eq!(result, 0xafd071e5);
    }

    #[test]
    fn test_fnv1a64() {
        let result = fnv1a64("test".as_bytes());
        assert_eq!(result, 0xf9e6e6ef197c2b25);
    }
}
