import SargonUniFFI

extension PackageAddress: AddressProtocol {
	public func embed() -> Address {
		.package(self)
	}
}
