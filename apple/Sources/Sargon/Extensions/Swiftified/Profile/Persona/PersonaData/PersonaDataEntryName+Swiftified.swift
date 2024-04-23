import Foundation
import SargonUniFFI

extension PersonaDataEntryName: SargonModel {}
extension PersonaDataEntryName: SargonObjectCodable {}
extension PersonaDataEntryName: CustomStringConvertible {
	public var description: String {
		formatted
	}
}

extension PersonaDataEntryName {
	public typealias Variant = Sargon.PersonaDataNameVariant
}

extension PersonaDataEntryName.Variant: CaseIterable {
	public static var allCases: [Self] {
		[.eastern, .western]
	}
}

extension PersonaDataEntryName {
	public var formatted: String {
		let names = { () -> [String] in
			switch variant {
			case .western: [givenNames, familyName]
			case .eastern: [familyName, givenNames]
			}
		}().filter({ !$0.isEmpty })
		
		var formatted: [String] = []
		formatted.append(names.joined(separator: " "))

		if !nickname.isEmpty {
			formatted.append("\"\(nickname)\"")
		}
		
		return formatted
			.filter({ !$0.isEmpty })
			.joined(separator: "\n")
	}
}


// MARK: - PersonaDataEntryName + PersonaDataEntryProtocol
extension PersonaDataEntryName: PersonaDataEntryProtocol {
	public static var kind: PersonaData.Entry.Kind {
		.fullName
	}
	
	public func embed() -> PersonaData.Entry {
		.name(self)
	}
	
	public static func extract(from entry: PersonaData.Entry) -> Self? {
		guard case let .name(value) = entry else { return nil }
		return value
	}
}


