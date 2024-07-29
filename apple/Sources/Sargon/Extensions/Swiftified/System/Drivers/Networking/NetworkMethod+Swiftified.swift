import Foundation
import SargonUniFFI

extension NetworkMethod: SargonModel {}
extension NetworkMethod: CustomStringConvertible {
	public var description: String {
		toString()
	}
}
