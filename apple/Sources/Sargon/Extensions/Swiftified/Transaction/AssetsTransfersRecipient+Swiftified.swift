extension AssetsTransfersRecipient: CustomStringConvertible {
	public var description: String {
		accountAddress.address
	}
}

extension AssetsTransfersRecipient {
	public var accountAddress: AccountAddress {
		switch self {
		case let .foreignAccount(address): address
		case let .myOwnAccount(account): account.address
		}
	}
}
