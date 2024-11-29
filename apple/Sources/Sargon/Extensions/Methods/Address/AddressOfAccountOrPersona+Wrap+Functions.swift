import SargonUniFFI

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

	public func formatted(_ format: AddressFormat) -> String {
		addressOfAccountOrPersonaFormatted(address: self, format: format)
	}
}

#if DEBUG
extension AddressOfAccountOrPersona {
	public static func random(networkID: NetworkID) -> Self {
		.account(newAccountAddressRandom(networkId: networkID))
	}

	public func mapTo(networkID: NetworkID) -> Self {
		addressOfAccountOrPersonaMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
