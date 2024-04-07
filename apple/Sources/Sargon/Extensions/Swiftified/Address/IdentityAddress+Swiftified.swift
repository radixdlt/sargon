import SargonUniFFI

extension IdentityAddress: EntityAddressProtocol {
	public var asGeneral: Address {
		.identity(self)
	}
}
