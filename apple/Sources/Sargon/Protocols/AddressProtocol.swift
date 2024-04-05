import SargonUniFFI

#if DEBUG
public protocol BaseBaseAddressProtocol: SargonModel, ExpressibleByStringLiteral {}
#else
public protocol BaseBaseAddressProtocol: SargonModel {}
#endif // DEBUG

public protocol BaseAddressProtocol: BaseBaseAddressProtocol, Codable, CustomStringConvertible, CaseIterable where Self.AllCases == [Self] {
	init(validatingAddress bech32String: String) throws
	var networkID: NetworkID { get }
	var address: String { get }
}

extension AddressProtocol {
	public var isOnMainnet: Bool {
		self.networkID == .mainnet
	}
}


extension BaseAddressProtocol {
	public var description: String {
		address
	}
}

extension BaseAddressProtocol where Self: Codable {
	public func encode(to encoder: Encoder) throws {
		var container = encoder.singleValueContainer()
		try container.encode(self.address)
	}

	public init(from decoder: Decoder) throws {
		let container = try decoder.singleValueContainer()
		let string = try container.decode(String.self)
		try self.init(validatingAddress: string)
	}
}

#if DEBUG
extension BaseAddressProtocol {
	public init(stringLiteral value: String) {
		self = try! Self(validatingAddress: value)
	}
}
#endif // DEBUG

public protocol AddressProtocol: BaseAddressProtocol & Identifiable where Self.ID == String {
    func formatted(_ format: AddressFormat) -> String
    var asGeneral: Address { get }
#if DEBUG
	static func random(networkID: NetworkID) -> Self
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
    
    public var id: ID {
        address
    }
	
	/// Returns the`ResourceAddress` of `XRD` on the same network
	/// as this address.
	public var xrdOnSameNetwork: ResourceAddress {
		ResourceAddress.xrd(on: networkID)
	}
}
