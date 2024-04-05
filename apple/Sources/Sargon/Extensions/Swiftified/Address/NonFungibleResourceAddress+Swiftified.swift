import SargonUniFFI

extension NonFungibleResourceAddress: AddressProtocol {}

extension NonFungibleResourceAddress {
	public var asGeneral: Address {
		.resource(asResourceAddress)
	}
}

