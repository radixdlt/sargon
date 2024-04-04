import SargonUniFFI

extension AddressOfAccountOrPersona: AddressProtocol {
	public func embed() -> Address {
		switch self {
		case let .account(accountAddress): Address.account(accountAddress)
		case let .identity(identityAddress): Address.identity(identityAddress)
		}
	}
}
