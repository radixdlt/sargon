import SargonUniFFI

extension ComponentAddress: AddressProtocol {
	public var asGeneral: Address {
		.component(self)
	}
}
