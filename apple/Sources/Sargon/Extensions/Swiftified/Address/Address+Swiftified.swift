import SargonUniFFI

extension Address: AddressProtocol {
	
	public var asGeneral: Address {
		self
	}
	
	public func asSpecific<A: AddressProtocol>(type: A.Type = A.self) throws -> A {
		try A(validatingAddress: self.address)
	}
	
	public static func random(networkID: NetworkID) -> Self {
		Self.account(.random(networkID: networkID))
	}
}

public func == (lhs: Address, rhs: some AddressProtocol) -> Bool {
	lhs == rhs.asGeneral
}

public func == (lhs: some AddressProtocol, rhs: Address) -> Bool {
	rhs == lhs
}
