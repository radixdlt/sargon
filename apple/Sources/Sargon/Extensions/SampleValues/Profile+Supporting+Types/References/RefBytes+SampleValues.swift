import Foundation
import SargonUniFFI

#if DEBUG
extension RefBytes {
	public static var sample: Self { newRefBytesSample() as! Self }
	public static var sampleOther: Self { newRefBytesSampleOther() as! Self }
}
#endif // DEBUG
