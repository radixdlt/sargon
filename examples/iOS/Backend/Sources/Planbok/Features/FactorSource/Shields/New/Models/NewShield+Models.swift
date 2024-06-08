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

public struct Factor: Hashable, Sendable, Identifiable {
	public let id = UUID()
	public var factorSource: FactorSource?
	public init(factorSource: FactorSource? = nil) {
		self.factorSource = factorSource
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

