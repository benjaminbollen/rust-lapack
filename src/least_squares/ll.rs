// Copyright 2014 Michael Yang. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use libc::{
    c_char,
    c_int,
};

use types::{
    CLPK_complex,
    CLPK_doublecomplex,
    CLPK_doublereal,
    CLPK_integer,
    CLPK_real,
};

#[link(name = "lapack")]
extern "C" {
    pub fn sgels_(trans: c_char, m: CLPK_integer, n: *const CLPK_integer,
            nrhs: *const CLPK_integer, a: *mut CLPK_real, lda: *const CLPK_integer,
            b: *mut CLPK_real, ldb: *const CLPK_integer, work: CLPK_real,
            lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn dgels_(trans: c_char, m: CLPK_integer, n: *const CLPK_integer,
            nrhs: *const CLPK_integer, a: *mut CLPK_doublereal, lda: *const CLPK_integer,
            b: *mut CLPK_doublereal, ldb: *const CLPK_integer,
            work: CLPK_doublereal, lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn cgels_(trans: c_char, m: CLPK_integer, n: *const CLPK_integer,
            nrhs: *const CLPK_integer, a: *mut CLPK_complex, lda: *const CLPK_integer,
            b: *mut CLPK_complex, ldb: *const CLPK_integer, work: CLPK_complex,
            lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
    pub fn zgels_(trans: c_char, m: CLPK_integer, n: *const CLPK_integer,
            nrhs: *const CLPK_integer, a: *mut CLPK_doublecomplex,
            lda: *const CLPK_integer, b: *mut CLPK_doublecomplex, ldb: *const CLPK_integer,
            work: CLPK_doublecomplex, lwork: CLPK_integer,
            info: *mut CLPK_integer) -> c_int;
}