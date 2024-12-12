import Foundation
import SargonUniFFI

extension SecurityProblemKind: CaseIterable {
	public static var allCases: [SecurityProblemKind] {
		[.securityShields, .securityFactors, .configurationBackup]
	}
}
