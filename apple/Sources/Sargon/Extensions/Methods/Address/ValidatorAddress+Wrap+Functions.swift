extension ValidatorAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newValidatorAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		validatorAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		validatorAddressNetworkId(address: self)
	}
}

#if DEBUG
extension ValidatorAddress {
	public func embed() -> Address {
		.validator(self)
	}
	public func mapTo(networkID: NetworkID) -> Self {
		validatorAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
