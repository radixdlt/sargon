extension AccountAddress: @unchecked Sendable {}

#if DEBUG
	extension AccountAddress {
		/// Namespace for preview values of `AccountAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnet: AccountAddress = newAccountAddressSampleMainnet()
			public let mainnetOther: AccountAddress =
				newAccountAddressSampleMainnetOther()

			public let stokenet: AccountAddress = newAccountAddressSampleStokenet()
			public let stokenetOther: AccountAddress =
				newAccountAddressSampleStokenetOther()
		}

		/// Preview values for `AccountAddress`, e.g.:
		/// `AccountAddress.preview.mainnet`
		/// or
		/// `AccountAddress.preview.stokenetOther`
		public static let preview = Preview.of
	}
#endif

#if DEBUG
	extension AccountAddress: CaseIterable {
		public typealias AllCases = [Self]
		public static var allCases: AllCases {
			let of = Preview.of
			return [
				of.mainnet,
				of.mainnetOther,
				of.stokenet,
				of.stokenetOther,
			]
		}
	}
#endif
