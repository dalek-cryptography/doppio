pub type Ristretto255Scalar = curve25519_dalek::scalar::Scalar;

mod edwards;
mod field;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
