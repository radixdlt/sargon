import SargonUniFFI

extension ThirdPartyDeposits: SargonModel {}

extension ThirdPartyDeposits {
	public var isAssetsExceptionsUnknown: Bool {
		assetsExceptionList == nil
	}

	public var isAllowedDepositorsUnknown: Bool {
		depositorsAllowList == nil
	}
}
