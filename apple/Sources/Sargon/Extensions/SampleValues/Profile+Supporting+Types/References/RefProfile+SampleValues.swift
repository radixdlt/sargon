import Foundation
import SargonUniFFI

#if DEBUG
extension RefProfile {
	public static var sample: Self { newRefProfileSample() as! Self }
	public static var sampleOther: Self { newRefProfileSampleOther() as! Self }
}
#endif // DEBUG
