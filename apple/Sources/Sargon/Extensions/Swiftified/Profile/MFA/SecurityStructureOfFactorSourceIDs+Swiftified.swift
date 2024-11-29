import Foundation
import SargonUniFFI

// MARK: - SecurityStructureOfFactorSourceIDs + SargonModel
extension SecurityStructureOfFactorSourceIDs: SargonModel {}

// MARK: - SecurityStructureOfFactorSourceIDs + Identifiable
extension SecurityStructureOfFactorSourceIDs: Identifiable {
	public typealias ID = UUID
	public var id: ID {
		metadata.id
	}
}
