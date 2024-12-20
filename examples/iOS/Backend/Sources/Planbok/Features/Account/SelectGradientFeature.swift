import ComposableArchitecture
import Foundation
import Sargon

// MARK: - SelectGradientFeature
@Reducer
public struct SelectGradientFeature {
	@ObservableState
	public struct State: Equatable {
		public var gradient: AppearanceID
		public init(
			gradient: AppearanceID
		) {
			self.gradient = gradient
		}
	}

	@CasePathable
	public enum Action: ViewAction {
		public enum Delegate {
			case selected(AppearanceID)
		}

		@CasePathable
		public enum ViewAction {
			case selectedGradient(AppearanceID)
			case confirmedGradientButtonTapped
		}

		case view(ViewAction)
		case delegate(Delegate)
	}

	public init() {}

	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .view(.selectedGradient(gradient)):
				state.gradient = gradient
				return .none

			case .view(.confirmedGradientButtonTapped):
				return .send(.delegate(.selected(state.gradient)))

			default:
				return .none
			}
		}
	}
}

// MARK: SelectGradientFeature.View
extension SelectGradientFeature {
	@ViewAction(for: SelectGradientFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<SelectGradientFeature>
		public init(store: StoreOf<SelectGradientFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			VStack {
				Text("Select account gradient").font(.title)
				ScrollView {
					let height: CGFloat = 20
					ForEach(AppearanceID.allCases) { appearanceID in
						let isSelected = appearanceID == store.state.gradient
						Button(action: { send(.selectedGradient(appearanceID)) }, label: {
							HStack {
								Text("Gradient \(String(describing: appearanceID))")
									.font(isSelected ? .headline : .subheadline)
									.fontWeight(isSelected ? .bold : .regular)

								Spacer()

								if isSelected {
									Image(systemName: "checkmark")
										.resizable()
										.scaledToFit()
								}
							}
						})
						.buttonStyle(.borderless)
						.foregroundColor(.app.white)
						.frame(maxWidth: .infinity, idealHeight: height, alignment: .leading)
						.padding()
						.background(appearanceID.gradient)
						.cornerRadius(height)
					}
				}
				Button("Confirm Gradient") {
					send(.confirmedGradientButtonTapped)
				}
				.buttonStyle(.borderedProminent)
			}
			.padding()
		}
	}
}
