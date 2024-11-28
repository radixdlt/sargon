import SargonUniFFI

#if DEBUG
extension Mnemonic {
	public static let sample: Self = newMnemonicSample()
	public static let sampleOther: Self = newMnemonicSampleOther()
	
	public static let sampleDevice: Self = newMnemonicSampleDevice()
	public static let sampleDeviceOther: Self = newMnemonicSampleDeviceOther()
	public static let sampleDevice12Words: Self = newMnemonicSampleDevice12Words()
	public static let sampleDevice12WordsOther: Self = newMnemonicSampleDevice12WordsOther()
	public static let sampleLedger: Self = newMnemonicSampleLedger()
	public static let sampleLedgerOther: Self = newMnemonicSampleLedgerOther()
	public static let sampleArculus: Self = newMnemonicSampleArculus()
	public static let sampleArculusOther: Self = newMnemonicSampleArculusOther()
	public static let sampleOffDeviceMnemonic: Self = newMnemonicSampleOffDevice()
	public static let sampleOffDeviceMnemonicOther: Self = newMnemonicSampleOffDeviceOther()
	public static let sampleSecurityQuestions: Self = newMnemonicSampleSecurityQuestions()
	public static let sampleSecurityQuestionsOther: Self = newMnemonicSampleSecurityQuestionsOther()
	public static let samplePassword: Self = newMnemonicSamplePassword()
	public static let samplePasswordOther: Self = newMnemonicSamplePasswordOther()

    public static let sample24ZooVote: Self = try! Self(phrase: "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote")
	
	public static let sampleValues: [Self] = [
		.sampleDevice,
		.sampleDeviceOther,
		.sampleDevice12Words,
		.sampleDevice12WordsOther,
		.sampleLedger,
		.sampleLedgerOther,
		.sampleArculus,
		.sampleArculusOther,
		.sampleOffDeviceMnemonic,
		.sampleOffDeviceMnemonicOther,
		.sampleSecurityQuestions,
		.sampleSecurityQuestionsOther,
		.samplePassword,
		.samplePasswordOther
	]
}
#endif // DEBUG
