//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI

#if DEBUG

/// ‼️ NEVER USE IN PRODUCTION ‼️
/// An INSECURE ephemeral storage conforming to `SecureStorageDriver` meant
/// for testing purposes only.
public final actor Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage {
	public typealias Key = SecureStorageKey
	fileprivate var dictionary: [Key: Data] = [:]
	public init(keychainService _: String) {}
}

// MARK: `SecureStorageDriver` conformance
extension Insecure︕！TestOnly︕！Ephemeral︕！SecureStorage: SecureStorageDriver {
	public func loadData(key: SecureStorageKey) async throws -> Data? {
		dictionary[key]
	}
	
	public func saveData(key: SecureStorageKey, data: Data) async throws {
		dictionary[key] = data
	}
	
	public func deleteDataForKey(key: SecureStorageKey) async throws {
		dictionary.removeValue(forKey: key)
	}
	
}
#endif
