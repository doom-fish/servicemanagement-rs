import Dispatch
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

@_cdecl("sm_app_service_status_for_legacy_plist_async")
public func sm_app_service_status_for_legacy_plist_async(
  _ path: UnsafePointer<CChar>?,
  _ context: UnsafeMutableRawPointer?,
  _ callback: SMAppServiceStatusCallback?
) {
  guard let callback else {
    return
  }
  guard #available(macOS 13.0, *) else {
    callback(context, -1, smCString("SMAppService requires macOS 13+"))
    return
  }
  guard let path else {
    callback(context, -1, smCString("missing legacy plist path"))
    return
  }

  let url = URL(fileURLWithPath: String(cString: path))
  DispatchQueue.global(qos: .userInitiated).async {
    callback(context, Int32(SMAppService.statusForLegacyPlist(at: url).rawValue), nil)
  }
}
