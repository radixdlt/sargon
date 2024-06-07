//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-07.
//

import Foundation
import Sargon
import ComposableArchitecture
import SwiftUI

@Reducer
public struct PickFactorSourceCoordinator {
	
	@ObservableState
	public struct State: Equatable {}
}

extension PickFactorSourceCoordinator {
	public typealias HostingFeature = Self
	
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			Text("PickFactorSourceCoordinator")
		}
	}
}
