import ComposableArchitecture
import Foundation
import Sargon

// MARK: - PickFactorSelectKindFeature
@Reducer
public struct PickFactorSelectKindFeature {
	@Dependency(\.dismiss) var dismiss

	@ObservableState
	public struct State: Equatable {
		@Shared(.newShieldDraft) var newShieldDraft

		public let role: Role
		public init(role: Role) {
			self.role = role
		}

		public var usedFactorsForRole: FactorSources {
			matrixOfFactorsForRole.usedFactorSources
		}

		public var matrixOfFactorsForRole: MatrixOfFactorsForRole {
			get { newShieldDraft[role] }
			set {
				newShieldDraft[role] = newValue
			}
		}
	}

	@CasePathable
	public enum Action: ViewAction {
		public enum ViewAction {
			case kindButtonTapped(FactorSourceKind)
			case dismissButtonTapped
		}

		public enum DelegateAction {
			case selectedKind(FactorSourceKind)
		}

		case view(ViewAction)
		case delegate(DelegateAction)
	}

	public var body: some ReducerOf<Self> {
		Reduce { _, action in
			switch action {
			case let .view(.kindButtonTapped(kind)):
				.send(.delegate(.selectedKind(kind)))
			case .view(.dismissButtonTapped):
				.run { _ in
					await dismiss()
				}
			case .delegate:
				.none
			}
		}
	}
}

extension PickFactorSelectKindFeature {
	public typealias HostingFeature = Self

	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>

		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			VStack(alignment: .leading) {
				ScrollView {
					ForEach(FactorSourceKind.allCases) { kind in
						let unavailability = kind.unavailabilityForRole(store.role, usedFactorsForRole: store.usedFactorsForRole)
						Button(action: {
							send(.kindButtonTapped(kind))
						}, label: {
							VStack(alignment: .leading) {
								Text("`\(kind.title)`")
								if let unavailability {
									Text("\(unavailability.toString(kind: kind))")
								}
							}
						})
						.disabled(unavailability != nil)
					}
				}
				.navigationTitle("Pick Factor Kind")
				.toolbar {
					ToolbarItem(placement: .cancellationAction) {
						Button("Close") {
							send(.dismissButtonTapped)
						}
						.foregroundStyle(.blue)
						.buttonStyle(.plain)
					}
				}
			}
			.padding()
		}
	}
}
