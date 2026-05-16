import Dispatch
import Foundation
import ServiceManagement

final class SMAppServiceHolder: NSObject {
  let service: SMAppService

  init(_ service: SMAppService) {
    self.service = service
  }
}

func smRetainedAppService(_ service: SMAppService) -> UnsafeMutableRawPointer {
  smRetain(SMAppServiceHolder(service))
}

func smBorrowAppService(
  _ rawService: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> SMAppService? {
  guard #available(macOS 13.0, *) else {
    smSetError(errorOut, "SMAppService requires macOS 13+")
    return nil
  }
  guard let rawService else {
    smSetError(errorOut, "missing SMAppService handle")
    return nil
  }
  let holder: SMAppServiceHolder = smBorrow(rawService)
  return holder.service
}

@_cdecl("sm_app_service_main_app")
public func sm_app_service_main_app(
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard #available(macOS 13.0, *) else {
    smSetError(errorOut, "SMAppService requires macOS 13+")
    return nil
  }
  return smRetainedAppService(SMAppService.mainApp)
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
  return smRetainedAppService(
    SMAppService.loginItem(identifier: String(cString: identifier))
  )
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
  return smRetainedAppService(
    SMAppService.agent(plistName: String(cString: plistName))
  )
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
  return smRetainedAppService(
    SMAppService.daemon(plistName: String(cString: plistName))
  )
}

@_cdecl("sm_app_service_status")
public func sm_app_service_status(_ rawService: UnsafeMutableRawPointer?) -> Int32 {
  guard let service = smBorrowAppService(rawService, nil) else {
    return -1
  }
  return Int32(service.status.rawValue)
}

@_cdecl("sm_app_service_register")
public func sm_app_service_register(
  _ rawService: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let service = smBorrowAppService(rawService, errorOut) else {
    return false
  }
  do {
    try service.register()
    return true
  } catch {
    smSetError(errorOut, smNSErrorMessage(error))
    return false
  }
}

@_cdecl("sm_app_service_unregister")
public func sm_app_service_unregister(
  _ rawService: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let service = smBorrowAppService(rawService, errorOut) else {
    return false
  }
  do {
    try service.unregister()
    return true
  } catch {
    smSetError(errorOut, smNSErrorMessage(error))
    return false
  }
}

@_cdecl("sm_app_service_unregister_with_completion")
public func sm_app_service_unregister_with_completion(
  _ rawService: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let service = smBorrowAppService(rawService, errorOut) else {
    return false
  }

  let semaphore = DispatchSemaphore(value: 0)
  var operationError: NSError?
  service.unregister { error in
    if let error {
      operationError = error as NSError
    }
    semaphore.signal()
  }
  semaphore.wait()

  if let operationError {
    smSetError(errorOut, operationError.localizedDescription)
    return false
  }
  return true
}

@_cdecl("sm_app_service_error_domain")
public func sm_app_service_error_domain(
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard #available(macOS 15.0, *) else {
    smSetError(errorOut, "SMAppServiceErrorDomain requires macOS 15+")
    return nil
  }
  return smCString(SMAppServiceErrorDomain as String)
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
