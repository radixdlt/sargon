import SargonUniFFI

#if DEBUG
public protocol BaseBaseEntityProtocol: SargonModel {
	static var sampleValues: [Self] { get }
}
#else
public protocol BaseBaseEntityProtocol: SargonModel {}
#endif // DEBUG

public protocol EntityBaseProtocol: BaseBaseEntityProtocol, CustomStringConvertible, Identifiable where ID == EntityAddress {
	associatedtype EntityAddress: BaseEntityAddressProtocol
	var networkId: NetworkID { get }
	var displayName: DisplayName { get }
	var address: EntityAddress { get }
	var flags: EntityFlags { get }
	var securityState: EntitySecurityState { get }
	var entityKind: EntityKind { get }
	var asGeneral: AccountOrPersona { get }
	
#if DEBUG
	static var sampleMainnet: Self { get }
	static var sampleMainnetOther: Self { get }
	static var sampleMainnetThird: Self { get }
	static var sampleStokenet: Self { get }
	static var sampleStokenetOther: Self { get }
	static var sampleStokenetThird: Self { get }
#endif // DEBUG
}

extension EntityBaseProtocol {
	public var id: ID { address }
	public var networkID: NetworkID { networkId }
	
	public var isHidden: Bool {
		flags.contains(.deletedByUser)
	}
	
	public var virtualHierarchicalDeterministicFactorInstances: Set<HierarchicalDeterministicFactorInstance> {
		var factorInstances = Set<HierarchicalDeterministicFactorInstance>()
		switch securityState {
		case let .unsecured(unsecuredEntityControl):
			factorInstances.insert(unsecuredEntityControl.transactionSigning)
			if let authSigning = unsecuredEntityControl.authenticationSigning {
				factorInstances.insert(authSigning)
			}
			return factorInstances
		}
	}
	
	public var hasAuthenticationSigningKey: Bool {
		switch securityState {
		case let .unsecured(unsecuredEntityControl):
			unsecuredEntityControl.authenticationSigning != nil
		}
	}
	
	public var deviceFactorSourceID: FactorSourceIDFromHash? {
		switch self.securityState {
		case let .unsecured(control):
			let factorSourceID = control.transactionSigning.factorSourceID
			guard factorSourceID.kind == .device else {
				return nil
			}
			
			return factorSourceID
		}
	}
}

extension EntityBaseProtocol {
	public var description: String {
		"\(displayName): \(address) @\(networkID)"
	}
}

#if DEBUG
extension EntityBaseProtocol {
	public static var sample: Self { Self.sampleMainnet }
	public static var sampleOther: Self { Self.sampleMainnetOther }
}
#endif // DEBUG

#if DEBUG
extension EntityBaseProtocol {
	public static var sampleValuesMainnet: [Self] {
		[
			Self.sampleMainnet,
			Self.sampleMainnetOther,
			Self.sampleMainnetThird,
		]
	}
	public static var sampleValuesStokenet: [Self] {
		[
			Self.sampleStokenet,
			Self.sampleStokenetOther,
			Self.sampleStokenetThird,
		]
	}
	
	public static var sampleValues: [Self] {
		Self.sampleValuesMainnet + Self.sampleValuesStokenet
	}
}
#endif // DEBUG


public protocol EntityProtocol: EntityBaseProtocol {
	
	associatedtype ExtraProperties: SargonModel
	
	static func deriveVirtualAddress(
		networkID: NetworkID,
		factorInstance: HierarchicalDeterministicFactorInstance
	) -> EntityAddress
	
	init(
		networkID: NetworkID,
		address: EntityAddress,
		securityState: EntitySecurityState,
		displayName: DisplayName,
		extraProperties: ExtraProperties
	)
	
	static var kind: EntityKind { get }
	static func extract(from someEntityProtocol: some EntityBaseProtocol) -> Self?
}

extension EntityProtocol {
	public var entityKind: EntityKind {
		Self.kind
	}

	public init(
		networkID: NetworkID,
		address: EntityAddress,
		factorInstance: HierarchicalDeterministicFactorInstance,
		displayName: DisplayName,
		extraProperties: ExtraProperties
	) {
		self.init(
			networkID: networkID,
			address: address,
			securityState: .unsecured(value: .init(transactionSigning: factorInstance, authenticationSigning: nil)),
			displayName: displayName,
			extraProperties: extraProperties
		)
	}

	public init(
		networkID: NetworkID,
		factorInstance: HierarchicalDeterministicFactorInstance,
		displayName: DisplayName,
		extraProperties: ExtraProperties
	) {
		let address = Self.deriveVirtualAddress(networkID: networkID, factorInstance: factorInstance)
		self.init(
			networkID: networkID,
			address: address,
			factorInstance: factorInstance,
			displayName: displayName,
			extraProperties: extraProperties
		)
	}
}
