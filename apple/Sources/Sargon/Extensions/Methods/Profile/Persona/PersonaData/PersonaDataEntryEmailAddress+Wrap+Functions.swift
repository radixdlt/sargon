import Foundation
import SargonUniFFI

// PersonaDataEntryEmailAddress -> SargonStringCodable
extension PersonaDataEntryEmailAddress {
	public init(jsonStringLiteral: String) throws {
		self = try newPersonaDataEntryEmailAddressFromJsonString(jsonString: jsonStringLiteral)
	}

	public func jsonStringLiteral() -> String {
		personaDataEntryEmailAddressToJsonString(personaDataEntryEmailAddress: self)
	}
}
