public protocol AddressProtocol: SargonModel, CustomStringConvertible, CaseIterable where Self.AllCases == [Self] {
	init(validatingAddress bech32String: String) throws
	var networkID: NetworkID { get }
	var address: String { get }
	
#if DEBUG
	static var sampleMainnet: Self { get }
	static var sampleMainnetOther: Self { get }
	static var sampleStokenet: Self { get }
	static var sampleStokenetOther: Self { get }
#endif // DEBUG
}

#if DEBUG
extension AddressProtocol {
	public static var sample: Self { Self.sampleMainnet }
	public static var sampleOther: Self { Self.sampleMainnetOther }
}
#endif // DEBUG

#if DEBUG
extension AddressProtocol where Self: CaseIterable, AllCases == [Self] {
	public static var allCases: AllCases {
		[
			Self.sampleMainnet,
			Self.sampleMainnetOther,
			Self.sampleStokenet,
			Self.sampleStokenetOther
		]
		
	}
}
#endif // DEBUG


extension AddressProtocol {
	public var description: String {
		address
	}
}
