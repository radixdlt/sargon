import Foundation
import SargonUniFFI

// MARK: - AssetException + SargonModel
extension AssetException: SargonModel {}

// MARK: - AssetException + Identifiable
extension AssetException: Identifiable {
	public typealias ID = ResourceAddress
	public var id: ID {
		address
	}
}
