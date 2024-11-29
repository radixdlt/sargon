import Foundation
import SargonUniFFI

// MARK: - SecurityStructureOfFactorSources + SargonModel
extension SecurityStructureOfFactorSources: SargonModel {}

// MARK: - SecurityStructureOfFactorSources + Identifiable
extension SecurityStructureOfFactorSources: Identifiable {
	public typealias ID = UUID
	public var id: ID {
		metadata.id
	}
}
