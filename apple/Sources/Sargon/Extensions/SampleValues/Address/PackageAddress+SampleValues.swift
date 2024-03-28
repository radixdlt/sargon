import SargonUniFFI

#if DEBUG
extension PackageAddress {
	public static let sampleMainnet: Self = newPackageAddressSampleMainnet()
	public static let sampleMainnetOther: Self = newPackageAddressSampleMainnetOther()
	
	public static let sampleStokenet: Self = newPackageAddressSampleStokenet()
	public static let sampleStokenetOther: Self = newPackageAddressSampleStokenetOther()
}
#endif
