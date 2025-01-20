// MARK: - SharedConstants
public enum SharedConstants {}

extension SharedConstants {
	public static let minRequiredXrdForAccountDeletion = constantMinRequiredXrdForAccountDeletion()
}

extension Account {
	public static let nameMaxLength = Int(constantEntityNameMaxLength())
}

extension Persona {
	public static let nameMaxLength = Int(constantEntityNameMaxLength())
}

extension DisplayName {
	public static let maxLength = Int(constantDisplayNameMaxLength())
}
