import Foundation
import SargonUniFFI

extension Profile {
	
	public static func analyzeFile(
		contents: some DataProtocol
	) -> ProfileFileContents {
		profileAnalyzeContentsOfFile(
			bytes: Data(
				contents
			)
		)
	}
	
	public init(
		jsonData bytes: some DataProtocol
	) throws {
		self = try newProfileFromJsonBytesArcInArcOut(json: .init(bytes: Data(bytes))).profile()
	}
	
	public init(
		encrypted bytes: some DataProtocol,
		decryptionPassword: String
	) throws {
		self = try newProfileFromEncryptionBytes(
			json: Data(bytes),
			decryptionPassword: decryptionPassword
		)
	}
	
	public func profileSnapshot() -> Data {
        profileToJsonBytesArcInArcOut(profile: JsonContainerProfile(profile: self)).bytes()
	}
	
	public func jsonData() -> Data {
		profileSnapshot()
	}
	
	public func encrypt(
		password: String
	) -> Data {
		profileEncryptWithPassword(
			profile: self,
			encryptionPassword: password
		)
	}
}
