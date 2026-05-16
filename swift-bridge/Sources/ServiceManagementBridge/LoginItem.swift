import Foundation
import ServiceManagement

@_cdecl("sm_login_item_service")
public func sm_login_item_service(
  _ identifier: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard #available(macOS 13.0, *) else {
    smSetError(errorOut, "SMAppService requires macOS 13+")
    return nil
  }
  guard let identifier else {
    smSetError(errorOut, "missing login item identifier")
    return nil
  }
  return smRetainedAppService(
    SMAppService.loginItem(identifier: String(cString: identifier))
  )
}
