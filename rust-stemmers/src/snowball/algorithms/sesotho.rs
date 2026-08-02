//! Generated from sesotho.sbl by Snowball 3.1.1 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use crate::snowball::SnowballEnv;
use crate::snowball::Among;

#[derive(Clone)]
struct Context {
    i_pV: i32,
}

static A_0: &'static [Among<Context>; 8] = &[
    Among("ba", -1, -1, None),
    Among("boi", -1, -1, None),
    Among("le", -1, -1, None),
    Among("li", -1, -1, None),
    Among("ma", -1, -1, None),
    Among("me", -1, -1, None),
    Among("mo", -1, -1, None),
    Among("se", -1, -1, None),
];

static A_1: &'static [Among<Context>; 9] = &[
    Among("a", -1, 1, None),
    Among("ela", 0, 1, None),
    Among("isa", 0, 1, None),
    Among("wa", 0, 1, None),
    Among("ile", -1, 1, None),
    Among("etse", -1, 1, None),
    Among("ang", -1, 1, None),
    Among("eng", -1, 1, None),
    Among("ong", -1, 1, None),
];

static A_2: &'static [Among<Context>; 5] = &[
    Among("ana", -1, 1, None),
    Among("nyana", 0, 1, None),
    Among("oa", -1, 1, None),
    Among("i", -1, 1, None),
    Among("ano", -1, 1, None),
];

static G_v: &'static [u8; 3] = &[17, 65, 16];

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let v_1 = env.cursor;
    if !env.go_out_grouping(G_v, 97, 117) {
        return false;
    }
    env.next_char();
    context.i_pV = env.cursor;
    env.cursor = v_1;
    let v_2 = env.cursor;
    if !env.hop(2) {
        return false;
    }
    'lab0: loop {
        if env.cursor <= context.i_pV {
            break 'lab0;
        }
        context.i_pV = env.cursor;
        break 'lab0;
    }
    env.cursor = v_2;
    return true
}

fn r_remove_noun_prefixes(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.bra = env.cursor;
    if (env.cursor + 1 >= env.limit || env.current.as_bytes()[(env.cursor + 1) as usize] as u8 >> 5 != 3 as u8 || ((33314 as i32 >> (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        return false;
    }

    if env.find_among(A_0, context) == 0 {
        return false;
    }
    env.ket = env.cursor;
    let v_1 = env.cursor;
    if env.cursor >= env.limit {
        return false;
    }
    env.next_char();
    if env.cursor >= env.limit {
        return false;
    }
    env.cursor = v_1;
    if !env.go_out_grouping(G_v, 97, 117) {
        return false;
    }
    env.next_char();
    env.slice_del();
    return true
}

fn r_remove_verb_suffixes(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_pV {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_pV;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((162 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        env.limit_backward = v_1;
        return false;
    }

    if env.find_among_b(A_1, context) == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    env.limit_backward = v_1;
    return true
}

fn r_remove_nominal_suffixes(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_pV {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_pV;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((33282 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
        env.limit_backward = v_1;
        return false;
    }

    if env.find_among_b(A_2, context) == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    env.limit_backward = v_1;
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_pV: 0,
    };
    if !r_mark_regions(env, context) {
        return false;
    }
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_1 = env.limit - env.cursor;
    r_remove_nominal_suffixes(env, context);
    env.cursor = env.limit - v_1;
    let v_2 = env.limit - env.cursor;
    r_remove_verb_suffixes(env, context);
    env.cursor = env.limit - v_2;
    env.cursor = env.limit_backward;
    let v_3 = env.cursor;
    r_remove_noun_prefixes(env, context);
    env.cursor = v_3;
    return true
}
