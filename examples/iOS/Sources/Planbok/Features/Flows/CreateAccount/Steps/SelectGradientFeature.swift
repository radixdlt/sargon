//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-02-16.
//

import Foundation

@Reducer
public struct SelectGradientFeature {
	
	@ObservableState
	public struct State: Equatable {
		public let name: DisplayName
		public var gradient: AppearanceID
		public init(
			name: DisplayName,
			gradient: AppearanceID = AppearanceID.allCases.first!
		) {
			self.name = name
			self.gradient = gradient
		}
	}
	
	@CasePathable
	public enum Action: ViewAction {
		public enum Delegate {
			case selected(AppearanceID, DisplayName)
		}
		@CasePathable
		public enum ViewAction {
			case selectedGradient(AppearanceID)
			case confirmedGradientButtonTapped
		}
		case view(ViewAction)
		case delegate(Delegate)
	}
	
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
						Button.init(action: { send(.selectedGradient(appearanceID)) }, label: {
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
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			
			case let .view(.selectedGradient(gradient)):
				state.gradient = gradient
				return .none
			
			case .view(.confirmedGradientButtonTapped):
				return .send(.delegate(.selected(state.gradient, state.name)))
				
			default:
				return .none
				
			}
		}
	}
}
