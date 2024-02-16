
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
						.foregroundColor(.app.whiteTransparent)
			}
			.padding(.horizontal, .medium1)
			.padding(.vertical, .medium2)
			.background(account.appearanceID.gradient)
			.cornerRadius(.small1)
			
		}
	}
}
extension AccountAddress: AddressProtocol {
	public enum Format: AddressFormatProtocol {
		case `default`
		case full
	}
	public func formatted(_ format: Format) -> String {
		switch format {
		case .full:
			self.address
		case .default:
			accountAddressToShort(address: self)
		}
	}
}
