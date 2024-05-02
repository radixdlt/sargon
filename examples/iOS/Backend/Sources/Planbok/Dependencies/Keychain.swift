//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-02-15.
//

@_exported import KeychainAccess
import Sargon
import SargonUniFFI

extension DependencyValues {
	/// A dependency that exposes an ``Keychain.Dependency`` value that you can use to read and
	/// write to `Keychain`.
	public var keychain: Keychain.Dependency {
		get { self[Keychain.Dependency.self] }
		set { self[Keychain.Dependency.self] = newValue }
	}
}

extension Keychain: @unchecked Sendable {}
extension Keychain: SecureStorage {
	@DependencyClient
	public struct Dependency: DependencyKey {

		public let loadData: @Sendable (SecureStorageKey) throws -> Data?
		public let saveData: @Sendable (SecureStorageKey, Data) throws -> Void
		public let deleteDataForKey: @Sendable (SecureStorageKey) throws -> Void

		public static func with(keychain: Keychain) -> Self {
			Self.init(
				loadData: keychain.loadData(key:),
				saveData: keychain.saveData(key:data:),
				deleteDataForKey: keychain.deleteDataForKey(key:)
			)
		}
		public static let liveValue = Self.with(keychain: .shared)
		public static var testValue: Self {
			final class Ephemeral {
				var dict = [SecureStorageKey: Data]()
				init() {}
			}
			let ephemeral = Ephemeral()
			return Self(
				loadData: { ephemeral.dict[$0] },
				saveData: { ephemeral.dict[$0] = $1 },
				deleteDataForKey: { ephemeral.dict[$0] = nil }
			)
		}
	}

	public static let shared = Keychain(service: "works.rdx.planbok")

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
