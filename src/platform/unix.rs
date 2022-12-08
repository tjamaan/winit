use crate::{
    event::KeyEvent,
    keyboard::{Key, KeyCode},
    platform::{modifier_supplement::KeyEventExtModifierSupplement, scancode::KeyCodeExtScancode},
    platform_impl::common::keymap,
};

impl KeyEventExtModifierSupplement for KeyEvent {
    #[inline]
    fn text_with_all_modifiers(&self) -> Option<&str> {
        self.platform_specific.text_with_all_modifiers
    }

    #[inline]
    fn key_without_modifiers(&self) -> Key<'static> {
        self.platform_specific.key_without_modifiers
    }
}

impl KeyCodeExtScancode for KeyCode {
    fn from_scancode(scancode: u32) -> KeyCode {
        keymap::raw_keycode_to_keycode(scancode)
    }

    fn to_scancode(self) -> Option<u32> {
        keymap::keycode_to_raw(self)
    }
}
