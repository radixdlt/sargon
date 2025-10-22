import SargonUniFFI

#if DEBUG
extension SargonError {
	public static let sample: Self = .Unknown(errorMessage: "Sample error")
	public static let sampleOther: Self = .BytesEmpty
}
#endif // DEBUG
