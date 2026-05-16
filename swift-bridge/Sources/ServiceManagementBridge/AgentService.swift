import Foundation
import ServiceManagement

@_cdecl("sm_agent_service")
public func sm_agent_service(
  _ plistName: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard #available(macOS 13.0, *) else {
    smSetError(errorOut, "SMAppService requires macOS 13+")
    return nil
  }
  guard let plistName else {
    smSetError(errorOut, "missing launch agent plist name")
    return nil
  }
  return smRetainedAppService(
    SMAppService.agent(plistName: String(cString: plistName))
  )
}
