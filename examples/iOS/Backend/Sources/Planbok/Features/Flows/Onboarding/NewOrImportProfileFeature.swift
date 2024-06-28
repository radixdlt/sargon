//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-02-16.
//

import Foundation
import ComposableArchitecture
import Sargon
import SwiftUI

@Reducer
public struct NewOrImportProfileFeature {
	public init() {}
	
	@ObservableState
	public struct State: Equatable {
		public init() {}
	}
	
	public enum Action: ViewAction {
		
		public enum DelegateAction {
			case newProfile
			case importProfile
		}
		
		public enum ViewAction {
			case newProfileButtonTapped
			case importProfileButtonTapped
		}
		
		case delegate(DelegateAction)
		case view(ViewAction)
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			
			case .view(.importProfileButtonTapped):
					.send(.delegate(.importProfile))
				
			case .view(.newProfileButtonTapped):
					.send(.delegate(.newProfile))
				
			case .delegate:
					.none
				
			}
		}
	}
}

extension NewOrImportProfileFeature {
	
	@ViewAction(for: NewOrImportProfileFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<NewOrImportProfileFeature>
		
		public var body: some SwiftUI.View {
			VStack {
				Text("Existing or new user?").font(.title)

				Button("New Profile") {
					send(.newProfileButtonTapped)
				}
				
				Button("Import Profile") {
					send(.importProfileButtonTapped)
				}
			}
			.padding()
		}
	}
}
