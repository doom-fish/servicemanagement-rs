import Foundation
import ServiceManagement

private func smSetError(
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
  _ message: String
) {
  errorOut?.pointee = smCString(message)
}

@_cdecl("sm_app_service_main_app")
public func sm_app_service_main_app(
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard #available(macOS 13.0, *) else {
    smSetError(errorOut, "SMAppService requires macOS 13+")
    return nil
  }
  return smRetain(SMAppService.mainApp)
}

@_cdecl("sm_app_service_login_item")
public func sm_app_service_login_item(
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
  return smRetain(SMAppService.loginItem(identifier: String(cString: identifier)))
}

@_cdecl("sm_app_service_agent")
public func sm_app_service_agent(
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
  return smRetain(SMAppService.agent(plistName: String(cString: plistName)))
}

@_cdecl("sm_app_service_daemon")
public func sm_app_service_daemon(
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
  return smRetain(SMAppService.daemon(plistName: String(cString: plistName)))
}

@_cdecl("sm_app_service_status")
public func sm_app_service_status(_ rawService: UnsafeMutableRawPointer?) -> Int32 {
  guard #available(macOS 13.0, *) else {
    return -1
  }
  guard let rawService else {
    return -2
  }
  let service: SMAppService = smBorrow(rawService)
  return Int32(service.status.rawValue)
}

@_cdecl("sm_app_service_register")
public func sm_app_service_register(
  _ rawService: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard #available(macOS 13.0, *) else {
    smSetError(errorOut, "SMAppService requires macOS 13+")
    return false
  }
  guard let rawService else {
    smSetError(errorOut, "missing SMAppService handle")
    return false
  }
  let service: SMAppService = smBorrow(rawService)
  do {
    try service.register()
    return true
  } catch {
    smSetError(errorOut, (error as NSError).localizedDescription)
    return false
  }
}

@_cdecl("sm_app_service_unregister")
public func sm_app_service_unregister(
  _ rawService: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard #available(macOS 13.0, *) else {
    smSetError(errorOut, "SMAppService requires macOS 13+")
    return false
  }
  guard let rawService else {
    smSetError(errorOut, "missing SMAppService handle")
    return false
  }
  let service: SMAppService = smBorrow(rawService)
  do {
    try service.unregister()
    return true
  } catch {
    smSetError(errorOut, (error as NSError).localizedDescription)
    return false
  }
}

@_cdecl("sm_open_system_settings_login_items")
public func sm_open_system_settings_login_items(
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard #available(macOS 13.0, *) else {
    smSetError(errorOut, "SMAppService requires macOS 13+")
    return false
  }
  SMAppService.openSystemSettingsLoginItems()
  return true
}

@_cdecl("sm_app_service_release")
public func sm_app_service_release(_ rawService: UnsafeMutableRawPointer?) {
  guard let rawService else { return }
  smRelease(rawService)
}
