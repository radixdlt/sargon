import SargonUniFFI

#if DEBUG
extension ValidatorAddress {
	public static let sample = Self.sampleMainnet
	public static let sampleOther = Self.sampleMainnetOther
	
	public static let sampleMainnet: Self = newValidatorAddressSampleMainnet()
	public static let sampleMainnetOther: Self = newValidatorAddressSampleMainnetOther()
	public static let sampleStokenet: Self = newValidatorAddressSampleStokenet()
	public static let sampleStokenetOther: Self = newValidatorAddressSampleStokenetOther()
}
#endif

