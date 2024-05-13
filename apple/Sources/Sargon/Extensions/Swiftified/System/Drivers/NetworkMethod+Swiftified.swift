import Foundation
import Sargon

extension NetworkMethod: SargonModel {}
extension NetworkMethod: CustomStringConvertible {
	public var description: String {
		toString()
	}
}
