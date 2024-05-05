//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import SargonUniFFI

@dynamicMemberLookup
public final actor OS {
	public let booted: SargonOS
	private init(sargonOS: SargonOS) {
		self.booted = sargonOS
	}
}

extension OS {
	private nonisolated(unsafe) static var _shared: OS?
	
	public static var shared: OS {
		guard let shared = Self._shared else {
			fatalError("OS not created, create it with `OS.createShared:sargonOS:`")
		}
		return shared
	}
}

extension OS {
	
	/// Can be access later with `OS.shared`
	@discardableResult
	public static func boot(bios: BIOS) async throws -> OS {
		let swiftBIOS = SwiftBIOS.createShared(bios: bios)
		return try await Self.boot(swiftBIOS: swiftBIOS)
	}
	
	/// Can be access later with `OS.shared`
	@discardableResult
	public static func boot(swiftBIOS: SwiftBIOS) async throws -> OS {
		let bios = swiftBIOS.bios
		let sargonOS = try await SargonOS.boot(bios: bios)
		return Self.createShared(sargonOS: sargonOS)
	}
	
	/// Can be access later with `OS.shared`
	@discardableResult
	public static func createShared(sargonOS: SargonOS) -> OS {
		assert(_shared == nil, "Should not be created twice")
		let shared = OS(sargonOS: sargonOS)
		Self._shared = shared
		return shared
	}
	
}

extension OS {
	public nonisolated subscript<T>(dynamicMember keypath: KeyPath<SargonOS, T>) -> T {
		booted[keyPath: keypath]
	}
}
