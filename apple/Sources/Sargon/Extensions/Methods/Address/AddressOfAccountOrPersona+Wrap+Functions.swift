
extension AddressOfAccountOrPersona {
	public init(validatingAddress bech32String: String) throws {
		self = try newAddressOfAccountOrPersonaFromBech32(string: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		addressOfAccountOrPersonaToString(address: self)
	}

	public var networkID: NetworkId {
		addressOfAccountOrPersonaNetworkId(address: self)
	}
}

#if DEBUG
extension AddressOfAccountOrPersona {
	public func embed() -> Address {
		switch self {
		case let .account(accountAddress): return Address.account(accountAddress)
		case let .persona(identityAddress): return Address.identity(identityAddress)
		}
	}
	public func mapTo(networkID: NetworkID) -> Self {
		addressOfAccountOrPersonaMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
