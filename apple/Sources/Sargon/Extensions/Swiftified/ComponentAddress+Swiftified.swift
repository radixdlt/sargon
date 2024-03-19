extension ComponentAddress: @unchecked Sendable {}

#if DEBUG
extension ComponentAddress {
	public static let sample: Self = newComponentAddressSample()
	public static let sampleOther: Self = newComponentAddressSampleOther()
}
#endif
