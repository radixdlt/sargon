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
	public static let test = BIOS(
		bundle: .main,
		keychainService: "Test",
		userDefaultsSuite: "works.rdx"
	)
}

@dynamicMemberLookup
public final class TestOS {
	public let os: SargonOS
	public init(bios: BIOS) async throws {
		self.os = try await SargonOS.boot(bios: bios)
	}
	public convenience init() async throws {
		try await self.init(bios: .test)
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
}


#endif // DEBUG
