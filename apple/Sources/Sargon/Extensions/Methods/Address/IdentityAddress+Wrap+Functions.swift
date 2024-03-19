extension IdentityAddress: EntityAddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newIdentityAddress(bech32: bech32String)
	}

	public init(publicKey: PublicKey, networkID: NetworkID) {
		self = newIdentityAddressFrom(
			publicKey: publicKey,
			networkId: networkID
		)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		identityAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		identityAddressNetworkId(address: self)
	}
}
