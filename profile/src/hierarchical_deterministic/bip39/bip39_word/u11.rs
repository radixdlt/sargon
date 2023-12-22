use nutype::nutype;

#[nutype(
    validate(less = 2048),
    derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Display, Clone)
)]
pub struct U11(u16);

#[cfg(test)]
mod tests {
    use super::{U11Error, U11};

    #[test]
    fn invalid_2048() {
        assert_eq!(U11::new(2048), Err(U11Error::LessViolated));
    }

    #[test]
    fn valid_2047() {
        assert!(U11::new(2047).is_ok())
    }

    #[test]
    fn inner() {
        assert_eq!(U11::new(1024).unwrap().into_inner(), 1024);
    }

    #[test]
    fn ord() {
        assert!(U11::new(0).unwrap() < U11::new(1).unwrap());
        assert!(U11::new(2047).unwrap() > U11::new(2046).unwrap());
    }
}
