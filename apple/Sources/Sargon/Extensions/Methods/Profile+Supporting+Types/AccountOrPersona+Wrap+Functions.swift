import Foundation
import SargonUniFFI

extension AccountOrPersona {
	public var id: AddressOfAccountOrPersona {
		accountOrPersonaGetId(entity: self)
	}
}
