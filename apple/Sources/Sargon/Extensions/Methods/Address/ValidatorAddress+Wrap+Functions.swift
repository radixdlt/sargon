import SargonUniFFI

extension ValidatorAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newValidatorAddress(bech32: bech32String)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        validatorAddressFormatted(address: self, format: format)
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
	
	public static func random(networkID: NetworkID) -> Self {
		newValidatorAddressRandom(networkId: networkID)
	}
	
	public func mapTo(networkID: NetworkID) -> Self {
		validatorAddressMapToNetwork(address: self, networkId: networkID)
	}
}
#endif // DEBUG
