extension IdentityAddress: @unchecked Sendable {}


#if DEBUG
	extension IdentityAddress {
		/// Namespace for preview values of `IdentityAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnet: IdentityAddress = newIdentityAddressPlaceholderMainnet()
			public let mainnetOther: IdentityAddress =
				newIdentityAddressPlaceholderMainnetOther()

			public let stokenet: IdentityAddress =
				newIdentityAddressPlaceholderStokenet()
			public let stokenetOther: IdentityAddress =
				newIdentityAddressPlaceholderStokenetOther()
		}

		/// Preview values for `IdentityAddress`, e.g.:
		/// `IdentityAddress.preview.mainnet`
		/// or
		/// `IdentityAddress.preview.stokenetOther`
		public static let preview = Preview.of
	}
#endif

#if DEBUG
	extension IdentityAddress: CaseIterable {
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
