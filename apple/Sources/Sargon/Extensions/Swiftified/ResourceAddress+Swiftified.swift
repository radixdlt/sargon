extension ResourceAddress: @unchecked Sendable {}

#if DEBUG
	extension ResourceAddress {
		/// Namespace for preview values of `ResourceAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnetXRD: ResourceAddress =
				newResourceAddressSampleMainnetXrd()
			public let mainnetCandy: ResourceAddress =
				newResourceAddressSampleMainnetCandy()
			/// Gumball Club membership NFT resource address
			public let mainnetNonFungbleGCMembership: ResourceAddress =
				newResourceAddressSampleMainnetNftGcMembership()

			public let stokenetXRD: ResourceAddress =
				newResourceAddressSampleStokenetXrd()
			public let stokenetGum: ResourceAddress =
				newResourceAddressSampleStokenetGum()
			public let stokenetGC: ResourceAddress =
				newResourceAddressSampleStokenetGcTokens()
			public let stokenetCandy: ResourceAddress =
				newResourceAddressSampleStokenetCandy()
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
