import Foundation
import SargonUniFFI

public typealias PersonaDataEntryEmailAddress = EmailAddress

// MARK: - EmailAddress + SargonModel
extension EmailAddress: SargonModel {}

// MARK: - EmailAddress + SargonStringCodable
extension EmailAddress: SargonStringCodable {}

// MARK: - EmailAddress + CustomStringConvertible
extension EmailAddress: CustomStringConvertible {
	public var description: String {
		email
	}
}

// MARK: - EmailAddress + PersonaDataEntryProtocol
extension EmailAddress: PersonaDataEntryProtocol {
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
