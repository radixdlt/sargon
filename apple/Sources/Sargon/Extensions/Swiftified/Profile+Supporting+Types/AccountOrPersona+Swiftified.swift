import Foundation
import SargonUniFFI

// MARK: - AddressOfAccountOrPersona + BaseEntityAddressProtocol
extension AddressOfAccountOrPersona: BaseEntityAddressProtocol {}

extension AccountOrPersona {
	public static func account(_ account: Account) -> Self {
		.accountEntity(account)
	}

	public static func persona(_ persona: Persona) -> Self {
		.personaEntity(persona)
	}
}

// MARK: - AccountOrPersona + EntityBaseProtocol
extension AccountOrPersona: EntityBaseProtocol {
	public var address: EntityAddress {
		id
	}

	public var asGeneral: AccountOrPersona {
		self
	}

	public typealias ID = AddressOfAccountOrPersona
	public typealias EntityAddress = AddressOfAccountOrPersona

	public var securityState: EntitySecurityState {
		property(\.securityState)
	}

	/// The ID of the network this entity exists on.
	public var networkId: NetworkID {
		property(\.networkID)
	}

	/// A required non empty display name, used by presentation layer and sent to Dapps when requested.
	public var displayName: DisplayName {
		property(\.displayName)
	}

	/// Flags that are currently set on entity.
	public var flags: [EntityFlag] {
		property(\.flags)
	}

	public var entityKind: EntityKind {
		switch self {
		case let .accountEntity(value): value.entityKind
		case let .personaEntity(value): value.entityKind
		}
	}

	public func asAccount() throws -> Account {
		try extract()
	}

	public func asPersona() throws -> Persona {
		try extract()
	}

	public func extract<F: EntityProtocol>(_ type: F.Type = F.self) -> F? {
		F.extract(from: self)
	}

	public func extract<F: EntityProtocol>(as _: F.Type = F.self) throws -> F {
		guard let extracted = extract(F.self) else {
			throw IncorrectEntityType(
				expectedKind: F.kind,
				actualKind: entityKind
			)
		}
		return extracted
	}

	public struct IncorrectEntityType: Swift.Error {
		public let expectedKind: EntityKind
		public let actualKind: EntityKind
	}

	public var virtualHierarchicalDeterministicFactorInstances: Set<HierarchicalDeterministicFactorInstance> {
		property(
			\.virtualHierarchicalDeterministicFactorInstances
		)
	}

	public var unsecuredControllingFactorInstance: SargonUniFFI.HierarchicalDeterministicFactorInstance? {
		property(
			\.unsecuredControllingFactorInstance
		)
	}
}

extension AccountOrPersona {
	private func property<Property>(_ keyPath: KeyPath<any EntityBaseProtocol, Property>) -> Property {
		switch self {
		case let .accountEntity(entity): entity[keyPath: keyPath]
		case let .personaEntity(entity): entity[keyPath: keyPath]
		}
	}
}
