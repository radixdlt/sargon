import Foundation
import SargonUniFFI

extension SecurityProblemKind: CaseIterable {
	public static var allCases: [SecurityProblemKind] {
		[.configurationBackup, .securityFactors]
	}
}
