public typealias NetworkID = NetworkId

extension NetworkID: @unchecked Sendable {}

extension NetworkID: SargonModel {}

#if DEBUG
extension NetworkID {
	public static var sample: Self {
		.mainnet
	}
	
	public static var sampleOther: Self {
		.stokenet
	}
}
#endif // DEBUG
