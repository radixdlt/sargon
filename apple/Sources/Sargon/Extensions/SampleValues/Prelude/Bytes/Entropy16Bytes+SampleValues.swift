import SargonUniFFI

#if DEBUG
extension Entropy16Bytes {
	public static let sample: Self = newEntropy16BytesSample()
	public static let sampleOther: Self = newEntropy16BytesSampleOther()
}
#endif // DEBUG
