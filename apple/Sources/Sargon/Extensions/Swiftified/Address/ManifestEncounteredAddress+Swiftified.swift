import SargonUniFFI

extension ManifestEncounteredAddress: AddressProtocol {
	public var asGeneral: Address {
		switch self {
		case let .component(componentAddress): Address.component(componentAddress)
		case let .locker(lockerAddress): Address.locker(lockerAddress)
		}
	}
}
