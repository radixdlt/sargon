import SargonUniFFI

extension AssetsTransfersRecipient {
	public var accountAddress: AccountAddress {
		assetsTransfersRecipientAccountAddress(recipient: self)
	}
}
