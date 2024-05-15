//
//  File.swift
//
//
//  Created by Alexander Cyon on 2024-05-05.
//

import Foundation
import Sargon
import DependenciesMacros

@DependencyClient
public struct GatewaysClient: Sendable {
	public typealias SwitchGatewayTo = @Sendable (Gateway) async throws -> Void
	public var switchGatewayTo: SwitchGatewayTo
}

extension GatewaysClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		Self(
			switchGatewayTo: { new in
				var profile = os.profile()
				try profile.appPreferences.gateways.changeCurrent(to: new)
				try await os.saveChangedProfile(profile)
			}
		)
	}
}
