import Foundation
import SargonUniFFI

// MARK: - PersonaDataEntryPhoneNumber + SargonModel
extension PersonaDataEntryPhoneNumber: SargonModel {}

// MARK: - PersonaDataEntryPhoneNumber + SargonStringCodable
extension PersonaDataEntryPhoneNumber: SargonStringCodable {}

// MARK: - PersonaDataEntryPhoneNumber + CustomStringConvertible
extension PersonaDataEntryPhoneNumber: CustomStringConvertible {
	public var description: String {
		number
	}
}

// MARK: - PersonaDataEntryPhoneNumber + PersonaDataEntryProtocol
extension PersonaDataEntryPhoneNumber: PersonaDataEntryProtocol {
	public static var kind: PersonaData.Entry.Kind {
		.phoneNumber
	}

	public static func extract(from entry: PersonaData.Entry) -> Self? {
		guard case let .phoneNumber(value) = entry else { return nil }
		return value
	}

	public func embed() -> PersonaData.Entry {
		.phoneNumber(self)
	}
}
