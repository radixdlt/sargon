import Foundation
import SargonUniFFI

public typealias BIP39WordCount = Bip39WordCount

// MARK: - BIP39WordCount + SargonModel
extension BIP39WordCount: SargonModel {
	public static let sample: Self = .twentyFour
	public static let sampleOther: Self = .twelve
}

extension BIP39WordCount {
	public init?(wordCount: Int) {
		self.init(rawValue: UInt8(wordCount))
	}
}

// MARK: - BIP39WordCount + Identifiable
extension BIP39WordCount: Identifiable {
	public typealias ID = RawValue
	public var id: ID {
		rawValue
	}
}

// MARK: - BIP39WordCount + Comparable
extension BIP39WordCount: Comparable {
	public static func < (lhs: Self, rhs: Self) -> Bool {
		lhs.rawValue < rhs.rawValue
	}
}

extension BIP39WordCount {
	public mutating func increaseBy3() {
		guard self < .twentyFour else {
			return assertionFailure("At max word count (24)")
		}
		self = .init(rawValue: rawValue + 3)!
	}

	public mutating func decreaseBy3() {
		guard self > .twelve else {
			return assertionFailure("At min word count (12)")
		}
		self = .init(rawValue: rawValue - 3)!
	}
}
