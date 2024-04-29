import SargonUniFFI

#if DEBUG
extension Mnemonic {
	public static let sample: Self = newMnemonicSample()
	public static let sampleOther: Self = newMnemonicSampleOther()

	public static let sample24ZooVote: Self = try! Self(phrase: "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote")
}
#endif // DEBUG
