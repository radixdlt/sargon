//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-03.
//

import Foundation
import SargonUniFFI


public typealias SargonOS = SargonOs

extension SargonOS: @unchecked Sendable {}

extension SargonOS {
	
	@available(*, deprecated, message: "SHOULD migrate to use more specialized methods on SargonOS instead, e.g. `createAndSaveNewAccount` - SargonOS should be the SOLE object to perform the mutation and persisting.")
	public func saveChangedProfile(_ profile: Profile) async throws {
		try await deprecatedSaveFfiChangedProfile(profile: profile)
	}
}
