import Foundation
import SargonUniFFI

// MARK: - PersonaData.Entry
extension PersonaData {
	public enum Entry:
		SargonModel,
		Codable,
		BasePersonaDataEntryProtocol,
		CustomStringConvertible
	{
		case name(PersonaDataEntryName)
		case emailAddress(PersonaDataEntryEmailAddress)
		case phoneNumber(PersonaDataEntryPhoneNumber)
	}
}

#if DEBUG
extension PersonaData.Entry {
	public static let sample: Self = .name(.sample)
	public static let sampleOther: Self = .emailAddress(.sample)
}
#endif // DEBUG

extension PersonaData.Entry {
	public func embed() -> PersonaData.Entry {
		self
	}

	public var description: String {
		switch self {
		case let .name(name):
			name.description
		case let .emailAddress(emailAddress):
			emailAddress.email.description
		case let .phoneNumber(phoneNumber):
			phoneNumber.number.description
		}
	}
}

// MARK: - PersonaData.Entry.Kind
extension PersonaData.Entry {
	public enum Kind: String, Sendable, Hashable, Codable {
		case fullName
		case emailAddress
		case phoneNumber
	}
}
