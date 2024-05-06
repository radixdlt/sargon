import Sargon
import ComposableArchitecture

public struct AccountView: SwiftUI.View {
	public let account: Account

	public var body: some SwiftUI.View {
		
		VStack(alignment: .leading, spacing: .medium3) {
			VStack(alignment: .leading, spacing: .zero) {
				Text(account.displayName.value)
					.lineLimit(1)
					.foregroundColor(.white)
					.frame(maxWidth: .infinity, alignment: .leading)
				
				AddressView(account.address)
					.foregroundColor(.app.whiteTransparent)
			}
			.padding(.horizontal, .medium1)
			.padding(.vertical, .medium2)
			.background(account.appearanceID.gradient)
			.cornerRadius(.small1)
		}
		
	}
}

public struct AccountCardView: SwiftUI.View {
	public let account: Account
	public let action: () -> Void
	
	public var body: some SwiftUI.View {
		Button.init(action: action, label: {
			AccountView(account: account)
		})
		.buttonStyle(.plain)
	}
}
