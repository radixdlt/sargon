//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-07.
//

import Foundation
import Sargon

extension FactorSourceKind: Identifiable {
	public typealias ID = String
	public var id: ID {
		toString()
	}
}

extension FactorSourceKind: CaseIterable {
	// FIXME: MOVE Into Rust Sargon!
	public static let allCases: [Self]  =  [
		.device,
		.arculusCard,
		.ledgerHqHardwareWallet,
		.offDeviceMnemonic,
		.trustedContact,
		.securityQuestions
	]
}

// FIXME: MOVE Into Rust Sargon!
public enum FactorSourceKindUnavailabilityReason: Hashable, Sendable {
	case canNeverBeUsedForRole(Role)
	case exceededLimitOfKindPerRoleReached(exceededLimit: UInt8, Role)
	
	public func toString(kind: FactorSourceKind) -> String {
		switch self {
		case let .canNeverBeUsedForRole(role): "Cannot be used as \(role)"
		case let .exceededLimitOfKindPerRoleReached(exceededLimit, role): "Cannot use more than \(exceededLimit) factors of kind \(kind) for \(role)"
		}
	}
}

extension FactorSourceKind {
	
	// FIXME: MOVE Into Rust Sargon!
	public func unavailabilityForRole(_ role: Role, usedFactorsForRole: FactorSources) -> FactorSourceKindUnavailabilityReason? {
		guard canBeUsedForRole(role) else {
			return .canNeverBeUsedForRole(role)
		}
		guard let limit = limitOfFactorSourceKindFor(role: role) else {
			return nil
		}
		if usedFactorsForRole.filter({ $0.kind == self }).count >= limit {
			return .exceededLimitOfKindPerRoleReached(exceededLimit: limit, role)
		} else {
			return nil // free to use
		}
	}
	
	// FIXME: MOVE Into Rust Sargon!
	private func limitOfFactorSourceKindFor(role: Role) -> UInt8? {
		switch self {
		case .device: return 1
		default: return nil
		}
	}
	
	// FIXME: MOVE Into Rust Sargon!
	private func canBeUsedForRole(_ role: Role) -> Bool {
		switch self {
		case .device:
			return role == .primary
		case .securityQuestions:
			return role == .confirmation
		case .offDeviceMnemonic:
			return role != .primary
		case .trustedContact:
			return role != .primary
		default: return true
		}
	}
}

extension FactorSourceKind {

	public var title: String {
		switch self {
		case .device: return "This Phone"
        case .arculusCard: return "Arculus Card"
        case .ledgerHqHardwareWallet: return "Ledger Hardware Wallet"
        case .trustedContact: return "Trusted Contact"
        case .securityQuestions: return "Security Questions"
        case .offDeviceMnemonic: return "Password"
		}
	}
	public var subtitle: String? {
		switch self {
		case .device: return "Face ID / PIN"
		default: return nil
		}
	}
}
