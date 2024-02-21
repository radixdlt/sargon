#if DEBUG
@Reducer
public struct PreviewAddressesFeature {
	
	@ObservableState
	public struct State: Equatable {}
	
	public enum Action {}
	
	public init() {}
	
	public struct View: SwiftUI.View {
		public let store: StoreOf<PreviewAddressesFeature>
		public var body: some SwiftUI.View {
			VStack(alignment: .leading) {
				Text("Previews of Addresses")
					.font(.title)
				ScrollView {
					VStack(alignment: .leading, spacing: .large2) {
						ForEach(Address.allCases, id: \.self) { address in
							Text("`\(address.address)`")
								.font(.footnote)
								.lineLimit(3)
								.multilineTextAlignment(.leading)
								.fixedSize(horizontal: false, vertical: true)
						}
					}
				}
			}
			.frame(maxWidth: .infinity, alignment: .leading)
			.padding()
		}
	}
}

#endif
