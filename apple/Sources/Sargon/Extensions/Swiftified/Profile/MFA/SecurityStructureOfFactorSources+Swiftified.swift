import Foundation
import SargonUniFFI

extension SecurityStructureOfFactorSources: SargonModel {}
extension SecurityStructureOfFactorSources: Identifiable {
	public typealias ID = UUID
	public var id: ID {
		metadata.id
	}
}
