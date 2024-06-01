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
public struct FactorSourcesClient: Sendable {
	public typealias AddFactorSource = @Sendable (FactorSource) async throws -> Void
	public var addFactorSource: AddFactorSource
}

extension FactorSourcesClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		Self(
			addFactorSource: { factorSource in
				log.notice("Adding New factorSource: \(factorSource)")
				let _ = try await os.fac
				log.info("Finished adding new factorSource.")
			}
		)
	}
}
