import SargonUniFFI

#if DEBUG
extension Address {
	
	public static let sampleMainnet: Self = newAddressSampleMainnet()
	public static let sampleMainnetOther: Self = newAddressSampleMainnetOther()
	public static let sampleStokenet: Self = newAddressSampleStokenet()
	public static let sampleStokenetOther: Self = newAddressSampleStokenetOther()
	
	public typealias AllCases = [Self]
	public static var allCases: AllCases {
		addressSampleValuesAll()
	}
}
#endif // DEBUG
