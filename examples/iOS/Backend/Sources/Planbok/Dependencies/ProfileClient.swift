//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import Sargon
import DependenciesMacros

@DependencyClient
public struct ProfileClient: Sendable {
	public typealias DeleteProfileAndMnemonicsThenCreateNew = @Sendable () async throws -> Void
	public var deleteProfileAndMnemonicsThenCreateNew: DeleteProfileAndMnemonicsThenCreateNew
}

extension ProfileClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		return Self(
			deleteProfileAndMnemonicsThenCreateNew: {
				try await os.deleteProfileThenCreateNewWithBdfs()
			}
		)
	}
}
