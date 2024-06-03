
public struct LabeledTextField: SwiftUI.View {
	
	public let label: LocalizedStringKey
	public let placeholder: LocalizedStringKey
	@Binding public var text: String
	public let hint: LocalizedStringKey?
	
	init(
		label: LocalizedStringKey,
		text: Binding<String>,
		placeholder: LocalizedStringKey? = nil,
		hint: LocalizedStringKey? = nil
	) {
		self.label = label
		self._text = text
		self.placeholder = placeholder ?? label
		self.hint = hint
	}
	
	public var body: some View {
		VStack(alignment: .leading) {
			Text(label)
				.padding(.leading, 5)
			TextField(placeholder, text: $text)
				.autocorrectionDisabled()
				.textInputAutocapitalization(.never)
			Text(hint ?? "")
				.font(.footnote)
				.padding(.leading, 5)
		}
		.textFieldStyle(.roundedBorder)
	}
}
