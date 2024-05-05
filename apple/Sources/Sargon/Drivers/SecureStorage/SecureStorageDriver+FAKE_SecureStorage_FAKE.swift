//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI

extension SecureStorageDriver where Self == FAKE_SecureStorage_FAKE {
	public init(keychainService: String) {
		self.init(keychainService: keychainService)
	}
}

public final actor FAKE_SecureStorage_FAKE {
	public typealias Key = SecureStorageKey
	fileprivate var dictionary: [Key: Data] = [:]
	public init(keychainService _: String) {}
}

extension FAKE_SecureStorage_FAKE: SecureStorageDriver {
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
