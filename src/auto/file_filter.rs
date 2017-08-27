// This file was generated by gir (9bd51ed) from gir-files (db49619)
// DO NOT EDIT

use FileFilterFlags;
use ffi;
#[cfg(feature = "v3_22")]
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FileFilter(Object<ffi::GtkFileFilter>);

    match fn {
        get_type => || ffi::gtk_file_filter_get_type(),
    }
}

impl FileFilter {
    pub fn new() -> FileFilter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gtk_file_filter_new())
        }
    }

    #[cfg(feature = "v3_22")]
    pub fn new_from_gvariant(variant: &glib::Variant) -> FileFilter {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_file_filter_new_from_gvariant(variant.to_glib_none().0))
        }
    }
}

impl Default for FileFilter {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FileFilterExt {
    //fn add_custom<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, needed: FileFilterFlags, func: /*Unknown conversion*//*Unimplemented*/FileFilterFunc, data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn add_mime_type(&self, mime_type: &str);

    fn add_pattern(&self, pattern: &str);

    fn add_pixbuf_formats(&self);

    //fn filter(&self, filter_info: /*Ignored*/&FileFilterInfo) -> bool;

    fn get_name(&self) -> Option<String>;

    fn get_needed(&self) -> FileFilterFlags;

    fn set_name<'a, P: Into<Option<&'a str>>>(&self, name: P);

    #[cfg(feature = "v3_22")]
    fn to_gvariant(&self) -> Option<glib::Variant>;
}

impl<O: IsA<FileFilter>> FileFilterExt for O {
    //fn add_custom<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, needed: FileFilterFlags, func: /*Unknown conversion*//*Unimplemented*/FileFilterFunc, data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_file_filter_add_custom() }
    //}

    fn add_mime_type(&self, mime_type: &str) {
        unsafe {
            ffi::gtk_file_filter_add_mime_type(self.to_glib_none().0, mime_type.to_glib_none().0);
        }
    }

    fn add_pattern(&self, pattern: &str) {
        unsafe {
            ffi::gtk_file_filter_add_pattern(self.to_glib_none().0, pattern.to_glib_none().0);
        }
    }

    fn add_pixbuf_formats(&self) {
        unsafe {
            ffi::gtk_file_filter_add_pixbuf_formats(self.to_glib_none().0);
        }
    }

    //fn filter(&self, filter_info: /*Ignored*/&FileFilterInfo) -> bool {
    //    unsafe { TODO: call ffi::gtk_file_filter_filter() }
    //}

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_file_filter_get_name(self.to_glib_none().0))
        }
    }

    fn get_needed(&self) -> FileFilterFlags {
        unsafe {
            from_glib(ffi::gtk_file_filter_get_needed(self.to_glib_none().0))
        }
    }

    fn set_name<'a, P: Into<Option<&'a str>>>(&self, name: P) {
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            ffi::gtk_file_filter_set_name(self.to_glib_none().0, name.0);
        }
    }

    #[cfg(feature = "v3_22")]
    fn to_gvariant(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::gtk_file_filter_to_gvariant(self.to_glib_none().0))
        }
    }
}
