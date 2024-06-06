import Foundation
import Sargon
import DependenciesMacros

@DependencyClient
public struct ShieldClient: Sendable {
	public typealias CreateNewSecurityShield = @Sendable (MatrixOfFactorSources, _ name: DisplayName, _ numberOfDaysToAutoConfirm: UInt16) -> Shield
	public var createNewSecurityShield: CreateNewSecurityShield
}


extension ShieldClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		Self(
			createNewSecurityShield: { matrixOfFactors, name, numberOfDaysToAutoConfirm in
		
			}
		)
	}
}
extension ShieldClient {
    public func createNewShield(
        matrix: MatrixOfFactorSources,
        name: DisplayName,
        numberOfDaysToAutoConfirm: UInt16 = 14
    ) -> Shield {
        self.createNewSecurityShield(matrix, name, numberOfDaysToAutoConfirm)
    }
}

