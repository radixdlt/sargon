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
			switchGatewayTo: { to in
				log.notice("Changing current gateway to: \(to)")
				let _ = try await os.changeCurrentGateway(to: to)
				log.info("Finished changing current gateway.")
			}
		)
	}
}
