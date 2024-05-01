import Foundation
import SargonUniFFI

extension RefProfile: @unchecked Sendable {}
extension RefBytes: @unchecked Sendable {}

extension Profile {

	public static func analyzeFile(
		reference: RefBytes
	) -> ProfileFileContents {
		profileAnalyzeContentsOfFileFastByRef(reference: reference)
	}
	
	public init(
		jsonBytesReference: RefBytes
	) throws {
		self = try newProfileFromJsonBytesFastByRef(
			reference: jsonBytesReference
		).take()
	}
	
	internal init(
		jsonBytes: some DataProtocol
	) throws {
		self = try newProfileFromJsonBytes(jsonBytes: Data(jsonBytes))
	}

	public init(
		encrypted bytes: some DataProtocol,
		decryptionPassword: String
	) throws {
		self = try newProfileFromEncryptionBytesFastByRef(
			reference: RefBytes(inner: Data(bytes)),
			decryptionPassword: decryptionPassword
		).take()
	}

	public func profileSnapshot() -> Data {
		try! profileSnapshotRef().take()
	}
	
	/// Call `try take()` on `RefBytes` to get the Profile bytes, **can only be called once.**, will throw next time called.
	public func profileSnapshotRef() -> RefBytes {
		profileToJsonBytesFastByRef(reference: RefProfile(inner: self))
	}

	public func encrypt(
		password: String
	) -> Data {
		try! profileEncryptWithPasswordFastByRef(
			reference: RefProfile(inner: self),
			encryptionPassword: password
		).take()
	}
}
