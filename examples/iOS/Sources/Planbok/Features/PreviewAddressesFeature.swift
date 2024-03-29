#if DEBUG
public struct SampleAddressesView: SwiftUI.View {
	public var body: some SwiftUI.View {
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

#endif
