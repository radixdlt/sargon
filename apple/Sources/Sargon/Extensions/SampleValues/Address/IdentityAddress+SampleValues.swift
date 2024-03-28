import SargonUniFFI

#if DEBUG
extension IdentityAddress {
	
	public static let sample = Self.sampleMainnet
	public static let sampleOther = Self.sampleMainnetOther
	
	public static let sampleMainnet: Self = newIdentityAddressSampleMainnet()
	public static let sampleMainnetOther: Self = newIdentityAddressSampleMainnetOther()

	public static let sampleStokenet: Self = newIdentityAddressSampleStokenet()
	public static let sampleStokenetOther: Self = newIdentityAddressSampleStokenetOther()
}
#endif

