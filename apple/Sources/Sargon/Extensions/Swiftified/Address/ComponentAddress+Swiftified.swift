import SargonUniFFI

extension ComponentAddress: AddressProtocol {
	public func embed() -> Address {
		.component(self)
	}
}
