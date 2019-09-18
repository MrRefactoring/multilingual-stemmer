//! This file was generated automatically by the Snowball to Rust compiler
//! https://snowballstem.org/

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use snowball::SnowballEnv;
use snowball::Among;

static A_0: &'static [Among<Context>; 32] = &[
    Among("hed", -1, 1, None),
    Among("ethed", 0, 1, None),
    Among("ered", -1, 1, None),
    Among("e", -1, 1, None),
    Among("erede", 3, 1, None),
    Among("ende", 3, 1, None),
    Among("erende", 5, 1, None),
    Among("ene", 3, 1, None),
    Among("erne", 3, 1, None),
    Among("ere", 3, 1, None),
    Among("en", -1, 1, None),
    Among("heden", 10, 1, None),
    Among("eren", 10, 1, None),
    Among("er", -1, 1, None),
    Among("heder", 13, 1, None),
    Among("erer", 13, 1, None),
    Among("s", -1, 2, None),
    Among("heds", 16, 1, None),
    Among("es", 16, 1, None),
    Among("endes", 18, 1, None),
    Among("erendes", 19, 1, None),
    Among("enes", 18, 1, None),
    Among("ernes", 18, 1, None),
    Among("eres", 18, 1, None),
    Among("ens", 16, 1, None),
    Among("hedens", 24, 1, None),
    Among("erens", 24, 1, None),
    Among("ers", 16, 1, None),
    Among("ets", 16, 1, None),
    Among("erets", 28, 1, None),
    Among("et", -1, 1, None),
    Among("eret", 30, 1, None),
];

static A_1: &'static [Among<Context>; 4] = &[
    Among("gd", -1, -1, None),
    Among("dt", -1, -1, None),
    Among("gt", -1, -1, None),
    Among("kt", -1, -1, None),
];

static A_2: &'static [Among<Context>; 5] = &[
    Among("ig", -1, 1, None),
    Among("lig", 0, 1, None),
    Among("elig", 1, 1, None),
    Among("els", -1, 1, None),
    Among("l\u{00F8}st", -1, 2, None),
];

static G_c: &'static [u8; 4] = &[119, 223, 119, 1];

static G_v: &'static [u8; 19] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 128];

static G_s_ending: &'static [u8; 17] = &[239, 254, 42, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16];

#[derive(Clone)]
struct Context {
    i_x: usize,
    i_p1: usize,
    S_ch: String,
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 31
    context.i_p1 = env.limit;
    // test, line 35
    let v_1 = env.cursor;
    // (, line 35
    // hop, line 35
    let c = env.byte_index_for_hop(3);
    if 0 as i32 > c || c > env.limit as i32 {
        return false;
    }
    env.cursor = c as usize;
    // setmark x, line 35
    context.i_x = env.cursor;
    env.cursor = v_1;
    // goto, line 36
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
    // gopast, line 36
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
    // setmark p1, line 36
    context.i_p1 = env.cursor;
    // try, line 37
    'lab4: loop {
        // (, line 37
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
    // (, line 42
    // setlimit, line 43
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_2 = env.limit_backward;
    env.limit_backward = context.i_p1;
    // (, line 43
    // [, line 43
    env.ket = env.cursor;
    // substring, line 43
    among_var = env.find_among_b(A_0, context);
    if among_var == 0 {
        env.limit_backward = v_2;
        return false;
    }
    // ], line 43
    env.bra = env.cursor;
    env.limit_backward = v_2;
    if among_var == 1 {
        // (, line 50
        // delete, line 50
        if !env.slice_del() {
            return false;
        }
    } else if among_var == 2 {
        // (, line 52
        if !env.in_grouping_b(G_s_ending, 97, 229) {
            return false;
        }
        // delete, line 52
        if !env.slice_del() {
            return false;
        }
    }
    return true;
}

fn r_consonant_pair(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 56
    // test, line 57
    let v_1 = env.limit - env.cursor;
    // (, line 57
    // setlimit, line 58
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_3 = env.limit_backward;
    env.limit_backward = context.i_p1;
    // (, line 58
    // [, line 58
    env.ket = env.cursor;
    // substring, line 58
    if env.find_among_b(A_1, context) == 0 {
        env.limit_backward = v_3;
        return false;
    }
    // ], line 58
    env.bra = env.cursor;
    env.limit_backward = v_3;
    env.cursor = env.limit - v_1;
    // next, line 64
    if env.cursor <= env.limit_backward {
        return false;
    }
    env.previous_char();
    // ], line 64
    env.bra = env.cursor;
    // delete, line 64
    if !env.slice_del() {
        return false;
    }
    return true;
}

fn r_other_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // (, line 67
    // do, line 68
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        // (, line 68
        // [, line 68
        env.ket = env.cursor;
        // literal, line 68
        if !env.eq_s_b(&"st") {
            break 'lab0;
        }
        // ], line 68
        env.bra = env.cursor;
        // literal, line 68
        if !env.eq_s_b(&"ig") {
            break 'lab0;
        }
        // delete, line 68
        if !env.slice_del() {
            return false;
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    // setlimit, line 69
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_3 = env.limit_backward;
    env.limit_backward = context.i_p1;
    // (, line 69
    // [, line 69
    env.ket = env.cursor;
    // substring, line 69
    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        env.limit_backward = v_3;
        return false;
    }
    // ], line 69
    env.bra = env.cursor;
    env.limit_backward = v_3;
    if among_var == 1 {
        // (, line 72
        // delete, line 72
        if !env.slice_del() {
            return false;
        }
        // do, line 72
        let v_4 = env.limit - env.cursor;
        // call consonant_pair, line 72
        r_consonant_pair(env, context);
        env.cursor = env.limit - v_4;
    } else if among_var == 2 {
        // (, line 74
        // <-, line 74
        if !env.slice_from("l\u{00F8}s") {
            return false;
        }
    }
    return true;
}

fn r_undouble(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 77
    // setlimit, line 78
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_2 = env.limit_backward;
    env.limit_backward = context.i_p1;
    // (, line 78
    // [, line 78
    env.ket = env.cursor;
    if !env.in_grouping_b(G_c, 98, 122) {
        env.limit_backward = v_2;
        return false;
    }
    // ], line 78
    env.bra = env.cursor;
    // -> ch, line 78
    context.S_ch = env.slice_to();
    if context.S_ch.is_empty() {
        return false;
    }
    env.limit_backward = v_2;
    // name ch, line 79
    if !env.eq_s_b(&context.S_ch) {
        return false;
    }
    // delete, line 80
    if !env.slice_del() {
        return false;
    }
    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_x: 0,
        i_p1: 0,
        S_ch: String::new(),
    };
    // (, line 84
    // do, line 86
    let v_1 = env.cursor;
    // call mark_regions, line 86
    r_mark_regions(env, context);
    env.cursor = v_1;
    // backwards, line 87
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    // (, line 87
    // do, line 88
    let v_2 = env.limit - env.cursor;
    // call main_suffix, line 88
    r_main_suffix(env, context);
    env.cursor = env.limit - v_2;
    // do, line 89
    let v_3 = env.limit - env.cursor;
    // call consonant_pair, line 89
    r_consonant_pair(env, context);
    env.cursor = env.limit - v_3;
    // do, line 90
    let v_4 = env.limit - env.cursor;
    // call other_suffix, line 90
    r_other_suffix(env, context);
    env.cursor = env.limit - v_4;
    // do, line 91
    let v_5 = env.limit - env.cursor;
    // call undouble, line 91
    r_undouble(env, context);
    env.cursor = env.limit - v_5;
    env.cursor = env.limit_backward;
    return true;
}
