//
//  PlanbokApp.swift
//  Planbok
//
//  Created by Alexander Cyon on 2024-02-14.
//

import SwiftUI
import Planbok
import ComposableArchitecture

extension BIOS: ObservableObject {}

@main
struct PlanbokApp: App {
	
	let bios = BIOS(
		bundle: .main,
		keychainService: "works.rdx.planbok",
		userDefaultsSuite: "works.rdx.planbok"
	)
	
	var body: some Scene {
		WindowGroup {
			AppFeature.View(
				store: Store(
					initialState: AppFeature.State(bios: bios)
				) {
					AppFeature()
				}
			)
			.textFieldStyle(.roundedBorder)
			.buttonStyle(.borderedProminent)
		}
    }
}
