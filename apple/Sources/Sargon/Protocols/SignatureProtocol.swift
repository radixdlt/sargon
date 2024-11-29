// MARK: - IntoSignatureProtocol
public protocol IntoSignatureProtocol {
	var signature: Signature { get }
}

// MARK: - SignatureProtocol
public protocol SignatureProtocol: BinaryProtocol & IntoSignatureProtocol {}
