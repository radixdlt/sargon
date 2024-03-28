import SargonUniFFI

#if DEBUG
extension Nonce {
    public static let sample: Self = newNonceSample()
    public static let sampleOther: Self = newNonceSampleOther()
}
#endif // DEBUG
