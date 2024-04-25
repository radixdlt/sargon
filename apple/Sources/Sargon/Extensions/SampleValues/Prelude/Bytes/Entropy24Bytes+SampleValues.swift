import SargonUniFFI

#if DEBUG
extension Entropy24Bytes {
	public static let sample: Self = newEntropy24BytesSample()
	public static let sampleOther: Self = newEntropy24BytesSampleOther()
}
#endif // DEBUG
