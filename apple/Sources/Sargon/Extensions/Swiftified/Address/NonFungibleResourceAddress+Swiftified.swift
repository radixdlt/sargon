import SargonUniFFI

extension NonFungibleResourceAddress: AddressProtocol {}

extension NonFungibleResourceAddress {
	public func embed() -> Address {
		.resource(asResourceAddress)
	}
}

