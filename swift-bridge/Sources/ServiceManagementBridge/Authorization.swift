import Foundation
import Security

final class AuthorizationHolder: NSObject {
  var authorization: AuthorizationRef?

  init(_ authorization: AuthorizationRef) {
    self.authorization = authorization
  }

  deinit {
    if let authorization {
      _ = AuthorizationFree(authorization, AuthorizationFlags())
    }
  }
}

func smAuthorizationFlags(_ rawValue: UInt32) -> AuthorizationFlags {
  AuthorizationFlags(rawValue: rawValue)
}

func smAuthorizationStatusPayload(_ status: OSStatus) -> String {
  let message = SecCopyErrorMessageString(status, nil) as String? ?? "Authorization status \(status)"
  return smErrorPayload(message: message, domain: NSOSStatusErrorDomain, code: Int(status))
}

func smAuthorizationHolder(
  _ rawAuthorization: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> AuthorizationHolder? {
  guard let rawAuthorization else {
    smSetError(errorOut, "missing Authorization handle")
    return nil
  }
  guard let holder: AuthorizationHolder = smBorrow(rawAuthorization) else {
    smSetError(errorOut, "invalid Authorization handle")
    return nil
  }
  return holder
}

func smRequiredAuthorizationRef(
  _ rawAuthorization: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> AuthorizationRef? {
  guard let holder = smAuthorizationHolder(rawAuthorization, errorOut) else {
    return nil
  }
  guard let authorization = holder.authorization else {
    smSetError(errorOut, "missing AuthorizationRef")
    return nil
  }
  return authorization
}

func smOptionalAuthorizationRef(
  _ rawAuthorization: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> AuthorizationRef? {
  guard rawAuthorization != nil else {
    return nil
  }
  return smRequiredAuthorizationRef(rawAuthorization, errorOut)
}

func smWithAuthorizationRights<T>(
  _ rights: [String],
  _ body: (UnsafePointer<AuthorizationRights>?) throws -> T
) throws -> T {
  if rights.isEmpty {
    return try body(nil)
  }

  let duplicated = try rights.map { right -> UnsafeMutablePointer<CChar> in
    guard let duplicated = strdup(right) else {
      throw NSError(
        domain: "ServiceManagementBridge.Authorization",
        code: 1,
        userInfo: [NSLocalizedDescriptionKey: "failed to copy authorization right"]
      )
    }
    return duplicated
  }
  defer {
    duplicated.forEach { free($0) }
  }

  var items = duplicated.map {
    AuthorizationItem(name: $0, valueLength: 0, value: nil, flags: 0)
  }
  return try items.withUnsafeMutableBufferPointer { buffer in
    guard let baseAddress = buffer.baseAddress else {
      return try body(nil)
    }
    var rightsSet = AuthorizationRights(
      count: UInt32(buffer.count),
      items: baseAddress
    )
    return try withUnsafePointer(to: &rightsSet) { rightsPointer in
      try body(rightsPointer)
    }
  }
}

func smWithRequiredAuthorizationRights<T>(
  _ rights: [String],
  _ body: (UnsafePointer<AuthorizationRights>) throws -> T
) throws -> T {
  try smWithAuthorizationRights(rights) { rightsPointer in
    guard let rightsPointer else {
      throw NSError(
        domain: "ServiceManagementBridge.Authorization",
        code: 2,
        userInfo: [NSLocalizedDescriptionKey: "authorization rights cannot be empty"]
      )
    }
    return try body(rightsPointer)
  }
}

@_cdecl("sm_authorization_create")
public func sm_authorization_create(
  _ flags: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  var authorization: AuthorizationRef?
  let status = AuthorizationCreate(nil, nil, smAuthorizationFlags(flags), &authorization)
  guard status == errAuthorizationSuccess, let authorization else {
    smSetError(errorOut, smAuthorizationStatusPayload(status))
    return nil
  }
  return smRetain(AuthorizationHolder(authorization))
}

@_cdecl("sm_authorization_create_with_rights")
public func sm_authorization_create_with_rights(
  _ rightsJSON: UnsafePointer<CChar>?,
  _ flags: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rights = smDecodeStringArray(rightsJSON, errorOut) else {
    return nil
  }

  do {
    var authorization: AuthorizationRef?
    let status = try smWithAuthorizationRights(rights) { rightsPointer in
      AuthorizationCreate(rightsPointer, nil, smAuthorizationFlags(flags), &authorization)
    }
    guard status == errAuthorizationSuccess, let authorization else {
      smSetError(errorOut, smAuthorizationStatusPayload(status))
      return nil
    }
    return smRetain(AuthorizationHolder(authorization))
  } catch {
    smSetError(errorOut, smNSErrorPayload(error))
    return nil
  }
}

@_cdecl("sm_authorization_copy_rights")
public func sm_authorization_copy_rights(
  _ rawAuthorization: UnsafeMutableRawPointer?,
  _ rightsJSON: UnsafePointer<CChar>?,
  _ flags: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let authorization = smRequiredAuthorizationRef(rawAuthorization, errorOut) else {
    return false
  }
  guard let rights = smDecodeStringArray(rightsJSON, errorOut) else {
    return false
  }

  if rights.isEmpty {
    return true
  }

  do {
    let status = try smWithRequiredAuthorizationRights(rights) { rightsPointer in
      AuthorizationCopyRights(
        authorization,
        rightsPointer,
        nil,
        smAuthorizationFlags(flags),
        nil
      )
    }
    if status == errAuthorizationSuccess {
      return true
    }
    smSetError(errorOut, smAuthorizationStatusPayload(status))
    return false
  } catch {
    smSetError(errorOut, smNSErrorPayload(error))
    return false
  }
}

@_cdecl("sm_authorization_external_form")
public func sm_authorization_external_form(
  _ rawAuthorization: UnsafeMutableRawPointer?,
  _ bytesOut: UnsafeMutableRawPointer?,
  _ capacity: Int,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let authorization = smRequiredAuthorizationRef(rawAuthorization, errorOut) else {
    return false
  }
  guard let bytesOut, capacity >= MemoryLayout<AuthorizationExternalForm>.size else {
    smSetError(errorOut, "Authorization external form buffer must hold \(MemoryLayout<AuthorizationExternalForm>.size) bytes")
    return false
  }

  var externalForm = AuthorizationExternalForm()
  let status = AuthorizationMakeExternalForm(authorization, &externalForm)
  guard status == errAuthorizationSuccess else {
    smSetError(errorOut, smAuthorizationStatusPayload(status))
    return false
  }

  withUnsafeBytes(of: externalForm) { bytes in
    UnsafeMutableRawBufferPointer(start: bytesOut, count: bytes.count).copyMemory(from: bytes)
  }
  return true
}

@_cdecl("sm_authorization_from_external_form")
public func sm_authorization_from_external_form(
  _ bytes: UnsafeRawPointer?,
  _ length: Int,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let bytes, length == MemoryLayout<AuthorizationExternalForm>.size else {
    smSetError(errorOut, "Authorization external form must be \(MemoryLayout<AuthorizationExternalForm>.size) bytes")
    return nil
  }

  var externalForm = AuthorizationExternalForm()
  withUnsafeMutableBytes(of: &externalForm) { buffer in
    buffer.copyMemory(from: UnsafeRawBufferPointer(start: bytes, count: length))
  }

  var authorization: AuthorizationRef?
  let status = AuthorizationCreateFromExternalForm(&externalForm, &authorization)
  guard status == errAuthorizationSuccess, let authorization else {
    smSetError(errorOut, smAuthorizationStatusPayload(status))
    return nil
  }

  return smRetain(AuthorizationHolder(authorization))
}

@_cdecl("sm_authorization_destroy_rights")
public func sm_authorization_destroy_rights(
  _ rawAuthorization: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let holder = smAuthorizationHolder(rawAuthorization, errorOut),
        let authorization = holder.authorization else {
    if rawAuthorization != nil {
      smSetError(errorOut, "missing AuthorizationRef")
    }
    return false
  }

  let status = AuthorizationFree(authorization, AuthorizationFlags(rawValue: UInt32(1 << 3)))
  if status == errAuthorizationSuccess {
    holder.authorization = nil
    return true
  }

  smSetError(errorOut, smAuthorizationStatusPayload(status))
  return false
}

@_cdecl("sm_authorization_release")
public func sm_authorization_release(_ rawAuthorization: UnsafeMutableRawPointer?) {
  guard let rawAuthorization else { return }
  smRelease(rawAuthorization)
}
