import Foundation
import SargonUniFFI

// MARK: - SecurityStructureMetadata + SargonModel
extension SecurityStructureMetadata: SargonModel {}

// MARK: - SecurityStructureMetadata + Identifiable
extension SecurityStructureMetadata: Identifiable {
	public typealias ID = UUID
}
