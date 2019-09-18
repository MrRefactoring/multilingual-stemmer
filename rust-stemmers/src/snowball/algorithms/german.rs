//! This file was generated automatically by the Snowball to Rust compiler
//! https://snowballstem.org/

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use snowball::SnowballEnv;
use snowball::Among;

static A_0: &'static [Among<Context>; 6] = &[
    Among("", -1, 5, None),
    Among("U", 0, 2, None),
    Among("Y", 0, 1, None),
    Among("\u{00E4}", 0, 3, None),
    Among("\u{00F6}", 0, 4, None),
    Among("\u{00FC}", 0, 2, None),
];

static A_1: &'static [Among<Context>; 7] = &[
    Among("e", -1, 2, None),
    Among("em", -1, 1, None),
    Among("en", -1, 2, None),
    Among("ern", -1, 1, None),
    Among("er", -1, 1, None),
    Among("s", -1, 3, None),
    Among("es", 5, 2, None),
];

static A_2: &'static [Among<Context>; 4] = &[
    Among("en", -1, 1, None),
    Among("er", -1, 1, None),
    Among("st", -1, 2, None),
    Among("est", 2, 1, None),
];

static A_3: &'static [Among<Context>; 2] = &[
    Among("ig", -1, 1, None),
    Among("lich", -1, 1, None),
];

static A_4: &'static [Among<Context>; 8] = &[
    Among("end", -1, 1, None),
    Among("ig", -1, 2, None),
    Among("ung", -1, 1, None),
    Among("lich", -1, 3, None),
    Among("isch", -1, 2, None),
    Among("ik", -1, 2, None),
    Among("heit", -1, 3, None),
    Among("keit", -1, 4, None),
];

static G_v: &'static [u8; 20] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 32, 8];

static G_s_ending: &'static [u8; 3] = &[117, 30, 5];

static G_st_ending: &'static [u8; 3] = &[117, 30, 4];

#[derive(Clone)]
struct Context {
    i_x: usize,
    i_p2: usize,
    i_p1: usize,
}

fn r_prelude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 33
    // test, line 35
    let v_1 = env.cursor;
    // repeat, line 35
    'replab0: loop{
        let v_2 = env.cursor;
        'lab1: for _ in 0..1 {
            // (, line 35
            // or, line 38
            'lab2: loop {
                let v_3 = env.cursor;
                'lab3: loop {
                    // (, line 36
                    // [, line 37
                    env.bra = env.cursor;
                    // literal, line 37
                    if !env.eq_s(&"\u{00DF}") {
                        break 'lab3;
                    }
                    // ], line 37
                    env.ket = env.cursor;
                    // <-, line 37
                    if !env.slice_from("ss") {
                        return false;
                    }
                    break 'lab2;
                }
                env.cursor = v_3;
                // next, line 38
                if env.cursor >= env.limit {
                    break 'lab1;
                }
                env.next_char();
                break 'lab2;
            }
            continue 'replab0;
        }
        env.cursor = v_2;
        break 'replab0;
    }
    env.cursor = v_1;
    // repeat, line 41
    'replab4: loop{
        let v_4 = env.cursor;
        'lab5: for _ in 0..1 {
            // goto, line 41
            'golab6: loop {
                let v_5 = env.cursor;
                'lab7: loop {
                    // (, line 41
                    if !env.in_grouping(G_v, 97, 252) {
                        break 'lab7;
                    }
                    // [, line 42
                    env.bra = env.cursor;
                    // or, line 42
                    'lab8: loop {
                        let v_6 = env.cursor;
                        'lab9: loop {
                            // (, line 42
                            // literal, line 42
                            if !env.eq_s(&"u") {
                                break 'lab9;
                            }
                            // ], line 42
                            env.ket = env.cursor;
                            if !env.in_grouping(G_v, 97, 252) {
                                break 'lab9;
                            }
                            // <-, line 42
                            if !env.slice_from("U") {
                                return false;
                            }
                            break 'lab8;
                        }
                        env.cursor = v_6;
                        // (, line 43
                        // literal, line 43
                        if !env.eq_s(&"y") {
                            break 'lab7;
                        }
                        // ], line 43
                        env.ket = env.cursor;
                        if !env.in_grouping(G_v, 97, 252) {
                            break 'lab7;
                        }
                        // <-, line 43
                        if !env.slice_from("Y") {
                            return false;
                        }
                        break 'lab8;
                    }
                    env.cursor = v_5;
                    break 'golab6;
                }
                env.cursor = v_5;
                if env.cursor >= env.limit {
                    break 'lab5;
                }
                env.next_char();
            }
            continue 'replab4;
        }
        env.cursor = v_4;
        break 'replab4;
    }
    return true;
}

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 47
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    // test, line 52
    let v_1 = env.cursor;
    // (, line 52
    // hop, line 52
    let c = env.byte_index_for_hop(3);
    if 0 as i32 > c || c > env.limit as i32 {
        return false;
    }
    env.cursor = c as usize;
    // setmark x, line 52
    context.i_x = env.cursor;
    env.cursor = v_1;
    // gopast, line 54
    'golab0: loop {
        'lab1: loop {
            if !env.in_grouping(G_v, 97, 252) {
                break 'lab1;
            }
            break 'golab0;
        }
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    // gopast, line 54
    'golab2: loop {
        'lab3: loop {
            if !env.out_grouping(G_v, 97, 252) {
                break 'lab3;
            }
            break 'golab2;
        }
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    // setmark p1, line 54
    context.i_p1 = env.cursor;
    // try, line 55
    'lab4: loop {
        // (, line 55
        if !(context.i_p1 < context.i_x){
            break 'lab4;
        }
        context.i_p1 = context.i_x;
        break 'lab4;
    }
    // gopast, line 56
    'golab5: loop {
        'lab6: loop {
            if !env.in_grouping(G_v, 97, 252) {
                break 'lab6;
            }
            break 'golab5;
        }
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    // gopast, line 56
    'golab7: loop {
        'lab8: loop {
            if !env.out_grouping(G_v, 97, 252) {
                break 'lab8;
            }
            break 'golab7;
        }
        if env.cursor >= env.limit {
            return false;
        }
        env.next_char();
    }
    // setmark p2, line 56
    context.i_p2 = env.cursor;
    return true;
}

fn r_postlude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // repeat, line 60
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            // (, line 60
            // [, line 62
            env.bra = env.cursor;
            // substring, line 62
            among_var = env.find_among(A_0, context);
            if among_var == 0 {
                break 'lab1;
            }
            // ], line 62
            env.ket = env.cursor;
            if among_var == 1 {
                // (, line 63
                // <-, line 63
                if !env.slice_from("y") {
                    return false;
                }
            } else if among_var == 2 {
                // (, line 64
                // <-, line 64
                if !env.slice_from("u") {
                    return false;
                }
            } else if among_var == 3 {
                // (, line 65
                // <-, line 65
                if !env.slice_from("a") {
                    return false;
                }
            } else if among_var == 4 {
                // (, line 66
                // <-, line 66
                if !env.slice_from("o") {
                    return false;
                }
            } else if among_var == 5 {
                // (, line 68
                // next, line 68
                if env.cursor >= env.limit {
                    break 'lab1;
                }
                env.next_char();
            }
            continue 'replab0;
        }
        env.cursor = v_1;
        break 'replab0;
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

fn r_standard_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // (, line 78
    // do, line 79
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        // (, line 79
        // [, line 80
        env.ket = env.cursor;
        // substring, line 80
        among_var = env.find_among_b(A_1, context);
        if among_var == 0 {
            break 'lab0;
        }
        // ], line 80
        env.bra = env.cursor;
        // call R1, line 80
        if !r_R1(env, context) {
            break 'lab0;
        }
        if among_var == 1 {
            // (, line 82
            // delete, line 82
            if !env.slice_del() {
                return false;
            }
        } else if among_var == 2 {
            // (, line 85
            // delete, line 85
            if !env.slice_del() {
                return false;
            }
            // try, line 86
            let v_2 = env.limit - env.cursor;
            'lab1: loop {
                // (, line 86
                // [, line 86
                env.ket = env.cursor;
                // literal, line 86
                if !env.eq_s_b(&"s") {
                    env.cursor = env.limit - v_2;
                    break 'lab1;
                }
                // ], line 86
                env.bra = env.cursor;
                // literal, line 86
                if !env.eq_s_b(&"nis") {
                    env.cursor = env.limit - v_2;
                    break 'lab1;
                }
                // delete, line 86
                if !env.slice_del() {
                    return false;
                }
                break 'lab1;
            }
        } else if among_var == 3 {
            // (, line 89
            if !env.in_grouping_b(G_s_ending, 98, 116) {
                break 'lab0;
            }
            // delete, line 89
            if !env.slice_del() {
                return false;
            }
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_1;
    // do, line 93
    let v_3 = env.limit - env.cursor;
    'lab2: loop {
        // (, line 93
        // [, line 94
        env.ket = env.cursor;
        // substring, line 94
        among_var = env.find_among_b(A_2, context);
        if among_var == 0 {
            break 'lab2;
        }
        // ], line 94
        env.bra = env.cursor;
        // call R1, line 94
        if !r_R1(env, context) {
            break 'lab2;
        }
        if among_var == 1 {
            // (, line 96
            // delete, line 96
            if !env.slice_del() {
                return false;
            }
        } else if among_var == 2 {
            // (, line 99
            if !env.in_grouping_b(G_st_ending, 98, 116) {
                break 'lab2;
            }
            // hop, line 99
            let c = env.byte_index_for_hop(-3);
            if env.limit_backward as i32 > c || c > env.limit as i32 {
                break 'lab2;
            }
            env.cursor = c as usize;
            // delete, line 99
            if !env.slice_del() {
                return false;
            }
        }
        break 'lab2;
    }
    env.cursor = env.limit - v_3;
    // do, line 103
    let v_4 = env.limit - env.cursor;
    'lab3: loop {
        // (, line 103
        // [, line 104
        env.ket = env.cursor;
        // substring, line 104
        among_var = env.find_among_b(A_4, context);
        if among_var == 0 {
            break 'lab3;
        }
        // ], line 104
        env.bra = env.cursor;
        // call R2, line 104
        if !r_R2(env, context) {
            break 'lab3;
        }
        if among_var == 1 {
            // (, line 106
            // delete, line 106
            if !env.slice_del() {
                return false;
            }
            // try, line 107
            let v_5 = env.limit - env.cursor;
            'lab4: loop {
                // (, line 107
                // [, line 107
                env.ket = env.cursor;
                // literal, line 107
                if !env.eq_s_b(&"ig") {
                    env.cursor = env.limit - v_5;
                    break 'lab4;
                }
                // ], line 107
                env.bra = env.cursor;
                // not, line 107
                let v_6 = env.limit - env.cursor;
                'lab5: loop {
                    // literal, line 107
                    if !env.eq_s_b(&"e") {
                        break 'lab5;
                    }
                    env.cursor = env.limit - v_5;
                    break 'lab4;
                }
                env.cursor = env.limit - v_6;
                // call R2, line 107
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_5;
                    break 'lab4;
                }
                // delete, line 107
                if !env.slice_del() {
                    return false;
                }
                break 'lab4;
            }
        } else if among_var == 2 {
            // (, line 110
            // not, line 110
            let v_7 = env.limit - env.cursor;
            'lab6: loop {
                // literal, line 110
                if !env.eq_s_b(&"e") {
                    break 'lab6;
                }
                break 'lab3;
            }
            env.cursor = env.limit - v_7;
            // delete, line 110
            if !env.slice_del() {
                return false;
            }
        } else if among_var == 3 {
            // (, line 113
            // delete, line 113
            if !env.slice_del() {
                return false;
            }
            // try, line 114
            let v_8 = env.limit - env.cursor;
            'lab7: loop {
                // (, line 114
                // [, line 115
                env.ket = env.cursor;
                // or, line 115
                'lab8: loop {
                    let v_9 = env.limit - env.cursor;
                    'lab9: loop {
                        // literal, line 115
                        if !env.eq_s_b(&"er") {
                            break 'lab9;
                        }
                        break 'lab8;
                    }
                    env.cursor = env.limit - v_9;
                    // literal, line 115
                    if !env.eq_s_b(&"en") {
                        env.cursor = env.limit - v_8;
                        break 'lab7;
                    }
                    break 'lab8;
                }
                // ], line 115
                env.bra = env.cursor;
                // call R1, line 115
                if !r_R1(env, context) {
                    env.cursor = env.limit - v_8;
                    break 'lab7;
                }
                // delete, line 115
                if !env.slice_del() {
                    return false;
                }
                break 'lab7;
            }
        } else if among_var == 4 {
            // (, line 119
            // delete, line 119
            if !env.slice_del() {
                return false;
            }
            // try, line 120
            let v_10 = env.limit - env.cursor;
            'lab10: loop {
                // (, line 120
                // [, line 121
                env.ket = env.cursor;
                // substring, line 121
                if env.find_among_b(A_3, context) == 0 {
                    env.cursor = env.limit - v_10;
                    break 'lab10;
                }
                // ], line 121
                env.bra = env.cursor;
                // call R2, line 121
                if !r_R2(env, context) {
                    env.cursor = env.limit - v_10;
                    break 'lab10;
                }
                // (, line 123
                // delete, line 123
                if !env.slice_del() {
                    return false;
                }
                break 'lab10;
            }
        }
        break 'lab3;
    }
    env.cursor = env.limit - v_4;
    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_x: 0,
        i_p2: 0,
        i_p1: 0,
    };
    // (, line 133
    // do, line 134
    let v_1 = env.cursor;
    // call prelude, line 134
    r_prelude(env, context);
    env.cursor = v_1;
    // do, line 135
    let v_2 = env.cursor;
    // call mark_regions, line 135
    r_mark_regions(env, context);
    env.cursor = v_2;
    // backwards, line 136
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    // do, line 137
    // call standard_suffix, line 137
    r_standard_suffix(env, context);
    env.cursor = env.limit_backward;
    // do, line 138
    let v_4 = env.cursor;
    // call postlude, line 138
    r_postlude(env, context);
    env.cursor = v_4;
    return true;
}
