extension AccountAddress: @unchecked Sendable {}


#if DEBUG
	extension AccountAddress {
		/// Namespace for preview values of `AccountAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnet: AccountAddress = newAccountAddressPlaceholderMainnet()
			public let mainnetOther: AccountAddress =
				newAccountAddressPlaceholderMainnetOther()

			public let stokenet: AccountAddress = newAccountAddressPlaceholderStokenet()
			public let stokenetOther: AccountAddress =
				newAccountAddressPlaceholderStokenetOther()
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
