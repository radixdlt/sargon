import SargonUniFFI

#if DEBUG
extension Account {
	public static let sampleMainnetAlice: Self = newAccountSampleMainnetAlice()
	public static let sampleMainnetBob: Self = newAccountSampleMainnetBob()
	public static let sampleMainnetCarol: Self = newAccountSampleMainnetCarol()
	public static let sampleMainnetDiana: Self = newAccountSampleMainnetDiana()

	public static let sampleStokenetNadia: Self = newAccountSampleStokenetNadia()
	public static let sampleStokenetOlivia: Self = newAccountSampleStokenetOlivia()
	public static let sampleStokenetPaige: Self = newAccountSampleStokenetPaige()

	public static let sampleMainnet = Self.sampleMainnetAlice
	public static let sampleMainnetOther = Self.sampleMainnetBob
	public static let sampleMainnetThird = Self.sampleMainnetCarol
	public static let sampleMainnetFourth = Self.sampleMainnetDiana

	public static let sampleStokenet = Self.sampleStokenetNadia
	public static let sampleStokenetOther = Self.sampleStokenetOlivia
	public static let sampleStokenetThird = Self.sampleStokenetPaige
}
#endif
