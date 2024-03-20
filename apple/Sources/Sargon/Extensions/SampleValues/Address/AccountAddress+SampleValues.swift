#if DEBUG
extension AccountAddress {
	
	public static let sample = Self.sampleMainnet
	public static let sampleOther = Self.sampleMainnetOther
	
	public static let sampleMainnet: Self = newAccountAddressSampleMainnet()
	public static let sampleMainnetOther: Self = newAccountAddressSampleMainnetOther()
	public static let sampleStokenet: Self = newAccountAddressSampleStokenet()
	public static let sampleStokenetOther: Self = newAccountAddressSampleStokenetOther()
	
}
#endif
