import Foundation
import ServiceManagement

@_cdecl("sm_daemon_service")
public func sm_daemon_service(
  _ plistName: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard #available(macOS 13.0, *) else {
    smSetError(errorOut, "SMAppService requires macOS 13+")
    return nil
  }
  guard let plistName else {
    smSetError(errorOut, "missing launch daemon plist name")
    return nil
  }
  return smRetainedAppService(
    SMAppService.daemon(plistName: String(cString: plistName))
  )
}
