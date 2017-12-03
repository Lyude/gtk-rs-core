// This file was generated by gir (8080733) from gir-files (469db10)
// DO NOT EDIT

use CellRenderer;
use ffi;
use gdk_pixbuf;
use gio;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct CellRendererPixbuf(Object<ffi::GtkCellRendererPixbuf, ffi::GtkCellRendererPixbufClass>): CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_pixbuf_get_type(),
    }
}

impl CellRendererPixbuf {
    pub fn new() -> CellRendererPixbuf {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_pixbuf_new()).downcast_unchecked()
        }
    }
}

impl Default for CellRendererPixbuf {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CellRendererPixbufExt {
    fn get_property_follow_state(&self) -> bool;

    fn set_property_follow_state(&self, follow_state: bool);

    fn get_property_gicon(&self) -> Option<gio::Icon>;

    fn set_property_gicon<P: IsA<gio::Icon> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, gicon: Option<&P>);

    fn get_property_icon_name(&self) -> Option<String>;

    fn set_property_icon_name(&self, icon_name: Option<&str>);

    fn get_property_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>);

    fn get_property_pixbuf_expander_closed(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn set_property_pixbuf_expander_closed(&self, pixbuf_expander_closed: Option<&gdk_pixbuf::Pixbuf>);

    fn get_property_pixbuf_expander_open(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn set_property_pixbuf_expander_open(&self, pixbuf_expander_open: Option<&gdk_pixbuf::Pixbuf>);

    fn get_property_stock_detail(&self) -> Option<String>;

    fn set_property_stock_detail(&self, stock_detail: Option<&str>);

    fn get_property_stock_id(&self) -> Option<String>;

    fn set_property_stock_id(&self, stock_id: Option<&str>);

    fn get_property_stock_size(&self) -> u32;

    fn set_property_stock_size(&self, stock_size: u32);

    fn connect_property_follow_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_expander_closed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pixbuf_expander_open_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_stock_detail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_stock_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_stock_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererPixbuf> + IsA<glib::object::Object>> CellRendererPixbufExt for O {
    fn get_property_follow_state(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "follow-state".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_follow_state(&self, follow_state: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "follow-state".to_glib_none().0, Value::from(&follow_state).to_glib_none().0);
        }
    }

    fn get_property_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            let mut value = Value::from_type(<gio::Icon as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "gicon".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_gicon<P: IsA<gio::Icon> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, gicon: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "gicon".to_glib_none().0, Value::from(gicon).to_glib_none().0);
        }
    }

    fn get_property_icon_name(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "icon-name".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "icon-name".to_glib_none().0, Value::from(icon_name).to_glib_none().0);
        }
    }

    fn get_property_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let mut value = Value::from_type(<gdk_pixbuf::Pixbuf as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pixbuf".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_pixbuf(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pixbuf".to_glib_none().0, Value::from(pixbuf).to_glib_none().0);
        }
    }

    fn get_property_pixbuf_expander_closed(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let mut value = Value::from_type(<gdk_pixbuf::Pixbuf as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pixbuf-expander-closed".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_pixbuf_expander_closed(&self, pixbuf_expander_closed: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pixbuf-expander-closed".to_glib_none().0, Value::from(pixbuf_expander_closed).to_glib_none().0);
        }
    }

    fn get_property_pixbuf_expander_open(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let mut value = Value::from_type(<gdk_pixbuf::Pixbuf as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "pixbuf-expander-open".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_pixbuf_expander_open(&self, pixbuf_expander_open: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "pixbuf-expander-open".to_glib_none().0, Value::from(pixbuf_expander_open).to_glib_none().0);
        }
    }

    fn get_property_stock_detail(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stock-detail".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_stock_detail(&self, stock_detail: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stock-detail".to_glib_none().0, Value::from(stock_detail).to_glib_none().0);
        }
    }

    fn get_property_stock_id(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stock-id".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_stock_id(&self, stock_id: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stock-id".to_glib_none().0, Value::from(stock_id).to_glib_none().0);
        }
    }

    fn get_property_stock_size(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "stock-size".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_stock_size(&self, stock_size: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "stock-size".to_glib_none().0, Value::from(&stock_size).to_glib_none().0);
        }
    }

    fn connect_property_follow_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::follow-state",
                transmute(notify_follow_state_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_gicon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::gicon",
                transmute(notify_gicon_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::icon-name",
                transmute(notify_icon_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pixbuf_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pixbuf",
                transmute(notify_pixbuf_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pixbuf_expander_closed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pixbuf-expander-closed",
                transmute(notify_pixbuf_expander_closed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pixbuf_expander_open_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pixbuf-expander-open",
                transmute(notify_pixbuf_expander_open_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_stock_detail_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stock-detail",
                transmute(notify_stock_detail_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_stock_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stock-id",
                transmute(notify_stock_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_stock_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::stock-size",
                transmute(notify_stock_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_follow_state_trampoline<P>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_gicon_trampoline<P>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_icon_name_trampoline<P>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixbuf_trampoline<P>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixbuf_expander_closed_trampoline<P>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pixbuf_expander_open_trampoline<P>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_stock_detail_trampoline<P>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_stock_id_trampoline<P>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_stock_size_trampoline<P>(this: *mut ffi::GtkCellRendererPixbuf, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererPixbuf> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellRendererPixbuf::from_glib_borrow(this).downcast_unchecked())
}
