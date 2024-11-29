import Foundation
import SargonUniFFI

// MARK: - NetworkMethod + SargonModel
extension NetworkMethod: SargonModel {}

// MARK: - NetworkMethod + CustomStringConvertible
extension NetworkMethod: CustomStringConvertible {
	public var description: String {
		toString()
	}
}
