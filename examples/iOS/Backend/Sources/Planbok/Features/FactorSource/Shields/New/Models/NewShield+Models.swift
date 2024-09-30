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
	init(count: UInt8, thresholdFactorsCount: Int) {
		let factorCount = UInt8(thresholdFactorsCount)
		if count == factorCount {
			self = .all
		} else if count == 1 {
			self = .any
		} else {
			self = .threshold(count)
		}
	}
	
	public func isValid(thresholdFactorCount: Int) -> Bool {
		switch self {
		case .any: return true
		case .all: return true
		case let .threshold(lhsThreshold):
			let isValid = thresholdFactorCount > lhsThreshold
			if !isValid {
				log.fault("Number of factors not greater than threshold")
			}
			return isValid
		}
	}
	
	public mutating func decrease() {
		switch self {
		case .any, .all: break
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

