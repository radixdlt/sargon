import SargonUniFFI

extension OwnedOrThirdPartyAccountAddress {
	public var accountAddress: AccountAddress {
		accountOrAddressOfAccountAddress(recipient: self)
	}
}
