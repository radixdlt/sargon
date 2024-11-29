import SargonUniFFI

#if DEBUG
extension AuthorizedPersonaSimple {
	public static let sample = Self.sampleMainnet
	public static let sampleOther = Self.sampleMainnetOther

	public static let sampleMainnet: Self = newAuthorizedPersonaSimpleSampleMainnet()
	public static let sampleMainnetOther: Self = newAuthorizedPersonaSimpleSampleMainnetOther()

	public static let sampleStokenet: Self = newAuthorizedPersonaSimpleSampleStokenet()
	public static let sampleStokenetOther: Self = newAuthorizedPersonaSimpleSampleStokenetOther()
}
#endif // DEBUG
