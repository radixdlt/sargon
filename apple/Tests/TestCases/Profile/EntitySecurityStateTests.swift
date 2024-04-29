import CustomDump
import Foundation
import Sargon
import SargonUniFFI
import XCTest

// MARK: - EntitySecurityStateTests
final class EntitySecurityStateTests: Test<EntitySecurityState> {}

#if DEBUG
extension EntitySecurityState {
	public var unsecured: UnsecuredEntityControl {
		get {
			switch self {
			case let .unsecured(value):
				value
			}
		}
		set {
			self = .unsecured(value: newValue)
		}
	}
}
#endif // DEBUG
