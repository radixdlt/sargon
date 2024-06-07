//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-06-06.
//

import Foundation
import Sargon
import ComposableArchitecture

public enum FactorThreshold: Hashable, Sendable {
	case any
	case all
	case threshold(UInt8)
}

public enum Factor: Hashable, Sendable, Identifiable {
	public enum ID: Hashable, Sendable {
		case placeholder(UUID)
		case factor(FactorSourceID)
	}
	case placeholder(UUID)
	case factor(FactorSource)
	var factorSource: FactorSource? {
		switch self {
		case .placeholder: return nil
		case let .factor(factor): return factor
		}
	}
	public var id: ID {
		switch self {
		case let .placeholder(id): .placeholder(id)
		case let .factor(factor): .factor(factor.id)
		}
	}
}

@Reducer
public struct PrimaryRoleFactorsFeature {
	public typealias Factors = IdentifiedArrayOf<Factor>
	
	@ObservableState
	public struct State: Equatable {
		//		@SharedReader(.factorSources) var allInProfile
		//
		//		var allPicked: FactorSources {
		//			var picked = FactorSources()
		//			func addFrom(_ factors: Factors) {
		//				picked.append(contentsOf: factors.compactMap(\.factorSource))
		//			}
		//			addFrom(self.thresholdFactors)
		//			addFrom(self.overrideFactors)
		//			return picked
		//		}
		//
		//		var available: FactorSources {
		//			let picked = Set(allPicked.map(\.id))
		//			return allInProfile.filter({ !picked.contains($0.id) }).asIdentified()
		//		}
		
		public var thresholdFactors: Factors = [.factor(.sample)]
		public var threshold: FactorThreshold = .any
		public var overrideFactors: Factors = []
	}
	
	@CasePathable
	public enum Action: ViewAction {
		@CasePathable
		public enum ViewAction {
			case confirmButtonTapped
			case pickButtonTapped(Factor.ID)
			case thresholdFactorsChanged(Factors)
			case overrideFactorsChanged(Factors)
		}
		public enum DelegateAction {
			case `continue`
		}
		case view(ViewAction)
		case delegate(DelegateAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
				
			case .view(.confirmButtonTapped):
				return .send(.delegate(.continue))
				
			case let .view(.pickButtonTapped(factorID)):
				print("pick: \(String(describing: factorID))")
				return .none
				
			case let .view(.thresholdFactorsChanged(new)):
				state.thresholdFactors = new
				return .none
				
			case let .view(.overrideFactorsChanged(new)):
				state.overrideFactors = new
				return .none
				
			case .delegate:
				return .none
			}
		}
	}
}

extension PrimaryRoleFactorsFeature {
	public typealias HostingFeature = Self
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			ScrollView {
				VStack(alignment: .center, spacing: 10) {
					Text("Sign Transactions").font(.largeTitle)
					
					Text("These factors are required to withdraw your assets and log in to dApps.")
						.foregroundStyle(Color.appDarkGray)
					
					FactorsBuilderView(
						factors: $store.thresholdFactors.sending(\.view.thresholdFactorsChanged),
						title: "Threshold Factors",
						titleAction: {
							log.info("Threshold factors rule!")
						},
						changeThresholdAction: {
							log.info("TODO change threshold")
						},
						pickAction: { id in
							send(.pickButtonTapped(id))
						}
					)
					
					FactorsBuilderView(
						factors: $store.overrideFactors.sending(\.view.overrideFactorsChanged),
						title: "Override Factors",
						titleAction: {
							log.info("Override factors are good.")
						},
						changeThresholdAction: nil,
						pickAction: { id in
							send(.pickButtonTapped(id))
						}
					)
					
					Button("Confirm") {
						send(.confirmButtonTapped)
					}
					.buttonStyle(.borderedProminent)
				}
				.padding()
			}
		}
	}
}

public struct LabelStyleFlip: LabelStyle {
	let imageColor: Color
	public func makeBody(configuration: Configuration) -> some View {
		HStack(alignment: .center) {
			configuration.title
			configuration.icon.foregroundStyle(imageColor)
		}
	}
}
extension LabelStyle where Self == LabelStyleFlip {
	public static func flipped(imageColor: Color = .gray) -> Self { LabelStyleFlip(imageColor: imageColor) }
}

public struct FactorsBuilderView: SwiftUI.View {
	
	@Binding var factors: IdentifiedArrayOf<Factor>
	
	public let title: LocalizedStringKey
	public let titleAction: () -> Void
	public let changeThresholdAction: (() -> Void)?
	public let pickAction: (Factor.ID) -> Void
	
	
	public var body: some SwiftUI.View {
		VStack(spacing: 0) {
			HStack {
				Button(
					action: titleAction,
					label: {
						Label(title, systemImage: "info.circle")
							.labelStyle(.flipped())
					}
				)
				Spacer()
			}
			.padding()
			
			Divider().background(Color.appGray)
			
			
			VStack(spacing: 0) {
				ForEach(factors) { factor in
					FactorView(
						factor: factor,
						pickAction: pickAction
					) {
						self.factors.remove(
							id: factor.id
						)
					}
				}
				.padding(.horizontal)
				.padding(.top, 10)
				
				Spacer()
				
				Button("Add factors") {
					self.factors.append(Factor.placeholder(.init()))
				}
				.foregroundStyle(Color.appDarkGray)
				.padding()
			}
			.frame(maxWidth: .infinity, minHeight: 50)
			.background(Color.appSuperLightGray)
			
			
			Divider().background(Color.appGray)
			
			Button.init(action: {
				changeThresholdAction?()
			}, label: {
				HStack {
					Text("Factors required to sign transactions?")
					Spacer()
					Text("Any")
						.fontWeight(.bold)
						.foregroundStyle(changeThresholdAction == nil ? Color.appDarkGray : Color.appBlue)
				}
				.multilineTextAlignment(.leading)
			})
			.padding()
			.disabled(changeThresholdAction == nil)
			
		}
		.foregroundStyle(Color.appBlack)
		.overlay(
			RoundedRectangle(cornerRadius: 15)
				.inset(by: 1)
				.stroke(.gray, lineWidth: 1)
		)
		.padding()
	}
	
	public struct FactorView: SwiftUI.View {
		public let factor: Factor
		public let pickAction: (Factor.ID) -> Void
		public let removeAction: () -> Void
		public var body: some SwiftUI.View {
			HStack {
				Button(action: { pickAction(factor.id) }, label: {
					switch factor {
					case .placeholder:
						Text("Select a factor")
							.fontWeight(.bold)
					case let .factor(factorSource):
						HStack {
							if let factorImageName = factorSource.kind.image {
								Image(systemName: factorImageName)
									.imageScale(.large)
								
							}
							VStack(alignment: .leading) {
								Text("\(factorSource.kind.title)")
								if let subtitle = factorSource.kind.subtitle {
									Text("\(subtitle)")
										.foregroundStyle(Color.gray)
								}
							}
						}
					}
				})
				.frame(maxWidth: .infinity, alignment: .leading)
				.padding()
				.background(Color.white)
				.clipShape(.rect(cornerRadius: 20))
				
				Spacer()
				
				Button(action: removeAction, label: {
					Image(systemName: "plus").rotationEffect(.degrees(45))
				})
			}
			
		}
	}
}

extension FactorSourceKind {
	public var image: String? {
		switch self {
		case .device: return "lock.iphone"
		default: return nil
		}
	}
	public var title: String {
		switch self {
		case .device: return "This Phone"
		default: return self.toString()
		}
	}
	public var subtitle: String? {
		switch self {
		case .device: return "Face ID / PIN"
		default: return nil
		}
	}
}

extension Color {
	static let appBlack = Color(red: 0x14, green: 0x31, blue: 0x55)
	static let appBlue = Color(red: 0, green: 0, blue: 0xb5)
	static let appGray = Color(red: 0xe0, green: 0xe2, blue: 0xe9)
	static let appDarkGray = Color(red: 0x8f, green: 0x93, blue: 0xa6)
	static let appLightGray = Color(red: 0x8b, green: 0x8f, blue: 0xa2)
	static let appSuperLightGray = Color(red: 0xf4, green: 0xf5, blue:  0xf9)
	
}

#Preview {
	PrimaryRoleFactorsFeature.View(
		store: Store(
			initialState: PrimaryRoleFactorsFeature.State(),
			reducer: {
				PrimaryRoleFactorsFeature()
			}
		)
	)
}

/*
#Preview {
	VStack {
		
		FactorsBuilderView(
			factors: .init(get: { [FactorSource.sample].map({ .factor($0) }).asIdentified() }, set: {
				print("Preview NOOP set factors sources to: \($0)")
			}),
			title: "Threshold",
			titleAction: {
				print("Preview NOOP - titleAction")
			},
			changeThresholdAction: { print("Preview NOOP - changeThresholdAction") },
			pickAction: { id in
				print("Preview NOOP - pickAction")
			}
		)
		FactorsBuilderView(
			factors: .init(get: { [] }, set: {
				print("Preview NOOP set factors sources to: \($0)")
			}),
			title: "Override",
			titleAction: {
				print("Preview NOOP - titleAction")
			},
			changeThresholdAction: nil,
			pickAction: { id in
				print("Preview NOOP - pickAction")
			}
		)
	}
	
	.foregroundStyle(Color.appBlack)
}
 */
