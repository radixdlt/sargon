extension TransactionIntent {
	public func hash() -> IntentHash {
		transactionIntentHash(intent: self)
	}
	
	public func compile() -> Data {
		transactionIntentCompile(intent: self)
	}
}
