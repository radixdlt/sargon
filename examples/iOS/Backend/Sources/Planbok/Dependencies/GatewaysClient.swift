import DependenciesMacros
import Foundation
import Sargon

// MARK: - GatewaysClient
@DependencyClient
public struct GatewaysClient: Sendable {
	public typealias SwitchGatewayTo = @Sendable (Gateway) async throws -> Void
	public var switchGatewayTo: SwitchGatewayTo
}

// MARK: DependencyKey
extension GatewaysClient: DependencyKey {
	public static let liveValue = Self.live(os: SargonOS.shared)
	public static func live(os: SargonOS) -> Self {
		Self(
			switchGatewayTo: { to in
				log.notice("Changing current gateway to: \(to)")
				_ = try await os.changeCurrentGateway(to: to)
				log.info("Finished changing current gateway.")
			}
		)
	}
}
