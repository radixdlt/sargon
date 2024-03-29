extension Address: AddressProtocol {
	
	public func embed() -> Address {
		self
	}
	
	public func into<A: AddressProtocol>(type: A.Type = A.self) throws -> A {
		try A(validatingAddress: self.address)
	}
}

