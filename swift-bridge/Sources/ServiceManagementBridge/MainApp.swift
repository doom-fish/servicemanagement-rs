import Foundation
import ServiceManagement

@_cdecl("sm_main_app_service")
public func sm_main_app_service(
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard #available(macOS 13.0, *) else {
    smSetError(errorOut, "SMAppService requires macOS 13+")
    return nil
  }
  return smRetainedAppService(SMAppService.mainApp)
}
