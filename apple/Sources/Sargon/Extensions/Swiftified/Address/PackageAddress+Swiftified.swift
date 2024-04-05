import SargonUniFFI

extension PackageAddress: AddressProtocol {
	public var asGeneral: Address {
		.package(self)
	}
}
