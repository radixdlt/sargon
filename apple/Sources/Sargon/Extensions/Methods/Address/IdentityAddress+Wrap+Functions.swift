import SargonUniFFI

extension IdentityAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newIdentityAddress(bech32: bech32String)
	}

	public init(publicKey: PublicKey, networkID: NetworkID) {
		self = newIdentityAddressFrom(
			publicKey: publicKey,
			networkId: networkID
		)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        identityAddressFormatted(address: self, format: format)
    }

	/// The bech32 encoded string for this address.
	public var address: String {
		identityAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		identityAddressNetworkId(address: self)
	}
}

#if DEBUG
extension IdentityAddress {
	public func embed() -> Address {
		.identity(self)
	}
	public func mapTo(networkID: NetworkID) -> Self {
		identityAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
