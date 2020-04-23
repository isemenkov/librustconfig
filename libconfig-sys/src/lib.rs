/******************************************************************************/
/*                               libRustConfig                                */
/*                   rust wrapper around libconfig library                    */
/*                  https://github.com/hyperrealm/libconfig                   */
/*                                                                            */
/* Copyright (c) 2020                                       Ivan Semenkov     */
/* https://github.com/isemenkov/librustconfig               ivan@semenkov.pro */
/*                                                          Ukraine           */
/******************************************************************************/
/*                                                                            */
/* Permission is hereby granted,  free of charge,  to any person obtaining a  */
/* copy of this software and associated documentation files (the "Software"), */
/* to deal in the Software without restriction, including without limitation  */
/* the rights to use, copy,  modify, merge, publish, distribute,  sublicense, */
/* and/or  sell copies  of the Software,  and to permit persons  to whom  the */
/* Software  is furnished to  do  so,  subject to  the following  conditions: */
/*                                                                            */
/* The above copyright notice and this permission notice shall be included in */
/* all copies or substantial portions of the Software.                        */
/*                                                                            */
/* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR */
/* IMPLIED,  INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF  MERCHANTABILITY, */
/* FITNESS  FOR A PARTICULAR PURPOSE  AND NONINFRINGEMENT. IN  NO EVENT SHALL */
/* THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER */
/* LIABILITY,  WHETHER IN AN ACTION  OF CONTRACT,  TORT OR OTHERWISE, ARISING */
/* FROM,  OUT OF  OR IN  CONNECTION WITH  THE SOFTWARE  OR THE  USE OR  OTHER */
/* DEALINGS IN THE SOFTWARE.                                                  */
/*                                                                            */
/******************************************************************************/

#![allow(non_camel_case_types, unused_extern_crates)]

extern crate libc;

use libc::{c_schar, c_short, c_ushort, c_int, c_uint, c_longlong, c_double};
use libc::FILE;
use std::os::raw::c_void;

pub const CONFIG_TYPE_NONE : usize                                      = 0;
pub const CONFIG_TYPE_GROUP : usize                                     = 1;
pub const CONFIG_TYPE_INT : usize                                       = 2;
pub const CONFIG_TYPE_INT64 : usize                                     = 3;
pub const CONFIG_TYPE_FLOAT : usize                                     = 4;
pub const CONFIG_TYPE_STRING : usize                                    = 5;
pub const CONFIG_TYPE_BOOL : usize                                      = 6;
pub const CONFIG_TYPE_ARRAY : usize                                     = 7;
pub const CONFIG_TYPE_LIST : usize                                      = 8;

pub const CONFIG_FORMAT_DEFAULT : usize                                 = 1;
pub const CONFIG_FORMAT_HEX : usize                                     = 2;

pub const CONFIG_OPTION_AUTOCONVERT : usize                             = 0x01;
pub const CONFIG_OPTION_SEMICOLON_SEPARATORS : usize                    = 0x02;
pub const CONFIG_OPTION_COLON_ASSIGNMENT_FOR_GROUPS : usize             = 0x04;
pub const CONFIG_OPTION_COLON_ASSIGNMENT_FOR_NON_GROUPS : usize         = 0x08;
pub const CONFIG_OPTION_OPEN_BRACE_ON_SEPARATE_LINE : usize             = 0x10;

pub const CONFIG_TRUE : usize                                           = 1;
pub const CONFIG_FALSE : usize                                          = 0;

#[derive(Copy, Clone)]
pub enum config_error_t {
    CONFIG_ERR_NONE                                                     = 0,
    CONFIG_ERR_FILE_IO                                                  = 1,
    CONFIG_ERR_PARSE                                                    = 2,
}

#[repr(C)]
pub struct config_value_t {
    pub ival : c_int,
    pub llval : c_longlong,
    pub fval : c_double,
    pub sval : *mut c_schar,
    pub list : *mut config_list_t,
}

#[repr(C)]
pub struct config_setting_t {
    pub name : *mut c_schar,
    pub setting_type : c_short,
    pub format : c_short,
    pub value : config_value_t,
    pub parent : *mut config_setting_t,
    pub config : *mut config_t,
    pub hook : *mut c_void,
    pub line : c_uint,
    pub file : *const c_schar,
}

#[repr(C)]
pub struct config_list_t {
    pub length : c_uint,
    pub elements : *mut *mut config_setting_t,
}

#[repr(C)]
pub struct config_t {
    pub root : *mut config_setting_t,
    pub destructor : Option<extern "C" fn(*mut c_void) -> ()>,
    pub tab_width : c_ushort,
    pub default_format : c_short,
    pub include_dir : *const c_schar,
    pub error_text : *const c_schar,
    pub error_file : *const c_schar,
    pub error_line : c_int,
    pub error_type : config_error_t,
    pub filenames : *mut *mut c_schar,
    pub num_filenames : c_uint,
}

pub type destructor_callback = extern "C" fn(ptr : *mut c_void) -> ();

#[link(name = "config")]
extern "C" {
    pub fn config_read (config : *mut config_t, stream : *mut FILE) -> c_int;
    pub fn config_write (config : *const config_t, stream : *mut FILE) -> ();

    pub fn config_set_default_format (config : *mut config_t, format :
        c_short) -> ();

    pub fn config_set_options (config : *mut config_t, options : c_int) -> ();
    pub fn config_get_options (config : *const config_t) -> c_int;

    pub fn config_set_auto_convert (config : *mut config_t, flag : c_int) -> ();
    pub fn config_get_auto_convert (config : *const config_t) -> c_int;

    pub fn config_read_string (config : *mut config_t, str : *const c_schar)
        -> c_int;

    pub fn config_read_file (config : *mut config_t, filename : *const c_schar)
        -> c_int;
    pub fn config_write_file (config : *mut config_t, filename : *const c_schar)
        -> c_int;

    pub fn config_set_destructor (config : *mut config, destructor :
        destructor_callback) -> ();
    pub fn config_set_include_dir (config : *mut config_t, include_dir :
        *const c_schar) -> ();

    pub fn config_init (config : *mut config_t) -> ();
    pub fn config_destroy (config : *mut config_t) -> ();

    pub fn config_setting_get_int (setting : *const config_setting_t) -> c_int;
    pub fn config_setting_get_int64 (setting : *const config_setting_t)
        -> c_longlong;
    pub fn config_setting_get_float (setting : *const config_setting_t)
        -> c_double;
    pub fn config_setting_get_bool (setting : *const config_setting_t)
        -> c_int;
    pub fn config_setting_get_string (setting : *const config_setting_t)
        -> *const c_schar;

    pub fn config_setting_lookup_int (setting : *const config_setting_t,
        name : *const c_schar, value : *mut c_int) -> c_int;
    pub fn config_setting_lookup_int64 (setting : *const config_setting_t,
        name : *const c_schar, value : *mut c_longlong) -> c_int;
    pub fn config_setting_lookup_float (setting : *const config_setting_t,
        name : *const c_schar, value : *mut c_double) -> c_int;
    pub fn config_setting_lookup_bool (setting : *const config_setting_t,
        name : *const c_schar, value : *mut c_int) -> c_int;
    pub fn config_setting_lookup_string (setting : *const config_setting_t,
        name : *const c_schar, value : *const *mut c_schar) -> c_int;

    pub fn config_setting_set_int (setting : *mut config_setting_t, value :
        c_int) -> c_int;
    pub fn config_setting_set_int64 (setting : *mut config_setting_t, value :
        c_longlong) -> c_int;
    pub fn config_setting_set_float (setting : *mut config_setting_t, value :
        c_double) -> c_int;
    pub fn config_setting_set_bool (setting : *mut config_setting_t, value :
        c_int) -> c_int;
    pub fn config_setting_set_string (setting : *mut config_setting_t, value :
        *const c_schar) -> c_int;

    pub fn config_setting_set_format (setting : *mut config_setting_t, format :
        c_short) -> c_int;
    pub fn config_setting_get_format (setting : *const config_setting_t)
        -> c_short;

    pub fn config_setting_get_int_elem (setting : *const config_setting_t, idx :
        c_int) -> c_int;
    pub fn config_setting_get_int64_elem (setting : *const config_setting_t,
        idx : c_int) -> c_longlong;
    pub fn config_setting_get_float_elem (setting : *const config_setting_t,
        idx : c_int) -> c_double;
    pub fn config_setting_get_bool_elem (setting : *const config_setting_t,
        idx : c_int) -> c_int;
    pub fn config_setting_get_string_elem (setting : *const config_setting_t,
        idx : c_int) -> *const c_schar;

    pub fn config_setting_set_int_elem (setting : *mut config_setting_t, idx :
        c_int, value : c_int) -> *mut config_setting_t;
    pub fn config_setting_set_int64_elem (setting : *mut config_setting_t, idx :
        c_int, value : c_longlong) -> *mut config_setting_t;
    pub fn config_setting_set_float_elem (setting : *mut config_setting_t, idx :
        c_int, value : c_double) -> *mut config_setting_t;
    pub fn config_setting_set_bool_elem (setting : *mut config_setting_t, idx :
        c_int, value : c_int) -> *mut config_setting_t;
    pub fn config_setting_set_string_elem (setting : *mut config_setting_t,
        idx : c_int, value : *const c_schar) -> *mut config_setting_t;

    pub fn config_setting_index (setting : *const config_setting_t) -> c_int;

    pub fn config_setting_length (setting : *const config_setting_t) -> c_int;
    pub fn config_setting_get_elem (setting : *const config_setting_t, idx :
        c_uint) -> *mut config_setting_t;

    pub fn config_setting_get_member (setting : *const config_setting_t, name :
        *const c_schar) -> *mut config_setting_t;

    pub fn config_setting_add (parent : *mut config_setting_t, name :
        *const c_schar, value_type : c_int) -> *mut config_setting_t;
    pub fn config_setting_remove (parent : *mut config_setting_t, name :
        *const c_schar) -> c_int;
    pub fn config_setting_remove_elem (parent : *mut config_setting_t, idx :
        c_uint) -> c_int;
    pub fn config_setting_set_hook (setting : *mut config_setting_t, hook :
        *mut c_void) -> ();

    pub fn config_lookup (config : *const config_t, path : *const c_schar)
        -> *mut config_setting_t;
    pub fn config_setting_lookup (setting : *mut config_setting_t, path :
        *const c_schar) -> *mut config_setting_t;

    pub fn config_lookup_int (config : *const config_t, path : *const c_schar,
        value : *mut c_int) -> c_int;
    pub fn config_lookup_int64 (config : *const config_t, path : *const c_schar,
        value : *mut c_longlong) -> c_int;
    pub fn config_lookup_float (config : *const config_t, path : *const c_schar,
        value : *mut c_double) -> c_int;
    pub fn config_lookup_bool (config : *const config_t, path : *const c_schar,
        value : *mut c_int) -> c_int;
    pub fn config_lookup_string (config : *const config_t, path :
        *const c_schar, value : *const *mut c_schar) -> c_int;
}

pub fn config_get_include_dir (config : *const config_t) -> *const c_schar {
    config.include_dir
}

pub fn config_setting_type (setting : *const config_setting_t) -> c_int {
    setting.setting_type as c_int
}

pub fn config_setting_is_group (setting : *const config_setting_t) -> c_int {
    match setting.setting_type {
        CONFIG_TYPE_GROUP   => { CONFIG_TRUE  },
        _                   => { CONFIG_FALSE },
    } as c_int
}

pub fn config_setting_is_array (setting : *const config_setting_t) -> c_int {
    match setting.setting_type {
        CONFIG_TYPE_ARRAY   => { CONFIG_TRUE  },
        _                   => { CONFIG_FALSE },
    } as c_int
}

pub fn config_setting_is_list (setting : *const config_setting_t) -> c_int {
    match setting.setting_type {
        CONFIG_TYPE_LIST    => { CONFIG_TRUE  },
        _                   => { CONFIG_FALSE },
    } as c_int
}

pub fn config_setting_is_aggregate (setting : *const config_setting_t)
    -> c_int {
    match setting.setting_type {
        CONFIG_TYPE_GROUP   |
        CONFIG_TYPE_LIST    |
        CONFIG_TYPE_ARRAY   => { CONFIG_TRUE  },
        _                   => { CONFIG_FALSE }
    } as c_int
}

pub fn config_setting_is_number (setting : *const config_setting_t) -> c_int {
    match setting.setting_type {
        CONFIG_TYPE_INT     |
        CONFIG_TYPE_INT64   |
        CONFIG_TYPE_FLOAT   => { CONFIG_TRUE  },
        _                   => { CONFIG_FALSE },
    } as c_int
}

pub fn config_setting_is_scalar (setting : *const config_setting_t) -> c_int {
    match setting.setting_type {
        CONFIG_TYPE_BOOL    |
        CONFIG_TYPE_STRING  |
        config_setting_is_number(setting) => { CONFIG_TRUE },
        _                   => { CONFIG_FALSE },
    } as c_int
}

pub fn config_setting_name (setting : *const config_setting_t)
    -> *const c_schar {
    setting.name
}

pub fn config_setting_parent (setting : *const config_setting_t)
    -> *mut config_setting_t {
    setting.parent
}

pub fn config_setting_is_root (setting : *const config_setting_t) -> c_int {
    if setting.parent.is_null() {
        CONFIG_TRUE as c_int
    } else {
        CONFIG_FALSE as c_int
    }
}

pub fn config_root_setting (config : *const config_t) -> *mut config_setting_t {
    config.root
}

pub fn config_set_default_format (config : *mut config_t, value : c_short)
    -> () {
    config.default_format = value;
}

pub fn config_get_default_format (config : *const config_t) -> c_short {
    config.default_format
}

pub fn config_set_tab_width (config : *mut config_t, value : c_ushort) -> () {
    config.tab_width = (value & 0x0F);
}

pub fn config_get_tab_width (config : *const config_t) -> c_ushort {
    config.tab_width
}

pub fn config_setting_source_line (config : *const config_setting_t)
    -> c_uint {
    config.line
}

pub fn config_setting_source_file (config : *const config_setting_t)
    -> *const c_schar {
    config.file
}

pub fn config_error_text (config : *const config_t) -> *const c_schar {
    config.error_text
}

pub fn config_error_file (config : *const config_t) -> *const c_schar {
    config.error_file
}

pub fn config_error_line (config : *const config_t) -> c_int {
    config.error_line
}

pub fn config_error_type (config : *const config_t) -> config_error_t {
    config.error_type
}