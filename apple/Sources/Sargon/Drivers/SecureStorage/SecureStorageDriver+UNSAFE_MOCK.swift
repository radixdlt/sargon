//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI

public final actor UnsafeMockSecureStorage {
	public typealias Key = SecureStorageKey
	fileprivate var dictionary: [Key: Data] = [:]
	public init() {}
	public static let shared = UnsafeMockSecureStorage()
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
