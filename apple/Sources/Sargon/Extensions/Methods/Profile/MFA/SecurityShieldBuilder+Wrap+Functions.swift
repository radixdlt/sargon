import Foundation
import SargonUniFFI

extension SecurityShieldBuilder {
	public static func sortedFactorSourcesForSelection(factorSources: [FactorSource]) -> [FactorSource] {
		securityShieldBuilderSortedFactorSourcesForSelection(factorSources: factorSources)
	}
}
