//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI

extension NetworkingDriver where Self == URLSession {
	public static var shared: Self { Self.shared }
}

extension SecureStorageDriver where Self == UnsafeMockSecureStorage {
	public static var shared: Self { Self.shared }
}

extension EntropyProviderDriver where Self == EntropyProvider {
	public static var shared: Self { Self.shared }
}

extension HostInfoDriver where Self == HostInfo {
	public static var shared: Self { Self.shared }
}

extension LoggingDriver where Self == Log {
	public static var shared: Self { Self.shared }
}

extension Drivers: @unchecked Sendable {}
extension SargonOs: @unchecked Sendable {}
extension Drivers {
	public convenience init() {
		self.init(networking: .shared, secureStorage: .shared, entropyProvider: .shared, hostInfo: .shared, loggingDriver: .shared)
	}
	public static let shared = Drivers()
}

public typealias SargonOS = SargonOs

extension SargonOS {
	public static func shared() async throws -> SargonOS {
		try await Self.withDrivers(drivers: .shared)
	}
}
