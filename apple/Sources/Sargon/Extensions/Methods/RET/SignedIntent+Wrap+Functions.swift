import SargonUniFFI

extension SignedIntent {
	public func hash() -> SignedTransactionIntentHash {
		signedIntentHash(signedIntent: self)
	}
}
