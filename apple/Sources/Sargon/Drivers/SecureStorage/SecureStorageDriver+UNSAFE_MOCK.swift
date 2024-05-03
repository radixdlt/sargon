//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI

extension SecureStorageDriver where Self == UnsafeMockSecureStorage {
	public init(keychainService: String) {
		self.init(keychainService: keychainService)
	}
}

public final actor UnsafeMockSecureStorage {
	public typealias Key = SecureStorageKey
	fileprivate var dictionary: [Key: Data] = [:]
	public init(keychainService _: String) {}
}

extension UnsafeMockSecureStorage: SecureStorageDriver {
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
