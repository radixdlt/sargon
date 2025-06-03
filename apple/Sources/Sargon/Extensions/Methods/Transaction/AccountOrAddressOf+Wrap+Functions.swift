import SargonUniFFI

extension TransferRecipient {
	public var accountAddress: AccountAddress {
		transferRecipientAddress(recipient: self)
	}
}
