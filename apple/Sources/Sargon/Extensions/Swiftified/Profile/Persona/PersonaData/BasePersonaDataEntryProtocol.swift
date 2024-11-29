import Foundation
import SargonUniFFI

// MARK: - BasePersonaDataEntryProtocol
public protocol BasePersonaDataEntryProtocol {
	func embed() -> PersonaData.Entry
}
