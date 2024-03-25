extension Mnemonic: @unchecked Sendable {}
extension Mnemonic: SargonModel {}

extension Mnemonic: CustomStringConvertible {
	public var description: String {
		phrase
	}
}
