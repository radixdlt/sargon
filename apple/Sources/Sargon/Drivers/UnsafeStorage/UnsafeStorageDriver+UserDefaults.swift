import Foundation
import SargonUniFFI

// MARK: - UserDefaults + @unchecked @retroactive Sendable
extension UserDefaults: @unchecked @retroactive Sendable {}

// Makes it possible to type `.shared` on an initalizer/func taking
// `some UnsafeStorageDriver` as parameter.
extension UnsafeStorageDriver where Self == UnsafeStorage {
	/// Singleton `UnsafeStorageDriver` of type `UnsafeStorage,
	/// which uses `UserDefaults.standard` as storage
	public static var shared: Self { Self.shared }
}

public typealias UnsafeStorageKeyMapping = [UnsafeStorageKey: String]

// MARK: - UnsafeStorage
/// An `UnsafeStorageDriver` implementation which
/// wraps `UserDefaults`.
public final class UnsafeStorage: Sendable {
	public typealias Key = UnsafeStorageKey
	fileprivate let userDefaults: UserDefaults

	/// A dictionary containing the custom String value used for a given `UnsafeStorageKey`.
	/// This is necessary since some UserDefaults were saved by the Host apps prior to Sargon.
	fileprivate let keyMapping: [UnsafeStorageKey: String]

	public init(userDefaults: UserDefaults = .standard, keyMapping: [UnsafeStorageKey: String] = [:]) {
		self.userDefaults = userDefaults
		self.keyMapping = keyMapping
	}

	/// Singleton `UnsafeStorageDriver` of type `UnsafeStorage,
	/// which uses `UserDefaults.standard` as storage
	public static let shared = UnsafeStorage()
}

extension UnsafeStorageKey {
	/// Translates this `UnsafeStorageKey` into a String
	/// identifier which we can use with `UserDefaults`
	public var identifier: String {
		unsafeStorageKeyIdentifier(key: self)
	}
}

// MARK: - UnsafeStorage + UnsafeStorageDriver
extension UnsafeStorage: UnsafeStorageDriver {
	public func loadData(key: Key) -> Data? {
		userDefaults.data(forKey: identifier(for: key))
	}

	public func saveData(key: Key, data: Data) {
		userDefaults.setValue(data, forKey: identifier(for: key))
	}

	public func deleteDataForKey(key: Key) {
		userDefaults.removeObject(forKey: identifier(for: key))
	}

	private func identifier(for key: Key) -> String {
		if let mapped = keyMapping[key] {
			mapped
		} else {
			key.identifier
		}
	}
}
