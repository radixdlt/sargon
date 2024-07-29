import Foundation
import Sargon
import DependenciesMacros

@DependencyClient
public struct ShieldClient: Sendable {
	public typealias SaveSecurityShield = @Sendable (Shield) async throws -> Bool
	public var saveSecurityShield: SaveSecurityShield
}


extension ShieldClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		Self(
			saveSecurityShield: { shield in
				try await os.addSecurityStructureOfFactorSources(structure: shield)
			}
		)
	}
}
