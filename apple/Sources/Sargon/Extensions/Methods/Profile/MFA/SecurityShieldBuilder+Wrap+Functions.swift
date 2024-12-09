import Foundation
import SargonUniFFI

extension SecurityShieldBuilder {
	public static func sortFactorSourcesForSelection(factorSources: [FactorSource]) -> [FactorSource] {
		securityShieldBuilderSortFactorSourcesForSelection(factorSources: factorSources)
	}

	public static func selectedFactorSourcesStatus(factorSources: [FactorSource]) -> SelectedFactorSourcesStatus {
		securityShieldBuilderSelectedFactorSourcesStatus(factorSources: factorSources)
	}
}
