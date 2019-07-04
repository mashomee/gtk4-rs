// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Display;
use KeymapKey;
use ModifierIntent;
use ModifierType;
use gdk_sys;
use glib::object::ObjectType as ObjectType_;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use pango;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Keymap(Object<gdk_sys::GdkKeymap, KeymapClass>);

    match fn {
        get_type => || gdk_sys::gdk_keymap_get_type(),
    }
}

impl Keymap {
    pub fn get_caps_lock_state(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_keymap_get_caps_lock_state(self.to_glib_none().0))
        }
    }

    pub fn get_direction(&self) -> pango::Direction {
        unsafe {
            from_glib(gdk_sys::gdk_keymap_get_direction(self.to_glib_none().0))
        }
    }

    pub fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(gdk_sys::gdk_keymap_get_display(self.to_glib_none().0))
        }
    }

    pub fn get_entries_for_keyval(&self, keyval: u32) -> Option<Vec<KeymapKey>> {
        unsafe {
            let mut keys = ptr::null_mut();
            let mut n_keys = mem::uninitialized();
            let ret = from_glib(gdk_sys::gdk_keymap_get_entries_for_keyval(self.to_glib_none().0, keyval, &mut keys, &mut n_keys));
            if ret { Some(FromGlibContainer::from_glib_full_num(keys, n_keys as usize)) } else { None }
        }
    }

    pub fn get_modifier_mask(&self, intent: ModifierIntent) -> ModifierType {
        unsafe {
            from_glib(gdk_sys::gdk_keymap_get_modifier_mask(self.to_glib_none().0, intent.to_glib()))
        }
    }

    pub fn get_modifier_state(&self) -> u32 {
        unsafe {
            gdk_sys::gdk_keymap_get_modifier_state(self.to_glib_none().0)
        }
    }

    pub fn get_num_lock_state(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_keymap_get_num_lock_state(self.to_glib_none().0))
        }
    }

    pub fn get_scroll_lock_state(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_keymap_get_scroll_lock_state(self.to_glib_none().0))
        }
    }

    pub fn have_bidi_layouts(&self) -> bool {
        unsafe {
            from_glib(gdk_sys::gdk_keymap_have_bidi_layouts(self.to_glib_none().0))
        }
    }

    pub fn lookup_key(&self, key: &KeymapKey) -> u32 {
        unsafe {
            gdk_sys::gdk_keymap_lookup_key(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    pub fn translate_keyboard_state(&self, hardware_keycode: u32, state: ModifierType, group: i32) -> Option<(u32, i32, i32, ModifierType)> {
        unsafe {
            let mut keyval = mem::uninitialized();
            let mut effective_group = mem::uninitialized();
            let mut level = mem::uninitialized();
            let mut consumed_modifiers = mem::uninitialized();
            let ret = from_glib(gdk_sys::gdk_keymap_translate_keyboard_state(self.to_glib_none().0, hardware_keycode, state.to_glib(), group, &mut keyval, &mut effective_group, &mut level, &mut consumed_modifiers));
            if ret { Some((keyval, effective_group, level, from_glib(consumed_modifiers))) } else { None }
        }
    }

    pub fn connect_direction_changed<F: Fn(&Keymap) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn direction_changed_trampoline<F: Fn(&Keymap) + 'static>(this: *mut gdk_sys::GdkKeymap, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"direction-changed\0".as_ptr() as *const _,
                Some(transmute(direction_changed_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_keys_changed<F: Fn(&Keymap) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn keys_changed_trampoline<F: Fn(&Keymap) + 'static>(this: *mut gdk_sys::GdkKeymap, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"keys-changed\0".as_ptr() as *const _,
                Some(transmute(keys_changed_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_state_changed<F: Fn(&Keymap) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn state_changed_trampoline<F: Fn(&Keymap) + 'static>(this: *mut gdk_sys::GdkKeymap, f: glib_sys::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"state-changed\0".as_ptr() as *const _,
                Some(transmute(state_changed_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Keymap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Keymap")
    }
}
