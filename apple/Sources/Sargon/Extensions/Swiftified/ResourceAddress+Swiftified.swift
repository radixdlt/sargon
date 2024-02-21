extension ResourceAddress: @unchecked Sendable {}


#if DEBUG
	extension ResourceAddress {
		/// Namespace for preview values of `ResourceAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnetXRD: ResourceAddress =
				newResourceAddressPlaceholderMainnetXrd()
			public let mainnetCandy: ResourceAddress =
				newResourceAddressPlaceholderMainnetCandy()
			/// Gumball Club membership NFT resource address
			public let mainnetNonFungbleGCMembership: ResourceAddress =
				newResourceAddressPlaceholderMainnetNftGcMembership()

			public let stokenetXRD: ResourceAddress =
				newResourceAddressPlaceholderStokenetXrd()
			public let stokenetGum: ResourceAddress =
				newResourceAddressPlaceholderStokenetGum()
			public let stokenetGC: ResourceAddress =
				newResourceAddressPlaceholderStokenetGcTokens()
			public let stokenetCandy: ResourceAddress =
				newResourceAddressPlaceholderStokenetCandy()
		}

		/// Preview values for `ResourceAddress`, e.g.:
		/// `ResourceAddress.preview.mainnetXRD`
		/// or
		/// `ResourceAddress.preview.stokenetCandy`
		public static let preview = Preview.of
	}
#endif

#if DEBUG
	extension ResourceAddress: CaseIterable {
		public typealias AllCases = [Self]
		public static var allCases: AllCases {
			let of = Preview.of
			return [
				of.mainnetXRD,
				of.mainnetCandy,
				of.mainnetNonFungbleGCMembership,
				of.stokenetXRD,
				of.stokenetGum,
				of.stokenetGC,
				of.stokenetCandy,
			]
		}
	}
#endif
