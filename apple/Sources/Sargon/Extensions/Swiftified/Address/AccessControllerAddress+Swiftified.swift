import SargonUniFFI

extension AccessControllerAddress: AddressProtocol {
	public var asGeneral: Address {
		.accessController(self)
	}
}
