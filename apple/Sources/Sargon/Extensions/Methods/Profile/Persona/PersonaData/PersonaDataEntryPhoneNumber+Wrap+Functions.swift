import Foundation
import SargonUniFFI

/// PersonaDataEntryPhoneNumber -> SargonStringCodable
extension PersonaDataEntryPhoneNumber {
	public init(jsonStringLiteral: String) throws {
		self = try newPersonaDataEntryPhoneNumberFromJsonString(jsonString: jsonStringLiteral)
	}

	public func jsonStringLiteral() -> String {
		personaDataEntryPhoneNumberToJsonString(personaDataEntryPhoneNumber: self)
	}
}
