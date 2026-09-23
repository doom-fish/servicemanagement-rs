import Darwin
import Foundation

func smCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
  string.withCString { strdup($0) }
}

func smRetain(_ object: AnyObject) -> UnsafeMutableRawPointer {
  Unmanaged.passRetained(object).toOpaque()
}

func smBorrow<T: AnyObject>(_ pointer: UnsafeMutableRawPointer) -> T? {
  Unmanaged<AnyObject>.fromOpaque(pointer).takeUnretainedValue() as? T
}

func smRelease(_ pointer: UnsafeMutableRawPointer) {
  Unmanaged<AnyObject>.fromOpaque(pointer).release()
}

func smSetError(
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
  _ message: String
) {
  errorOut?.pointee = smCString(message)
}

func smJSONString(
  _ object: Any,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>? = nil
) -> UnsafeMutablePointer<CChar>? {
  do {
    let data = try JSONSerialization.data(withJSONObject: object, options: [.sortedKeys])
    guard let string = String(data: data, encoding: .utf8) else {
      smSetError(errorOut, "bridge could not encode UTF-8 JSON")
      return nil
    }
    return smCString(string)
  } catch {
    smSetError(errorOut, (error as NSError).localizedDescription)
    return nil
  }
}

func smDecodeStringArray(
  _ jsonCString: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> [String]? {
  guard let jsonCString else {
    return []
  }

  let payload = String(cString: jsonCString)
  guard let data = payload.data(using: .utf8) else {
    smSetError(errorOut, "bridge received non-UTF-8 JSON")
    return nil
  }

  do {
    let decoded = try JSONSerialization.jsonObject(with: data, options: [])
    guard let strings = decoded as? [String] else {
      smSetError(errorOut, "bridge expected a JSON string array")
      return nil
    }
    return strings
  } catch {
    smSetError(errorOut, (error as NSError).localizedDescription)
    return nil
  }
}

func smErrorPayload(message: String, domain: String, code: Int) -> String {
  let payload: [String: Any] = ["message": message, "domain": domain, "code": code]
  guard let data = try? JSONSerialization.data(withJSONObject: payload, options: [.sortedKeys]),
        let string = String(data: data, encoding: .utf8)
  else {
    return message
  }
  return string
}

func smNSErrorPayload(_ error: Error) -> String {
  let nsError = error as NSError
  return smErrorPayload(message: nsError.localizedDescription, domain: nsError.domain, code: nsError.code)
}

@_cdecl("sm_string_free")
public func sm_string_free(_ pointer: UnsafeMutablePointer<CChar>?) {
  free(pointer)
}
