@Reducer
public struct OnboardingFeature {
	public init() {}
	
	@ObservableState
	public struct State {
		public init() {}
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
