extension PackageAddress: @unchecked Sendable {}

#if DEBUG
extension PackageAddress {
	public static let sample: Self = newPackageAddressSample()
	public static let sampleOther: Self = newPackageAddressSampleOther()
}
#endif
