import Foundation
import SargonUniFFI

extension Profile {
	
	public static func analyzeFile(contents: some DataProtocol) -> ProfileFileContents {
		profileAnalyzeContentsOfFile(bytes: Data(contents))
	}
	
	public init(jsonData bytes: some DataProtocol) throws {
		self = try newProfileFromJsonBytes(jsonBytes: Data(bytes))
	}
	
	public init(encrypted bytes: some DataProtocol, decryptionPassword: String) throws {
		self = try newProfileFromEncryptionBytes(
			json: Data(bytes),
			decryptionPassword: decryptionPassword
		)
	}
	
	public func profileSnapshot() -> Data {
		profileToJsonBytes(profile: self)
	}
	
	public func jsonData() -> Data {
		profileSnapshot()
	}
	
	public func encrypt(password: String) -> Data {
		profileEncryptWithPassword(profile: self, encryptionPassword: password)
	}

	public static func checkIfProfileJsonContainsLegacyP2PLinks(contents: some DataProtocol) -> Bool {
	    checkIfProfileJsonContainsLegacyP2pLinks(json: Data(contents))
    }
}
