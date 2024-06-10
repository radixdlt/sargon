//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-09.
//

import Foundation
import Sargon
import ComposableArchitecture

@Reducer
public struct ShieldDetailsFeature {
	
	@ObservableState
	public struct State {
		public let shield: Shield
	}
	
	public enum Action: ViewAction {
		public enum ViewAction {}
		case view(ViewAction)
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
					section(role: store.shield.matrixOfFactors.primaryRole)
				}
			}
			.navigationTitle(store.state.shield.metadata.displayName.value)
		}
		func section<R: RoleFromDraft>(role roleWithFactors: R) -> some SwiftUI.View {
			VStack {
				VStack(alignment: .leading) {
					let role = R.role
					Text(role.title)
					HStack {
						Image(systemName: role.icon)
						VStack {
							Text(role.action)
							Text(role.actionDetailed)
						}
					}
				}
				
				Text("Must present \(roleWithFactors.thresholdAmount) of the following")
				ForEach(roleWithFactors.thresholdFactors) { factorSource in
					FactorView(factor: Factor(factorSource: factorSource), pickAction: nil, removeAction: nil)
				}
			}
		}
	}
	
}

extension RoleFromDraft {
	var thresholdAmount: String {
		
		if threshold == thresholdFactors.count {
			return "All"
		} else if threshold == 1 {
			return "Any"
		} else {
			return String(describing: threshold)
		}
		
	}
}

extension Role {
	var icon: String {
		switch self {
		case .primary: return "pencil.and.scribble"
		case .recovery: return "cross.case.circle.fill"
		case .confirmation: return "person.badge.shield.checkmark"
		}
	}
}
