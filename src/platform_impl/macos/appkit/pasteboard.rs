use objc2::foundation::{NSObject, NSString};
use objc2::rc::{Id, Shared};
use objc2::{extern_class, extern_methods, ClassType};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSPasteboard;

    unsafe impl ClassType for NSPasteboard {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSPasteboard {
        #[method_id(propertyListForType:)]
        pub fn propertyListForType(&self, type_: &NSPasteboardType) -> Id<NSObject, Shared>;
    }
);

pub type NSPasteboardType = NSString;

extern "C" {
    pub static NSFilenamesPboardType: &'static NSPasteboardType;
}
