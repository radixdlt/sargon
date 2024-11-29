import Foundation
import SargonUniFFI

extension EntityFlag: SargonModel & Identifiable {
	public typealias ID = Self
	public var id: ID {
		self
	}
}
