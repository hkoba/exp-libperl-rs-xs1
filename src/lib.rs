use std::ffi::CStr;

use libperl_sys::*;
use libperl_sys::perl_core::PerlInterpreter;


#[unsafe(no_mangle)]
pub extern "C" fn boot_Mytest(my_perl_ptr: *mut PerlInterpreter, _cv: *mut CV) -> () {

    if my_perl_ptr.is_null() {
        panic!()
    }

    static NAME1: &CStr = unsafe {
        CStr::from_bytes_with_nul_unchecked(b"Mytest::mySum\0")
    };
    unsafe {
        Perl_newXS_deffile(my_perl_ptr, NAME1.as_ptr(), Some(mySum_C))
    };

    unsafe {
        Perl_xs_boot_epilog(my_perl_ptr, 1);
    };
}


#[unsafe(no_mangle)]
pub extern "C" fn mySum_C(my_perl_ptr: *mut PerlInterpreter, cv: *mut CV) -> () {
    if let Some(my_perl) = unsafe {my_perl_ptr.as_mut()} {
        mySum(my_perl, cv)
    } else {
        
    }
}

fn mySum(my_perl: &mut PerlInterpreter, cv: *mut CV) -> () {
    // dSP
    let mut sp = my_perl.Istack_sp;

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
