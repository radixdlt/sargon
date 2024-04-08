
public struct LabeledTextField: SwiftUI.View {
	
	public let label: LocalizedStringKey
	@Binding public var text: String
	
	public var body: some View {
		VStack(alignment: .leading) {
			Text(label).padding(.leading, 5)
			TextField(label, text: $text)
		}
		.textFieldStyle(.roundedBorder)
	}
}
