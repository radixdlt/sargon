extension AccessControllerAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newAccessControllerAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		accessControllerAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		accessControllerAddressNetworkId(address: self)
	}
}
