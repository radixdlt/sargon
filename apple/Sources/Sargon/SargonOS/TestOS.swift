//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI

#if DEBUG

extension BIOS {
	public static func test(
		bundle: Bundle = .main,
		userDefaultsSuite: String = "Test",
		secureStorageDriver: SecureStorageDriver
	) -> BIOS {
		BIOS(
			bundle: bundle,
			userDefaultsSuite: userDefaultsSuite,
			secureStorageDriver: secureStorageDriver
		)
	}
}

@dynamicMemberLookup
public final class TestOS {
	public let os: SargonOS
	public init(bios: BIOS) async throws {
		self.os = try await SargonOS.boot(bios: bios)
	}
	public convenience init(
		bundle: Bundle = .main,
		userDefaultsSuite: String = "Test",
		secureStorageDriver: SecureStorageDriver
	) async throws {
		try await self.init(
			bios: .test(
				bundle: bundle,
				userDefaultsSuite: userDefaultsSuite,
				secureStorageDriver: secureStorageDriver
			)
		)
	}
}
extension TestOS: SargonOSProtocol {}

// MARK: DynamaicMemberLookup
// Enables us to access properties on `os` as if those
// were properties on this `TestOS` type (does not work for methods, for that we rely on SargonOSProtocol).
extension TestOS {
	public nonisolated subscript<T>(dynamicMember keypath: KeyPath<SargonOS, T>) -> T {
		os[keyPath: keypath]
	}
}

// MARK: Private
extension TestOS {
	private func nextAccountName() -> DisplayName {
		let index = accounts().count
		return DisplayName(value: "Unnamed \(index)")
	}
}

// MARK: Public
extension TestOS {
	
	@discardableResult
	public func createAccount(
		name: String? = nil
	) async throws -> Self {
		
		let accountName = try name.map {
			try DisplayName(
				validating: $0
			)
		} ?? nextAccountName()
	
		let _ = try await os.createAccount(
			named: accountName
		)
		return self
	}
	
	@discardableResult
	public func batchCreateAccounts(
		count: UInt16,
		namePrefix: DisplayName
	) async throws -> Self {
		let _ = try await os.batchCreateManyAccountsThenSaveOnce(count: count, networkId: currentNetworkID, namePrefix: namePrefix.value)
		return self
	}
}


#endif // DEBUG
