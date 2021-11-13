initSidebarItems({"constant":[["ANALYSIS_FLAG_CENTERED_BASELINE","Whether the segment should be shifted to center around the baseline."],["ANALYSIS_FLAG_IS_ELLIPSIS","Whether this run holds ellipsized text."],["ANALYSIS_FLAG_NEED_HYPHEN","Whether to add a hyphen at the end of the run during shaping."],["ATTR_INDEX_FROM_TEXT_BEGINNING","Value for `start_index` in [`Attribute`][crate::Attribute] that indicates the beginning of the text."],["ATTR_INDEX_TO_TEXT_END","Value for `end_index` in [`Attribute`][crate::Attribute] that indicates the end of the text."],["GLYPH_EMPTY","A `PangoGlyph` value that indicates a zero-width empty glpyh."],["GLYPH_INVALID_INPUT","A `PangoGlyph` value for invalid input."],["GLYPH_UNKNOWN_FLAG","Flag used in `PangoGlyph` to turn a `gunichar` value of a valid Unicode character into an unknown-character glyph for that `gunichar`."],["SCALE","The scale between dimensions used for Pango distances and device units."],["SCALE_LARGE","The scale factor for one magnification step (1.2)."],["SCALE_MEDIUM","The scale factor for normal size (1.0)."],["SCALE_SMALL","The scale factor for one shrinking step (1 / 1.2)."],["SCALE_XX_LARGE","The scale factor for three magnification steps (1.2 * 1.2 * 1.2)."],["SCALE_XX_SMALL","The scale factor for three shrinking steps (1 / (1.2 * 1.2 * 1.2))."],["SCALE_X_LARGE","The scale factor for two magnification steps (1.2 * 1.2)."],["SCALE_X_SMALL","The scale factor for two shrinking steps (1 / (1.2 * 1.2))."]],"enum":[["Alignment","[`Alignment`][crate::Alignment] describes how to align the lines of a [`Layout`][crate::Layout] within the available space."],["AttrType","The [`AttrType`][crate::AttrType] distinguishes between different types of attributes."],["BaselineShift","An enumeration that affects baseline shifts between runs."],["BidiType","[`BidiType`][crate::BidiType] represents the bidirectional character type of a Unicode character."],["CoverageLevel","[`CoverageLevel`][crate::CoverageLevel] is used to indicate how well a font can represent a particular Unicode character for a particular script."],["Direction","[`Direction`][crate::Direction] represents a direction in the Unicode bidirectional algorithm."],["EllipsizeMode","[`EllipsizeMode`][crate::EllipsizeMode] describes what sort of ellipsization should be applied to text."],["FontScale","An enumeration that affects font sizes for superscript and subscript positioning."],["Gravity","[`Gravity`][crate::Gravity] represents the orientation of glyphs in a segment of text."],["GravityHint","[`GravityHint`][crate::GravityHint] defines how horizontal scripts should behave in a vertical context."],["Overline","The [`Overline`][crate::Overline] enumeration is used to specify whether text should be overlined, and if so, the type of line."],["RenderPart","[`RenderPart`][crate::RenderPart] defines different items to render for such purposes as setting colors."],["Script","The [`Script`][crate::Script] enumeration identifies different writing systems."],["Stretch","An enumeration specifying the width of the font relative to other designs within a family."],["Style","An enumeration specifying the various slant styles possible for a font."],["TabAlign","[`TabAlign`][crate::TabAlign] specifies where a tab stop appears relative to the text."],["TextTransform","An enumeration that affects how Pango treats characters during shaping."],["Underline","The [`Underline`][crate::Underline] enumeration is used to specify whether text should be underlined, and if so, the type of underlining."],["Variant","An enumeration specifying capitalization variant of the font."],["Weight","An enumeration specifying the weight (boldness) of a font."],["WrapMode","[`WrapMode`][crate::WrapMode] describes how to wrap the lines of a [`Layout`][crate::Layout] to the desired width."]],"fn":[["extents_to_pixels","Converts extents from Pango units to device units."],["find_base_dir","Searches a string the first character that has a strong direction, according to the Unicode bidirectional algorithm."],["find_paragraph_boundary","Locates a paragraph boundary in `text`."],["is_zero_width","Checks if a character that should not be normally rendered."],["itemize","Breaks a piece of text into segments with consistent directional level and font."],["itemize_with_base_dir","Like `[`itemize()`][crate::itemize()]`, but with an explicitly specified base direction."],["parse_markup","Parses marked-up text to create a plain-text string and an attribute list."],["parse_stretch","Parses a font stretch."],["parse_style","Parses a font style."],["parse_variant","Parses a font variant."],["parse_weight","Parses a font weight."],["quantize_line_geometry","Quantizes the thickness and position of a line to whole device pixels."],["reorder_items","Reorder items from logical order to visual order."],["shape","Convert the characters in `text` into glyphs."],["shape_full","Convert the characters in `text` into glyphs."],["shape_with_flags","Convert the characters in `text` into glyphs."],["unichar_direction","Determines the inherent direction of a character."],["units_from_double","Converts a floating-point number to Pango units."],["units_to_double","Converts a number in Pango units to floating-point."],["version","Returns the encoded version of Pango available at run-time."],["version_check","Checks that the Pango library in use is compatible with the given version."],["version_string","Returns the version of Pango available at run-time."]],"mod":[["functions",""],["prelude","Traits and essential types intended for blanket imports."]],"struct":[["Analysis","The [`Analysis`][crate::Analysis] structure stores information about the properties of a segment of text."],["AttrClass","The [`AttrClass`][crate::AttrClass] structure stores the type and operations for a particular type of attribute."],["AttrColor",""],["AttrFloat",""],["AttrFontDesc",""],["AttrFontFeatures",""],["AttrInt",""],["AttrIterator","A [`AttrIterator`][crate::AttrIterator] is used to iterate through a [`AttrList`][crate::AttrList]."],["AttrLanguage",""],["AttrList","A [`AttrList`][crate::AttrList] represents a list of attributes that apply to a section of text."],["AttrShape",""],["AttrSize",""],["AttrString",""],["Attribute","The [`Attribute`][crate::Attribute] structure represents the common portions of all attributes."],["Color","The [`Color`][crate::Color] structure is used to represent a color in an uncalibrated RGB color-space."],["Context","A [`Context`][crate::Context] stores global information used to control the itemization process."],["Coverage","A [`Coverage`][crate::Coverage] structure is a map from Unicode characters to [`CoverageLevel`][crate::CoverageLevel] values."],["Font","A [`Font`][crate::Font] is used to represent a font in a rendering-system-independent manner."],["FontDescription","A [`FontDescription`][crate::FontDescription] describes a font in an implementation-independent manner."],["FontFace","A [`FontFace`][crate::FontFace] is used to represent a group of fonts with the same family, slant, weight, and width, but varying sizes."],["FontFamily","A [`FontFamily`][crate::FontFamily] is used to represent a family of related font faces."],["FontMap","A [`FontMap`][crate::FontMap] represents the set of fonts available for a particular rendering system."],["FontMask","The bits in a [`FontMask`][crate::FontMask] correspond to the set fields in a [`FontDescription`][crate::FontDescription]."],["FontMetrics","A [`FontMetrics`][crate::FontMetrics] structure holds the overall metric information for a font."],["Fontset","A [`Fontset`][crate::Fontset] represents a set of [`Font`][crate::Font] to use when rendering text."],["FontsetSimple","[`FontsetSimple`][crate::FontsetSimple] is a implementation of the abstract [`Fontset`][crate::Fontset] base class as an array of fonts."],["GlyphGeometry","The [`GlyphGeometry`][crate::GlyphGeometry] structure contains width and positioning information for a single glyph."],["GlyphInfo","A [`GlyphInfo`][crate::GlyphInfo] structure represents a single glyph with positioning information and visual attributes."],["GlyphItem","A [`GlyphItem`][crate::GlyphItem] is a pair of a [`Item`][crate::Item] and the glyphs resulting from shaping the items text."],["GlyphItemIter","A [`GlyphItemIter`][crate::GlyphItemIter] is an iterator over the clusters in a [`GlyphItem`][crate::GlyphItem]."],["GlyphString","A [`GlyphString`][crate::GlyphString] is used to store strings of glyphs with geometry and visual attribute information."],["HitPosition","The result of [`LayoutLine::x_to_index`]."],["Item","The [`Item`][crate::Item] structure stores information about a segment of text."],["Language","The [`Language`][crate::Language] structure is used to represent a language."],["Layout","A [`Layout`][crate::Layout] structure represents an entire paragraph of text."],["LayoutIter","A [`LayoutIter`][crate::LayoutIter] can be used to iterate over the visual extents of a [`Layout`][crate::Layout]."],["LayoutLine","A [`LayoutLine`][crate::LayoutLine] represents one of the lines resulting from laying out a paragraph via [`Layout`][crate::Layout]."],["Matrix","A [`Matrix`][crate::Matrix] specifies a transformation between user-space and device coordinates."],["Rectangle","The [`Rectangle`][crate::Rectangle] structure represents a rectangle."],["Renderer","[`Renderer`][crate::Renderer] is a base class for objects that can render text provided as [`GlyphString`][crate::GlyphString] or [`Layout`][crate::Layout]."],["ShapeFlags","Flags influencing the shaping process."],["ShowFlags","These flags affect how Pango treats characters that are normally not visible in the output."],["TabArray","A [`TabArray`][crate::TabArray] contains an array of tab stops."]],"trait":[["IsAttribute",""]],"type":[["Glyph",""],["GlyphUnit",""],["LayoutRun",""]]});