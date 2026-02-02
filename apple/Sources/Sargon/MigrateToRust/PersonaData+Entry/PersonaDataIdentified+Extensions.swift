import Foundation
import SargonUniFFI

extension PersonaDataIdentifiedPhoneNumber {
	public func embed() -> AnyIdentifiedPersonaEntry {
		.init(id: self.id, value: self.value.embed())
	}
}

extension PersonaDataIdentifiedEmailAddress {
	public func embed() -> AnyIdentifiedPersonaEntry {
		.init(id: self.id, value: self.value.embed())
	}
}

extension PersonaDataIdentifiedName {
	public func embed() -> AnyIdentifiedPersonaEntry {
		.init(id: self.id, value: self.value.embed())
	}
}

extension PersonaData.Entry {
	public var discriminator: PersonaData.Entry.Kind {
		switch self {
		case .name: .fullName
		case .emailAddress: .emailAddress
		case .phoneNumber: .phoneNumber
		}
	}
}

// MARK: - DuplicateValuesFound
struct DuplicateValuesFound: Swift.Error {}

// MARK: - DuplicateIDOfValueFound
struct DuplicateIDOfValueFound: Swift.Error {}

// MARK: - PersonaFieldCollectionValueWithIDNotFound
struct PersonaFieldCollectionValueWithIDNotFound: Swift.Error {
	let id: PersonaDataEntryID
}

public typealias PersonaDataEntryID = UUID

extension PersonaData.Entry {
	public func extract<F: PersonaDataEntryProtocol>(
		_ type: F.Type = F.self
	) -> F? {
		F.extract(from: self)
	}

	public func extract<F: PersonaDataEntryProtocol>(
		as _: F.Type = F.self
	) throws -> F {
		guard let extracted = extract(F.self) else {
			throw IncorrectPersonaFieldType(expectedKind: F.kind, actualKind: discriminator)
		}
		return extracted
	}
}

// MARK: - IncorrectPersonaFieldType
public struct IncorrectPersonaFieldType: Swift.Error {
	public let expectedKind: PersonaData.Entry.Kind
	public let actualKind: PersonaData.Entry.Kind
}
