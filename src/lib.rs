#![allow(nonstandard_style)]

use std::ffi::CStr;

use libperl_sys::*;
use libperl_sys::perl_core::PerlInterpreter;

#[unsafe(no_mangle)]
pub extern "C" fn boot_Mytest(my_perl_ptr: *mut PerlInterpreter, _cv: *mut CV) -> () {

    if my_perl_ptr.is_null() {
        panic!()
    }

    static NAME1: &CStr = unsafe {
        CStr::from_bytes_with_nul_unchecked(b"Mytest::is_even\0")
    };
    unsafe {
        Perl_newXS_deffile(my_perl_ptr, NAME1.as_ptr(), Some(is_even_C))
    };

    unsafe {
        Perl_xs_boot_epilog(my_perl_ptr, 1);
    };
}


#[unsafe(no_mangle)]
pub extern "C" fn is_even_C(my_perl_ptr: *mut PerlInterpreter, cv: *mut CV) -> () {
    if let Some(my_perl) = unsafe {my_perl_ptr.as_mut()} {
        is_even(my_perl, cv)
    } else {
        
    }
}

fn is_even(my_perl: &mut PerlInterpreter, cv: *mut CV) -> () {
    // dSP
    let sp = my_perl.Istack_sp;

    // dAXMARK
    let mut ax: Stack_off_t = unsafe {*my_perl.Imarkstack_ptr};
    my_perl.Imarkstack_ptr = unsafe {my_perl.Imarkstack_ptr.sub(1)};

    // POPMARK
    let mark = unsafe {my_perl.Istack_base.add(ax as usize)};
    ax = ax+1;

    // dITEMS
    let items = unsafe {sp.offset_from(mark)};
    if items != 1 {
        let msg = "input";
        unsafe {Perl_croak_xs_usage(cv, msg.as_bytes().as_ptr() as *const i8)};
    }

    // int     input = (int)SvIV(ST(0))
    let src = unsafe {*my_perl.Istack_base.add((ax + 0) as usize)};
    let input = SvIV(my_perl, src);

    let RETVAL = (input % 2) == 0;
    println!("input {} RETVAL {}", input, RETVAL);

    let targ = unsafe {Perl_sv_newmortal(my_perl)};
    
    // PUSHi((IV)RETVAL);
    unsafe {
        Perl_sv_setiv(my_perl, targ, RETVAL as i64)
    };

    unsafe {*sp = targ};

    let off = 1;
    my_perl.Istack_sp = unsafe {my_perl.Istack_base.add((ax + (off - 1)).try_into().unwrap())};

    return ()

}

pub fn SvFLAGS(sv: *const libperl_sys::sv) -> u32 {
    assert_ne!(sv, std::ptr::null_mut());
    unsafe {(*sv).sv_flags}
}

fn SvIV(my_perl: &mut PerlInterpreter, sv: *mut libperl_sys::sv) -> i64 {
    if (SvFLAGS(sv) & (SVf_IOK|SVs_GMG)) == SVf_IOK {
        let xpviv = (unsafe {(*sv).sv_any}) as *const libperl_sys::xpviv;
        return (unsafe {(*xpviv).xiv_u.xivu_iv}) as i64
    }

    let perl: *mut PerlInterpreter = my_perl;

    return unsafe {Perl_sv_2iv(perl, sv)}
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
