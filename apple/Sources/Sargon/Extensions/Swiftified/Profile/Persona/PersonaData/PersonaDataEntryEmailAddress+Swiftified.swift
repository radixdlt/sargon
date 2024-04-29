import Foundation
import SargonUniFFI

// MARK: - PersonaDataEntryEmailAddress + SargonModel
extension PersonaDataEntryEmailAddress: SargonModel {}

// MARK: - PersonaDataEntryEmailAddress + SargonStringCodable
extension PersonaDataEntryEmailAddress: SargonStringCodable {}

// MARK: - PersonaDataEntryEmailAddress + CustomStringConvertible
extension PersonaDataEntryEmailAddress: CustomStringConvertible {
	public var description: String {
		email
	}
}

// MARK: - PersonaDataEntryEmailAddress + PersonaDataEntryProtocol
extension PersonaDataEntryEmailAddress: PersonaDataEntryProtocol {
	public static var kind: PersonaData.Entry.Kind {
		.emailAddress
	}

	public static func extract(from entry: PersonaData.Entry) -> Self? {
		guard case let .emailAddress(value) = entry else { return nil }
		return value
	}

	public func embed() -> PersonaData.Entry {
		.emailAddress(self)
	}
}
