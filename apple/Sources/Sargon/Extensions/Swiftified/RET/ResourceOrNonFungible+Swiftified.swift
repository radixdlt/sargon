import Foundation
import SargonUniFFI

// MARK: - ResourceOrNonFungible + SargonModel
extension ResourceOrNonFungible: SargonModel {}

// MARK: - ResourceOrNonFungible + Identifiable
extension ResourceOrNonFungible: Identifiable {
	public typealias ID = Self
	public var id: ID {
		self
	}
}

extension ResourceOrNonFungible {
	public var resourceAddress: ResourceAddress {
		switch self {
		case let .nonFungible(value): value.resourceAddress
		case let .resource(value): value
		}
	}
}
