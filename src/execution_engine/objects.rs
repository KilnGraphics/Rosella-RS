use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_OBJECT_ID: AtomicU64 = AtomicU64::new(1);

macro_rules! define_object_reference {
    ($name: ident, $id_ty: ident, $def_ty: ident, $ref_ty: ident, $info_ty: ident) => {
        #[doc = concat!("A unique id referencing a ", stringify!($name))]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $id_ty(u64);

        impl $id_ty {
            pub fn new() -> Self {
                Self(NEXT_OBJECT_ID.fetch_add(1, Ordering::Relaxed))
            }

            pub fn as_u64(&self) -> u64 {
                self.0
            }
        }

        #[derive(Copy, Clone)]
        pub struct $def_ty<'a> {
            info: &'a $info_ty,
            id: $id_ty,
        }

        impl<'a> $def_ty<'a> {
            pub fn get_id(&self) -> $id_ty {
                self.id
            }

            pub fn get_info(&self) -> &'a $info_ty {
                self.info
            }
        }

        impl<'a> PartialEq for $def_ty<'a> {
            fn eq(&self, other: &Self) -> bool {
                self.get_id() == other.get_id()
            }
        }

        impl<'a> PartialEq<$id_ty> for $def_ty<'a> {
            fn eq(&self, other: &$id_ty) -> bool {
                &self.get_id() == other
            }
        }


        #[derive(Copy, Clone)]
        pub enum $ref_ty<'a> {
            Defined($def_ty<'a>),
            Placeholder($id_ty),
        }

        impl<'a> $ref_ty<'a> {
            pub fn get_id(&self) -> $id_ty {
                match self {
                    $ref_ty::Defined(ref def) => def.get_id(),
                    $ref_ty::Placeholder(id) => *id,
                }
            }

            pub fn get_info(&self) -> Option<&'a $info_ty> {
                match self {
                    $ref_ty::Defined(ref def) => Some(def.get_info()),
                    $ref_ty::Placeholder(_) => None,
                }
            }

            pub fn is_defined(&self) -> bool {
                match self {
                    $ref_ty::Defined(_) => true,
                    $ref_ty::Placeholder(_) => false,
                }
            }

            pub fn is_placeholder(&self) -> bool {
                match self {
                    $ref_ty::Defined(_) => false,
                    $ref_ty::Placeholder(_) => true,
                }
            }
        }

        impl<'a> PartialEq for $ref_ty<'a> {
            fn eq(&self, other: &Self) -> bool {
                self.get_id() == other.get_id()
            }
        }

        impl<'a> PartialEq<$id_ty> for $ref_ty<'a> {
            fn eq(&self, other: &$id_ty) -> bool {
                &self.get_id() == other
            }
        }

        impl<'a> PartialEq<$def_ty<'a>> for $ref_ty<'a> {
            fn eq(&self, other: &$def_ty<'a>) -> bool {
                self.get_id() == other.get_id()
            }
        }
    }
}

pub struct BufferInfo {
}

pub struct BufferViewInfo {
}

pub struct ImageInfo {
}

pub struct ImageViewInfo {
}

define_object_reference!(Buffer, BufferId, DefinedBuffer, BufferReference, BufferInfo);
define_object_reference!(BufferView, BufferViewId, DefinedBufferView, BufferViewReference, BufferViewInfo);
define_object_reference!(Image, ImageId, DefinedImage, ImageReference, ImageInfo);
define_object_reference!(ImageView, ImageViewId, DefinedImageView, ImageViewReference, ImageViewInfo);
