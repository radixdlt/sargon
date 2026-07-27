import SargonUniFFI

// MARK: - Address + AddressProtocol
extension Address: AddressProtocol {
	public var asGeneral: Address {
		self
	}

	public func asSpecific<A: AddressProtocol>(type: A.Type = A.self) throws -> A {
		try A(validatingAddress: self.address)
	}

	#if DEBUG
	public static func random(networkID: NetworkID) -> Self {
		account(.random(networkID: networkID))
	}
	#endif // DEBUG
}

public func == (
	lhs: some AddressProtocol,
	rhs: some AddressProtocol
) -> Bool {
	lhs.address == rhs.address
}
