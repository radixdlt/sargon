@Reducer
public struct MainFeature {
	public init() {}
	
	@ObservableState
	public struct State {
		public let profile: Profile
		public init(profile: Profile) {
			self.profile = profile
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
