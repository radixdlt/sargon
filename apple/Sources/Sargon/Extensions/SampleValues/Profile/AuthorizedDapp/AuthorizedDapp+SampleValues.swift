import SargonUniFFI

#if DEBUG
extension AuthorizedDapp {
	public static let sample = Self.sampleMainnet
	public static let sampleOther = Self.sampleMainnetOther

	public static let sampleMainnet: Self = newAuthorizedDappSampleMainnetDashboard()
	public static let sampleMainnetOther: Self = newAuthorizedDappSampleMainnetGumballclub()

	public static let sampleStokenet: Self = newAuthorizedDappSampleStokenetSandbox()
	public static let sampleStokenetOther: Self = newAuthorizedDappSampleStokenetDevconsole()
}
#endif // DEBUG
