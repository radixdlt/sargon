extension AccessControllerAddress: @unchecked Sendable {}


#if DEBUG
extension AccessControllerAddress {
	public static let sample: Self = newAccessControllerAddressSample()
	public static let sampleOther: Self = newAccessControllerAddressSampleOther()
}
#endif

