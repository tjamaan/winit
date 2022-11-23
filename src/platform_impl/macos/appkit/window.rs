use objc2::encode::{Encode, Encoding};
use objc2::foundation::{
    CGFloat, NSArray, NSInteger, NSObject, NSPoint, NSRect, NSSize, NSString, NSUInteger,
};
use objc2::rc::{Id, Shared};
use objc2::runtime::Object;
use objc2::{extern_class, extern_methods, ClassType};

use super::{NSButton, NSColor, NSEvent, NSPasteboardType, NSResponder, NSScreen, NSView};

extern_class!(
    /// Main-Thread-Only!
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub(crate) struct NSWindow;

    unsafe impl ClassType for NSWindow {
        #[inherits(NSObject)]
        type Super = NSResponder;
    }
);

// Documented as "Main Thread Only", but:
// > Thread safe in that you can create and manage them on a secondary thread.
// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CocoaFundamentals/AddingBehaviortoaCocoaProgram/AddingBehaviorCocoa.html#//apple_ref/doc/uid/TP40002974-CH5-SW47>
// <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/ThreadSafetySummary/ThreadSafetySummary.html#//apple_ref/doc/uid/10000057i-CH12-123364>
//
// So could in theory be `Send`, and perhaps also `Sync` - but we would like
// interior mutability on windows, since that's just much easier, and in that
// case, they can't be!

extern_methods!(
    unsafe impl NSWindow {
        #[method(frame)]
        pub fn frame(&self) -> NSRect;

        #[method(backingScaleFactor)]
        pub fn backingScaleFactor(&self) -> CGFloat;

        #[method_id(contentView)]
        pub fn contentView(&self) -> Id<NSView, Shared>;

        #[method(setContentView:)]
        pub fn setContentView(&self, view: &NSView);

        #[method(setInitialFirstResponder:)]
        pub fn setInitialFirstResponder(&self, view: &NSView);

        #[method(makeFirstResponder:)]
        #[must_use]
        pub fn makeFirstResponder(&self, responder: Option<&NSResponder>) -> bool;

        #[method(contentRectForFrameRect:)]
        pub fn contentRectForFrameRect(&self, windowFrame: NSRect) -> NSRect;

        #[method_id(screen)]
        pub fn screen(&self) -> Option<Id<NSScreen, Shared>>;

        #[method(setContentSize:)]
        pub fn setContentSize(&self, contentSize: NSSize);

        #[method(setFrameTopLeftPoint:)]
        pub fn setFrameTopLeftPoint(&self, point: NSPoint);

        #[method(setMinSize:)]
        pub fn setMinSize(&self, minSize: NSSize);

        #[method(setMaxSize:)]
        pub fn setMaxSize(&self, maxSize: NSSize);

        #[method(setResizeIncrements:)]
        pub fn setResizeIncrements(&self, increments: NSSize);

        #[method(contentResizeIncrements)]
        pub fn contentResizeIncrements(&self) -> NSSize;

        #[method(setContentResizeIncrements:)]
        pub fn setContentResizeIncrements(&self, increments: NSSize);

        #[method(setFrame:display:)]
        pub fn setFrame_display(&self, frameRect: NSRect, flag: bool);

        #[method(setMovable:)]
        pub fn setMovable(&self, movable: bool);

        #[method(setSharingType:)]
        pub fn setSharingType(&self, sharingType: NSWindowSharingType);

        #[method(setOpaque:)]
        pub fn setOpaque(&self, opaque: bool);

        #[method(hasShadow)]
        pub fn hasShadow(&self) -> bool;

        #[method(setHasShadow:)]
        pub fn setHasShadow(&self, has_shadow: bool);

        #[method(setIgnoresMouseEvents:)]
        pub fn setIgnoresMouseEvents(&self, ignores: bool);

        #[method(setBackgroundColor:)]
        pub fn setBackgroundColor(&self, color: &NSColor);

        #[method(styleMask)]
        pub fn styleMask(&self) -> NSWindowStyleMask;

        #[method(setStyleMask:)]
        pub fn setStyleMask(&self, mask: NSWindowStyleMask);

        #[method(registerForDraggedTypes:)]
        pub fn registerForDraggedTypes(&self, types: &NSArray<NSPasteboardType>);

        #[method(makeKeyAndOrderFront:)]
        pub fn makeKeyAndOrderFront(&self, sender: Option<&Object>);

        #[method(miniaturize:)]
        pub fn miniaturize(&self, sender: Option<&Object>);

        #[method(sender:)]
        pub fn deminiaturize(&self, sender: Option<&Object>);

        #[method(toggleFullScreen:)]
        pub fn toggleFullScreen(&self, sender: Option<&Object>);

        #[method(orderOut:)]
        pub fn orderOut(&self, sender: Option<&Object>);

        #[method(zoom:)]
        pub fn zoom(&self, sender: Option<&Object>);

        #[method(selectNextKeyView:)]
        pub fn selectNextKeyView(&self, sender: Option<&Object>);

        #[method(selectPreviousKeyView:)]
        pub fn selectPreviousKeyView(&self, sender: Option<&Object>);

        #[method_id(firstResponder)]
        pub fn firstResponder(&self) -> Option<Id<NSResponder, Shared>>;

        #[method_id(standardWindowButton:)]
        pub fn standardWindowButton(&self, kind: NSWindowButton) -> Option<Id<NSButton, Shared>>;

        #[method(setTitle:)]
        pub fn setTitle(&self, title: &NSString);

        #[method_id(title)]
        pub fn title_(&self) -> Id<NSString, Shared>;

        #[method(setReleasedWhenClosed:)]
        pub fn setReleasedWhenClosed(&self, val: bool);

        #[method(setAcceptsMouseMovedEvents:)]
        pub fn setAcceptsMouseMovedEvents(&self, val: bool);

        #[method(setTitlebarAppearsTransparent:)]
        pub fn setTitlebarAppearsTransparent(&self, val: bool);

        #[method(setTitleVisibility:)]
        pub fn setTitleVisibility(&self, visibility: NSWindowTitleVisibility);

        #[method(setMovableByWindowBackground:)]
        pub fn setMovableByWindowBackground(&self, val: bool);

        #[method(setLevel:)]
        pub fn setLevel(&self, level: NSWindowLevel);

        #[method(setDocumentEdited:)]
        pub fn setDocumentEdited(&self, val: bool);

        #[method(occlusionState)]
        pub fn occlusionState(&self) -> NSWindowOcclusionState;

        #[method(center)]
        pub fn center(&self);

        #[method(isResizable)]
        pub fn isResizable(&self) -> bool;

        #[method(isMiniaturizable)]
        pub fn isMiniaturizable(&self) -> bool;

        #[method(hasCloseBox)]
        pub fn hasCloseBox(&self) -> bool;

        #[method(isMiniaturized)]
        pub fn isMiniaturized(&self) -> bool;

        #[method(isVisible)]
        pub fn isVisible(&self) -> bool;

        #[method(isZoomed)]
        pub fn isZoomed(&self) -> bool;

        #[method(isDocumentEdited)]
        pub fn isDocumentEdited(&self) -> bool;

        #[method(close)]
        pub fn close(&self);

        #[method(performWindowDragWithEvent:)]
        // TODO: Can this actually accept NULL?
        pub fn performWindowDragWithEvent(&self, event: Option<&NSEvent>);

        #[method(invalidateCursorRectsForView:)]
        pub fn invalidateCursorRectsForView(&self, view: &NSView);

        #[method(setDelegate:)]
        pub fn setDelegate(&self, delegate: Option<&NSObject>);

        #[method(sendEvent:)]
        pub unsafe fn sendEvent(&self, event: &NSEvent);
    }
);

#[allow(dead_code)]
#[repr(isize)] // NSInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NSWindowTitleVisibility {
    #[doc(alias = "NSWindowTitleVisible")]
    Visible = 0,
    #[doc(alias = "NSWindowTitleHidden")]
    Hidden = 1,
}

unsafe impl Encode for NSWindowTitleVisibility {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

#[allow(dead_code)]
#[repr(usize)] // NSUInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NSWindowButton {
    #[doc(alias = "NSWindowCloseButton")]
    Close = 0,
    #[doc(alias = "NSWindowMiniaturizeButton")]
    Miniaturize = 1,
    #[doc(alias = "NSWindowZoomButton")]
    Zoom = 2,
    #[doc(alias = "NSWindowToolbarButton")]
    Toolbar = 3,
    #[doc(alias = "NSWindowDocumentIconButton")]
    DocumentIcon = 4,
    #[doc(alias = "NSWindowDocumentVersionsButton")]
    DocumentVersions = 6,
    #[doc(alias = "NSWindowFullScreenButton")]
    #[deprecated = "Deprecated since macOS 10.12"]
    FullScreen = 7,
}

unsafe impl Encode for NSWindowButton {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

// CGWindowLevel.h
//
// Note: There are two different things at play in this header:
// `CGWindowLevel` and `CGWindowLevelKey`.
//
// It seems like there was a push towards using "key" values instead of the
// raw window level values, and then you were supposed to use
// `CGWindowLevelForKey` to get the actual level.
//
// But the values that `NSWindowLevel` has are compiled in, and as such has
// to remain ABI compatible, so they're safe for us to hardcode as well.
#[allow(dead_code)]
mod window_level {
    const kCGNumReservedWindowLevels: i32 = 16;
    const kCGNumReservedBaseWindowLevels: i32 = 5;

    pub const kCGBaseWindowLevel: i32 = i32::MIN;
    pub const kCGMinimumWindowLevel: i32 = kCGBaseWindowLevel + kCGNumReservedBaseWindowLevels;
    pub const kCGMaximumWindowLevel: i32 = i32::MAX - kCGNumReservedWindowLevels;

    pub const kCGDesktopWindowLevel: i32 = kCGMinimumWindowLevel + 20;
    pub const kCGDesktopIconWindowLevel: i32 = kCGDesktopWindowLevel + 20;
    pub const kCGBackstopMenuLevel: i32 = -20;
    pub const kCGNormalWindowLevel: i32 = 0;
    pub const kCGFloatingWindowLevel: i32 = 3;
    pub const kCGTornOffMenuWindowLevel: i32 = 3;
    pub const kCGModalPanelWindowLevel: i32 = 8;
    pub const kCGUtilityWindowLevel: i32 = 19;
    pub const kCGDockWindowLevel: i32 = 20;
    pub const kCGMainMenuWindowLevel: i32 = 24;
    pub const kCGStatusWindowLevel: i32 = 25;
    pub const kCGPopUpMenuWindowLevel: i32 = 101;
    pub const kCGOverlayWindowLevel: i32 = 102;
    pub const kCGHelpWindowLevel: i32 = 200;
    pub const kCGDraggingWindowLevel: i32 = 500;
    pub const kCGScreenSaverWindowLevel: i32 = 1000;
    pub const kCGAssistiveTechHighWindowLevel: i32 = 1500;
    pub const kCGCursorWindowLevel: i32 = kCGMaximumWindowLevel - 1;
}
use window_level::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NSWindowLevel(pub NSInteger);

#[allow(dead_code)]
impl NSWindowLevel {
    #[doc(alias = "BelowNormalWindowLevel")]
    pub const BELOW_NORMAL: Self = Self((kCGNormalWindowLevel - 1) as _);
    #[doc(alias = "NSNormalWindowLevel")]
    pub const Normal: Self = Self(kCGNormalWindowLevel as _);
    #[doc(alias = "NSFloatingWindowLevel")]
    pub const Floating: Self = Self(kCGFloatingWindowLevel as _);
    #[doc(alias = "NSTornOffMenuWindowLevel")]
    pub const TornOffMenu: Self = Self(kCGTornOffMenuWindowLevel as _);
    #[doc(alias = "NSModalPanelWindowLevel")]
    pub const ModalPanel: Self = Self(kCGModalPanelWindowLevel as _);
    #[doc(alias = "NSMainMenuWindowLevel")]
    pub const MainMenu: Self = Self(kCGMainMenuWindowLevel as _);
    #[doc(alias = "NSStatusWindowLevel")]
    pub const Status: Self = Self(kCGStatusWindowLevel as _);
    #[doc(alias = "NSPopUpMenuWindowLevel")]
    pub const PopUpMenu: Self = Self(kCGPopUpMenuWindowLevel as _);
    #[doc(alias = "NSScreenSaverWindowLevel")]
    pub const ScreenSaver: Self = Self(kCGScreenSaverWindowLevel as _);
}

unsafe impl Encode for NSWindowLevel {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

bitflags! {
    pub struct NSWindowOcclusionState: NSUInteger {
        const NSWindowOcclusionStateVisible = 1 << 1;
    }
}

unsafe impl Encode for NSWindowOcclusionState {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

bitflags! {
    pub struct NSWindowStyleMask: NSUInteger {
        const NSBorderlessWindowMask      = 0;
        const NSTitledWindowMask          = 1 << 0;
        const NSClosableWindowMask        = 1 << 1;
        const NSMiniaturizableWindowMask  = 1 << 2;
        const NSResizableWindowMask       = 1 << 3;
        const NSTexturedBackgroundWindowMask  = 1 << 8;
        const NSUnifiedTitleAndToolbarWindowMask  = 1 << 12;
        const NSFullScreenWindowMask      = 1 << 14;
        const NSFullSizeContentViewWindowMask = 1 << 15;
    }
}

unsafe impl Encode for NSWindowStyleMask {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[allow(dead_code)]
#[repr(usize)] // NSUInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NSBackingStoreType {
    NSBackingStoreRetained = 0,
    NSBackingStoreNonretained = 1,
    NSBackingStoreBuffered = 2,
}

unsafe impl Encode for NSBackingStoreType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

#[allow(dead_code)]
#[repr(usize)] // NSUInteger
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NSWindowSharingType {
    NSWindowSharingNone = 0,
    NSWindowSharingReadOnly = 1,
    NSWindowSharingReadWrite = 2,
}

unsafe impl Encode for NSWindowSharingType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}
