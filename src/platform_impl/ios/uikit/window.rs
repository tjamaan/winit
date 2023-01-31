use objc2::rc::{Id, Shared};
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods, ClassType};

use super::{UIResponder, UIScreen, UIView};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct UIWindow;

    unsafe impl ClassType for UIWindow {
        #[inherits(UIResponder, NSObject)]
        type Super = UIView;
    }
);

extern_methods!(
    unsafe impl UIWindow {
        #[method_id(screen)]
        pub fn screen(&self) -> Id<UIScreen, Shared>;

        #[method(setScreen:)]
        pub fn setScreen(&self, screen: &UIScreen);

        #[method(setHidden:)]
        pub fn setHidden(&self, flag: bool);

        #[method(makeKeyAndVisible)]
        pub fn makeKeyAndVisible(&self);

        #[method(isKeyWindow)]
        pub fn isKeyWindow(&self) -> bool;
    }
);
