import CoreFoundation
import Foundation
import ServiceManagement

@_cdecl("sm_legacy_login_item_set_enabled")
public func sm_legacy_login_item_set_enabled(
  _ identifier: UnsafePointer<CChar>?,
  _ enabled: Bool,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let identifier else {
    smSetError(errorOut, "missing SMLoginItem bundle identifier")
    return false
  }

  let bundleIdentifier = String(cString: identifier)
  let ok = SMLoginItemSetEnabled(bundleIdentifier as CFString, enabled)
  if !ok {
    smSetError(
      errorOut,
      "SMLoginItemSetEnabled returned false for \(bundleIdentifier)"
    )
  }
  return ok
}
