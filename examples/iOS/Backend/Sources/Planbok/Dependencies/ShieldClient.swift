import Foundation
import Sargon
import DependenciesMacros

@DependencyClient
public struct ShieldClient: Sendable {
	public typealias CreateNewSecurityShield = @Sendable (MatrixOfFactorSources, _ numberOfEpochsUntilAutoConfirmation: UInt64, _ name: DisplayName) -> Shield
	public var createNewSecurityShield: CreateNewSecurityShield
}


extension ShieldClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		Self(
			createNewSecurityShield: { matrixOfFactors, name in
				let metadata = SecurityStructureMetadata(name: name)
				Shield(
					metadata: metadata, 
					numberOfEpochsUntilAutoConfirmation: <#T##UInt64#>, matrixOfFactors: <#T##MatrixOfFactorSources#>)
			},
		)
	}
}
extension ShieldClient {
	public func createNewShield(name: DisplayName, epochsUntilAutoConfirm)
}

