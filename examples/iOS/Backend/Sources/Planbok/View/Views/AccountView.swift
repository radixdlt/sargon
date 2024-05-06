import Sargon
import ComposableArchitecture

public struct AccountCardView: SwiftUI.View {
	public let account: Account
	public let action: () -> Void
	
	public var body: some SwiftUI.View {
		
		Button.init(action: action, label: {
			AccountView(account: account)
		})
		.buttonStyle(.plain)
		.cornerRadius(.small1)
	}
}

public struct AccountView: SwiftUI.View {
	public let account: Account
	public let format: AddressFormat
	
	init(account: Account, format: AddressFormat = .default) {
		self.account = account
		self.format = format
	}
	
	public var body: some SwiftUI.View {
		
			VStack(alignment: .leading, spacing: .zero) {
				Text(account.displayName.value)
					.lineLimit(1)
					.foregroundColor(.white)
					.frame(maxWidth: .infinity, alignment: .leading)
				
				AddressView(account.address, format: format)
					.foregroundColor(.app.whiteTransparent)
			}
			.padding(.horizontal, .medium1)
			.padding(.vertical, .medium2)
			.background(account.appearanceID.gradient)
	
		
	}
}
