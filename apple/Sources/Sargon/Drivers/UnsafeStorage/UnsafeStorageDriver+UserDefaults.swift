import Foundation
import SargonUniFFI

// MARK: - UserDefaults + @unchecked Sendable
extension UserDefaults: @unchecked Sendable {}

// Makes it possible to type `.shared` on an initalizer/func taking
// `some UnsafeStorageDriver` as parameter.
extension UnsafeStorageDriver where Self == UnsafeStorage {
	/// Singleton `UnsafeStorageDriver` of type `UnsafeStorage,
	/// which uses `UserDefaults.standard` as storage
	public static var shared: Self { Self.shared }
}

// MARK: - UnsafeStorage
/// An `UnsafeStorageDriver` implementation which
/// wraps `UserDefaults`.
public final class UnsafeStorage: Sendable {
	public typealias Key = UnsafeStorageKey
	fileprivate let userDefaults: UserDefaults
	public init(userDefaults: UserDefaults = .standard) {
		self.userDefaults = userDefaults
	}

	/// Singleton `UnsafeStorageDriver` of type `UnsafeStorage,
	/// which uses `UserDefaults.standard` as storage
	public static let shared = UnsafeStorage()
}

extension UnsafeStorageKey {
	/// Translates this `UnsafeStorageKey` into a String
	/// identifier which we can use with `UserDefaults`
	var identifier: String {
		unsafeStorageKeyIdentifier(key: self)
	}
}

// MARK: - UnsafeStorage + UnsafeStorageDriver
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
