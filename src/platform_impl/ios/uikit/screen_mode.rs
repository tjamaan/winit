use icrate::Foundation::{CGSize, NSObject};
use objc2::{extern_class, extern_methods, ClassType};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct UIScreenMode;

    unsafe impl ClassType for UIScreenMode {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl UIScreenMode {
        #[method(size)]
        pub fn size(&self) -> CGSize;
    }
);
