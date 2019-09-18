//! This file was generated automatically by the Snowball to Rust compiler
//! https://snowballstem.org/

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use snowball::SnowballEnv;
use snowball::Among;

static A_0: &'static [Among<Context>; 29] = &[
    Among("a", -1, 1, None),
    Among("e", -1, 1, None),
    Among("ede", 1, 1, None),
    Among("ande", 1, 1, None),
    Among("ende", 1, 1, None),
    Among("ane", 1, 1, None),
    Among("ene", 1, 1, None),
    Among("hetene", 6, 1, None),
    Among("erte", 1, 3, None),
    Among("en", -1, 1, None),
    Among("heten", 9, 1, None),
    Among("ar", -1, 1, None),
    Among("er", -1, 1, None),
    Among("heter", 12, 1, None),
    Among("s", -1, 2, None),
    Among("as", 14, 1, None),
    Among("es", 14, 1, None),
    Among("edes", 16, 1, None),
    Among("endes", 16, 1, None),
    Among("enes", 16, 1, None),
    Among("hetenes", 19, 1, None),
    Among("ens", 14, 1, None),
    Among("hetens", 21, 1, None),
    Among("ers", 14, 1, None),
    Among("ets", 14, 1, None),
    Among("et", -1, 1, None),
    Among("het", 25, 1, None),
    Among("ert", -1, 3, None),
    Among("ast", -1, 1, None),
];

static A_1: &'static [Among<Context>; 2] = &[
    Among("dt", -1, -1, None),
    Among("vt", -1, -1, None),
];

static A_2: &'static [Among<Context>; 11] = &[
    Among("leg", -1, 1, None),
    Among("eleg", 0, 1, None),
    Among("ig", -1, 1, None),
    Among("eig", 2, 1, None),
    Among("lig", 2, 1, None),
    Among("elig", 4, 1, None),
    Among("els", -1, 1, None),
    Among("lov", -1, 1, None),
    Among("elov", 7, 1, None),
    Among("slov", 7, 1, None),
    Among("hetslov", 9, 1, None),
];

static G_v: &'static [u8; 19] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 128];

static G_s_ending: &'static [u8; 4] = &[119, 125, 149, 1];

#[derive(Clone)]
struct Context {
    i_x: usize,
    i_p1: usize,
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 26
    context.i_p1 = env.limit;
    // test, line 30
    let v_1 = env.cursor;
    // (, line 30
    // hop, line 30
    let c = env.byte_index_for_hop(3);
    if 0 as i32 > c || c > env.limit as i32 {
        return false;
    }
    env.cursor = c as usize;
    // setmark x, line 30
    context.i_x = env.cursor;
    env.cursor = v_1;
    // goto, line 31
    'golab0: loop {
        let v_2 = env.cursor;
        'lab1: loop {
            if !env.in_grouping(G_v, 97, 248) {
                break 'lab1;
            }
            env.cursor = v_2;
            break 'golab0;
        }
        env.cursor = v_2;
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    // gopast, line 31
    'golab2: loop {
        'lab3: loop {
            if !env.out_grouping(G_v, 97, 248) {
                break 'lab3;
            }
            break 'golab2;
        }
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    // setmark p1, line 31
    context.i_p1 = env.cursor;
    // try, line 32
    'lab4: loop {
        // (, line 32
        if !(context.i_p1 < context.i_x){
            break 'lab4;
        }
        context.i_p1 = context.i_x;
        break 'lab4;
    }
    return true;
}

fn r_main_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // (, line 37
    // setlimit, line 38
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_2 = env.limit_backward;
    env.limit_backward = context.i_p1;
    // (, line 38
    // [, line 38
    env.ket = env.cursor;
    // substring, line 38
    among_var = env.find_among_b(A_0, context);
    if among_var == 0 {
        env.limit_backward = v_2;
        return false;
    }
    // ], line 38
    env.bra = env.cursor;
    env.limit_backward = v_2;
    if among_var == 1 {
        // (, line 44
        // delete, line 44
        if !env.slice_del() {
            return false;
        }
    } else if among_var == 2 {
        // (, line 46
        // or, line 46
        'lab0: loop {
            let v_3 = env.limit - env.cursor;
            'lab1: loop {
                if !env.in_grouping_b(G_s_ending, 98, 122) {
                    break 'lab1;
                }
                break 'lab0;
            }
            env.cursor = env.limit - v_3;
            // (, line 46
            // literal, line 46
            if !env.eq_s_b(&"k") {
                return false;
            }
            if !env.out_grouping_b(G_v, 97, 248) {
                return false;
            }
            break 'lab0;
        }
        // delete, line 46
        if !env.slice_del() {
            return false;
        }
    } else if among_var == 3 {
        // (, line 48
        // <-, line 48
        if !env.slice_from("er") {
            return false;
        }
    }
    return true;
}

fn r_consonant_pair(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 52
    // test, line 53
    let v_1 = env.limit - env.cursor;
    // (, line 53
    // setlimit, line 54
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_3 = env.limit_backward;
    env.limit_backward = context.i_p1;
    // (, line 54
    // [, line 54
    env.ket = env.cursor;
    // substring, line 54
    if env.find_among_b(A_1, context) == 0 {
        env.limit_backward = v_3;
        return false;
    }
    // ], line 54
    env.bra = env.cursor;
    env.limit_backward = v_3;
    env.cursor = env.limit - v_1;
    // next, line 59
    if env.cursor <= env.limit_backward {
        return false;
    }
    env.previous_char();
    // ], line 59
    env.bra = env.cursor;
    // delete, line 59
    if !env.slice_del() {
        return false;
    }
    return true;
}

fn r_other_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 62
    // setlimit, line 63
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_2 = env.limit_backward;
    env.limit_backward = context.i_p1;
    // (, line 63
    // [, line 63
    env.ket = env.cursor;
    // substring, line 63
    if env.find_among_b(A_2, context) == 0 {
        env.limit_backward = v_2;
        return false;
    }
    // ], line 63
    env.bra = env.cursor;
    env.limit_backward = v_2;
    // (, line 67
    // delete, line 67
    if !env.slice_del() {
        return false;
    }
    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_x: 0,
        i_p1: 0,
    };
    // (, line 72
    // do, line 74
    let v_1 = env.cursor;
    // call mark_regions, line 74
    r_mark_regions(env, context);
    env.cursor = v_1;
    // backwards, line 75
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    // (, line 75
    // do, line 76
    let v_2 = env.limit - env.cursor;
    // call main_suffix, line 76
    r_main_suffix(env, context);
    env.cursor = env.limit - v_2;
    // do, line 77
    let v_3 = env.limit - env.cursor;
    // call consonant_pair, line 77
    r_consonant_pair(env, context);
    env.cursor = env.limit - v_3;
    // do, line 78
    let v_4 = env.limit - env.cursor;
    // call other_suffix, line 78
    r_other_suffix(env, context);
    env.cursor = env.limit - v_4;
    env.cursor = env.limit_backward;
    return true;
}
