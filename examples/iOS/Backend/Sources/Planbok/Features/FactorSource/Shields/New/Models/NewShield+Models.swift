//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-07.
//

import Foundation
import Sargon
import IdentifiedCollections

public enum Role: Sendable, Hashable {
	case primary, recovery, confirmation
}

public enum Factor: Hashable, Sendable, Identifiable {
	public enum ID: Hashable, Sendable {
		case placeholder(UUID)
		case factor(FactorSourceID)
	}
	case placeholder(UUID)
	case factor(FactorSource)
	var factorSource: FactorSource? {
		switch self {
		case .placeholder: return nil
		case let .factor(factor): return factor
		}
	}
	public var id: ID {
		switch self {
		case let .placeholder(id): .placeholder(id)
		case let .factor(factor): .factor(factor.id)
		}
	}
}

public typealias Factors = IdentifiedArrayOf<Factor>


public enum FactorThreshold: Hashable, Sendable, CustomStringConvertible {
	case any
	case all
	case threshold(UInt8)
	
	public var description: String {
		switch self {
		case .any: return "Any"
		case .all: return "All"
		case let .threshold(threshold): return "\(threshold)"
		}
	}
}

