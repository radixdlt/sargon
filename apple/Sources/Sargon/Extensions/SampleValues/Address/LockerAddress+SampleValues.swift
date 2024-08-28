import SargonUniFFI

#if DEBUG
extension LockerAddress {
	
	public static let sample = Self.sampleMainnet
	public static let sampleOther = Self.sampleMainnetOther
	
	public static let sampleMainnet: Self = newLockerAddressSampleMainnet()
	public static let sampleMainnetOther: Self = newLockerAddressSampleMainnetOther()

	public static let sampleStokenet: Self = newLockerAddressSampleStokenet()
	public static let sampleStokenetOther: Self = newLockerAddressSampleStokenetOther()
}
#endif

