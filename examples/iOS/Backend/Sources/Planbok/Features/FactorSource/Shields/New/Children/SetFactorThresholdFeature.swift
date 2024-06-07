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
		public let role: Role
		public let alreadySet: FactorThreshold
		public var recommended: FactorThreshold {
			.any
		}
		public var threshold: FactorThreshold
		public let numberOfFactors: Int
		
		public var options: [FactorThreshold] {
			var options: [FactorThreshold] = [.any, .all]
			let exceeding1 = UInt8(10 - 1)
//			if exceeding1 > 1 {
				options.append(contentsOf: (1..<exceeding1).map(FactorThreshold.threshold))
//			}
			return options
		}
		
		public init(role: Role, numberOfFactors: Int, threshold alreadySet: FactorThreshold) {
			self.role = role
			self.numberOfFactors = numberOfFactors
			self.alreadySet = alreadySet
			self.threshold = alreadySet
		}
	}
	
	public enum Action: ViewAction {
		public enum ViewAction {}
		case view(ViewAction)
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
				
				ScrollView {
					HStack {
						ForEach(store.options, id: \.self) { option in
							VStack {
								Text("\(option)").font(.largeTitle)
									.fontWeight(.bold)
									.foregroundStyle(option == store.threshold ? Color.app.blue1 : Color.app.gray5)
								if option == store.recommended {
									Text("Recommended")
								}
							}
							.frame(width: 70)
						}
					}
				}
				
				Text("Use \(store.threshold) of your security factors to \(store.role.action)")
			}
			.lineLimit(nil)
			.padding()
		}
	}
}

extension Role {
	public var title: String {
		switch self {
		case .primary: return "Signing"
		case .recovery: return "Initiate Recovery"
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
}
