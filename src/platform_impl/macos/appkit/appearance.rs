use objc2::foundation::{NSArray, NSObject, NSString};
use objc2::rc::{Id, Shared};
use objc2::{extern_class, extern_methods, ClassType};

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSAppearance;

    unsafe impl ClassType for NSAppearance {
        type Super = NSObject;
    }
);

type NSAppearanceName = NSString;

extern_methods!(
    unsafe impl NSAppearance {
        #[method_id(appearanceNamed:)]
        pub fn appearanceNamed(name: &NSAppearanceName) -> Id<Self, Shared>;

        #[method_id(bestMatchFromAppearancesWithNames:)]
        pub fn bestMatchFromAppearancesWithNames(
            &self,
            appearances: &NSArray<NSAppearanceName>,
        ) -> Id<NSAppearanceName, Shared>;
    }
);
