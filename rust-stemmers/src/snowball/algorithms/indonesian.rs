//! This file was generated automatically by the Snowball to Rust compiler
//! https://snowballstem.org/

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use snowball::SnowballEnv;
use snowball::Among;

static A_0: &'static [Among<Context>; 3] = &[
    Among("kah", -1, 1, None),
    Among("lah", -1, 1, None),
    Among("pun", -1, 1, None),
];

static A_1: &'static [Among<Context>; 3] = &[
    Among("nya", -1, 1, None),
    Among("ku", -1, 1, None),
    Among("mu", -1, 1, None),
];

static A_2: &'static [Among<Context>; 3] = &[
    Among("i", -1, 1, Some(&r_SUFFIX_I_OK)),
    Among("an", -1, 1, Some(&r_SUFFIX_AN_OK)),
    Among("kan", 1, 1, Some(&r_SUFFIX_KAN_OK)),
];

static A_3: &'static [Among<Context>; 12] = &[
    Among("di", -1, 1, None),
    Among("ke", -1, 2, None),
    Among("me", -1, 1, None),
    Among("mem", 2, 5, None),
    Among("men", 2, 1, None),
    Among("meng", 4, 1, None),
    Among("meny", 4, 3, Some(&r_VOWEL)),
    Among("pem", -1, 6, None),
    Among("pen", -1, 2, None),
    Among("peng", 8, 2, None),
    Among("peny", 8, 4, Some(&r_VOWEL)),
    Among("ter", -1, 1, None),
];

static A_4: &'static [Among<Context>; 6] = &[
    Among("be", -1, 3, Some(&r_KER)),
    Among("belajar", 0, 4, None),
    Among("ber", 0, 3, None),
    Among("pe", -1, 1, None),
    Among("pelajar", 3, 2, None),
    Among("per", 3, 1, None),
];

static G_vowel: &'static [u8; 3] = &[17, 65, 16];

#[derive(Clone)]
struct Context {
    i_prefix: i32,
    i_measure: i32,
}

fn r_remove_particle(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 50
    // [, line 51
    env.ket = env.cursor;
    // substring, line 51
    if env.find_among_b(A_0, context) == 0 {
        return false;
    }
    // ], line 51
    env.bra = env.cursor;
    // (, line 52
    // delete, line 52
    if !env.slice_del() {
        return false;
    }
    context.i_measure -= 1;
    return true;
}

fn r_remove_possessive_pronoun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 56
    // [, line 57
    env.ket = env.cursor;
    // substring, line 57
    if env.find_among_b(A_1, context) == 0 {
        return false;
    }
    // ], line 57
    env.bra = env.cursor;
    // (, line 58
    // delete, line 58
    if !env.slice_del() {
        return false;
    }
    context.i_measure -= 1;
    return true;
}

fn r_SUFFIX_KAN_OK(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 63
    // and, line 85
    if !(context.i_prefix != 3){
        return false;
    }
    if !(context.i_prefix != 2){
        return false;
    }
    return true;
}

fn r_SUFFIX_AN_OK(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 89
    if !(context.i_prefix != 1){
        return false;
    }
    return true;
}

fn r_SUFFIX_I_OK(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 91
    if !(context.i_prefix <= 2){
        return false;
    }
    // not, line 128
    let v_1 = env.limit - env.cursor;
    'lab0: loop {
        // literal, line 128
        if !env.eq_s_b(&"s") {
            break 'lab0;
        }
        return false;
    }
    env.cursor = env.limit - v_1;
    return true;
}

fn r_remove_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 131
    // [, line 132
    env.ket = env.cursor;
    // substring, line 132
    if env.find_among_b(A_2, context) == 0 {
        return false;
    }
    // ], line 132
    env.bra = env.cursor;
    // (, line 134
    // delete, line 134
    if !env.slice_del() {
        return false;
    }
    context.i_measure -= 1;
    return true;
}

fn r_VOWEL(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 141
    if !env.in_grouping(G_vowel, 97, 117) {
        return false;
    }
    return true;
}

fn r_KER(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 143
    if !env.out_grouping(G_vowel, 97, 117) {
        return false;
    }
    // literal, line 143
    if !env.eq_s(&"er") {
        return false;
    }
    return true;
}

fn r_remove_first_order_prefix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // (, line 145
    // [, line 146
    env.bra = env.cursor;
    // substring, line 146
    among_var = env.find_among(A_3, context);
    if among_var == 0 {
        return false;
    }
    // ], line 146
    env.ket = env.cursor;
    if among_var == 1 {
        // (, line 147
        // delete, line 147
        if !env.slice_del() {
            return false;
        }
        context.i_prefix = 1;
        context.i_measure -= 1;
    } else if among_var == 2 {
        // (, line 148
        // delete, line 148
        if !env.slice_del() {
            return false;
        }
        context.i_prefix = 3;
        context.i_measure -= 1;
    } else if among_var == 3 {
        // (, line 149
        context.i_prefix = 1;
        // <-, line 149
        if !env.slice_from("s") {
            return false;
        }
        context.i_measure -= 1;
    } else if among_var == 4 {
        // (, line 150
        context.i_prefix = 3;
        // <-, line 150
        if !env.slice_from("s") {
            return false;
        }
        context.i_measure -= 1;
    } else if among_var == 5 {
        // (, line 151
        context.i_prefix = 1;
        context.i_measure -= 1;
        // or, line 151
        'lab0: loop {
            let v_1 = env.cursor;
            'lab1: loop {
                // and, line 151
                let v_2 = env.cursor;
                if !env.in_grouping(G_vowel, 97, 117) {
                    break 'lab1;
                }
                env.cursor = v_2;
                // <-, line 151
                if !env.slice_from("p") {
                    return false;
                }
                break 'lab0;
            }
            env.cursor = v_1;
            // delete, line 151
            if !env.slice_del() {
                return false;
            }
            break 'lab0;
        }
    } else if among_var == 6 {
        // (, line 152
        context.i_prefix = 3;
        context.i_measure -= 1;
        // or, line 152
        'lab2: loop {
            let v_3 = env.cursor;
            'lab3: loop {
                // and, line 152
                let v_4 = env.cursor;
                if !env.in_grouping(G_vowel, 97, 117) {
                    break 'lab3;
                }
                env.cursor = v_4;
                // <-, line 152
                if !env.slice_from("p") {
                    return false;
                }
                break 'lab2;
            }
            env.cursor = v_3;
            // delete, line 152
            if !env.slice_del() {
                return false;
            }
            break 'lab2;
        }
    }
    return true;
}

fn r_remove_second_order_prefix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // (, line 156
    // [, line 162
    env.bra = env.cursor;
    // substring, line 162
    among_var = env.find_among(A_4, context);
    if among_var == 0 {
        return false;
    }
    // ], line 162
    env.ket = env.cursor;
    if among_var == 1 {
        // (, line 163
        // delete, line 163
        if !env.slice_del() {
            return false;
        }
        context.i_prefix = 2;
        context.i_measure -= 1;
    } else if among_var == 2 {
        // (, line 164
        // <-, line 164
        if !env.slice_from("ajar") {
            return false;
        }
        context.i_measure -= 1;
    } else if among_var == 3 {
        // (, line 165
        // delete, line 165
        if !env.slice_del() {
            return false;
        }
        context.i_prefix = 4;
        context.i_measure -= 1;
    } else if among_var == 4 {
        // (, line 166
        // <-, line 166
        if !env.slice_from("ajar") {
            return false;
        }
        context.i_prefix = 4;
        context.i_measure -= 1;
    }
    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_prefix: 0,
        i_measure: 0,
    };
    // (, line 171
    context.i_measure = 0;
    // do, line 173
    let v_1 = env.cursor;
    'lab0: loop {
        // (, line 173
        // repeat, line 173
        'replab1: loop{
            let v_2 = env.cursor;
            'lab2: for _ in 0..1 {
                // (, line 173
                // gopast, line 173
                'golab3: loop {
                    'lab4: loop {
                        if !env.in_grouping(G_vowel, 97, 117) {
                            break 'lab4;
                        }
                        break 'golab3;
                    }
                    if env.cursor >= env.limit {
                        break 'lab2;
                    }
                    env.next_char();
                }
                context.i_measure += 1;
                continue 'replab1;
            }
            env.cursor = v_2;
            break 'replab1;
        }
        break 'lab0;
    }
    env.cursor = v_1;
    if !(context.i_measure > 2){
        return false;
    }
    context.i_prefix = 0;
    // backwards, line 176
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    // (, line 176
    // do, line 177
    let v_4 = env.limit - env.cursor;
    // call remove_particle, line 177
    r_remove_particle(env, context);
    env.cursor = env.limit - v_4;
    if !(context.i_measure > 2){
        return false;
    }
    // do, line 179
    let v_5 = env.limit - env.cursor;
    // call remove_possessive_pronoun, line 179
    r_remove_possessive_pronoun(env, context);
    env.cursor = env.limit - v_5;
    env.cursor = env.limit_backward;
    if !(context.i_measure > 2){
        return false;
    }
    // or, line 188
    'lab5: loop {
        let v_6 = env.cursor;
        'lab6: loop {
            // test, line 182
            let v_7 = env.cursor;
            // (, line 182
            // call remove_first_order_prefix, line 183
            if !r_remove_first_order_prefix(env, context) {
                break 'lab6;
            }
            // do, line 184
            let v_8 = env.cursor;
            'lab7: loop {
                // (, line 184
                // test, line 185
                let v_9 = env.cursor;
                // (, line 185
                if !(context.i_measure > 2){
                    break 'lab7;
                }
                // backwards, line 185
                env.limit_backward = env.cursor;
                env.cursor = env.limit;
                // call remove_suffix, line 185
                if !r_remove_suffix(env, context) {
                    break 'lab7;
                }
                env.cursor = env.limit_backward;
                env.cursor = v_9;
                if !(context.i_measure > 2){
                    break 'lab7;
                }
                // call remove_second_order_prefix, line 186
                if !r_remove_second_order_prefix(env, context) {
                    break 'lab7;
                }
                break 'lab7;
            }
            env.cursor = v_8;
            env.cursor = v_7;
            break 'lab5;
        }
        env.cursor = v_6;
        // (, line 188
        // do, line 189
        let v_10 = env.cursor;
        // call remove_second_order_prefix, line 189
        r_remove_second_order_prefix(env, context);
        env.cursor = v_10;
        // do, line 190
        let v_11 = env.cursor;
        'lab8: loop {
            // (, line 190
            if !(context.i_measure > 2){
                break 'lab8;
            }
            // backwards, line 190
            env.limit_backward = env.cursor;
            env.cursor = env.limit;
            // call remove_suffix, line 190
            if !r_remove_suffix(env, context) {
                break 'lab8;
            }
            env.cursor = env.limit_backward;
            break 'lab8;
        }
        env.cursor = v_11;
        break 'lab5;
    }
    return true;
}
