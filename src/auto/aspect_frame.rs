// This file was generated by gir (9bd51ed) from gir-files (db49619)
// DO NOT EDIT

use Bin;
use Container;
use Frame;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct AspectFrame(Object<ffi::GtkAspectFrame>): Frame, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_aspect_frame_get_type(),
    }
}

impl AspectFrame {
    pub fn new<'a, P: Into<Option<&'a str>>>(label: P, xalign: f32, yalign: f32, ratio: f32, obey_child: bool) -> AspectFrame {
        assert_initialized_main_thread!();
        let label = label.into();
        let label = label.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_aspect_frame_new(label.0, xalign, yalign, ratio, obey_child.to_glib())).downcast_unchecked()
        }
    }
}

pub trait AspectFrameExt {
    fn set(&self, xalign: f32, yalign: f32, ratio: f32, obey_child: bool);

    fn get_property_obey_child(&self) -> bool;

    fn set_property_obey_child(&self, obey_child: bool);

    fn get_property_ratio(&self) -> f32;

    fn set_property_ratio(&self, ratio: f32);

    fn get_property_xalign(&self) -> f32;

    fn set_property_xalign(&self, xalign: f32);

    fn get_property_yalign(&self) -> f32;

    fn set_property_yalign(&self, yalign: f32);

    fn connect_property_obey_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_ratio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<AspectFrame> + IsA<glib::object::Object>> AspectFrameExt for O {
    fn set(&self, xalign: f32, yalign: f32, ratio: f32, obey_child: bool) {
        unsafe {
            ffi::gtk_aspect_frame_set(self.to_glib_none().0, xalign, yalign, ratio, obey_child.to_glib());
        }
    }

    fn get_property_obey_child(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "obey-child".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_obey_child(&self, obey_child: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "obey-child".to_glib_none().0, Value::from(&obey_child).to_glib_none().0);
        }
    }

    fn get_property_ratio(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "ratio".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_ratio(&self, ratio: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "ratio".to_glib_none().0, Value::from(&ratio).to_glib_none().0);
        }
    }

    fn get_property_xalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "xalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_xalign(&self, xalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "xalign".to_glib_none().0, Value::from(&xalign).to_glib_none().0);
        }
    }

    fn get_property_yalign(&self) -> f32 {
        let mut value = Value::from(&0f32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "yalign".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_yalign(&self, yalign: f32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "yalign".to_glib_none().0, Value::from(&yalign).to_glib_none().0);
        }
    }

    fn connect_property_obey_child_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::obey-child",
                transmute(notify_obey_child_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_ratio_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::ratio",
                transmute(notify_ratio_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_xalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::xalign",
                transmute(notify_xalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_yalign_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::yalign",
                transmute(notify_yalign_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_obey_child_trampoline<P>(this: *mut ffi::GtkAspectFrame, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AspectFrame> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AspectFrame::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ratio_trampoline<P>(this: *mut ffi::GtkAspectFrame, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AspectFrame> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AspectFrame::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_xalign_trampoline<P>(this: *mut ffi::GtkAspectFrame, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AspectFrame> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AspectFrame::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_yalign_trampoline<P>(this: *mut ffi::GtkAspectFrame, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<AspectFrame> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&AspectFrame::from_glib_none(this).downcast_unchecked())
}
