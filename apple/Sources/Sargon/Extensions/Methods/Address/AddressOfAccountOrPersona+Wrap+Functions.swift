
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

