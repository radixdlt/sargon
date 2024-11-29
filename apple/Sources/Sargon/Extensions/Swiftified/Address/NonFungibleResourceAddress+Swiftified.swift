import SargonUniFFI

// MARK: - NonFungibleResourceAddress + AddressProtocol
extension NonFungibleResourceAddress: AddressProtocol {}

extension NonFungibleResourceAddress {
	public var asGeneral: Address {
		.resource(asResourceAddress)
	}
}
