pub const U32_MAX: u32 = u32::MAX;
pub const U31_MAX: u32 = 2u32.pow(31) - 1;
pub const U30_MAX: u32 = 2u32.pow(30) - 1;

/// 2^31 (0x80000000)
pub const GLOBAL_OFFSET_HARDENED: u32 = 2u32.pow(31);

/// 2^30
///
/// Does NOT also offset by `GLOBAL_OFFSET_HARDENED`
/// `RELATIVELY_LOCAL` meaning it is not global, but it is not local either,
/// meaning the offset is locally relative to `GLOBAL_OFFSET_HARDENED`
pub const RELATIVELY_LOCAL_OFFSET_SECURIFIED: u32 = 2u32.pow(30);

/// 2^31 + 2^30
pub const GLOBAL_OFFSET_HARDENED_SECURIFIED: u32 =
    GLOBAL_OFFSET_HARDENED + RELATIVELY_LOCAL_OFFSET_SECURIFIED;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u32_max() {
        assert_eq!(U32_MAX as u64, 2u64.pow(32) - 1);
    }

    #[test]
    fn u31_max() {
        assert_eq!(U31_MAX, u32::MAX / 2);
    }

    #[test]
    fn u30_max() {
        assert_eq!(U30_MAX, U31_MAX / 2);
        assert_eq!(U30_MAX, u32::MAX / 4);
    }

    #[test]
    fn offsets() {
        assert_eq!(2u32.pow(31), 0x80000000);
        assert_eq!(GLOBAL_OFFSET_HARDENED, 0x80000000);
        assert_eq!(2u32.pow(30), 0x40000000);
        assert_eq!(RELATIVELY_LOCAL_OFFSET_SECURIFIED, 0x40000000);
        assert_eq!(GLOBAL_OFFSET_HARDENED, 2u32.pow(31));
        assert_eq!(RELATIVELY_LOCAL_OFFSET_SECURIFIED, 2u32.pow(30));
        assert_eq!(
            GLOBAL_OFFSET_HARDENED_SECURIFIED,
            2u32.pow(31) + 2u32.pow(30)
        );
        assert_eq!(
            GLOBAL_OFFSET_HARDENED_SECURIFIED,
            GLOBAL_OFFSET_HARDENED + RELATIVELY_LOCAL_OFFSET_SECURIFIED
        );
    }

    #[test]
    fn max_values() {
        assert_eq!(U32_MAX, 0xFFFFFFFF);
        assert_eq!(U31_MAX, 0x7FFFFFFF);
        assert_eq!(U31_MAX, 2_147_483_647);
        assert_eq!(U30_MAX, 0x3FFFFFFF);
        assert_eq!(U30_MAX, 1073741823);
    }

    #[test]
    fn local_offset_securified() {
        assert_eq!(RELATIVELY_LOCAL_OFFSET_SECURIFIED, 0x40000000);
        assert_eq!(RELATIVELY_LOCAL_OFFSET_SECURIFIED - 1, U30_MAX);
    }

    #[test]
    fn test_securified() {
        assert_eq!(
            GLOBAL_OFFSET_HARDENED_SECURIFIED,
            GLOBAL_OFFSET_HARDENED + RELATIVELY_LOCAL_OFFSET_SECURIFIED
        );
    }
}
