import SargonUniFFI

extension IdentityAddress: EntityAddressProtocol {
	public func embed() -> Address {
		.identity(self)
	}
}
