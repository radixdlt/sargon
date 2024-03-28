import SargonUniFFI

#if DEBUG
extension VaultAddress {
	
	public static let sampleMainnet = Self.sampleMainnetFungible
	public static let sampleMainnetOther = Self.sampleMainnetNonFungible
	
	public static let sampleStokenet = Self.sampleStokenetFungible
	public static let sampleStokenetOther = Self.sampleStokenetNonFungible
	
	public static let sampleMainnetFungible: Self = newVaultAddressSampleMainnetFungible()
	public static let sampleMainnetNonFungible: Self = newVaultAddressSampleMainnetNonFungible()
	public static let sampleStokenetFungible: Self = newVaultAddressSampleStokenetFungible()
	public static let sampleStokenetNonFungible: Self = newVaultAddressSampleStokenetNonFungible()
}
#endif
