import Foundation
import Sargon

// MARK: - FactorSourceKind + Identifiable
extension FactorSourceKind: Identifiable {
	public typealias ID = String
	public var id: ID {
		toString()
	}
}

// MARK: - FactorSourceKind + CaseIterable
extension FactorSourceKind: CaseIterable {
	// FIXME: MOVE Into Rust Sargon!
	public static let allCases: [Self] = [
		.device,
		.arculusCard,
		.ledgerHqHardwareWallet,
		.offDeviceMnemonic,
		.trustedContact,
		.securityQuestions,
	]
}

// MARK: - FactorSourceKindUnavailabilityReason
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
		case .device: 1
		default: nil
		}
	}

	// FIXME: MOVE Into Rust Sargon!
	private func canBeUsedForRole(_ role: Role) -> Bool {
		switch self {
		case .device:
			role == .primary
		case .securityQuestions:
			role == .confirmation
		case .offDeviceMnemonic:
			role != .primary
		case .trustedContact:
			role != .primary
		default: true
		}
	}
}

extension FactorSourceKind {
	public var title: String {
		switch self {
		case .device: "This Phone"
		case .arculusCard: "Arculus Card"
		case .ledgerHqHardwareWallet: "Ledger Hardware Wallet"
		case .trustedContact: "Trusted Contact"
		case .securityQuestions: "Security Questions"
		case .offDeviceMnemonic: "Password"
		}
	}

	public var subtitle: String? {
		switch self {
		case .device: "Face ID / PIN"
		default: nil
		}
	}
}
