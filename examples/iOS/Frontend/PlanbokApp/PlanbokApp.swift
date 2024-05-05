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
	

	init() {
		let bios = BIOS.init(
			drivers: .init(
				networking: URLSession.shared,
				secureStorage: Keychain(service: "rdx.works.planbok"),
				entropyProvider: EntropyProvider.shared,
				hostInfo: HostInfo(
					appVersion: "0.0.01"
				),
				logging: Log.shared,
				eventBus: EventBus.shared,
				fileSystem: FileSystem.shared,
				unsafeStorage: UnsafeStorage.init(
					userDefaults: .init(
						suiteName: "rdx.works"
					)!
				)
			)
		)
		
		BIOS.settingShared(shared: bios)
	}
	
	var body: some Scene {
		WindowGroup {
			AppFeature.View(
				store: Store(
					initialState: AppFeature.State()
				) {
					AppFeature()
				}
			)
			.textFieldStyle(.roundedBorder)
			.buttonStyle(.borderedProminent)
		}
    }
}
