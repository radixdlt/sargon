import Foundation
import SargonUniFFI

// MARK: - PersonaData + SargonModel
extension PersonaData: SargonModel {}

public typealias AnyIdentifiedPersonaEntry = PersonaData.IdentifiedEntry<PersonaData.Entry>

#if DEBUG
extension AnyIdentifiedPersonaEntry: SargonModel {
	public static let sample = Self(
		id: UUID(
			uuidString: "00000000-0000-0000-0000-000000000001"
		)!,
		value: PersonaData.Entry.phoneNumber(.sample)
	)
	public static let sampleOther = Self(
		id: UUID(
			uuidString: "00000000-0000-0000-0000-000000000002"
		)!,
		value: PersonaData.Entry.phoneNumber(.sampleOther)
	)
}
#endif

// MARK: - PersonaData.IdentifiedEntry
extension PersonaData {
	public struct IdentifiedEntry<Value>:
		BaseSargonModel,
		Codable,
		Identifiable,
		CustomStringConvertible
		where
		Value:
		SargonModel &
		Codable & BasePersonaDataEntryProtocol
	{
		public typealias ID = PersonaDataEntryID
		public let id: ID
		public var value: Value

		public init(
			id: ID,
			value: Value
		) {
			self.id = id
			self.value = value
		}

		public var description: String {
			"""
			\(value)
			id: \(id)
			"""
		}
	}
}

extension PersonaData {
	public init() {
		self.init(
			name: nil,
			phoneNumbers: .init(
				collection: []
			),
			emailAddresses: .init(
				collection: []
			)
		)
	}

	public static var `default`: Self {
		self.init()
	}

	public var entries: [AnyIdentifiedPersonaEntry] {
		var sequence: [AnyIdentifiedPersonaEntry?] = []
		sequence.append(name?.embed())
		sequence.append(contentsOf: emailAddresses.collection.map { $0.embed() })
		sequence.append(contentsOf: phoneNumbers.collection.map { $0.embed() })
		return sequence.compactMap { $0 }
	}
}
