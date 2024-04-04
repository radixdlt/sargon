import SargonUniFFI

extension VaultAddress: AddressProtocol {
	public func embed() -> Address {
		.vault(self)
	}
}
