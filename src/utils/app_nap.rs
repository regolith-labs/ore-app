use cocoa::base::nil;
use cocoa::foundation::NSString;
use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};

/// A struct that disables App Nap while it is alive.
pub struct AppNapDisabler {
    token: *mut Object,
}

impl AppNapDisabler {
    /// Call this to disable App Nap. Keep the instance alive as long
    /// as you need to remain active.
    pub fn new(reason: &str) -> Self {
        // Apple’s documented bitmask to disable system sleep, user-initiated activity, etc.
        const NS_ACTIVITY_IDLE_SYSTEM_SLEEP_DISABLED: u64 = 1 << 20;
        const NS_ACTIVITY_USER_INITIATED: u64 = 0x00FFFFFF | NS_ACTIVITY_IDLE_SYSTEM_SLEEP_DISABLED;

        unsafe {
            // Get the shared [NSProcessInfo processInfo] object
            let cls = class!(NSProcessInfo);
            let process_info: *mut Object = msg_send![cls, processInfo];

            // Create an NSString for the reason
            let reason_nsstring = NSString::alloc(nil).init_str(reason);

            // Begin the activity, retrieving a “token”
            let token: *mut Object = msg_send![
                process_info,
                beginActivityWithOptions: NS_ACTIVITY_USER_INITIATED
                reason: reason_nsstring
            ];

            Self { token }
        }
    }
}

/// When the struct is dropped, we end the “no-sleep” activity,
/// allowing the system to App-Nap your app again.
impl Drop for AppNapDisabler {
    fn drop(&mut self) {
        unsafe {
            let cls = class!(NSProcessInfo);
            let process_info: *mut Object = msg_send![cls, processInfo];
            let _: () = msg_send![process_info, endActivity: self.token];
        }
    }
}
