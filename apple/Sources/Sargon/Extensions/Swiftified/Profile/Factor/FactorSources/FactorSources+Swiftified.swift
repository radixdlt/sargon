import SargonUniFFI

extension Array: CaseIterable where Element == FactorSource {
	
}

extension [FactorSource]: BaseSargonModel {}
extension [FactorSource]: SargonModel {}
