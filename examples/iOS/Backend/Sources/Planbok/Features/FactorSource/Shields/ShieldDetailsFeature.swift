import ComposableArchitecture
import Foundation
import Sargon

// MARK: - ShieldDetailsFeature
@Reducer
public struct ShieldDetailsFeature {
	@ObservableState
	public struct State {
		public let shield: Shield
	}

	public enum Action: ViewAction {
		public enum ViewAction {
			case copyAndEditButtonTapped
		}

		public enum DelegateAction {
			case copyAndEdit(Shield)
		}

		case view(ViewAction)
		case delegate(DelegateAction)
	}

	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.copyAndEditButtonTapped):
				.send(.delegate(.copyAndEdit(state.shield)))
			case .delegate:
				.none
			}
		}
	}
}

extension ShieldDetailsFeature {
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
					VStack(alignment: .leading, spacing: 30) {
						section(role: store.shield.matrixOfFactors.primaryRole)

						section(role: store.shield.matrixOfFactors.recoveryRole)

						section(role: store.shield.matrixOfFactors.confirmationRole)
					}
					.padding()
				}
			}
			.background(Color.app.gray5)
			.navigationTitle(store.state.shield.metadata.displayName.value)
			.navigationBarTitleDisplayMode(.large)
			.toolbar {
				ToolbarItem(placement: .topBarTrailing) {
					Button("Copy & Edit") {
						send(.copyAndEditButtonTapped)
					}
					.foregroundStyle(.blue)
					.buttonStyle(.plain)
				}
			}
		}

		func section<R: RoleFromDraft>(role roleWithFactors: R) -> some SwiftUI.View {
			VStack(alignment: .leading, spacing: 3) {
				VStack(alignment: .leading) {
					let role = R.role
					if let detailTitle = role.detailTitle, let icon = role.smallIcon {
						Label(detailTitle, systemImage: icon)
							.imageScale(.small)
							.font(.title2)
							.fontWeight(.bold)
							.foregroundStyle(Color.app.gray2)
					}
					HStack(spacing: 15) {
						Image(systemName: role.largeIcon)
							.resizable()
							.aspectRatio(contentMode: .fit)
							.imageScale(.medium)
							.frame(width: 40)
							.padding()
							.foregroundStyle(Color.app.white)
							.background(Color.app.gray3)
							.clipShape(.rect(cornerRadius: 10))

						VStack(alignment: .leading) {
							Text(role.action.capitalized)
								.font(.title2)
								.fontWeight(.bold)
							Text("Required to " + role.actionVeryDetailed)
								.font(.footnote)
								.foregroundStyle(Color.app.gray2)
						}
						Spacer()
					}
					.frame(maxWidth: .infinity)
					.padding()
					.background(Color.app.white)
					.clipShape(.rect(topLeadingRadius: 10, topTrailingRadius: 10))
				}
				.multilineTextAlignment(.leading)

				sectionFactors(
					factors: roleWithFactors.thresholdFactors,
					factorAmount: "Must present **\(roleWithFactors.thresholdAmount)** of the following",
					emptyFactors: "No threshold factors set",
					roundCorners: false
				)

				sectionFactors(
					factors: roleWithFactors.overrideFactors,
					factorAmount: "Or must present **1** of the following",
					emptyFactors: "No override factors set"
				)
			}
		}

		func sectionFactors(
			factors: [FactorSource],
			factorAmount: LocalizedStringKey,
			emptyFactors: LocalizedStringKey,
			roundCorners: Bool = true
		) -> some SwiftUI.View {
			VStack(alignment: .leading) {
				if !factors.isEmpty {
					HStack {
						Text(factorAmount)
						Spacer()
					}

					ForEach(factors) { factorSource in
						FactorView(factorSource)
							.foregroundStyle(Color.app.gray1)
							.overlay(
								RoundedRectangle(cornerRadius: 15)
									.inset(by: 1)
									.stroke(.gray, lineWidth: 1)
							)
					}
				} else {
					HStack {
						Text(emptyFactors)
						Spacer()
					}
				}
			}
			.foregroundStyle(Color.app.gray1)
			.frame(maxWidth: .infinity)
			.multilineTextAlignment(.leading)
			.padding()
			.background(Color.app.white)
			.clipShape(.rect(bottomLeadingRadius: roundCorners ? 10 : 0, bottomTrailingRadius: roundCorners ? 10 : 0))
			.shadow(color: Color.app.gray3, radius: 4, x: 0, y: 1)
		}
	}
}

extension RoleFromDraft {
	var thresholdAmount: String {
		if threshold == thresholdFactors.count {
			"All"
		} else if threshold == 1 {
			"Any"
		} else {
			String(describing: threshold)
		}
	}
}

extension Role {
	var detailTitle: String? {
		switch self {
		case .primary: "Transactions"
		case .recovery: "Recovery Assistance"
		case .confirmation: nil
		}
	}

	public var actionVeryDetailed: String {
		switch self {
		case .primary: "withdraw your assets and log in to dApps."
		case .recovery: "lock your account or start recovering your accounts if you lose your phone."
		case .confirmation: "confirm recovery"
		}
	}

	var smallIcon: String? {
		switch self {
		case .primary: "pencil.circle"
		case .recovery: "wrench.and.screwdriver"
		case .confirmation: nil
		}
	}

	var largeIcon: String {
		switch self {
		case .primary: "pencil.and.list.clipboard"
		case .recovery: "cross.case"
		case .confirmation: "person.badge.shield.checkmark"
		}
	}
}

#Preview {
	NavigationStack {
		ShieldDetailsFeature.View(
			store: Store(
				initialState: ShieldDetailsFeature.State(shield: .sample)
			) {
				ShieldDetailsFeature()
			}
		)
	}
}
