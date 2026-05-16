import CoreFoundation
import Foundation
import ServiceManagement

func smLaunchdDomain(
  _ rawDomain: Int32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> CFString? {
  switch rawDomain {
  case 0:
    return kSMDomainSystemLaunchd
  case 1:
    return kSMDomainUserLaunchd
  default:
    smSetError(errorOut, "unknown launchd domain \(rawDomain)")
    return nil
  }
}

func smLegacyJobPayload(
  _ dictionary: NSDictionary,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> [String: Any]? {
  do {
    let plistData = try PropertyListSerialization.data(
      fromPropertyList: dictionary,
      format: .xml,
      options: 0
    )
    return [
      "label": dictionary["Label"] as? String ?? NSNull(),
      "plist_xml": String(decoding: plistData, as: UTF8.self),
      "description": dictionary.description,
    ]
  } catch {
    smSetError(errorOut, smNSErrorMessage(error))
    return nil
  }
}

func smLegacyCFErrorMessage(_ error: Unmanaged<CFError>?) -> String? {
  guard let error else {
    return nil
  }
  let retained = error.takeRetainedValue()
  return (retained as Error as NSError).localizedDescription
}

@_cdecl("sm_legacy_copy_job_dictionary")
public func sm_legacy_copy_job_dictionary(
  _ rawDomain: Int32,
  _ jobLabel: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let domain = smLaunchdDomain(rawDomain, errorOut) else {
    return nil
  }
  guard let jobLabel else {
    smSetError(errorOut, "missing launchd job label")
    return nil
  }
  guard let dictionary = SMJobCopyDictionary(
    domain,
    String(cString: jobLabel) as CFString
  )?.takeRetainedValue() else {
    return nil
  }
  guard let payload = smLegacyJobPayload(dictionary as NSDictionary, errorOut) else {
    return nil
  }
  return smJSONString(payload, errorOut)
}

@_cdecl("sm_legacy_copy_all_job_dictionaries")
public func sm_legacy_copy_all_job_dictionaries(
  _ rawDomain: Int32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let domain = smLaunchdDomain(rawDomain, errorOut) else {
    return nil
  }
  guard let dictionaries = SMCopyAllJobDictionaries(domain)?.takeRetainedValue() else {
    smSetError(errorOut, "SMCopyAllJobDictionaries returned null")
    return nil
  }

  var payload: [[String: Any]] = []
  for case let dictionary as NSDictionary in dictionaries as NSArray {
    guard let jobPayload = smLegacyJobPayload(dictionary, errorOut) else {
      return nil
    }
    payload.append(jobPayload)
  }
  return smJSONString(payload, errorOut)
}

@_cdecl("sm_legacy_job_submit_plist")
public func sm_legacy_job_submit_plist(
  _ rawDomain: Int32,
  _ plistXML: UnsafePointer<CChar>?,
  _ rawAuthorization: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let domain = smLaunchdDomain(rawDomain, errorOut) else {
    return false
  }
  guard let plistXML else {
    smSetError(errorOut, "missing launchd property list XML")
    return false
  }
  let authorization = smOptionalAuthorizationRef(rawAuthorization, errorOut)
  if rawAuthorization != nil && authorization == nil {
    return false
  }

  let plistString = String(cString: plistXML)
  guard let plistData = plistString.data(using: .utf8) else {
    smSetError(errorOut, "launchd property list XML must be UTF-8")
    return false
  }

  do {
    let plist = try PropertyListSerialization.propertyList(
      from: plistData,
      options: [],
      format: nil
    )
    guard let dictionary = plist as? NSDictionary else {
      smSetError(errorOut, "launchd property list XML must decode to a dictionary")
      return false
    }

    var cfError: Unmanaged<CFError>?
    let ok = SMJobSubmit(domain, dictionary as CFDictionary, authorization, &cfError)
    if !ok {
      smSetError(
        errorOut,
        smLegacyCFErrorMessage(cfError) ?? "SMJobSubmit returned false"
      )
    }
    return ok
  } catch {
    smSetError(errorOut, smNSErrorMessage(error))
    return false
  }
}

@_cdecl("sm_legacy_job_remove")
public func sm_legacy_job_remove(
  _ rawDomain: Int32,
  _ jobLabel: UnsafePointer<CChar>?,
  _ rawAuthorization: UnsafeMutableRawPointer?,
  _ wait: Bool,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let domain = smLaunchdDomain(rawDomain, errorOut) else {
    return false
  }
  guard let jobLabel else {
    smSetError(errorOut, "missing launchd job label")
    return false
  }
  let authorization = smOptionalAuthorizationRef(rawAuthorization, errorOut)
  if rawAuthorization != nil && authorization == nil {
    return false
  }

  var cfError: Unmanaged<CFError>?
  let ok = SMJobRemove(
    domain,
    String(cString: jobLabel) as CFString,
    authorization,
    wait,
    &cfError
  )
  if !ok {
    smSetError(
      errorOut,
      smLegacyCFErrorMessage(cfError) ?? "SMJobRemove returned false"
    )
  }
  return ok
}

@_cdecl("sm_legacy_job_bless")
public func sm_legacy_job_bless(
  _ rawDomain: Int32,
  _ executableLabel: UnsafePointer<CChar>?,
  _ rawAuthorization: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let domain = smLaunchdDomain(rawDomain, errorOut) else {
    return false
  }
  guard let executableLabel else {
    smSetError(errorOut, "missing privileged helper label")
    return false
  }
  let authorization = smOptionalAuthorizationRef(rawAuthorization, errorOut)
  if rawAuthorization != nil && authorization == nil {
    return false
  }

  var cfError: Unmanaged<CFError>?
  let ok = SMJobBless(
    domain,
    String(cString: executableLabel) as CFString,
    authorization,
    &cfError
  )
  if !ok {
    smSetError(
      errorOut,
      smLegacyCFErrorMessage(cfError) ?? "SMJobBless returned false"
    )
  }
  return ok
}
