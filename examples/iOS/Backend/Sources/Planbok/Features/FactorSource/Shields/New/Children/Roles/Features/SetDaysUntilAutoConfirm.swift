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
public struct SetDaysUntilAutoConfirm {
	@ObservableState
	public struct State: Equatable {
		@Shared(.newShieldDraft) var newShieldDraft
		public var daysString = ""
		public init() {
			daysString = daysUntilAutoConfirm.description
		}
		public var daysUntilAutoConfirm: UInt16 {
			get {
				newShieldDraft.numberOfDaysUntilAutoConfirmation
			}
			set {
				newShieldDraft.numberOfDaysUntilAutoConfirmation = newValue
			}
		}
		
		public var daysFromString: UInt16? {
			UInt16(daysString)
		}
	}
	
	@CasePathable
	public enum Action: ViewAction {
		
		public enum DelegateAction {
			case done
		}
		@CasePathable
		public enum ViewAction {
			case confirmButtonTapped
			case numberOfDaysChanged(String)
		}
		case view(ViewAction)
		case delegate(DelegateAction)
	}
	
	
	public var body: some ReducerOf<Self> {
		
		Reduce { state, action in
			switch action {
				
			case let .view(.numberOfDaysChanged(daysString)):
				state.daysString = daysString
				if let days = state.daysFromString {
					state.daysUntilAutoConfirm = days
				}
				return .none
			case .view(.confirmButtonTapped):
				return .send(.delegate(.done))
				
			case .delegate:
				return .none
			}
		}
	}
				
}

extension SetDaysUntilAutoConfirm {
	public typealias HostingFeature = SetDaysUntilAutoConfirm
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			VStack {
				Text("Number of days until auto confirm")
					.font(.title)
				
				Text("Will auto confirm after \(store.daysUntilAutoConfirm.description) days.")
				
				LabeledTextField(label: "#Days", text: $store.daysUntilAutoConfirm.description.sending(\.view.numberOfDaysChanged))
					.keyboardType(.numberPad)
				
				Button("Confirm") {
					send(.confirmButtonTapped)
				}
				.disabled(store.daysFromString == nil)
			}
			.foregroundStyle(Color.app.blue1)
			.navigationTitle("Set Days")
			.padding()
		}
	}
	
}
