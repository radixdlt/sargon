//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation

extension SargonOS: SargonOSProtocol {
	public var os: SargonOS { self }
}

// MARK: SargonOSProtocol Conformance
extension SargonOS {
	
	@discardableResult
	public func createAccount(
		named accountName: DisplayName
	) async throws -> Account {
		try await createAndSaveNewAccount(networkId: currentNetworkID, name: accountName)
	}
	
}
