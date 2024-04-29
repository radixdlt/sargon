// MARK: - TransactionHashProtocol

public protocol TransactionHashProtocol: IdentifiableByStringProtocol {
	var networkId: NetworkID { get }
	var bech32EncodedTxId: String { get }
}

extension TransactionHashProtocol {
	public var networkID: NetworkID {
		networkId
	}

	public func toRawString() -> String {
		bech32EncodedTxId
	}
}
