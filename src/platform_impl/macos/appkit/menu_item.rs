use objc2::foundation::{NSObject, NSString};
use objc2::rc::{Id, Shared};
use objc2::runtime::Sel;
use objc2::{extern_class, extern_methods, msg_send_id, ClassType};

use super::{NSEventModifierFlags, NSMenu};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSMenuItem;

    unsafe impl ClassType for NSMenuItem {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSMenuItem {
        #[method_id(new)]
        pub fn new() -> Id<Self, Shared>;

        pub fn newWithTitle(
            title: &NSString,
            action: Sel,
            key_equivalent: &NSString,
        ) -> Id<Self, Shared> {
            unsafe {
                msg_send_id![
                    Self::alloc(),
                    initWithTitle: title,
                    action: action,
                    keyEquivalent: key_equivalent,
                ]
            }
        }

        #[method_id(separatorItem)]
        pub fn separatorItem() -> Id<Self, Shared>;

        #[method(setKeyEquivalentModifierMask:)]
        pub fn setKeyEquivalentModifierMask(&self, mask: NSEventModifierFlags);

        #[method(setSubmenu:)]
        pub fn setSubmenu(&self, submenu: &NSMenu);
    }
);
