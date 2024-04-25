import SargonUniFFI

#if DEBUG
extension Entropy32Bytes {
	public static let sample: Self = newEntropy32BytesSample()
	public static let sampleOther: Self = newEntropy32BytesSampleOther()
}
#endif // DEBUG
