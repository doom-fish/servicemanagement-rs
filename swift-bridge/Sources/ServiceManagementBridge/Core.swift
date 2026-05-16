import Darwin
import Foundation

func smCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
  string.withCString { strdup($0) }
}

func smRetain(_ object: AnyObject) -> UnsafeMutableRawPointer {
  Unmanaged.passRetained(object).toOpaque()
}

func smBorrow<T: AnyObject>(_ pointer: UnsafeMutableRawPointer) -> T {
  Unmanaged<T>.fromOpaque(pointer).takeUnretainedValue()
}

func smRelease(_ pointer: UnsafeMutableRawPointer) {
  Unmanaged<AnyObject>.fromOpaque(pointer).release()
}

@_cdecl("sm_string_free")
public func sm_string_free(_ pointer: UnsafeMutablePointer<CChar>?) {
  free(pointer)
}
