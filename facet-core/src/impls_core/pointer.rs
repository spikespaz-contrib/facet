use core::{fmt, hash::Hash, mem::transmute};

use crate::{
    CmpFn, CmpFnTyped, DebugFn, DebugFnTyped, DisplayFn, DisplayFnTyped, Facet, HashFn,
    HashFnTyped, HasherProxy, MarkerTraits, PartialEqFn, PartialEqFnTyped, PartialOrdFn,
    PartialOrdFnTyped, PointerType, Shape, Type, TypeParam, ValuePointerType, ValueVTable,
};

macro_rules! impl_facet_for_pointer {
    ($variant:ident: $type:ty => $shape:expr => $vtable_builder:expr => $ptr_type:ident, $mutable:expr) => {
        unsafe impl<'a, T: Facet<'a> + ?Sized> Facet<'a> for $type {
            const VTABLE: &'static ValueVTable = &const {
                $vtable_builder
                    .type_name(|f, opts| {
                        if let Some(opts) = opts.for_children() {
                            if stringify!($ptr_type) == "Raw" {
                                if $mutable {
                                    write!(f, "*mut ")?;
                                } else {
                                    write!(f, "*const ")?;
                                }
                            } else {
                                write!(f, "&")?;
                                if $mutable {
                                    write!(f, "mut ")?;
                                }
                            }
                            (T::VTABLE.type_name)(f, opts)
                        } else {
                            if stringify!($ptr_type) == "Raw" {
                                if $mutable {
                                    write!(f, "*mut ⋯")
                                } else {
                                    write!(f, "*const ⋯")
                                }
                            } else {
                                write!(f, "&")?;
                                if $mutable {
                                    write!(f, "mut ⋯")
                                } else {
                                    write!(f, "⋯")
                                }
                            }
                        }
                    })
                    .build()
            };

            const SHAPE: &'static Shape<'static> = &const {
                $shape
                    .type_identifier(
                        const {
                            let ptr_type = stringify!($ptr_type);
                            let is_raw = ptr_type.len() == 3
                                && ptr_type.as_bytes()[0] == b'R'
                                && ptr_type.as_bytes()[1] == b'a'
                                && ptr_type.as_bytes()[2] == b'w';
                            if is_raw {
                                if $mutable { "*mut _" } else { "*const _" }
                            } else {
                                if $mutable { "&mut _" } else { "&_" }
                            }
                        },
                    )
                    .type_params(&[TypeParam {
                        name: "T",
                        shape: || T::SHAPE,
                    }])
                    .ty({
                        let is_wide =
                            ::core::mem::size_of::<$type>() != ::core::mem::size_of::<*const ()>();
                        let vpt = ValuePointerType {
                            mutable: $mutable,
                            wide: is_wide,
                            target: || T::SHAPE,
                        };

                        Type::Pointer(PointerType::$ptr_type(vpt))
                    })
                    .build()
            };
        }
    };
}

// *const pointers
impl_facet_for_pointer!(
    Raw: *const T
        => Shape::builder_for_sized::<Self>()
            .inner(|| T::SHAPE)
        => ValueVTable::builder::<Self>()
            .marker_traits(|| {
                let mut marker_traits = MarkerTraits::EQ
                    .union(MarkerTraits::COPY)
                    .union(MarkerTraits::UNPIN);

                if T::SHAPE.vtable.marker_traits().contains(MarkerTraits::REF_UNWIND_SAFE) {
                    marker_traits = marker_traits.union(MarkerTraits::UNWIND_SAFE).union(MarkerTraits::REF_UNWIND_SAFE);
                }

                marker_traits
            })
            .debug(|| Some(fmt::Debug::fmt))
            .clone_into(|| Some(|src, dst| unsafe { dst.put(*src) }))
            .partial_eq(|| Some(|&left, &right| core::ptr::eq(left, right)))
            .partial_ord(|| Some(|&left, &right| {
                // https://github.com/rust-lang/rust/issues/141510
                #[allow(ambiguous_wide_pointer_comparisons)]
                left.partial_cmp(&right)
            }))
            .ord(|| Some(|&left, &right| {
                #[allow(ambiguous_wide_pointer_comparisons)]
                left.cmp(&right)
            }))
            .hash(|| Some(|value, hasher_this, hasher_write_fn| {
                value.hash(&mut unsafe {
                    HasherProxy::new(hasher_this, hasher_write_fn)
                })
            }))
        => Raw, false
);

// *mut pointers
impl_facet_for_pointer!(
    Raw: *mut T
        => Shape::builder_for_sized::<Self>()
            .inner(|| T::SHAPE)
        => ValueVTable::builder::<Self>()
            .marker_traits(|| {
                let mut marker_traits = MarkerTraits::EQ
                    .union(MarkerTraits::COPY)
                    .union(MarkerTraits::UNPIN);

                if T::SHAPE.vtable.marker_traits().contains(MarkerTraits::REF_UNWIND_SAFE) {
                    marker_traits = marker_traits.union(MarkerTraits::UNWIND_SAFE).union(MarkerTraits::REF_UNWIND_SAFE);
                }

                marker_traits
            })
            .debug(|| Some(fmt::Debug::fmt))
            .clone_into(|| Some(|src, dst| unsafe { dst.put(*src) }))
            .partial_eq(|| Some(|&left, &right| core::ptr::eq(left, right)))
            .partial_ord(|| Some(|&left, &right| {
                // https://github.com/rust-lang/rust/issues/141510
                #[allow(ambiguous_wide_pointer_comparisons)]
                left.partial_cmp(&right)
            }))
            .ord(|| Some(|&left, &right| {
                #[allow(ambiguous_wide_pointer_comparisons)]
                left.cmp(&right)
            }))
            .hash(|| Some(|value, hasher_this, hasher_write_fn| {
                value.hash(&mut unsafe {
                    HasherProxy::new(hasher_this, hasher_write_fn)
                })
            }))
        => Raw, true
);

// &T references
impl_facet_for_pointer!(
    Reference: &'a T
        => Shape::builder_for_sized::<Self>()
        => {
            ValueVTable::builder::<Self>()
                .marker_traits(|| {
                    let mut marker_traits = MarkerTraits::COPY.union(MarkerTraits::UNPIN);
                    if T::SHAPE.vtable.marker_traits().contains(MarkerTraits::EQ) {
                        marker_traits = marker_traits.union(MarkerTraits::EQ);
                    }
                    if T::SHAPE.vtable.marker_traits().contains(MarkerTraits::SYNC) {
                        marker_traits = marker_traits.union(MarkerTraits::SEND).union(MarkerTraits::SYNC);
                    }
                    if T::SHAPE.vtable.marker_traits().contains(MarkerTraits::REF_UNWIND_SAFE) {
                        marker_traits = marker_traits.union(MarkerTraits::UNWIND_SAFE).union(MarkerTraits::REF_UNWIND_SAFE);
                    }

                    marker_traits
                })
                .clone_into(|| Some(|src, dst| unsafe { dst.put(core::ptr::read(src)) }))
                .debug(|| {
                    if (T::VTABLE.debug)().is_some() {
                        Some(|value, f| {
                            let debug_fn = unsafe { transmute::<DebugFn, DebugFnTyped<T>>((T::VTABLE.debug)().unwrap()) };
                            debug_fn(*value, f)
                        })
                    } else {
                        None
                    }
                })
                .display(|| {
                    if (T::VTABLE.display)().is_some() {
                        Some(|value, f| {
                            let display_fn = unsafe { transmute::<DisplayFn, DisplayFnTyped<T>>((T::VTABLE.display)().unwrap()) };
                            display_fn(*value, f)
                        })
                    } else {
                        None
                    }
                })
                .partial_eq(|| {
                    if (T::VTABLE.partial_eq)().is_some() {
                        Some(|a, b| {
                            let eq_fn = unsafe { transmute::<PartialEqFn, PartialEqFnTyped<T>>((T::VTABLE.partial_eq)().unwrap()) };
                            eq_fn(*a, *b)
                        })
                    } else {
                        None
                    }
                })
                .partial_ord(|| {
                    if (T::VTABLE.partial_ord)().is_some() {
                        Some(|a, b| {
                            let partial_ord_fn = unsafe { transmute::<PartialOrdFn, PartialOrdFnTyped<T>>((T::VTABLE.partial_ord)().unwrap()) };
                            partial_ord_fn(*a, *b)
                        })
                    } else {
                        None
                    }
                })
                .ord(|| {
                    if (T::VTABLE.ord)().is_some() {
                        Some(|a, b| {
                            let ord_fn = unsafe { transmute::<CmpFn, CmpFnTyped<T>>((T::VTABLE.ord)().unwrap()) };
                            ord_fn(*a, *b)
                        })
                    } else {
                        None
                    }
                })
                .hash(|| {
                    if (T::VTABLE.hash)().is_some() {
                        Some(|value, hasher_this, hasher_write_fn| {
                            let hash_fn = unsafe { transmute::<HashFn, HashFnTyped<T>>((T::VTABLE.hash)().unwrap()) };
                            hash_fn(*value, hasher_this, hasher_write_fn)
                        })
                    } else {
                        None
                    }
                })
        }
        => Reference, false
);

// &mut T references
impl_facet_for_pointer!(
    Reference: &'a mut T
        => Shape::builder_for_sized::<Self>()
        => {
            ValueVTable::builder::<Self>()
                .marker_traits(|| {
                    let mut marker_traits = MarkerTraits::UNPIN;
                    if T::SHAPE.vtable.marker_traits().contains(MarkerTraits::EQ) {
                        marker_traits = marker_traits.union(MarkerTraits::EQ);
                    }
                    if T::SHAPE.vtable.marker_traits().contains(MarkerTraits::SEND) {
                        marker_traits = marker_traits.union(MarkerTraits::SEND);
                    }
                    if T::SHAPE.vtable.marker_traits().contains(MarkerTraits::SYNC) {
                        marker_traits = marker_traits.union(MarkerTraits::SYNC);
                    }
                    if T::SHAPE.vtable.marker_traits().contains(MarkerTraits::REF_UNWIND_SAFE) {
                        marker_traits = marker_traits.union(MarkerTraits::REF_UNWIND_SAFE);
                    }

                    marker_traits
                })
                .debug(|| {
                    if (T::VTABLE.debug)().is_some() {
                        Some(|value, f| {
                            let debug_fn = unsafe { transmute::<DebugFn, DebugFnTyped<T>>((T::VTABLE.debug)().unwrap()) };
                            debug_fn(*value, f)
                        })
                    } else {
                        None
                    }
                })
                .display(|| {
                    if (T::VTABLE.display)().is_some() {
                        Some(|value, f| {
                            let display_fn = unsafe { transmute::<DisplayFn, DisplayFnTyped<T>>((T::VTABLE.display)().unwrap()) };
                            display_fn(*value, f)
                        })
                    } else {
                        None
                    }
                })
                .partial_eq(|| {
                    if (T::VTABLE.partial_eq)().is_some() {
                        Some(|a, b| {
                            let eq_fn = unsafe { transmute::<PartialEqFn, PartialEqFnTyped<T>>((T::VTABLE.partial_eq)().unwrap()) };
                            eq_fn(*a, *b)
                        })
                    } else {
                        None
                    }
                })
                .partial_ord(|| {
                    if (T::VTABLE.partial_ord)().is_some() {
                        Some(|a, b| {
                            let partial_ord_fn = unsafe { transmute::<PartialOrdFn, PartialOrdFnTyped<T>>((T::VTABLE.partial_ord)().unwrap()) };
                            partial_ord_fn(*a, *b)
                        })
                    } else {
                        None
                    }
                })
                .ord(|| {
                    if (T::VTABLE.ord)().is_some() {
                        Some(|a, b| {
                            let ord_fn = unsafe { transmute::<CmpFn, CmpFnTyped<T>>((T::VTABLE.ord)().unwrap()) };
                            ord_fn(*a, *b)
                        })
                    } else {
                        None
                    }
                })
                .hash(|| {
                    if (T::VTABLE.hash)().is_some() {
                        Some(|value, hasher_this, hasher_write_fn| {
                            let hash_fn = unsafe { transmute::<HashFn, HashFnTyped<T>>((T::VTABLE.hash)().unwrap()) };
                            hash_fn(*value, hasher_this, hasher_write_fn)
                        })
                    } else {
                        None
                    }
                })
        }
        => Reference, true
);

#[cfg(test)]
mod test {
    use core::panic::{RefUnwindSafe, UnwindSafe};
    use impls::impls;

    #[allow(unused)]
    const fn assert_impls_unwind_safe<T: UnwindSafe>() {}
    #[allow(unused)]
    const fn assert_impls_ref_unwind_safe<T: RefUnwindSafe>() {}

    #[allow(unused)]
    const fn ref_unwind_safe<T: RefUnwindSafe>() {
        assert_impls_unwind_safe::<&T>();
        assert_impls_ref_unwind_safe::<&T>();

        assert_impls_ref_unwind_safe::<&mut T>();

        assert_impls_unwind_safe::<*const T>();
        assert_impls_ref_unwind_safe::<*const T>();

        assert_impls_unwind_safe::<*mut T>();
        assert_impls_ref_unwind_safe::<*mut T>();
    }

    #[test]
    fn mut_ref_not_unwind_safe() {
        assert!(impls!(&mut (): !UnwindSafe));
    }
}
