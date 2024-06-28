import Sargon
import ComposableArchitecture

public struct AccountCardView: SwiftUI.View {
	public let accountForDisplay: AccountForDisplay
	public let action: () -> Void
	
	public var body: some SwiftUI.View {
		
		Button.init(action: action, label: {
			AccountView(accountForDisplay: accountForDisplay)
		})
		.buttonStyle(.plain)
		.cornerRadius(.small1)
	}
}

public struct AccountView: SwiftUI.View {
	public let accountForDisplay: AccountForDisplay
	public let format: AddressFormat
	
	init(accountForDisplay: AccountForDisplay, format: AddressFormat = .default) {
		self.accountForDisplay = accountForDisplay
		self.format = format
	}
	
	public var body: some SwiftUI.View {
		
			VStack(alignment: .leading, spacing: .zero) {
				Text(accountForDisplay.displayName.value)
					.lineLimit(1)
					.foregroundColor(.white)
					.frame(maxWidth: .infinity, alignment: .leading)
				
				AddressView(accountForDisplay.address, format: format)
					.foregroundColor(.app.whiteTransparent)
			}
			.padding(.horizontal, .medium1)
			.padding(.vertical, .medium2)
			.background(accountForDisplay.appearanceId.gradient)
	
		
	}
}
