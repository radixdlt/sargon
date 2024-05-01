import Foundation
import SargonUniFFI

extension RefProfile: @unchecked Sendable {}
extension RefBytes: @unchecked Sendable {}

extension ProfileFileContents: @unchecked Sendable, Hashable, Equatable {
	public func hash(into hasher: inout Hasher) {
		hasher.combine(profileFileContentsHashValue(contents: self))
	}
	
	public static func == (lhs: Self, rhs: Self) -> Bool {
		profileFileContentsEquals(lhs: rhs, rhs: rhs)
	}
}

extension Profile {


	public static func analyzeFile(
		reference: RefBytes
	) -> ProfileFileContents {
		profileAnalyzeContentsOfFile(reference: reference)
	}

	
	public init(
		jsonBytesReference: RefBytes
	) throws {
		self = try newProfileFromJsonBytes(
			reference: jsonBytesReference
		).takeProfile()
	}

	public init(
		encrypted bytes: some DataProtocol,
		decryptionPassword: String
	) throws {
		self = try newProfileFromEncryptionBytes(
			reference: RefBytes(bytes: Data(bytes)),
			decryptionPassword: decryptionPassword
		).takeProfile()
	}

	public func profileSnapshot() -> RefBytes {
		profileToJsonBytes(reference: RefProfile(profile: self))
	}

	public func encrypt(
		password: String
	) -> Data {
		profileEncryptWithPassword(
			reference: RefProfile(profile: self),
			encryptionPassword: password
		).bytes()
	}
}
