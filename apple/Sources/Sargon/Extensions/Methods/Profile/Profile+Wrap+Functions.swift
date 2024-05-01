import Foundation
import SargonUniFFI

extension RefProfile: @unchecked Sendable, Hashable, Equatable {
	public static func == (lhs: RefProfile, rhs: RefProfile) -> Bool {
		lhs.profile() == rhs.profile()
	}
	public func hash(into hasher: inout Hasher) {
		hasher.combine(profile())
	}
}

extension ProfileFileContents: @unchecked Sendable, Hashable, Equatable {
	public func hash(into hasher: inout Hasher) {
		switch self {
		case .encryptedProfile:
			hasher.combine("encryptedProfile")
		case .notProfile:
			hasher.combine("notProfile")
		case let .plaintextProfile(ref):
			hasher.combine(ref)
		}
	}
	
	public static func == (lhs: ProfileFileContents, rhs: ProfileFileContents) -> Bool {
		switch (lhs, rhs) {
		case let (.plaintextProfile(lhsRef), .plaintextProfile(rhsRef)):
			return lhsRef == rhsRef
		case (.encryptedProfile, .encryptedProfile):
			return true
		case (.notProfile, .notProfile):
			return true
		default:
			return false
		}
	}
}

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

	
	public static func reference(
		jsonBytesReference: RefBytes
	) throws -> RefProfile {
		try newProfileFromJsonBytes(
			reference: jsonBytesReference
		)
	}

	public init(
		encrypted bytes: some DataProtocol,
		decryptionPassword: String
	) throws {
		self = try newProfileFromEncryptionBytes(
			reference: RefBytes(bytes: Data(bytes)),
			decryptionPassword: decryptionPassword
		).profile()
	}

	public func profileSnapshot() -> Data {
		profileToJsonBytes(reference: RefProfile(profile: self)).bytes()
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
