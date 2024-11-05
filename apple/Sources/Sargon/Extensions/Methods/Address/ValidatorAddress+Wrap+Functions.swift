import SargonUniFFI

extension ValidatorAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newValidatorAddress(bech32: bech32String)
	}
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        self.formatted(format)
    }


	public var networkID: NetworkId {
        self.networkId
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
