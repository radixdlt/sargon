import SargonUniFFI
import Foundation

extension Gateway: SargonModel {}
extension Gateway: CustomStringConvertible {
	public var description: String {
		toString()
	}
}
extension Gateway: Identifiable {
	public typealias ID = URL
	public var id: ID {
		getID()
	}
}

