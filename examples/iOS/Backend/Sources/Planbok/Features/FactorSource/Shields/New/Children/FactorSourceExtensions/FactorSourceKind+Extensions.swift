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
	public static let allCases: [Self]  =  [
		.device,
		.arculusCard,
		.ledgerHqHardwareWallet,
		.offDeviceMnemonic,
		.trustedContact,
		.securityQuestions
	]
}

extension FactorSourceKind {
	public var image: String? {
		switch self {
		case .device: return "lock.iphone"
		default: return nil
		}
	}
	public var title: String {
		switch self {
		case .device: return "This Phone"
		default: return self.toString()
		}
	}
	public var subtitle: String? {
		switch self {
		case .device: return "Face ID / PIN"
		default: return nil
		}
	}
}
