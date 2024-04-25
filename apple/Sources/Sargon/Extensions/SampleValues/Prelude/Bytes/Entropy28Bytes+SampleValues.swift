import SargonUniFFI

#if DEBUG
extension Entropy28Bytes {
	public static let sample: Self = newEntropy28BytesSample()
	public static let sampleOther: Self = newEntropy28BytesSampleOther()
}
#endif // DEBUG
