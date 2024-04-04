import SargonUniFFI

extension AccessControllerAddress: AddressProtocol {
	public func embed() -> Address {
		.accessController(self)
	}
}
