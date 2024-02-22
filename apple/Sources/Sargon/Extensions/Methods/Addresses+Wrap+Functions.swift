public protocol AddressProtocol: CustomStringConvertible {
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

extension AccessControllerAddress: AddressProtocol {
	public init(validatingAddress bech32String: String) throws {
		self = try newAccessControllerAddress(bech32: bech32String)
	}

	/// The bech32 encoded string for this address.
	public var address: String {
		accessControllerAddressBech32Address(address: self)
	}

	public var networkID: NetworkId {
		accessControllerAddressNetworkId(address: self)
	}
}
