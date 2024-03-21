public protocol EntityProtocol: SargonModel, CustomStringConvertible, Identifiable, CaseIterable where Self.AllCases == [Self], ID == Address {
	associatedtype Address: EntityAddressProtocol
	var networkId: NetworkID { get }
	var displayName: DisplayName { get }
	var address: Address { get }
	var flags: [EntityFlag] { get }
	var securityState: EntitySecurityState { get }
	
#if DEBUG
	static var sampleMainnet: Self { get }
	static var sampleMainnetOther: Self { get }
	static var sampleMainnetThird: Self { get }
	static var sampleStokenet: Self { get }
	static var sampleStokenetOther: Self { get }
	static var sampleStokenetThird: Self { get }
#endif // DEBUG
}

extension EntityProtocol {
	public var id: ID { address }
	public var networkID: NetworkID { networkId }
}

extension EntityProtocol {
	public var description: String {
		"\(displayName): \(address) @\(networkID)"
	}
}

#if DEBUG
extension EntityProtocol {
	public static var sample: Self { Self.sampleMainnet }
	public static var sampleOther: Self { Self.sampleMainnetOther }
}
#endif // DEBUG

#if DEBUG
extension EntityProtocol {
	public static var sampleValuesMainnet: [Self] {
		[
			Self.sampleMainnet,
			Self.sampleMainnetOther,
			Self.sampleMainnetThird,
		]
	}
	public static var sampleValuesStokenet: [Self] {
		[
			Self.sampleStokenet,
			Self.sampleStokenetOther,
			Self.sampleStokenetThird,
		]
	}
}
extension EntityProtocol where Self: CaseIterable, AllCases == [Self] {
	public static var allCases: AllCases {
		Self.sampleValuesMainnet + Self.sampleValuesStokenet
	}
}
#endif // DEBUG

