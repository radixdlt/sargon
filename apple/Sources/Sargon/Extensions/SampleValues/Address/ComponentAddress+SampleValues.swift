import SargonUniFFI

#if DEBUG
extension ComponentAddress {
	public static let sampleMainnet: Self = newComponentAddressSampleMainnetGlobal()
	public static let sampleMainnetOther: Self = newComponentAddressSampleMainnetInternal()

	public static let sampleStokenet: Self = newComponentAddressSampleStokenetGlobal()
	public static let sampleStokenetOther: Self = newComponentAddressSampleStokenetInternal()
}
#endif
