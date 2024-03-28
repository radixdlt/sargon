import SargonUniFFI

extension SignedIntent {
    public func hash() -> SignedIntentHash {
        signedIntentHash(signedIntent: self)
    }
}
