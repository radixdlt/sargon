import Foundation
import SargonUniFFI

extension Profile {
	
	public static func analyzeFile(contents: some DataProtocol) -> ProfileFileContents {
		profileAnalyzeContentsOfFile(bytes: Data(contents))
	}
	
#if DEBUG
	///
	internal init(jsonData bytes: some DataProtocol) throws {
		self = try newProfileFromJsonBytes(jsonBytes: Data(bytes))
	}
	internal func profileSnapshot() -> Data {
		profileToJsonBytes(profile: self)
	}
	internal func jsonData() -> Data {
		profileSnapshot()
	}
	
	#endif
	
	internal init(jsonString: String) throws {
		self = try newProfileFromJsonString(json: jsonString)
	}

	internal func jsonString(prettyPrint: Bool) -> String {
		profileToJsonString(profile: self, prettyPrinted: prettyPrint)
	}

	
	public init(encrypted bytes: some DataProtocol, decryptionPassword: String) throws {
		self = try newProfileFromEncryptionBytes(
			json: Data(bytes),
			decryptionPassword: decryptionPassword
		)
	}
	
	public func toString() -> String {
		profileToString(profile: self)
	}
	
	
	public func toDebugString() -> String {
		profileToDebugString(profile: self)
	}

	public func encrypt(password: String) -> Data {
		profileEncryptWithPassword(profile: self, encryptionPassword: password)
	}

	public static func checkIfProfileJsonContainsLegacyP2PLinks(contents: some DataProtocol) -> Bool {
	    checkIfProfileJsonContainsLegacyP2pLinks(json: Data(contents))
	}

	public static func checkIfEncryptedProfileJsonContainsLegacyP2PLinks(contents: some DataProtocol, password: String) -> Bool {
		checkIfEncryptedProfileJsonContainsLegacyP2pLinks(json: Data(contents), password: password)
	}
}
