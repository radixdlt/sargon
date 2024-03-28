import SargonUniFFI

#if DEBUG
extension ResourceAddress {
	
	public static let sampleMainnet = Self.sampleMainnetXRD
	public static let sampleMainnetOther = Self.sampleMainnetCandy
	public static let sampleStokenet = Self.sampleStokenetXRD
	public static let sampleStokenetOther = Self.sampleStokenetGum
	
	public static let sampleMainnetXRD: Self = newResourceAddressSampleMainnetXrd()
	public static let sampleMainnetCandy: Self = newResourceAddressSampleMainnetCandy()
	
	/// Gumball Club membership NFT resource address
	public static let sampleMainnetNonFungibleGCMembership: Self = newResourceAddressSampleMainnetNftGcMembership()
	
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
			.sampleMainnetNonFungibleGCMembership,
			.sampleStokenetXRD,
			.sampleStokenetGum,
			.sampleStokenetGC,
			.sampleStokenetCandy,
		]
	}
}
#endif
