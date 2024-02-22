extension ValidatorAddress: @unchecked Sendable {}


#if DEBUG
	extension ValidatorAddress {
		/// Namespace for preview values of `ValidatorAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnet: ValidatorAddress =
				newValidatorAddressPlaceholderMainnet()

			public let mainnetOther: ValidatorAddress =
				newValidatorAddressPlaceholderMainnetOther()

			public let stokenet: ValidatorAddress =
				newValidatorAddressPlaceholderStokenet()

			public let stokenetOther: ValidatorAddress =
				newValidatorAddressPlaceholderStokenetOther()

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