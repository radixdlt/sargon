
#if DEBUG
extension FactorSource {
	public static let sample: Self = newFactorSourceSample()
	public static let sampleOther: Self = newFactorSourceSampleOther()
}
#endif // DEBUG

extension [FactorSource]: SargonModel {
	
}

#if DEBUG
extension [FactorSource] {
	public static let sample: Self = newFactorSourcesSample()
	public static let sampleOther: Self = newFactorSourcesSampleOther()
}
#endif // DEBUG
