extension NonFungibleResourceAddress: @unchecked Sendable {}

#if DEBUG
extension NonFungibleResourceAddress {
	
	public static let sample = Self.sampleMainnet
	public static let sampleOther = Self.sampleMainnetOther
	
	public static let sampleMainnet: Self = newNonFungibleResourceAddressSampleMainnet()
	public static let sampleMainnetOther: Self = newNonFungibleResourceAddressSampleMainnetOther()
	public static let sampleStokenet: Self = newNonFungibleResourceAddressSampleStokenet()
	public static let sampleStokenetOther: Self = newNonFungibleResourceAddressSampleStokenetOther()
	
}
#endif

#if DEBUG
extension NonFungibleResourceAddress {
	public typealias AllCases = [Self]
	public static var allCases: AllCases {
		[
			Self.sampleMainnet,
			.sampleMainnetOther,
			.sampleStokenet,
			.sampleStokenetOther,
		]
	}
}
#endif
