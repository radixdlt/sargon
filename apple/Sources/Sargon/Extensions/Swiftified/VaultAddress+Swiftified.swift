extension VaultAddress: @unchecked Sendable {}

#if DEBUG
	extension VaultAddress {
		/// Namespace for preview values of `VaultAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnetFungible: VaultAddress =
				newVaultAddressSampleMainnetFungible()

			public let mainnetNonFungible: VaultAddress =
				newVaultAddressSampleMainnetNonFungible()

			public let stokenetFungible: VaultAddress =
				newVaultAddressSampleStokenetFungible()

			public let stokenetNonFungible: VaultAddress =
				newVaultAddressSampleStokenetNonFungible()

		}

		/// Preview values for `VaultAddress`, e.g.:
		/// `VaultAddress.preview.mainnetFungible`
		/// or
		/// `VaultAddress.preview.stokenetNonFungible`
		public static let preview = Preview.of
	}
#endif

#if DEBUG
	extension VaultAddress: CaseIterable {
		public typealias AllCases = [Self]
		public static var allCases: AllCases {
			let of = Preview.of
			return [
				of.mainnetFungible,
				of.mainnetNonFungible,
				of.stokenetFungible,
				of.stokenetNonFungible,
			]
		}
	}
#endif
