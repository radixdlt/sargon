import SargonUniFFI

extension AccountOrAddressOf {
	public var accountAddress: AccountAddress {
		accountOrAddressOfAccountAddress(recipient: self)
	}
}
