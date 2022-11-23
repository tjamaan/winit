use objc2::foundation::NSObject;
use objc2::rc::{Id, Shared};
use objc2::{extern_class, extern_methods, ClassType};

use super::NSMenuItem;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSMenu;

    unsafe impl ClassType for NSMenu {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSMenu {
        #[method_id(new)]
        pub fn new() -> Id<Self, Shared>;

        #[method(addItem:)]
        pub fn addItem(&self, item: &NSMenuItem);
    }
);
