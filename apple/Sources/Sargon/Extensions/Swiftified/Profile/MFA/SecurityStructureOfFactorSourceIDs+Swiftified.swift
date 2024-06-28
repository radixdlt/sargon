import Foundation
import SargonUniFFI

extension SecurityStructureOfFactorSourceIDs: SargonModel {}
extension SecurityStructureOfFactorSourceIDs: Identifiable {
	public typealias ID = UUID
	public var id: ID {
		metadata.id
	}
}
