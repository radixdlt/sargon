import SargonUniFFI

extension ThirdPartyDeposits: SargonModel {}

extension ThirdPartyDeposits {

	/// With `assetsExceptionList` and `depositorsAllowList` set to `nil`, marking they are unknown.
	public static func accountRecoveryScanned(depositRule: DepositRule = .acceptAll) -> Self {
		Self.init(depositRule: depositRule, assetsExceptionList: nil, depositorsAllowList: nil)
	}
	
	public var isAssetsExceptionsUnknown: Bool {
		assetsExceptionList == nil
	}

	public var isAllowedDepositorsUnknown: Bool {
		depositorsAllowList == nil
	}
}
