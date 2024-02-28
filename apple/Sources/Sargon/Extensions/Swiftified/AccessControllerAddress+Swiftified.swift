extension AccessControllerAddress: @unchecked Sendable {}

#if DEBUG
	extension AccessControllerAddress {
		/// Namespace for preview values of `AccessControllerAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let first: AccessControllerAddress =
				newAccessControllerAddressSample()

			public let second: AccessControllerAddress =
				newAccessControllerAddressSampleOther()

		}

		/// Preview values for `AccessControllerAddress`, e.g.:
		/// `AccessControllerAddress.preview.first`
		/// or
		/// `AccessControllerAddress.preview.second`
		public static let preview = Preview.of
	}
#endif

#if DEBUG
	extension AccessControllerAddress: CaseIterable {
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
