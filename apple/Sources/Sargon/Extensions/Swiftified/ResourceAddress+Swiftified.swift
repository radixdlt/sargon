extension ResourceAddress: @unchecked Sendable {}

#if DEBUG
extension ResourceAddress {
	
	public static let sample = Self.sampleMainnetXRD
	public static let sampleOther = Self.sampleMainnetCandy
	
	public static let sampleMainnetXRD: Self = newResourceAddressSampleMainnetXrd()
	public static let sampleMainnetCandy: Self = newResourceAddressSampleMainnetCandy()
	
	/// Gumball Club membership NFT resource address
	public static let sampleMainnetNonFungbleGCMembership: Self = newResourceAddressSampleMainnetNftGcMembership()
	
	public static let sampleStokenetXRD: Self = newResourceAddressSampleStokenetXrd()
	public static let sampleStokenetGum: Self = newResourceAddressSampleStokenetGum()
	public static let sampleStokenetGC: Self = newResourceAddressSampleStokenetGcTokens()
	public static let sampleStokenetCandy: Self = newResourceAddressSampleStokenetCandy()
	
}
#endif

#if DEBUG
extension ResourceAddress {
	public typealias AllCases = [Self]
	public static var allCases: AllCases {
		return [
			Self.sampleMainnetXRD,
			.sampleMainnetCandy,
			.sampleMainnetNonFungbleGCMembership,
			.sampleStokenetXRD,
			.sampleStokenetGum,
			.sampleStokenetGC,
			.sampleStokenetCandy,
		]
	}
}
#endif
