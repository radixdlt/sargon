import SargonUniFFI

#if DEBUG
public protocol BaseBaseEntityProtocol: SargonModel {
	static var sampleValues: [Self] { get }
}
#else
public protocol BaseBaseEntityProtocol: SargonModel {}
#endif // DEBUG

// MARK: - EntityBaseProtocol
public protocol EntityBaseProtocol: BaseBaseEntityProtocol, CustomStringConvertible, Identifiable
	where ID == EntityAddress
{
	associatedtype EntityAddress: BaseEntityAddressProtocol
	var networkId: NetworkID { get }
	var displayName: DisplayName { get }
	var address: EntityAddress { get }
	var flags: [EntityFlag] { get }
	var securityState: EntitySecurityState { get }
	var entityKind: EntityKind { get }
	var asGeneral: AccountOrPersona { get }
	var unsecuredControllingFactorInstance: HierarchicalDeterministicFactorInstance? { get }

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

	public var isDeleted: Bool {
		flags.contains(.tombstonedByUser)
	}

	public var isHidden: Bool {
		flags.contains(.hiddenByUser)
	}

	public var virtualHierarchicalDeterministicFactorInstances:
		Set<HierarchicalDeterministicFactorInstance>
	{
		var factorInstances = [HierarchicalDeterministicFactorInstance]()

        switch securityState {
        case let .unsecured(control):
            factorInstances.append(control.transactionSigning)
        case let .securified(control):
            let instances = control.securityStructure.matrixOfFactors.primaryRole.thresholdFactors.map {
                switch $0.badge {
                case let .virtual(.hierarchicalDeterministic(key)):
                    HierarchicalDeterministicFactorInstance(factorSourceId: try! $0.factorSourceId.extract(), publicKey: key)
                }
            }
            
            factorInstances.append(contentsOf: instances)
        }

		return Set(factorInstances)
	}

	public var deviceFactorSourceID: FactorSourceIDFromHash? {
		guard let factorSourceId = unsecuredControllingFactorInstance?.factorSourceId, factorSourceId.kind == .device else {
			return nil
		}

		return factorSourceId
	}
}

extension EntityBaseProtocol {
	public var description: String {
		"\(displayName): \(address) @\(networkID)"
	}
}

#if DEBUG
extension EntityBaseProtocol {
	public static var sample: Self { sampleMainnet }
	public static var sampleOther: Self { sampleMainnetOther }
}
#endif // DEBUG

#if DEBUG
extension EntityBaseProtocol {
	public static var sampleValuesMainnet: [Self] {
		[
			sampleMainnet,
			sampleMainnetOther,
			sampleMainnetThird,
		]
	}

	public static var sampleValuesStokenet: [Self] {
		[
			sampleStokenet,
			sampleStokenetOther,
			sampleStokenetThird,
		]
	}

	public static var sampleValues: [Self] {
		sampleValuesMainnet + sampleValuesStokenet
	}
}
#endif // DEBUG

// MARK: - EntityProtocol
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
			securityState: .unsecured(
				value: .init(transactionSigning: factorInstance, provisionalSecurifiedConfig: nil)),
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
