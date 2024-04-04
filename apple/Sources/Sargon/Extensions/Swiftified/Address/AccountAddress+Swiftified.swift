import SargonUniFFI

extension AccountAddress: EntityAddressProtocol {
	public func embed() -> Address {
		.account(self)
	}
}
