extension ValidatorAddress: @unchecked Sendable {}

#if DEBUG
	extension ValidatorAddress {
		/// Namespace for preview values of `ValidatorAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnet: ValidatorAddress =
				newValidatorAddressSampleMainnet()

			public let mainnetOther: ValidatorAddress =
				newValidatorAddressSampleMainnetOther()

			public let stokenet: ValidatorAddress =
				newValidatorAddressSampleStokenet()

			public let stokenetOther: ValidatorAddress =
				newValidatorAddressSampleStokenetOther()

		}

		/// Preview values for `ValidatorAddress`, e.g.:
		/// `ValidatorAddress.preview.mainnet`
		/// or
		/// `ValidatorAddress.preview.stokenetOther`
		public static let preview = Preview.of
	}
#endif

#if DEBUG
	extension ValidatorAddress: CaseIterable {
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
