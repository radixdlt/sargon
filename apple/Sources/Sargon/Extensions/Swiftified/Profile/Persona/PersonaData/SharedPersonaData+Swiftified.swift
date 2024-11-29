import Foundation
import SargonUniFFI

// MARK: - SharedPersonaData + SargonModel
extension SharedPersonaData: SargonModel {}

extension SharedPersonaData {
	public static let `default` = Self(
		name: nil,
		emailAddresses: nil,
		phoneNumbers: nil
	)

	public var entryIDs: Set<PersonaDataEntryID> {
		var ids: [PersonaDataEntryID] = [
			name,
		].compactMap { $0 }
		ids.append(contentsOf: emailAddresses?.ids ?? [])
		ids.append(contentsOf: phoneNumbers?.ids ?? [])

		return Set(ids)
	}
}
