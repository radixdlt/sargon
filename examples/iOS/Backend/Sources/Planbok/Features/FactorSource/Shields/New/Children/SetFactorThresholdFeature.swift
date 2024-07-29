//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-06-07.
//

import Foundation
import ComposableArchitecture
import Sargon
import SwiftUI

@Reducer
public struct SetFactorThresholdFeature {
	
	@ObservableState
	public struct State: Equatable {
		@Shared(.newShieldDraft) var newShieldDraft
		public let role: Role
		public var threshold: FactorThreshold
		
		public init(role: Role) {
			self.role = role
			self.threshold = .all
			
			self.threshold = alreadySet
		}
	
		public var matrixOfFactorsForRole: MatrixOfFactorsForRole {
			newShieldDraft[role]
		}
		
		public var numberOfFactors: Int {
			matrixOfFactorsForRole.thresholdFactors.count
		}
		public var options: [FactorThreshold] {
			var options: [FactorThreshold] = [.any, .all]
			guard numberOfFactors > 0 else {
				return options
			}
			let exceeding1 = UInt16(numberOfFactors - 1)
			if exceeding1 > 1 {
				options.append(contentsOf: (1...exceeding1).map(FactorThreshold.threshold))
			}
			return options
		}
		
		
		public var alreadySet: FactorThreshold {
			matrixOfFactorsForRole.threshold
		}
		public var recommended: FactorThreshold {
			.any
		}

		
	}
	
	public enum Action: ViewAction {
		public enum ViewAction {
			case changedThreshold(Int)
			case confirmButtonTapped
		}
		public enum DelegateAction {
			case confirm
		}
		case view(ViewAction)
		case delegate(DelegateAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
		
			case let .view(.changedThreshold(index)):
				var index = max(index, 0)
				index = min(index, state.options.count - 1)
				state.threshold = state.options[index]
				return .none
				
			case .view(.confirmButtonTapped):
				state.newShieldDraft[state.role].threshold = state.threshold
				return .send(.delegate(.confirm))
				
			case .delegate:
				return .none
			}
		}
	}
}

extension SetFactorThresholdFeature {
	
	public typealias HostingFeature = Self
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			VStack(alignment: .center) {
				Text("Choose the number of security factors required for \(store.role.title)")
				scrollView
				Text("Use \(store.threshold) of your security factors to \(store.role.action)")
				Button("Confirm") {
					send(.confirmButtonTapped)
				}
			}
			.lineLimit(nil)
			.padding()
		}
		
		var scrollView: some SwiftUI.View {
			GeometryReader { geo in
				let cellWidth = max(geo.size.width / 4, 50) // must use `max`, will be `0` initially...
				let contentMarginX = (geo.size.width - cellWidth) / 2
				ScrollView(.horizontal, showsIndicators: false) {
					HStack(alignment: .top, spacing: 0) {
						ForEach(store.options, id: \.self) { option in
							VStack(alignment: .center, spacing: 0) {
								Text("\(option)").font(.system(size: 45))
									.fontWeight(.bold)
									.frame(width: cellWidth)
									.foregroundStyle(option == store.threshold ? Color.app.blue1 : Color.app.gray5)
								Group {
									if option == store.alreadySet || option == store.recommended  {
										Text(option == store.alreadySet ? "Current" : "Recommended")
											.font(.system(size: 10))
											.padding(5)
											.background(Color.app.gray4)
											.mask(Capsule())
									} else {
										Spacer()
									}
								}
								.frame(width: cellWidth, height: 20)
							}
							.id(option)
							.frame(width: cellWidth)
						}
					}
					.background(
						GeometryReader {
							Color.clear.preference(
								key: ViewOffsetKey.self,
								value: -$0.frame(
									in: .named(coordinateSpaceScrollView)
								)
								.origin.x
							)
						}
					)
					.onPreferenceChange(ViewOffsetKey.self) {
						let positionX = contentMarginX + $0
						
						send(.changedThreshold(Int(CGFloat(positionX / cellWidth).rounded())))
					}
				}
				.coordinateSpace(name: coordinateSpaceScrollView)
				.contentMargins(contentMarginX)
			}
		}
		private let coordinateSpaceScrollView = "coordinateSpaceScrollView"
	}
}

struct ViewOffsetKey: PreferenceKey {
	typealias Value = CGFloat
	static let defaultValue = CGFloat.zero
	
	static func reduce(
		value: inout Value,
		nextValue: () -> Value
	) {
		value += nextValue()
	}
}

extension Role {
	public var title: String {
		switch self {
		case .primary: return "Signing"
		case .recovery: return "Wallet lock & recovery"
		case .confirmation: return "Confirm Recovery"
		}
	}
	public var action: String {
		switch self {
		case .primary: return "sign transaction"
		case .recovery: return "initiate recovery"
		case .confirmation: return "confirm recovery"
		}
	}
	public var actionDetailed: String {
		switch self {
		case .primary: return "withdraw your assets and log in to dApps."
		case .recovery: return "initiate recovery"
		case .confirmation: return "confirm recovery"
		}
	}

}

//#Preview {
//	let role: Role = .primary
//	@Shared(.thresholdFactors) var thresholdFactors
//	thresholdFactors[role] = FactorThreshold.all
//SetFactorThresholdFeature.View(
//	store: Store(
//		initialState: SetFactorThresholdFeature.State(
//			role: role,
//			numberOfFactors: 5
//		),
//	reducer: {
//		SetFactorThresholdFeature()
//	}
//))
//}
