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

public final class TestOS {
	public let os: SargonOS
	
	public init(bios: BIOS) async throws {
		self.os = try await SargonOS.boot(bios: bios)
	}
	
}
extension TestOS: SargonOSProtocol {}

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
		named name: String? = nil
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
