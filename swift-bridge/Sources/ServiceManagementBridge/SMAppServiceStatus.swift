import Foundation
import ServiceManagement

@_cdecl("sm_app_service_status_for_legacy_plist")
public func sm_app_service_status_for_legacy_plist(
  _ path: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
  guard #available(macOS 13.0, *) else {
    smSetError(errorOut, "SMAppService requires macOS 13+")
    return -1
  }
  guard let path else {
    smSetError(errorOut, "missing legacy plist path")
    return -1
  }
  let url = URL(fileURLWithPath: String(cString: path))
  return Int32(SMAppService.statusForLegacyPlist(at: url).rawValue)
}
