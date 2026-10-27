//! Generated from sesotho.sbl by Snowball 3.2.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use crate::snowball::SnowballEnv;
use crate::snowball::Among;

#[derive(Clone)]
struct Context {
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

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
    };
    let mut i_pV : i32;
    let v_1 = env.cursor;
    if !env.go_out_grouping(G_v, 97, 117) {
        return false;
    }
    env.next_char();
    i_pV = env.cursor;
    env.cursor = v_1;
    let v_2 = env.cursor;
    if !env.hop(2) {
        return false;
    }
    'lab0: loop {
        if env.cursor <= i_pV {
            break 'lab0;
        }
        i_pV = env.cursor;
        break 'lab0;
    }
    env.cursor = v_2;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_3 = env.limit - env.cursor;
    'lab1: loop {
        if env.cursor < i_pV {
            break 'lab1;
        }
        let v_4 = env.limit_backward;
        env.limit_backward = i_pV;
        env.ket = env.cursor;
        if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((33282 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            env.limit_backward = v_4;
            break 'lab1;
        }

        if env.find_among_b(A_2, context) == 0 {
            env.limit_backward = v_4;
            break 'lab1;
        }
        env.bra = env.cursor;
        env.slice_del();
        env.limit_backward = v_4;
        break 'lab1;
    }
    env.cursor = env.limit - v_3;
    let v_5 = env.limit - env.cursor;
    'lab2: loop {
        if env.cursor < i_pV {
            break 'lab2;
        }
        let v_6 = env.limit_backward;
        env.limit_backward = i_pV;
        env.ket = env.cursor;
        if (env.cursor <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 3 as u8 || ((162 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            env.limit_backward = v_6;
            break 'lab2;
        }

        if env.find_among_b(A_1, context) == 0 {
            env.limit_backward = v_6;
            break 'lab2;
        }
        env.bra = env.cursor;
        env.slice_del();
        env.limit_backward = v_6;
        break 'lab2;
    }
    env.cursor = env.limit - v_5;
    env.cursor = env.limit_backward;
    let v_7 = env.cursor;
    'lab3: loop {
        env.bra = env.cursor;
        if (env.cursor + 1 >= env.limit || env.current.as_bytes()[(env.cursor + 1) as usize] as u8 >> 5 != 3 as u8 || ((33314 as i32 >> (env.current.as_bytes()[(env.cursor + 1) as usize] as u8 & 0x1f)) & 1) == 0) {
            break 'lab3;
        }

        if env.find_among(A_0, context) == 0 {
            break 'lab3;
        }
        env.ket = env.cursor;
        let v_8 = env.cursor;
        if env.cursor >= env.limit {
            break 'lab3;
        }
        env.next_char();
        if env.cursor >= env.limit {
            break 'lab3;
        }
        env.cursor = v_8;
        if !env.go_out_grouping(G_v, 97, 117) {
            break 'lab3;
        }
        env.next_char();
        env.slice_del();
        break 'lab3;
    }
    env.cursor = v_7;
    return true
}
