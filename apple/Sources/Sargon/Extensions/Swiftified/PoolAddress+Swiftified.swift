extension PoolAddress: @unchecked Sendable {}

#if DEBUG
	extension PoolAddress {
		/// Namespace for preview values of `PoolAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnetSingle: PoolAddress = newPoolAddressSampleSingle()

			public let mainnetTwo: PoolAddress = newPoolAddressSampleTwo()

			public let mainnetMulti: PoolAddress = newPoolAddressSampleMulti()

		}

		/// Preview values for `PoolAddress`, e.g.:
		/// `PoolAddress.preview.mainnetSingle`
		/// or
		/// `PoolAddress.preview.mainnetMulti`
		public static let preview = Preview.of
	}
#endif

#if DEBUG
	extension PoolAddress: CaseIterable {
		public typealias AllCases = [Self]
		public static var allCases: AllCases {
			let of = Preview.of
			return [
				of.mainnetSingle,
				of.mainnetTwo,
				of.mainnetMulti,
			]
		}
	}
#endif
