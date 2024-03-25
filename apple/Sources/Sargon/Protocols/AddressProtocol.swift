#if DEBUG
public protocol BaseBaseAddressProtocol: SargonModel, ExpressibleByStringLiteral {}
#else
public protocol BaseBaseAddressProtocol: SargonModel {}
#endif // DEBUG

public protocol BaseAddressProtocol: BaseBaseAddressProtocol, CustomStringConvertible, CaseIterable where Self.AllCases == [Self] {
	init(validatingAddress bech32String: String) throws
	var networkID: NetworkID { get }
	var address: String { get }
}

extension BaseAddressProtocol {
	public var description: String {
		address
	}
}

#if DEBUG
extension BaseAddressProtocol {
	public init(stringLiteral value: String) {
		self = try! Self(validatingAddress: value)
	}
}
#endif // DEBUG

public protocol AddressProtocol: BaseAddressProtocol {
	
#if DEBUG
	func embed() -> Address
	func mapTo(networkID: NetworkID) -> Self
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
	
	/// Returns the`ResourceAddress` of `XRD` on the same network
	/// as this address.
	public var xrd: ResourceAddress {
		ResourceAddress.xrd(on: networkID)
	}
}
