import SargonUniFFI

extension DeviceFactorSource {
	/// **B**abylon **D**evice **F**actor **S**ource
	public var isBDFS: Bool {
		guard supportsBabylon else { return false }
		if hint.mnemonicWordCount == .twentyFour {
			return true
		} else {
			log.error("BDFS with non 24 words mnemonic found, probably this profile originated from Android? Which with 'BDFS Error' with 1.0.0 allowed usage of 12 word Olympia Mnemonic.")
			return false
		}
	}
}
