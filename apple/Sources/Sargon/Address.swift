
#if DEBUG
/// TODO: Declare in Rust land? Make non-DEBUG?
public enum Address: Hashable, Equatable, Sendable {
	case accesscontroller(AccessControllerAddress)
	case account(AccountAddress)
	case component(ComponentAddress)
	case identity(IdentityAddress)
	case package(PackageAddress)
	case pool(PoolAddress)
	
	/// Both Non-Fungible and Fungible Resource addresses
	case resource(ResourceAddress)
	
	/// Only Non-Fungible Resource addresses
	case nonFungibleResource(NonFungibleResourceAddress)
	
	case validator(ValidatorAddress)
	case vault(VaultAddress)
}
extension Address: AddressProtocol {
	
	public static let sampleMainnet = Self.account(.sampleMainnet)
	public static let sampleMainnetOther = Self.resource(.sampleMainnetOther)
	public static let sampleStokenet = Self.account(.sampleStokenet)
	public static let sampleStokenetOther = Self.resource(.sampleStokenetOther)
	
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
		} else if let address = try? ValidatorAddress(
			validatingAddress: bech32String)
		{
			self = .validator(address)
		} else if let address = try? VaultAddress(validatingAddress: bech32String) {
			self = .vault(address)
		} else if let address = try? NonFungibleResourceAddress(validatingAddress: bech32String) /* Must try `NonFungibleResourceAddress` before ResourceAddress */ {
			self = .nonFungibleResource(address)
		} else if let address = try? ResourceAddress(
			validatingAddress: bech32String)
		{
			self = .resource(address)
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
	
	private func property<Property>(
		_ keyPath: KeyPath<any AddressProtocol, Property>
	) -> Property {
		switch self {
		case let .accesscontroller(address): address[keyPath: keyPath]
		case let .account(address): address[keyPath: keyPath]
		case let .component(address): address[keyPath: keyPath]
		case let .identity(address): address[keyPath: keyPath]
		case let .package(address): address[keyPath: keyPath]
		case let .pool(address): address[keyPath: keyPath]
		case let .resource(address): address[keyPath: keyPath]
		case let .nonFungibleResource(address): address[keyPath: keyPath]
		case let .validator(address): address[keyPath: keyPath]
		case let .vault(address): address[keyPath: keyPath]
		}
	}
}
extension Address: CaseIterable {
	public typealias AllCases = [Self]
	public static var allCases: AllCases {
		var addresses: [Self] = []
		
		// Using `+` operator results in Swift compiler dying.
		addresses.append(contentsOf: AccountAddress.allCases.map(Self.account))
		addresses.append(contentsOf: AccessControllerAddress.allCases.map(Self.accesscontroller))
		addresses.append(contentsOf: ComponentAddress.allCases.map(Self.component))
		addresses.append(contentsOf: IdentityAddress.allCases.map(Self.identity))
		addresses.append(contentsOf: PackageAddress.allCases.map(Self.package))
		addresses.append(contentsOf: PoolAddress.allCases.map(Self.pool))
		addresses.append(contentsOf: ResourceAddress.allCases.map(Self.resource))
		addresses.append(contentsOf: NonFungibleResourceAddress.allCases.map(Self.nonFungibleResource))
		addresses.append(contentsOf: ValidatorAddress.allCases.map(Self.validator))
		addresses.append(contentsOf: VaultAddress.allCases.map(Self.vault))
		
		return addresses
	}
}
#endif
