import ComposableArchitecture
@_exported import KeychainAccess
import Sargon

// MARK: - Keychain + @unchecked Sendable
extension Keychain: @unchecked Sendable {}

// MARK: - Keychain + SecureStorageDriver
extension Keychain: SecureStorageDriver {
	@Sendable
	public func loadData(key: SecureStorageKey) throws -> Data? {
		try getData(key.identifier)
	}

	@Sendable
	public func saveData(key: SecureStorageKey, data: Data) throws {
		self[data: key.identifier] = data
	}

	@Sendable
	public func deleteDataForKey(key: SecureStorageKey) throws {
		try self.remove(key.identifier)
	}
}

extension SecureStorageKey {
	public var identifier: String {
		secureStorageKeyIdentifier(key: self)
	}
}
