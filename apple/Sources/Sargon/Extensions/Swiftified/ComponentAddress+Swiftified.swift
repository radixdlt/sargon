extension ComponentAddress: @unchecked Sendable {}

#if DEBUG
	extension ComponentAddress {
		/// Namespace for preview values of `ComponentAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnet: ComponentAddress = newComponentAddressSample()

			public let mainnetOther: ComponentAddress =
				newComponentAddressSampleOther()

		}

		/// Preview values for `ComponentAddress`, e.g.:
		/// `ComponentAddress.preview.mainnet`
		/// or
		/// `ComponentAddress.preview.mainnetOther`
		public static let preview = Preview.of
	}
#endif

#if DEBUG
	extension ComponentAddress: CaseIterable {
		public typealias AllCases = [Self]
		public static var allCases: AllCases {
			let of = Preview.of
			return [
				of.mainnet,
				of.mainnetOther,
			]
		}
	}
#endif
