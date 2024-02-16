@Reducer
public struct OnboardingFeature {
	public init() {}
	
	@ObservableState
	public struct State {
		public let walletHolder: WalletHolder
		public init(walletHolder: WalletHolder) {
			self.walletHolder = walletHolder
		}
		public init(wallet: Wallet) {
			self.init(walletHolder: .init(wallet: wallet))
		}
	}
	
	public enum Action {}
	
	public struct View: SwiftUI.View {
		public let store: StoreOf<OnboardingFeature>
		public init(store: StoreOf<OnboardingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			Text("Onboarding")
		}
	}
}
