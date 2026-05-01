#[cfg(todo)]
use std::collections::HashMap;
#[cfg(todo)]
use std::os::raw::c_void;
use sys::{ImTextureRef, ImTextureID};

/// An opaque texture identifier
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct TextureId(ImTextureRef);

impl TextureId {
    /// Creates a new texture id with the given identifier.
    #[inline]
    pub const fn new(id: usize) -> Self {
        Self(ImTextureRef::from_id(id as ImTextureID))
    }
    #[inline]
    #[cfg(todo)]
    pub const unsafe fn from_ptr(ptr: *const c_void) -> Self {
    }

    #[inline]
    pub const unsafe fn from_raw_ref(tex_ref: ImTextureRef) -> Self {
        Self(tex_ref)
    }

    /// Returns the id of the TextureId.
    #[inline]
    pub const fn id(self) -> ImTextureRef {
        self.0
    }
    #[inline]
    pub const fn tex_ref(&self) -> &ImTextureRef {
        &self.0
    }
}

impl From<TextureId> for ImTextureRef {
    #[inline]
    fn from(id: TextureId) -> Self {
        id.0
    }
}
#[cfg(todo)]
impl From<usize> for TextureId {
    #[inline]
    fn from(id: usize) -> Self {
        TextureId::new(id)
    }
}

//#[cfg(todo)]
#[cfg(feature = "compat-080")]
impl<T> From<*const T> for TextureId {
    #[inline]
    fn from(ptr: *const T) -> Self {
        TextureId::new(ptr as usize)
    }
}

#[cfg(todo)]
impl<T> From<*mut T> for TextureId {
    #[inline]
    fn from(ptr: *mut T) -> Self {
        TextureId(ptr as usize)
    }
}

#[test]
fn test_texture_id_memory_layout() {
    use std::mem;
    assert_eq!(
        mem::size_of::<TextureId>(),
        mem::size_of::<sys::ImTextureID>()
    );
    assert_eq!(
        mem::align_of::<TextureId>(),
        mem::align_of::<sys::ImTextureID>()
    );
}

/// Generic texture mapping for use by renderers.
#[derive(Debug)]
#[cfg(todo)]
pub struct Textures<T> {
    textures: HashMap<usize, T>,
    next: usize,
}

/// We manually impl Default as `#[derive(Default)]`
/// incorrectly requires `T: Default` which is
/// not necessary at all.
#[cfg(todo)]
impl<T> Default for Textures<T> {
    fn default() -> Self {
        Self {
            textures: Default::default(),
            next: Default::default(),
        }
    }
}

#[cfg(todo)]
impl<T> Textures<T> {
    // TODO: hasher like rustc_hash::FxHashMap or something would let this be
    // `const fn`
    pub fn new() -> Self {
        Textures {
            textures: HashMap::new(),
            next: 0,
        }
    }

    pub fn insert(&mut self, texture: T) -> TextureId {
        let id = self.next;
        self.textures.insert(id, texture);
        self.next += 1;
        TextureId::from(id)
    }

    pub fn replace(&mut self, id: TextureId, texture: T) -> Option<T> {
        self.textures.insert(id.0, texture)
    }

    pub fn remove(&mut self, id: TextureId) -> Option<T> {
        self.textures.remove(&id.0)
    }

    pub fn get(&self, id: TextureId) -> Option<&T> {
        self.textures.get(&id.0)
    }

    pub fn get_mut(&mut self, id: TextureId) -> Option<&mut T> {
        self.textures.get_mut(&id.0)
    }
}
