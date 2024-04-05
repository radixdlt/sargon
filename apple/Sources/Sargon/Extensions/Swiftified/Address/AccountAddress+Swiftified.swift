import SargonUniFFI

extension AccountAddress: EntityAddressProtocol {
	public var asGeneral: Address {
		.account(self)
	}
}
