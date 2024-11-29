import Foundation
import SargonUniFFI

extension SecurityProblem {
	public var kind: SecurityProblemKind {
		securityProblemKind(value: self)
	}
}
