import Foundation
import SargonUniFFI

extension Profile {
	
	/// Just a convenience for `analyzeContents(of: String(data: data, encoding: .utf8)!)`
	public static func analyzeContents(data: some DataProtocol) -> ProfileFileContents {
		let contents = String(data: Data(data), encoding: .utf8)!
		return analyzeContents(of: contents)
	}
	
	public static func analyzeContents(of contents: String) -> ProfileFileContents {
		profileAnalyzeContentsOfFile(contents: contents)
	}
	
	public func toJSONString(prettyPrinted: Bool = false) -> String {
		profileToJsonString(profile: self, prettyPrinted: prettyPrinted)
	}

	@available(*, deprecated, message: "Use `toJSONString(prettyPrinted:) instead")
	public func profileSnapshot() -> Data {
		Data(toJSONString(prettyPrinted: false).utf8)
	}
	
	@available(*, deprecated, message: "Use `toJSONString(prettyPrinted:) instead")
	public func jsonData() -> Data {
		profileSnapshot()
	}
	
	@available(*, deprecated, message: "Use `init(jsonString:) instead")
	public init(jsonData: some DataProtocol) throws {
		let jsonString = String(data: Data(jsonData), encoding: .utf8)!
		try self.init(jsonString: jsonString)
	}
	
	public init(jsonString: String) throws {
		self = try newProfileFromJsonString(jsonStr: jsonString)
	}
	
	@available(*, deprecated, message: "Use `init(encryptedProfileJSONString:decryptionPassword)` instead")
	public init(encrypted bytes: some DataProtocol, decryptionPassword: String) throws {
		let jsonString = String(data: Data(bytes), encoding: .utf8)!
		try self.init(encryptedProfileJSONString: jsonString, decryptionPassword: decryptionPassword)
	}
	
	public init(
		encryptedProfileJSONString jsonString: String,
		decryptionPassword: String
	) throws {
		self = try newProfileFromEncryptionBytes(
			jsonString: jsonString,
			decryptionPassword: decryptionPassword
		)
	}
	
	public func toString() -> String {
		profileToString(profile: self)
	}
	
	
	public func toDebugString() -> String {
		profileToDebugString(profile: self)
	}

	/// Returns an Encrypted Profile as JSON Data
	@available(*, deprecated, message: "Use `encryptedJsonString:password` instead")
	public func encrypt(password: String) -> Data {
		let jsonString = encryptedJsonString(password: password)
		return Data(jsonString.utf8)
	}
	
	/// Returns an Encrypted Profile as JSON String
	public func encryptedJsonString(password: String) -> String {
		profileEncryptWithPassword(profile: self, encryptionPassword: password)
	}

	/// This delegates to `checkIfProfileJsonStringContainsLegacyP2PLinks`
	@available(*, deprecated, message: "Use `checkIfProfileJsonStringContainsLegacyP2PLinks` instead")
	public static func checkIfProfileJsonContainsLegacyP2PLinks(contents: some DataProtocol) -> Bool {
		let jsonData = Data(contents)
		let jsonString = String(data: jsonData, encoding: .utf8)!
		return checkIfProfileJsonStringContainsLegacyP2PLinks(jsonString: jsonString)
	}
	
	public static func checkIfProfileJsonStringContainsLegacyP2PLinks(jsonString: String) -> Bool {
		checkIfProfileJsonContainsLegacyP2pLinks(jsonStr: jsonString)
	}

	/// This delegates to `checkIfEncryptedProfileJsonStringContainsLegacyP2PLinks:jsonString:password`
	@available(*, deprecated, message: "Use `checkIfEncryptedProfileJsonStringContainsLegacyP2PLinks:jsonString:password` instead")
	public static func checkIfEncryptedProfileJsonContainsLegacyP2PLinks(contents: some DataProtocol, password: String) -> Bool {
		let jsonData = Data(contents)
		let jsonString = String(data: jsonData, encoding: .utf8)!
		return checkIfEncryptedProfileJsonStringContainsLegacyP2PLinks(jsonString: jsonString, password: password)
	}
	
	public static func checkIfEncryptedProfileJsonStringContainsLegacyP2PLinks(jsonString: String, password: String) -> Bool {
		checkIfEncryptedProfileJsonContainsLegacyP2pLinks(jsonStr: jsonString, password: password)
	}
}
