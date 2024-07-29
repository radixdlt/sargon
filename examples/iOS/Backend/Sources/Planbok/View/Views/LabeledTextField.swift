
public struct LabeledTextField<Trailing: SwiftUI.View>: SwiftUI.View {
	
	public let label: LocalizedStringKey
	public let placeholder: LocalizedStringKey
	@Binding public var text: String
	public let hint: LocalizedStringKey?
	public let trailingView: Trailing
	
	init(
		label: LocalizedStringKey,
		text: Binding<String>,
		placeholder: LocalizedStringKey? = nil,
		hint: LocalizedStringKey? = nil,
		@ViewBuilder trailingView: () -> Trailing
	) {
		self.label = label
		self._text = text
		self.placeholder = placeholder ?? label
		self.hint = hint
		self.trailingView = trailingView()
	}
}
extension LabeledTextField where Trailing == EmptyView {
	public init(
		label: LocalizedStringKey,
		text: Binding<String>,
		placeholder: LocalizedStringKey? = nil,
		hint: LocalizedStringKey? = nil
	) {
		self.init(label: label, text: text, placeholder: placeholder, hint: hint, trailingView: { EmptyView() })
	}
}

extension LabeledTextField {
	public var body: some View {
		VStack(alignment: .leading) {
			Text(label)
				.padding(.leading, 5)
			
			HStack {
				TextField(placeholder, text: $text)
					.autocorrectionDisabled()
					.textInputAutocapitalization(.never)
					.textFieldStyle(.roundedBorder)

				trailingView
			}
			
			Text(hint ?? "")
				.font(.footnote)
				.padding(.leading, 5)
		}
	}
}
