import SargonUniFFI

#if DEBUG
extension TransactionIntent {
	public static let sample: Self = newTransactionIntentSample()
	public static let sampleOther: Self = newTransactionIntentSampleOther()
}
#endif // DEBUG
