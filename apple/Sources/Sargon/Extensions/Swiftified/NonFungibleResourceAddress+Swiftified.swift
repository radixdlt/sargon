extension NonFungibleResourceAddress: @unchecked Sendable {}

#if DEBUG
    extension NonFungibleResourceAddress {
        /// Namespace for preview values of `NonFungibleResourceAddress`
        public struct Preview {
            fileprivate init() {}
            public static let of = Self()

            public let mainnet: NonFungibleResourceAddress = newNonFungibleResourceAddressSampleMainnet()
            public let mainnetOther: NonFungibleResourceAddress =
            newNonFungibleResourceAddressSampleMainnetOther()

            public let stokenet: NonFungibleResourceAddress = newNonFungibleResourceAddressSampleStokenet()
            public let stokenetOther: NonFungibleResourceAddress =
            newNonFungibleResourceAddressSampleStokenetOther()
        }

        /// Preview values for `NonFungibleResourceAddress`, e.g.:
        /// `NonFungibleResourceAddress.preview.mainnet`
        /// or
        /// `NonFungibleResourceAddress.preview.stokenetOther`
        public static let preview = Preview.of
    }
#endif

#if DEBUG
    extension NonFungibleResourceAddress: CaseIterable {
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
