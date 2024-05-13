import Foundation
import SargonUniFFI

extension NetworkMethod {
	public func toString() -> String {
		networkMethodToString(method: self)
	}
}
