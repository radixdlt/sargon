extension Mnemonic: @unchecked Sendable {}

extension Mnemonic: SargonModel {}

extension Mnemonic: CustomStringConvertible {
	public var description: String {
		phrase
	}
}

#if DEBUG
extension Mnemonic {
	public static var sample: Self {
		newMnemonicSample()
	}
	
	public static var sampleOther: Self {
		newMnemonicSampleOther()
	}
}
#endif // DEBUG
