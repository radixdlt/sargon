import Foundation
import SargonUniFFI

extension Profile {
    public init(json bytes: some DataProtocol) throws {
		self = try newProfileFromJsonBytes(json: BagOfBytes(bytes))
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
	
	
	public func encrypt(password: String) -> Data {
		profileEncryptWithPassword(profile: self, encryptionPassword: password)
	}
}
