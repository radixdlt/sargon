//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-27.
//

import Foundation
import SargonUniFFI

extension AddressOfAccountOrPersona: BaseEntityAddressProtocol {}

extension AccountOrPersona: EntityProtocol {
	public var address: EntityAddress {
		id
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
	public var flags: EntityFlags {
		property(\.flags)
	}
	
}

extension AccountOrPersona {
	private func property<Property>(_ keyPath: KeyPath<any EntityProtocol, Property>) -> Property {
		switch self {
		case let .account(entity): entity[keyPath: keyPath]
		case let .persona(entity): entity[keyPath: keyPath]
		}
	}
	
}
