import ComposableArchitecture
import SwiftUI

// MARK: - OverlayFeature
@Reducer
struct OverlayFeature {
	@Dependency(OverlayWindowClient.self) var overlayWindowClient
	@Dependency(\.continuousClock) var clock

	@Reducer(state: .equatable)
	enum Destination: Sendable {
		case hud(HUDFeature)
	}

	@ObservableState
	struct State: Equatable {
		var itemsQueue: IdentifiedArrayOf<HUDMessage> = []

		var isPresenting: Bool {
			destination != nil
		}

		@Presents
		var destination: Destination.State?
	}

	@CasePathable
	enum Action: ViewAction, Sendable {
		@CasePathable
		enum ViewAction {
			case task
		}

		enum InternalAction {
			case scheduleItem(HUDMessage)
			case showNextItemIfPossible
		}

		case `internal`(InternalAction)
		case view(ViewAction)
		case destination(PresentationAction<Destination.Action>)
	}

	var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.task):
				return .run { send in
					for try await item in overlayWindowClient.getScheduledItems() {
						guard !Task.isCancelled else { return }
						await send(.internal(.scheduleItem(item)))
					}
				}

			case let .internal(.scheduleItem(hudMessage)):
				state.itemsQueue.append(hudMessage)
				return showItemIfPossible(state: &state)

			case .internal(.showNextItemIfPossible):
				return showItemIfPossible(state: &state)

			case .destination(.presented(.hud(.delegate(.dismiss)))):
				return dismiss(&state)

			case .destination:
				return .none
			}
		}
		.ifLet(\.$destination, action: \.destination)
	}

	private func showItemIfPossible(state: inout State) -> Effect<Action> {
		guard !state.itemsQueue.isEmpty else {
			return .none
		}

		if state.isPresenting {
			guard let presentedItem = state.itemsQueue.first else {
				return .none
			}

			// A HUD is force dismissed when next item comes in, AKA it is a lower priority.
			state.destination = nil
			state.itemsQueue.removeFirst()
			return .run { send in
				// Hacky - A very minor delay is needed before showing the next item is a HUD.
				try await clock.sleep(for: .milliseconds(200))
				await send(.internal(.showNextItemIfPossible))
			}
		}

		let nextItem = state.itemsQueue[0]

		state.destination = .hud(.init(content: nextItem))
		return .none
	}

	private func dismiss(_ state: inout State) -> Effect<Action> {
		state.destination = nil
		state.itemsQueue.removeFirst()
		return showItemIfPossible(state: &state)
	}
}

extension OverlayFeature {
	typealias HostingFeature = Self

	@ViewAction(for: HostingFeature.self)
	struct View: SwiftUI.View {
		@Bindable var store: StoreOf<HostingFeature>
		init(store: StoreOf<HostingFeature>) {
			self.store = store
		}

		var body: some SwiftUI.View {
			Color.clear
				.task { send(.task) }
				.fullScreenCover(
					item: $store.scope(
						state: \.destination?.hud,
						action: \.destination.hud
					)
				) { store in
					HUDFeature.View(store: store)
						.background(TransparentBackground())
				}
		}
	}
}
