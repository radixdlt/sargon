import ComposableArchitecture
import Sargon

// MARK: - GatewaysFeature
@Reducer
public struct GatewaysFeature {
	@Dependency(GatewaysClient.self) var gatewaysClient

	@ObservableState
	public struct State: Equatable {
		@SharedReader(.savedGateways) var savedGateways
	}

	@CasePathable
	public enum Action: ViewAction {
		@CasePathable
		public enum ViewAction {
			case gatewayTapped(Gateway, isCurrent: Bool)
		}

		case view(ViewAction)
	}

	public var body: some ReducerOf<Self> {
		Reduce { _, action in
			switch action {
			case let .view(.gatewayTapped(gateway, isCurrent)):
				if isCurrent {
					log.debug("Tapped \(gateway), but not switching since it is already current.")
					return .none
				} else {
					return .run { _ in
						try await gatewaysClient.switchGatewayTo(gateway)
					}
				}
			}
		}
	}
}

// MARK: GatewaysFeature.View
extension GatewaysFeature {
	@ViewAction(for: GatewaysFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<GatewaysFeature>
		public var body: some SwiftUI.View {
			VStack {
				Text("Saved gateways").font(.title)

				ScrollView {
					ForEach(store.state.savedGateways.all.sorted()) { gateway in
						let isCurrent = gateway == store.state.savedGateways.current
						VStack {
							GatewayView(gateway: gateway, isCurrent: isCurrent) {
								send(.gatewayTapped(gateway, isCurrent: isCurrent))
							}
							.padding(.bottom, 10)
						}
					}
				}
			}
			.padding([.leading, .trailing], 20)
		}
	}
}

// MARK: - Gateway + Comparable
extension Gateway: Comparable {
	public static func < (lhs: Self, rhs: Self) -> Bool {
		if lhs.networkID == .mainnet { return true }
		if rhs.networkID == .mainnet { return false }
		return lhs.networkID.rawValue < rhs.networkID.rawValue && lhs.url.absoluteString < rhs.url.absoluteString
	}
}

// MARK: - GatewayView
public struct GatewayView: SwiftUI.View {
	public let gateway: Gateway
	public let isCurrent: Bool
	public let action: () -> Void

	public var body: some SwiftUI.View {
		Button(action: action, label: {
			HStack {
				Text(isCurrent ? "✅" : "☑️").font(.title)
				VStack {
					Text("\(gateway.network.displayDescription)").font(.body)
					Text("\(gateway.networkID.toString())")
				}
			}
		})
		.buttonStyle(.plain)
		.frame(maxWidth: .infinity, alignment: .leading)
		.cornerRadius(.small1)
	}
}
