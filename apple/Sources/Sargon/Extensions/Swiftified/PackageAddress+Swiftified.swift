extension PackageAddress: @unchecked Sendable {}


#if DEBUG
	extension PackageAddress {
		/// Namespace for preview values of `PackageAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let first: PackageAddress = newPackageAddressPlaceholder()
			public let second: PackageAddress = newPackageAddressPlaceholderOther()
		}

		/// Preview values for `PackageAddress`, e.g.:
		/// `PackageAddress.preview.first`
		/// or
		/// `PackageAddress.preview.second`
		public static let preview = Preview.of
	}
#endif

#if DEBUG
	extension PackageAddress: CaseIterable {
		public typealias AllCases = [Self]
		public static var allCases: AllCases {
			let of = Preview.of
			return [
				of.first,
				of.second,
			]
		}
	}
#endif
