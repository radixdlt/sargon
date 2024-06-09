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
	public func isGreaterThan(count rhs: Int) -> Bool {
		switch self {
		case .any: return false
		case .all: return rhs <= 1
		case let .threshold(lhsThreshold): return lhsThreshold > (rhs - 1)
		}
	}
	public mutating func decrease() {
		switch self {
		case .any: break
		case .all: self = .any
		case let .threshold(thres) where thres <= 1:
			self = .all
		case let .threshold(thres) where thres > 1:
			self = .threshold(thres - 1)
		default: fatalError("not possible")
		}
	}
	
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

