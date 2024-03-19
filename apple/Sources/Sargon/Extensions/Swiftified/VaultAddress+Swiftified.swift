extension VaultAddress: @unchecked Sendable {}

#if DEBUG
extension VaultAddress {
	
	public static let sample = Self.sampleMainnetFungible
	public static let sampleOther = Self.sampleMainnetNonFungible
	
	public static let sampleMainnetFungible: Self = newVaultAddressSampleMainnetFungible()
	public static let sampleMainnetNonFungible: Self = newVaultAddressSampleMainnetNonFungible()
	public static let sampleStokenetFungible: Self = newVaultAddressSampleStokenetFungible()
	public static let sampleStokenetNonFungible: Self = newVaultAddressSampleStokenetNonFungible()
}
#endif

#if DEBUG
extension VaultAddress {
	public typealias AllCases = [Self]
	public static var allCases: AllCases {
		return [
			Self.sampleMainnetFungible,
			.sampleMainnetNonFungible,
			.sampleStokenetFungible,
			.sampleStokenetNonFungible,
		]
	}
}
#endif
