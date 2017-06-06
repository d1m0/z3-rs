use z3_sys::*;
use Context;
use Symbol;
use Sort;
use Z3_MUTEX;
use std::cmp::{PartialEq, Eq};
use std::ffi::CString;
use std::fmt::{Display, Formatter, Error};

impl<'ctx> Sort<'ctx> {

    pub fn uninterpretd(ctx: &'ctx Context, sym: &Symbol<'ctx>) -> Sort<'ctx> {
        Sort {
            ctx: ctx,
            z3_sort: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                Z3_mk_uninterpreted_sort(ctx.z3_ctx, sym.z3_sym)
            }
        }
    }

    pub fn bool(ctx: &Context) -> Sort {
        Sort {
            ctx: ctx,
            z3_sort: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                Z3_mk_bool_sort(ctx.z3_ctx)
            }
        }
    }

    pub fn int(ctx: &Context) -> Sort {
        Sort {
            ctx: ctx,
            z3_sort: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                Z3_mk_int_sort(ctx.z3_ctx)
            }
        }
    }

    pub fn real(ctx: &Context) -> Sort {
        Sort {
            ctx: ctx,
            z3_sort: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                Z3_mk_real_sort(ctx.z3_ctx)
            }
        }
    }

    pub fn bitvector(ctx: &Context, sz: u32) -> Sort {
        Sort {
            ctx: ctx,
            z3_sort: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                Z3_mk_bv_sort(ctx.z3_ctx, sz as ::libc::c_uint)
            }
        }
    }

    pub fn array(ctx: &'ctx Context,
                 domain: &Sort<'ctx>,
                 range: &Sort<'ctx>) -> Sort<'ctx> {
        Sort {
            ctx: ctx,
            z3_sort: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                Z3_mk_array_sort(ctx.z3_ctx, domain.z3_sort, range.z3_sort)
            }
        }
    }

    pub fn set(ctx: &'ctx Context, elt: &Sort<'ctx>) -> Sort<'ctx> {
        Sort {
            ctx: ctx,
            z3_sort: unsafe {
                let guard = Z3_MUTEX.lock().unwrap();
                Z3_mk_set_sort(ctx.z3_ctx, elt.z3_sort)
            }
        }
    }

}

impl<'ctx> PartialEq<Sort<'ctx>> for Sort<'ctx> {
    fn eq(&self, other: &Sort<'ctx>) -> bool {
        unsafe {
            Z3_TRUE == Z3_is_eq_sort(self.ctx.z3_ctx,
                                    self.z3_sort,
                                    other.z3_sort)
        }
    }
}

impl<'ctx> Eq for Sort<'ctx> { }


impl<'ctx> Display for Sort<'ctx> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let s;
        unsafe {
            let p = CString::from_raw(Z3_sort_to_string(self.ctx.z3_ctx, self.z3_sort) as *mut i8);
            if p.as_ptr().is_null() {
                return Result::Err(Error);
            }
            match p.into_string() {
                Ok(parsed_s) => s = parsed_s,
                Err(_) => return Result::Err(Error),
            }
        }
        write!(f, "{}", s)
    }
}
