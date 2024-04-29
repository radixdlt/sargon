import SargonUniFFI

// MARK: - IntentSignature + SargonModel
extension IntentSignature: SargonModel {}

// MARK: - IntentSignature + CustomStringConvertible
extension IntentSignature: CustomStringConvertible {
	public var description: String {
		signatureWithPublicKey.description
	}
}
