import SargonUniFFI

extension VaultAddress: AddressProtocol {
	public var asGeneral: Address {
		.vault(self)
	}
}
