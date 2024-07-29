//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-02-15.
//

@_exported import KeychainAccess
import Sargon
import ComposableArchitecture


extension Keychain: @unchecked Sendable {}
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
