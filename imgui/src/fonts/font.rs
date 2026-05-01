use std::mem::transmute;

#[cfg(todo)]
use crate::fonts::atlas::{FontAtlas, FontId};
use crate::internal::RawCast;
use crate::sys;

/// Runtime data for a single font within a font atlas
#[repr(transparent)]
pub struct Font {
    font: sys::ImFont,
}
#[repr(C)]
#[cfg(todo)]
pub struct Font {
    last_baked: *mut sys::ImFontBaked,
    container_atlas: *mut sys::ImFontAtlas,
    owner: *mut sys::ImFontBaked,
    flags: sys::ImFontFlags,
    current_rasterizer_density: f32,
    font_id: sys::ImGuiID,
    #[deprecated]
    pub font_size: f32,
    sources: ImVector<*mut sys::ImFontConfig>,
    pub ellipsis_char: sys::ImWchar,
    pub fallback_char: sys::ImWchar,
    pub used_8k_pages_map: [u8; Self::USED_PAGES_MAX],
    pub ellipsis_auto_bake: bool,
    remap_pairs: sys::ImGuiStorage,
}

unsafe impl RawCast<sys::ImFont> for Font {}

impl Font {
    #[cfg(todo)]
    pub const USED_PAGES_MAX: usize = match size_of::<sys::ImWchar>() {
        2 => 1,
        4 => 17,
        _ => panic!("unknown char size"),
    };
    /// Returns the identifier of this font
    #[cfg(todo)]
    pub fn id(&self) -> FontId {
        FontId(self as *const _)
    }
    pub fn font_id(&self) -> sys::ImGuiID {
        self.font.FontId
    }

    pub fn bake(&mut self, font_size: f32, density: Option<f32>) -> Option<&sys::ImFontBaked> {
        unsafe {
            let baked = sys::ImFont_GetFontBaked(self.raw_mut() as *mut sys::ImFont, font_size, density.unwrap_or(-1.0f32));
            transmute::<*mut sys::ImFontBaked, _>(baked)
        }
    }

    #[inline(always)]
    pub fn sys(&self) -> &sys::ImFont {
        unsafe { self.raw() }
    }
    #[inline(always)]
    pub unsafe fn sys_mut(&mut self) -> &mut sys::ImFont {
        self.raw_mut()
    }
}
impl std::ops::Deref for Font {
    type Target = sys::ImFont;
    #[inline(always)]
    fn deref(&self) -> &Self::Target { self.sys() }
}

#[test]
fn test_font_memory_layout() {
    use std::mem;
    assert_eq!(mem::size_of::<Font>(), mem::size_of::<sys::ImFont>());
    assert_eq!(mem::align_of::<Font>(), mem::align_of::<sys::ImFont>());
    use sys::ImFont;
    macro_rules! assert_field_offset {
        ($l:ident, $r:ident) => {
            assert_eq!(
                memoffset::offset_of!(Font, $l),
                memoffset::offset_of!(ImFont, $r)
            );
        };
    }

    assert_field_offset!(flags, Flags);
    assert_field_offset!(last_baked, LastBaked);
    assert_field_offset!(font_size, LegacySize);
    assert_field_offset!(current_rasterizer_density, CurrentRasterizerDensity);
    assert_field_offset!(glyphs, Glyphs);
    assert_field_offset!(fallback_glyph, FallbackGlyph);
    assert_field_offset!(container_atlas, OwnerAtlas);
    assert_field_offset!(sources, Sources);
    assert_field_offset!(font_id, FontId);
    assert_field_offset!(fallback_char, FallbackChar);
    assert_field_offset!(ellipsis_char, EllipsisChar);
    assert_field_offset!(ellipsis_auto_bake, EllipsisAutoBake);
    assert_field_offset!(remap_pairs, RemapPairs);
    assert_field_offset!(used_8k_pages_map, Used8kPagesMap);
}
