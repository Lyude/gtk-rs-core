// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`Renderer`](crate::Renderer).

use crate::{
    AttrShape, Font, Glyph, GlyphItem, GlyphString, LayoutRun, Renderer, RenderPart,
    subclass::prelude::*,
};
use glib::{
    Cast, GString,
    translate::*,
};

pub trait RendererImpl: ObjectImpl + RendererImplExt {
    fn begin(&self, renderer: &Self::Type) {
        self.parent_begin(renderer)
    }

    fn draw_error_underline(&self, renderer: &Self::Type, x: i32, y: i32, width: i32, height: i32) {
        self.parent_draw_error_underline(renderer, x, y, width, height)
    }

    fn draw_glyph(&self, renderer: &Self::Type, font: &Font, glyph: Glyph, x: f64, y: f64) {
        self.parent_draw_glyph(renderer, font, glyph, x, y)
    }

    fn draw_glyph_item(
        &self,
        renderer: &Self::Type,
        text: Option<&str>,
        glyph_item: &mut GlyphItem,
        x: i32,
        y: i32
    ) {
        self.parent_draw_glyph_item(renderer, text, glyph_item, x, y)
    }

    fn draw_glyphs(
        &self,
        renderer: &Self::Type,
        font: &Font,
        glyphs: &mut GlyphString,
        x: i32,
        y: i32,
    ) {
        self.parent_draw_glyphs(renderer, font, glyphs, x, y)
    }

    fn draw_rectangle(
        &self,
        renderer: &Self::Type,
        part: RenderPart,
        x: i32,
        y: i32,
        width: i32,
        height: i32
    ) {
        self.parent_draw_rectangle(renderer, part, x, y, width, height)
    }

    fn draw_shape(&self, renderer: &Self::Type, attr: &mut AttrShape, x: i32, y: i32) {
        self.parent_draw_shape(renderer, attr, x, y)
    }

    fn draw_trapezoid(
        &self,
        renderer: &Self::Type,
        part: RenderPart,
        y1: f64,
        x11: f64,
        x21: f64,
        y2: f64,
        x12: f64,
        x22: f64,
    ) {
        self.parent_draw_trapezoid(renderer, part, y1, x11, x21, y2, x12, x22)
    }

    fn end(&self, renderer: &Self::Type) {
        self.parent_end(renderer)
    }

    fn part_changed(&self, renderer: &Self::Type, part: RenderPart) {
        self.parent_part_changed(renderer, part)
    }

    fn prepare_run(&self, renderer: &Self::Type, run: &mut LayoutRun) {
        self.parent_prepare_run(renderer, run)
    }
}

pub trait RendererImplExt: ObjectSubclass {
    fn parent_begin(&self, renderer: &Self::Type);
    fn parent_draw_error_underline(
        &self,
        renderer: &Self::Type,
        x: i32,
        y: i32,
        width: i32,
        height: i32
    );
    fn parent_draw_glyph(&self, renderer: &Self::Type, font: &Font, glyph: Glyph, x: f64, y: f64);
    fn parent_draw_glyph_item(
        &self,
        renderer: &Self::Type,
        text: Option<&str>,
        glyph_item: &mut GlyphItem,
        x: i32,
        y: i32
    );
    fn parent_draw_glyphs(
        &self,
        renderer: &Self::Type,
        font: &Font,
        glyphs: &mut GlyphString,
        x: i32,
        y: i32
    );
    fn parent_draw_rectangle(
        &self,
        renderer: &Self::Type,
        part: RenderPart,
        x: i32,
        y: i32,
        width: i32,
        height: i32
    );
    fn parent_draw_shape(&self, renderer: &Self::Type, attr: &mut AttrShape, x: i32, y: i32);
    fn parent_draw_trapezoid(
        &self,
        renderer: &Self::Type,
        part: RenderPart,
        y1: f64,
        x11: f64,
        x21: f64,
        y2: f64,
        x12: f64,
        x22: f64
    );
    fn parent_end(&self, renderer: &Self::Type);
    fn parent_part_changed(&self, renderer: &Self::Type, part: RenderPart);
    fn parent_prepare_run(&self, renderer: &Self::Type, run: &mut LayoutRun);
}

impl<T: RendererImpl> RendererImplExt for T {
    fn parent_begin(&self, renderer: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PangoRendererClass;
            let f = (*parent_class)
                .begin
                .expect(r#"No parent class implementation for "begin""#);
            f(renderer.unsafe_cast_ref::<Renderer>().to_glib_none().0)
        }
    }

    fn parent_draw_error_underline(
        &self,
        renderer: &Self::Type,
        x: i32,
        y: i32,
        width: i32,
        height: i32
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PangoRendererClass;
            let f = (*parent_class)
                .draw_error_underline
                .expect(r#"No parent class implementation for "draw_error_underline""#);
            f(
                renderer.unsafe_cast_ref::<Renderer>().to_glib_none().0,
                x,
                y,
                width,
                height
            )
        }
    }

    fn parent_draw_glyph(&self, renderer: &Self::Type, font: &Font, glyph: Glyph, x: f64, y: f64) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PangoRendererClass;
            let f = (*parent_class)
                .draw_glyph
                .expect(r#"No parent class implementation for "draw_glyph""#);
            f(
                renderer.unsafe_cast_ref::<Renderer>().to_glib_none().0,
                font.unsafe_cast_ref::<Font>().to_glib_none().0,
                glyph,
                x,
                y
            )
        }
    }

    fn parent_draw_glyph_item(
        &self,
        renderer: &Self::Type,
        text: Option<&str>,
        glyph_item: &mut GlyphItem,
        x: i32,
        y: i32
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PangoRendererClass;
            let f = (*parent_class)
                .draw_glyph_item
                .expect(r#"No parent class implementation for "draw_glyph_item""#);
            f(
                renderer.unsafe_cast_ref::<Renderer>().to_glib_none().0,
                text.to_glib_none().0,
                glyph_item.to_glib_none_mut().0,
                x,
                y
            )
        }
    }

    fn parent_draw_glyphs(
        &self,
        renderer: &Self::Type,
        font: &Font,
        glyphs: &mut GlyphString,
        x: i32,
        y: i32
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PangoRendererClass;
            let f = (*parent_class)
                .draw_glyphs
                .expect(r#"No parent class implementation for "draw_glyphs""#);
            f(
                renderer.unsafe_cast_ref::<Renderer>().to_glib_none().0,
                font.unsafe_cast_ref::<Font>().to_glib_none().0,
                glyphs.to_glib_none_mut().0,
                x,
                y
            )
        }
    }

    fn parent_draw_rectangle(
        &self,
        renderer: &Self::Type,
        part: RenderPart,
        x: i32,
        y: i32,
        width: i32,
        height: i32
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PangoRendererClass;
            let f = (*parent_class)
                .draw_rectangle
                .expect(r#"No parent class implementation for "draw_rectangle""#);
            f(
                renderer.unsafe_cast_ref::<Renderer>().to_glib_none().0,
                part.into_glib(),
                x,
                y,
                width,
                height
            )
        }
    }

    fn parent_draw_shape(&self, renderer: &Self::Type, attr: &mut AttrShape, x: i32, y: i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PangoRendererClass;
            let f = (*parent_class)
                .draw_shape
                .expect(r#"No parent class implementation for "draw_shape""#);
            f(
                renderer.unsafe_cast_ref::<Renderer>().to_glib_none().0,
                attr.to_glib_none_mut().0,
                x,
                y
            )
        }
    }

    fn parent_draw_trapezoid(
        &self,
        renderer: &Self::Type,
        part: RenderPart,
        y1: f64,
        x11: f64,
        x21: f64,
        y2: f64,
        x12: f64,
        x22: f64
    ) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PangoRendererClass;
            let f = (*parent_class)
                .draw_trapezoid
                .expect(r#"No parent class implementation for "draw_trapezoid""#);
            f(
                renderer.unsafe_cast_ref::<Renderer>().to_glib_none().0,
                part.into_glib(),
                y1,
                x11,
                x21,
                y2,
                x12,
                x22
            )
        }
    }

    fn parent_end(&self, renderer: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PangoRendererClass;
            let f = (*parent_class)
                .end
                .expect(r#"No parent class implementation for "end""#);
            f(renderer.unsafe_cast_ref::<Renderer>().to_glib_none().0)
        }
    }

    fn parent_part_changed(&self, renderer: &Self::Type, part: RenderPart) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PangoRendererClass;
            let f = (*parent_class)
                .part_changed
                .expect(r#"No parent class implementation for "part_changed"#);
            f(renderer.unsafe_cast_ref::<Renderer>().to_glib_none().0, part.into_glib())
        }
    }

    fn parent_prepare_run(&self, renderer: &Self::Type, run: &mut LayoutRun) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::PangoRendererClass;
            let f = (*parent_class)
                .prepare_run
                .expect(r#"No parent class implementation for "prepare_run"#);
            f(renderer.unsafe_cast_ref::<Renderer>().to_glib_none().0, run.to_glib_none_mut().0)
        }
    }
}

unsafe impl<T: RendererImpl> IsSubclassable<T> for Renderer {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.begin = Some(begin::<T>);
        klass.draw_error_underline = Some(draw_error_underline::<T>);
        klass.draw_glyph = Some(draw_glyph::<T>);
        klass.draw_glyph_item = Some(draw_glyph_item::<T>);
        klass.draw_glyphs = Some(draw_glyphs::<T>);
        klass.draw_rectangle = Some(draw_rectangle::<T>);
        klass.draw_shape = Some(draw_shape::<T>);
        klass.draw_trapezoid = Some(draw_trapezoid::<T>);
        klass.end = Some(end::<T>);
        klass.part_changed = Some(part_changed::<T>);
        klass.prepare_run = Some(prepare_run::<T>);
    }
}

// TODO: Verify that we're handling the strange &muts correctly
unsafe extern "C" fn begin<T: RendererImpl>(ptr: *mut ffi::PangoRenderer) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Renderer> = from_glib_borrow(ptr);

    imp.begin(wrap.unsafe_cast_ref());
}

unsafe extern "C" fn draw_error_underline<T: RendererImpl>(
    ptr: *mut ffi::PangoRenderer,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Renderer> = from_glib_borrow(ptr);

    imp.draw_error_underline(wrap.unsafe_cast_ref(), x, y, width, height);
}

unsafe extern "C" fn draw_glyph<T: RendererImpl>(
    ptr: *mut ffi::PangoRenderer,
    font: *mut ffi::PangoFont,
    glyph: ffi::PangoGlyph,
    x: f64,
    y: f64,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Renderer> = from_glib_borrow(ptr);

    imp.draw_glyph(wrap.unsafe_cast_ref(), &from_glib_borrow(font), glyph, x, y);
}

unsafe extern "C" fn draw_glyph_item<T: RendererImpl>(
    ptr: *mut ffi::PangoRenderer,
    text: *const libc::c_char,
    glyph_item_ptr: *mut ffi::PangoGlyphItem,
    x: i32,
    y: i32
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Renderer> = from_glib_borrow(ptr);
    let text: Borrowed<GString> = from_glib_borrow(text);
    let mut glyph_item = from_glib_none(glyph_item_ptr);

    imp.draw_glyph_item(wrap.unsafe_cast_ref(), Some(&text), &mut glyph_item, x, y);
    *glyph_item_ptr = *glyph_item.to_glib_none().0;
}

unsafe extern "C" fn draw_glyphs<T: RendererImpl>(
    ptr: *mut ffi::PangoRenderer,
    font: *mut ffi::PangoFont,
    glyphs_ptr: *mut ffi::PangoGlyphString,
    x: i32,
    y: i32
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Renderer> = from_glib_borrow(ptr);
    let mut glyphs = from_glib_none(glyphs_ptr);

    imp.draw_glyphs(wrap.unsafe_cast_ref(), &from_glib_borrow(font), &mut glyphs, x, y);
    *glyphs_ptr = *glyphs.to_glib_none().0;
}

unsafe extern "C" fn draw_rectangle<T: RendererImpl>(
    ptr: *mut ffi::PangoRenderer,
    part: ffi::PangoRenderPart,
    x: i32,
    y: i32,
    width: i32,
    height: i32
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Renderer> = from_glib_borrow(ptr);

    imp.draw_rectangle(wrap.unsafe_cast_ref(), from_glib(part), x, y, width, height);
}

unsafe extern "C" fn draw_shape<T: RendererImpl>(
    ptr: *mut ffi::PangoRenderer,
    attr_ptr: *mut ffi::PangoAttrShape,
    x: i32,
    y: i32
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Renderer> = from_glib_borrow(ptr);
    let mut attr = from_glib_none(attr_ptr);

    imp.draw_shape(wrap.unsafe_cast_ref(), &mut attr, x, y);
    *attr_ptr = *attr.to_glib_none().0;
}

unsafe extern "C" fn draw_trapezoid<T: RendererImpl>(
    ptr: *mut ffi::PangoRenderer,
    part: ffi::PangoRenderPart,
    y1: f64,
    x11: f64,
    x21: f64,
    y2: f64,
    x12: f64,
    x22: f64,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Renderer> = from_glib_borrow(ptr);

    imp.draw_trapezoid(wrap.unsafe_cast_ref(), from_glib(part), y1, x11, x21, y2, x12, x22);
}

unsafe extern "C" fn end<T: RendererImpl>(ptr: *mut ffi::PangoRenderer) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Renderer> = from_glib_borrow(ptr);

    imp.end(wrap.unsafe_cast_ref());
}

unsafe extern "C" fn part_changed<T: RendererImpl>(
    ptr: *mut ffi::PangoRenderer,
    part: ffi::PangoRenderPart
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Renderer> = from_glib_borrow(ptr);

    imp.part_changed(wrap.unsafe_cast_ref(), from_glib(part));
}

unsafe extern "C" fn prepare_run<T: RendererImpl>(
    ptr: *mut ffi::PangoRenderer,
    run_ptr: *mut ffi::PangoLayoutRun,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();
    let wrap: Borrowed<Renderer> = from_glib_borrow(ptr);
    let mut run = from_glib_none(run_ptr);

    imp.prepare_run(wrap.unsafe_cast_ref(), &mut run);
    *run_ptr = *run.to_glib_none().0;
}
