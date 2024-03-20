#if DEBUG
extension PoolAddress {
	public static let sampleMainnet = Self.sampleMainnetTwo
	public static let sampleMainnetOther = Self.sampleMainnetSingle
	
	public static let sampleStokenet = Self.sampleStokenetTwo
	public static let sampleStokenetOther = Self.sampleStokenetSingle
	
	public static let sampleMainnetSingle: Self = newPoolAddressSampleMainnetSingle()
	public static let sampleMainnetTwo: Self = newPoolAddressSampleMainnetTwo()
	public static let sampleMainnetMulti: Self = newPoolAddressSampleMainnetMulti()
	
	public static let sampleStokenetSingle: Self = newPoolAddressSampleStokenetSingle()
	public static let sampleStokenetTwo: Self = newPoolAddressSampleStokenetTwo()
	public static let sampleStokenetMulti: Self = newPoolAddressSampleStokenetMulti()
}
#endif

#if DEBUG
extension PoolAddress {
	public typealias AllCases = [Self]
	public static var allCases: AllCases {
		[
			Self.sampleMainnetSingle,
			.sampleMainnetTwo,
			.sampleMainnetMulti,
			.sampleStokenetSingle,
			.sampleStokenetTwo,
			.sampleStokenetMulti
		]
	}
}
#endif
