import Foundation
import SargonUniFFI

extension SecurityProblem {
	public var kind: SecurityProblemKind {
		securityProblemKind(value: self)
	}
}

// MARK: - SecurityProblem + Identifiable
extension SecurityProblem: Identifiable {
	public var id: UInt64 {
		securityProblemId(value: self)
	}
}
