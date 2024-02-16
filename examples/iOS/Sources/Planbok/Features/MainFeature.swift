@Reducer
public struct MainFeature {
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
		public let store: StoreOf<MainFeature>
		public init(store: StoreOf<MainFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			Text("Main")
		}
	}
}
