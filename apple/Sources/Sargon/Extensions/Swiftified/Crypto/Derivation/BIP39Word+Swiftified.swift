import SargonUniFFI

public typealias BIP39Word = Bip39Word

// MARK: SargonModel
extension BIP39Word: SargonModel {}

// MARK: Identifiable
extension BIP39Word: Identifiable {
	public typealias ID = U11

	public var id: ID {
		index
	}
}
