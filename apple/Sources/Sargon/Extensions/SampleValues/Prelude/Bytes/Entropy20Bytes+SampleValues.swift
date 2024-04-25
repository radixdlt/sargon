import SargonUniFFI

#if DEBUG
extension Entropy20Bytes {
	public static let sample: Self = newEntropy20BytesSample()
	public static let sampleOther: Self = newEntropy20BytesSampleOther()
}
#endif // DEBUG
