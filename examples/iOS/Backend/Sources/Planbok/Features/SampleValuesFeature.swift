import Sargon
import ComposableArchitecture

#if DEBUG
@Reducer
public struct SampleValuesFeature {
	
	@ObservableState
	public struct State: Equatable {}
	
	public enum Action {}
	
	public init() {}
	
	public struct View: SwiftUI.View {
		public let store: StoreOf<SampleValuesFeature>
		public var body: some SwiftUI.View {
			Form {
				Section("TX Manifests") {
					Text("`\(TransactionManifest.sample.description)`")
				}
				.font(.footnote)
				
				Section("Addresses") {
					SampleAddressesView()
				}
			}
			.frame(maxWidth: .infinity, alignment: .leading)
			.padding()
		}
	}
}

#endif
