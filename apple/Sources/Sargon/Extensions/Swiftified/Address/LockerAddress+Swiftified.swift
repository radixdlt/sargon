import SargonUniFFI

extension LockerAddress: AddressProtocol {
	public var asGeneral: Address {
		.locker(self)
	}
}
