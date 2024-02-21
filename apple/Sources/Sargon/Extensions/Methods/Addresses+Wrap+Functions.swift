public protocol AddressProtocol: Sendable, CustomStringConvertible {
	init(validatingAddress bech32String: String) throws
	var networkID: NetworkID { get }
	var address: String { get }
}
extension AddressProtocol {
	public var description: String {
		address
	}
}

public protocol EntityAddressProtocol: AddressProtocol {
	init(publicKey: PublicKey, networkID: NetworkID)
}

extension AccountAddress: EntityAddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newAccountAddress(bech32: bech32String)
	}

	public init(publicKey: PublicKey, networkID: NetworkID) {
		self = newAccountAddressFrom(
			publicKey: publicKey,
			networkId: networkID
		)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		accountAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		accountAddressNetworkId(address: self)
	}

	/// Returns `false` for all addresses created with `Ed25519PublicKey`s, i.e.
	/// for all accounts created by the Babylon Radix Wallets.
	/// Returns `true` for all addresses created with `Secp256k1PublicKey`s, i.e.
	/// imported from the Olympia Wallet.
	public var isLegacy: Bool {
		accountAddressIsLegacy(address: self)
	}

	/// Formats the AccountAddress to its abbreviated form which is what the user
	/// is most used to, since it is what we most commonly display in the Radix
	/// ecosystem.
	///
	/// The abbreviated form returns:
	///
	/// `acco...please`
	///
	/// For the account address:
	///
	/// `account_rdx16xlfcpp0vf7e3gqnswv8j9k58n6rjccu58vvspmdva22kf3aplease`
	///
	public var shortFormat: String {
		accountAddressToShort(address: self)
	}
}
#if DEBUG
	extension AccountAddress {
		/// Namespace for preview values of `AccountAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnet: AccountAddress = newAccountAddressPlaceholderMainnet()
			public let mainnetOther: AccountAddress =
				newAccountAddressPlaceholderMainnetOther()

			public let stokenet: AccountAddress = newAccountAddressPlaceholderStokenet()
			public let stokenetOther: AccountAddress =
				newAccountAddressPlaceholderStokenetOther()
		}

		/// Preview values for `AccountAddress`, e.g.:
		/// `AccountAddress.preview.mainnet`
		/// or
		/// `AccountAddress.preview.stokenetOther`
		public static let preview = Preview.of
	}
#endif

#if DEBUG
	extension AccountAddress: CaseIterable {
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

extension IdentityAddress: EntityAddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newIdentityAddress(bech32: bech32String)
	}

	public init(publicKey: PublicKey, networkID: NetworkID) {
		self = newIdentityAddressFrom(
			publicKey: publicKey,
			networkId: networkID
		)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		identityAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		identityAddressNetworkId(address: self)
	}
}

#if DEBUG
	extension IdentityAddress {
		/// Namespace for preview values of `IdentityAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnet: IdentityAddress = newIdentityAddressPlaceholderMainnet()
			public let mainnetOther: IdentityAddress =
				newIdentityAddressPlaceholderMainnetOther()

			public let stokenet: IdentityAddress =
				newIdentityAddressPlaceholderStokenet()
			public let stokenetOther: IdentityAddress =
				newIdentityAddressPlaceholderStokenetOther()
		}

		/// Preview values for `IdentityAddress`, e.g.:
		/// `IdentityAddress.preview.mainnet`
		/// or
		/// `IdentityAddress.preview.stokenetOther`
		public static let preview = Preview.of
	}
#endif

#if DEBUG
	extension IdentityAddress: CaseIterable {
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

extension PackageAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newPackageAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		packageAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		packageAddressNetworkId(address: self)
	}
}


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


extension ResourceAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newResourceAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		resourceAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		resourceAddressNetworkId(address: self)
	}

	/// If this is an address of a **fungible** resource or not.
	public var isFungible: Bool {
		resourceAddressIsFungible(address: self)
	}

	/// If this is an address of a **non-fungible** resource or not.
	public var isNonFungible: Bool {
		resourceAddressIsNonFungible(address: self)
	}
}

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

#if DEBUG
	/// TODO: Declare in Rust land? Make non-DEBUG?
	public enum Address: Hashable, Equatable, Sendable {
		case accesscontroller(AccessControllerAddress)
		case account(AccountAddress)
		case component(ComponentAddress)
		case identity(IdentityAddress)
		case package(PackageAddress)
		case pool(PoolAddress)
		case resource(ResourceAddress)
		case validator(ValidatorAddress)
		case vault(VaultAddress)
	}
	extension Address: AddressProtocol {
		public init(validatingAddress bech32String: String) throws {
			if let address = try? AccessControllerAddress(
				validatingAddress: bech32String)
			{
				self = .accesscontroller(address)
			} else if let address = try? AccountAddress(validatingAddress: bech32String)
			{
				self = .account(address)
			} else if let address = try? ComponentAddress(
				validatingAddress: bech32String)
			{
				self = .component(address)
			} else if let address = try? IdentityAddress(
				validatingAddress: bech32String)
			{
				self = .identity(address)
			} else if let address = try? PackageAddress(validatingAddress: bech32String)
			{
				self = .package(address)
			} else if let address = try? PoolAddress(validatingAddress: bech32String) {
				self = .pool(address)
			} else if let address = try? ResourceAddress(
				validatingAddress: bech32String)
			{
				self = .resource(address)
			} else if let address = try? ValidatorAddress(
				validatingAddress: bech32String)
			{
				self = .validator(address)
			} else if let address = try? VaultAddress(validatingAddress: bech32String) {
				self = .vault(address)
			} else {
				struct UnknownAddressType: Swift.Error {}
				throw UnknownAddressType()
			}
		}

		public var networkID: NetworkID {
			property(\.networkID)
		}

		public var address: String {
			property(\.address)
		}

		private func property<Property>(_ keyPath: KeyPath<any AddressProtocol, Property>)
			-> Property
		{
			switch self {
			case let .accesscontroller(address): address[keyPath: keyPath]
			case let .account(address): address[keyPath: keyPath]
			case let .component(address): address[keyPath: keyPath]
			case let .identity(address): address[keyPath: keyPath]
			case let .package(address): address[keyPath: keyPath]
			case let .pool(address): address[keyPath: keyPath]
			case let .resource(address): address[keyPath: keyPath]
			case let .validator(address): address[keyPath: keyPath]
			case let .vault(address): address[keyPath: keyPath]
			}
		}
	}
	extension Address: CaseIterable {
		public typealias AllCases = [Self]
		public static var allCases: AllCases {
			AccountAddress.allCases.map(Self.account)
				+ AccessControllerAddress.allCases.map(Self.accesscontroller)
				+ ComponentAddress.allCases.map(Self.component)
				+ IdentityAddress.allCases.map(Self.identity)
				+ PackageAddress.allCases.map(Self.package)
				+ PoolAddress.allCases.map(Self.pool)
				+ ResourceAddress.allCases.map(Self.resource)
				+ ValidatorAddress.allCases.map(Self.validator)
				+ VaultAddress.allCases.map(Self.vault)
		}
	}
#endif

extension VaultAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newVaultAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		vaultAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		vaultAddressNetworkId(address: self)
	}

	/// If this is an address of a **fungible** vault or not.
	public var isFungible: Bool {
		vaultAddressIsFungible(address: self)
	}

	/// If this is an address of a **non-fungible** vault or not.
	public var isNonFungible: Bool {
		vaultAddressIsNonFungible(address: self)
	}
}

#if DEBUG
	extension VaultAddress {
		/// Namespace for preview values of `VaultAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnetFungible: VaultAddress =
				newVaultAddressPlaceholderMainnetFungible()

			public let mainnetNonFungible: VaultAddress =
				newVaultAddressPlaceholderMainnetNonFungible()

			public let stokenetFungible: VaultAddress =
				newVaultAddressPlaceholderStokenetFungible()

			public let stokenetNonFungible: VaultAddress =
				newVaultAddressPlaceholderStokenetNonFungible()

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

extension PoolAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newPoolAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		poolAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		poolAddressNetworkId(address: self)
	}

	/// Returns the kind of pool, either 1, 2 or Multi resources.
	public var poolKind: PoolKind {
		poolAddressKind(address: self)
	}
}

#if DEBUG
	extension PoolAddress {
		/// Namespace for preview values of `PoolAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnetSingle: PoolAddress = newPoolAddressPlaceholderSingle()

			public let mainnetTwo: PoolAddress = newPoolAddressPlaceholderTwo()

			public let mainnetMulti: PoolAddress = newPoolAddressPlaceholderMulti()

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

extension ValidatorAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newValidatorAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		validatorAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		validatorAddressNetworkId(address: self)
	}
}

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

extension ComponentAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newComponentAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		componentAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		componentAddressNetworkId(address: self)
	}
}

#if DEBUG
	extension ComponentAddress {
		/// Namespace for preview values of `ComponentAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let mainnet: ComponentAddress = newComponentAddressPlaceholder()

			public let mainnetOther: ComponentAddress =
				newComponentAddressPlaceholderOther()

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

extension AccessControllerAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newAccesscontrollerAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		accesscontrollerAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		accesscontrollerAddressNetworkId(address: self)
	}
}

#if DEBUG
	extension AccessControllerAddress {
		/// Namespace for preview values of `AccessControllerAddress`
		public struct Preview {
			fileprivate init() {}
			public static let of = Self()

			public let first: AccessControllerAddress =
				newAccesscontrollerAddressPlaceholder()

			public let second: AccessControllerAddress =
				newAccesscontrollerAddressPlaceholderOther()

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
