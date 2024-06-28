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
public struct MnemonicClient: Sendable {
	public typealias LoadMnemonic = @Sendable (FactorSourceIDFromHash) async throws -> PrivateHierarchicalDeterministicFactorSource
	public var loadMnemonic: LoadMnemonic
}

extension MnemonicClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		Self(
			loadMnemonic: { id in
				try await os.loadPrivateDeviceFactorSourceById(id: id)
			}
		)
	}
}
