extension NonFungibleResourceAddress {
	public init(validatingAddress bech32String: String) throws {
		self = try newNonFungibleResourceAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		nonFungibleResourceAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		nonFungibleResourceAddressNetworkId(address: self)
	}
    
    public var asResourceAddress: ResourceAddress {
        nonFungibleResourceAddressAsResourceAddress(address: self)
    }
    
    public func formatted(_ format: AddressFormat = .default) -> String {
        asResourceAddress.formatted(format)
    }
}

#if DEBUG
extension NonFungibleResourceAddress {
	
	public func embed() -> Address {
		.nonFungibleResource(self)
	}
	
	public func mapTo(networkID: NetworkID) -> Self {
		nonFungibleResourceAddressMapToNetwork(address: self, networkId: networkID)
	}
	
}
#endif // DEBUG
