extension PoolAddress: @unchecked Sendable {}

#if DEBUG
extension PoolAddress {
	public static let sample = Self.sampleMainnetTwo
	public static let sampleOther = Self.sampleMainnetSingle
	
	public static let sampleMainnetSingle: Self = newPoolAddressSampleSingle()
	public static let sampleMainnetTwo: Self = newPoolAddressSampleTwo()
	public static let sampleMainnetMulti: Self = newPoolAddressSampleMulti()
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
		]
	}
}
#endif
