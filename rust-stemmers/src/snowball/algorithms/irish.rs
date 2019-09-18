//! This file was generated automatically by the Snowball to Rust compiler
//! https://snowballstem.org/

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use snowball::SnowballEnv;
use snowball::Among;

static A_0: &'static [Among<Context>; 24] = &[
    Among("b'", -1, 1, None),
    Among("bh", -1, 4, None),
    Among("bhf", 1, 2, None),
    Among("bp", -1, 8, None),
    Among("ch", -1, 5, None),
    Among("d'", -1, 1, None),
    Among("d'fh", 5, 2, None),
    Among("dh", -1, 6, None),
    Among("dt", -1, 9, None),
    Among("fh", -1, 2, None),
    Among("gc", -1, 5, None),
    Among("gh", -1, 7, None),
    Among("h-", -1, 1, None),
    Among("m'", -1, 1, None),
    Among("mb", -1, 4, None),
    Among("mh", -1, 10, None),
    Among("n-", -1, 1, None),
    Among("nd", -1, 6, None),
    Among("ng", -1, 7, None),
    Among("ph", -1, 8, None),
    Among("sh", -1, 3, None),
    Among("t-", -1, 1, None),
    Among("th", -1, 9, None),
    Among("ts", -1, 3, None),
];

static A_1: &'static [Among<Context>; 16] = &[
    Among("\u{00ED}ochta", -1, 1, None),
    Among("a\u{00ED}ochta", 0, 1, None),
    Among("ire", -1, 2, None),
    Among("aire", 2, 2, None),
    Among("abh", -1, 1, None),
    Among("eabh", 4, 1, None),
    Among("ibh", -1, 1, None),
    Among("aibh", 6, 1, None),
    Among("amh", -1, 1, None),
    Among("eamh", 8, 1, None),
    Among("imh", -1, 1, None),
    Among("aimh", 10, 1, None),
    Among("\u{00ED}ocht", -1, 1, None),
    Among("a\u{00ED}ocht", 12, 1, None),
    Among("ir\u{00ED}", -1, 2, None),
    Among("air\u{00ED}", 14, 2, None),
];

static A_2: &'static [Among<Context>; 25] = &[
    Among("\u{00F3}ideacha", -1, 6, None),
    Among("patacha", -1, 5, None),
    Among("achta", -1, 1, None),
    Among("arcachta", 2, 2, None),
    Among("eachta", 2, 1, None),
    Among("grafa\u{00ED}ochta", -1, 4, None),
    Among("paite", -1, 5, None),
    Among("ach", -1, 1, None),
    Among("each", 7, 1, None),
    Among("\u{00F3}ideach", 8, 6, None),
    Among("gineach", 8, 3, None),
    Among("patach", 7, 5, None),
    Among("grafa\u{00ED}och", -1, 4, None),
    Among("pataigh", -1, 5, None),
    Among("\u{00F3}idigh", -1, 6, None),
    Among("acht\u{00FA}il", -1, 1, None),
    Among("eacht\u{00FA}il", 15, 1, None),
    Among("gineas", -1, 3, None),
    Among("ginis", -1, 3, None),
    Among("acht", -1, 1, None),
    Among("arcacht", 19, 2, None),
    Among("eacht", 19, 1, None),
    Among("grafa\u{00ED}ocht", -1, 4, None),
    Among("arcachta\u{00ED}", -1, 2, None),
    Among("grafa\u{00ED}ochta\u{00ED}", -1, 4, None),
];

static A_3: &'static [Among<Context>; 12] = &[
    Among("imid", -1, 1, None),
    Among("aimid", 0, 1, None),
    Among("\u{00ED}mid", -1, 1, None),
    Among("a\u{00ED}mid", 2, 1, None),
    Among("adh", -1, 2, None),
    Among("eadh", 4, 2, None),
    Among("faidh", -1, 1, None),
    Among("fidh", -1, 1, None),
    Among("\u{00E1}il", -1, 2, None),
    Among("ain", -1, 2, None),
    Among("tear", -1, 2, None),
    Among("tar", -1, 2, None),
];

static G_v: &'static [u8; 20] = &[17, 65, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 17, 4, 2];

#[derive(Clone)]
struct Context {
    i_p2: usize,
    i_p1: usize,
    i_pV: usize,
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 28
    context.i_pV = env.limit;
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    // do, line 34
    let v_1 = env.cursor;
    'lab0: loop {
        // (, line 34
        // gopast, line 35
        'golab1: loop {
            'lab2: loop {
                if !env.in_grouping(G_v, 97, 250) {
                    break 'lab2;
                }
                break 'golab1;
            }
            if env.cursor >= env.limit {
                break 'lab0;
            }
            env.next_char();
        }
        // setmark pV, line 35
        context.i_pV = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    // do, line 37
    let v_3 = env.cursor;
    'lab3: loop {
        // (, line 37
        // gopast, line 38
        'golab4: loop {
            'lab5: loop {
                if !env.in_grouping(G_v, 97, 250) {
                    break 'lab5;
                }
                break 'golab4;
            }
            if env.cursor >= env.limit {
                break 'lab3;
            }
            env.next_char();
        }
        // gopast, line 38
        'golab6: loop {
            'lab7: loop {
                if !env.out_grouping(G_v, 97, 250) {
                    break 'lab7;
                }
                break 'golab6;
            }
            if env.cursor >= env.limit {
                break 'lab3;
            }
            env.next_char();
        }
        // setmark p1, line 38
        context.i_p1 = env.cursor;
        // gopast, line 39
        'golab8: loop {
            'lab9: loop {
                if !env.in_grouping(G_v, 97, 250) {
                    break 'lab9;
                }
                break 'golab8;
            }
            if env.cursor >= env.limit {
                break 'lab3;
            }
            env.next_char();
        }
        // gopast, line 39
        'golab10: loop {
            'lab11: loop {
                if !env.out_grouping(G_v, 97, 250) {
                    break 'lab11;
                }
                break 'golab10;
            }
            if env.cursor >= env.limit {
                break 'lab3;
            }
            env.next_char();
        }
        // setmark p2, line 39
        context.i_p2 = env.cursor;
        break 'lab3;
    }
    env.cursor = v_3;
    return true;
}

fn r_initial_morph(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // (, line 43
    // [, line 44
    env.bra = env.cursor;
    // substring, line 44
    among_var = env.find_among(A_0, context);
    if among_var == 0 {
        return false;
    }
    // ], line 44
    env.ket = env.cursor;
    if among_var == 1 {
        // (, line 46
        // delete, line 46
        if !env.slice_del() {
            return false;
        }
    } else if among_var == 2 {
        // (, line 52
        // <-, line 52
        if !env.slice_from("f") {
            return false;
        }
    } else if among_var == 3 {
        // (, line 58
        // <-, line 58
        if !env.slice_from("s") {
            return false;
        }
    } else if among_var == 4 {
        // (, line 61
        // <-, line 61
        if !env.slice_from("b") {
            return false;
        }
    } else if among_var == 5 {
        // (, line 63
        // <-, line 63
        if !env.slice_from("c") {
            return false;
        }
    } else if among_var == 6 {
        // (, line 65
        // <-, line 65
        if !env.slice_from("d") {
            return false;
        }
    } else if among_var == 7 {
        // (, line 69
        // <-, line 69
        if !env.slice_from("g") {
            return false;
        }
    } else if among_var == 8 {
        // (, line 71
        // <-, line 71
        if !env.slice_from("p") {
            return false;
        }
    } else if among_var == 9 {
        // (, line 75
        // <-, line 75
        if !env.slice_from("t") {
            return false;
        }
    } else if among_var == 10 {
        // (, line 89
        // <-, line 89
        if !env.slice_from("m") {
            return false;
        }
    }
    return true;
}

fn r_RV(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !(context.i_pV <= env.cursor){
        return false;
    }
    return true;
}

fn r_R1(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !(context.i_p1 <= env.cursor){
        return false;
    }
    return true;
}

fn r_R2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if !(context.i_p2 <= env.cursor){
        return false;
    }
    return true;
}

fn r_noun_sfx(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // (, line 103
    // [, line 104
    env.ket = env.cursor;
    // substring, line 104
    among_var = env.find_among_b(A_1, context);
    if among_var == 0 {
        return false;
    }
    // ], line 104
    env.bra = env.cursor;
    if among_var == 1 {
        // (, line 108
        // call R1, line 108
        if !r_R1(env, context) {
            return false;
        }
        // delete, line 108
        if !env.slice_del() {
            return false;
        }
    } else if among_var == 2 {
        // (, line 110
        // call R2, line 110
        if !r_R2(env, context) {
            return false;
        }
        // delete, line 110
        if !env.slice_del() {
            return false;
        }
    }
    return true;
}

fn r_deriv(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // (, line 113
    // [, line 114
    env.ket = env.cursor;
    // substring, line 114
    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    // ], line 114
    env.bra = env.cursor;
    if among_var == 1 {
        // (, line 116
        // call R2, line 116
        if !r_R2(env, context) {
            return false;
        }
        // delete, line 116
        if !env.slice_del() {
            return false;
        }
    } else if among_var == 2 {
        // (, line 118
        // <-, line 118
        if !env.slice_from("arc") {
            return false;
        }
    } else if among_var == 3 {
        // (, line 120
        // <-, line 120
        if !env.slice_from("gin") {
            return false;
        }
    } else if among_var == 4 {
        // (, line 122
        // <-, line 122
        if !env.slice_from("graf") {
            return false;
        }
    } else if among_var == 5 {
        // (, line 124
        // <-, line 124
        if !env.slice_from("paite") {
            return false;
        }
    } else if among_var == 6 {
        // (, line 126
        // <-, line 126
        if !env.slice_from("\u{00F3}id") {
            return false;
        }
    }
    return true;
}

fn r_verb_sfx(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // (, line 129
    // [, line 130
    env.ket = env.cursor;
    // substring, line 130
    among_var = env.find_among_b(A_3, context);
    if among_var == 0 {
        return false;
    }
    // ], line 130
    env.bra = env.cursor;
    if among_var == 1 {
        // (, line 133
        // call RV, line 133
        if !r_RV(env, context) {
            return false;
        }
        // delete, line 133
        if !env.slice_del() {
            return false;
        }
    } else if among_var == 2 {
        // (, line 138
        // call R1, line 138
        if !r_R1(env, context) {
            return false;
        }
        // delete, line 138
        if !env.slice_del() {
            return false;
        }
    }
    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_p2: 0,
        i_p1: 0,
        i_pV: 0,
    };
    // (, line 143
    // do, line 144
    let v_1 = env.cursor;
    // call initial_morph, line 144
    r_initial_morph(env, context);
    env.cursor = v_1;
    // do, line 145
    // call mark_regions, line 145
    r_mark_regions(env, context);
    // backwards, line 146
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    // (, line 146
    // do, line 147
    let v_3 = env.limit - env.cursor;
    // call noun_sfx, line 147
    r_noun_sfx(env, context);
    env.cursor = env.limit - v_3;
    // do, line 148
    let v_4 = env.limit - env.cursor;
    // call deriv, line 148
    r_deriv(env, context);
    env.cursor = env.limit - v_4;
    // do, line 149
    let v_5 = env.limit - env.cursor;
    // call verb_sfx, line 149
    r_verb_sfx(env, context);
    env.cursor = env.limit - v_5;
    env.cursor = env.limit_backward;
    return true;
}
