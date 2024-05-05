//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI

extension UserDefaults: @unchecked Sendable {}

extension UnsafeStorageDriver where Self == UnsafeStorage {
	public static var shared: Self { Self.shared }
}

public final class UnsafeStorage: Sendable {
	public typealias Key = UnsafeStorageKey
	fileprivate let userDefaults: UserDefaults
	public init(userDefaults: UserDefaults = .standard) {
		self.userDefaults = userDefaults
	}
	public static let shared = UnsafeStorage()
}

extension UnsafeStorageKey {
	var identifier: String {
		unsafeStorageKeyIdentifier(key: self)
	}
}

extension UnsafeStorage: UnsafeStorageDriver {
	public func loadData(key: Key) -> Data? {
		userDefaults.data(forKey: key.identifier)
	}
	
	public func saveData(key: Key, data: Data) {
		userDefaults.setValue(data, forKey: key.identifier)
	}
	
	public func deleteDataForKey(key: Key) {
		userDefaults.removeObject(forKey: key.identifier)
	}
	
}
