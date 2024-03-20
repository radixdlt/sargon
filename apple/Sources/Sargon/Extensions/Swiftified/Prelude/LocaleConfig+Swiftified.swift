extension LocaleConfig: @unchecked Sendable {}
extension LocaleConfig {
	public init(locale: Locale) {
		self.init(
			decimalSeparator: locale.decimalSeparator,
			groupingSeparator: locale.groupingSeparator
		)
	}
}
