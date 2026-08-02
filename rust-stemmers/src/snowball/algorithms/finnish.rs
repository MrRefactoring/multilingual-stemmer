//! Generated from finnish.sbl by Snowball 3.1.1 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use crate::snowball::SnowballEnv;
use crate::snowball::Among;

#[derive(Clone)]
struct Context {
    b_ending_removed: bool,
    S_x: String,
    i_p2: i32,
    i_p1: i32,
}

static A_0: &'static [Among<Context>; 10] = &[
    Among("pa", -1, 1, None),
    Among("sti", -1, 2, None),
    Among("kaan", -1, 1, None),
    Among("han", -1, 1, None),
    Among("kin", -1, 1, None),
    Among("hän", -1, 1, None),
    Among("kään", -1, 1, None),
    Among("ko", -1, 1, None),
    Among("pä", -1, 1, None),
    Among("kö", -1, 1, None),
];

static A_1: &'static [Among<Context>; 6] = &[
    Among("lla", -1, -1, None),
    Among("na", -1, -1, None),
    Among("ssa", -1, -1, None),
    Among("ta", -1, -1, None),
    Among("lta", 3, -1, None),
    Among("sta", 3, -1, None),
];

static A_2: &'static [Among<Context>; 6] = &[
    Among("llä", -1, -1, None),
    Among("nä", -1, -1, None),
    Among("ssä", -1, -1, None),
    Among("tä", -1, -1, None),
    Among("ltä", 3, -1, None),
    Among("stä", 3, -1, None),
];

static A_3: &'static [Among<Context>; 2] = &[
    Among("lle", -1, -1, None),
    Among("ine", -1, -1, None),
];

static A_4: &'static [Among<Context>; 9] = &[
    Among("nsa", -1, 3, None),
    Among("mme", -1, 3, None),
    Among("nne", -1, 3, None),
    Among("ni", -1, 2, None),
    Among("si", -1, 1, None),
    Among("an", -1, 4, None),
    Among("en", -1, 6, None),
    Among("än", -1, 5, None),
    Among("nsä", -1, 3, None),
];

static A_5: &'static [Among<Context>; 7] = &[
    Among("aa", -1, -1, None),
    Among("ee", -1, -1, None),
    Among("ii", -1, -1, None),
    Among("oo", -1, -1, None),
    Among("uu", -1, -1, None),
    Among("ää", -1, -1, None),
    Among("öö", -1, -1, None),
];

static A_6: &'static [Among<Context>; 8] = &[
    Among("'", -1, -1, None),
    Among("ai", -1, -1, None),
    Among("ei", -1, -1, None),
    Among("ii", -1, -1, None),
    Among("oi", -1, -1, None),
    Among("ui", -1, -1, None),
    Among("äi", -1, -1, None),
    Among("öi", -1, -1, None),
];

static A_7: &'static [Among<Context>; 31] = &[
    Among("a", -1, 2, None),
    Among("lla", 0, -1, None),
    Among("na", 0, -1, None),
    Among("ssa", 0, -1, None),
    Among("ta", 0, -1, None),
    Among("lta", 4, -1, None),
    Among("sta", 4, -1, None),
    Among("tta", 4, 3, None),
    Among("lle", -1, -1, None),
    Among("ine", -1, -1, None),
    Among("ksi", -1, -1, None),
    Among("n", -1, 1, None),
    Among("han", 11, -1, Some(&r_A)),
    Among("den", 11, -1, Some(&r_VI)),
    Among("seen", 11, -1, Some(&r_LV)),
    Among("hen", 11, -1, Some(&r_E)),
    Among("tten", 11, -1, Some(&r_VI)),
    Among("hin", 11, -1, Some(&r_I)),
    Among("siin", 11, -1, Some(&r_VI)),
    Among("hon", 11, -1, Some(&r_O)),
    Among("hun", 11, -1, Some(&r_U)),
    Among("hän", 11, -1, Some(&r_A_)),
    Among("hön", 11, -1, Some(&r_O_)),
    Among("ä", -1, 2, None),
    Among("llä", 23, -1, None),
    Among("nä", 23, -1, None),
    Among("ssä", 23, -1, None),
    Among("tä", 23, -1, None),
    Among("ltä", 27, -1, None),
    Among("stä", 27, -1, None),
    Among("ttä", 27, 3, None),
];

static A_8: &'static [Among<Context>; 14] = &[
    Among("eja", -1, -1, None),
    Among("mma", -1, 1, None),
    Among("imma", 1, -1, None),
    Among("mpa", -1, 1, None),
    Among("impa", 3, -1, None),
    Among("mmi", -1, 1, None),
    Among("immi", 5, -1, None),
    Among("mpi", -1, 1, None),
    Among("impi", 7, -1, None),
    Among("ejä", -1, -1, None),
    Among("mmä", -1, 1, None),
    Among("immä", 10, -1, None),
    Among("mpä", -1, 1, None),
    Among("impä", 12, -1, None),
];

static A_10: &'static [Among<Context>; 2] = &[
    Among("mma", -1, 1, None),
    Among("imma", 0, -1, None),
];

static G_AEI: &'static [u8; 17] = &[17, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8];

static G_C: &'static [u8; 4] = &[119, 223, 119, 1];

static G_v: &'static [u8; 19] = &[17, 65, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 32];

static G_particle_end: &'static [u8; 19] = &[17, 97, 24, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 32];

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    if !env.go_out_grouping(G_v, 97, 246) {
        return false;
    }
    env.next_char();
    if !env.go_in_grouping(G_v, 97, 246) {
        return false;
    }
    env.next_char();
    context.i_p1 = env.cursor;
    if !env.go_out_grouping(G_v, 97, 246) {
        return false;
    }
    env.next_char();
    if !env.go_in_grouping(G_v, 97, 246) {
        return false;
    }
    env.next_char();
    context.i_p2 = env.cursor;
    return true
}

fn r_particle_etc(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_0, context);
    if among_var == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    match among_var {
        1 => {
            if !env.in_grouping_b(G_particle_end, 97, 246) {
                return false;
            }
        }
        2 => {
            if context.i_p2 > env.cursor {
                return false;
            }
        }
        _ => ()
    }
    env.slice_del();
    return true
}

fn r_possessive(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_4, context);
    if among_var == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    match among_var {
        1 => {
            'lab0: loop {
                if !env.eq_s_b(&"k") {
                    break 'lab0;
                }
                return false;
            }
            env.slice_del();
        }
        2 => {
            env.slice_del();
            env.ket = env.cursor;
            if !env.eq_s_b(&"kse") {
                return false;
            }
            env.bra = env.cursor;
            env.slice_from("ksi");
        }
        3 => {
            env.slice_del();
        }
        4 => {
            if (env.cursor - 1 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 97 as u8) {
                return false;
            }

            if env.find_among_b(A_1, context) == 0 {
                return false;
            }
            env.slice_del();
        }
        5 => {
            if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 164 as u8) {
                return false;
            }

            if env.find_among_b(A_2, context) == 0 {
                return false;
            }
            env.slice_del();
        }
        6 => {
            if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 101 as u8) {
                return false;
            }

            if env.find_among_b(A_3, context) == 0 {
                return false;
            }
            env.slice_del();
        }
        _ => ()
    }
    return true
}

fn r_LV(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return env.find_among_b(A_5, context) != 0;
}

fn r_VI(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if (env.cursor <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 39 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 105 as u8)) {
        return false;
    }

    return env.find_among_b(A_6, context) != 0;
}

fn r_A(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        'lab1: loop {
            if !env.eq_s_b(&"a") {
                break 'lab1;
            }
            break 'lab0;
        }
        if !env.eq_s_b(&"'") {
            return false;
        }
        break 'lab0;
    }
    return true
}

fn r_E(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        'lab1: loop {
            if !env.eq_s_b(&"e") {
                break 'lab1;
            }
            break 'lab0;
        }
        if !env.eq_s_b(&"'") {
            return false;
        }
        break 'lab0;
    }
    return true
}

fn r_I(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        'lab1: loop {
            if !env.eq_s_b(&"i") {
                break 'lab1;
            }
            break 'lab0;
        }
        if !env.eq_s_b(&"'") {
            return false;
        }
        break 'lab0;
    }
    return true
}

fn r_O(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        'lab1: loop {
            if !env.eq_s_b(&"o") {
                break 'lab1;
            }
            break 'lab0;
        }
        if !env.eq_s_b(&"'") {
            return false;
        }
        break 'lab0;
    }
    return true
}

fn r_U(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        'lab1: loop {
            if !env.eq_s_b(&"u") {
                break 'lab1;
            }
            break 'lab0;
        }
        if !env.eq_s_b(&"'") {
            return false;
        }
        break 'lab0;
    }
    return true
}

fn r_A_(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        'lab1: loop {
            if !env.eq_s_b(&"ä") {
                break 'lab1;
            }
            break 'lab0;
        }
        if !env.eq_s_b(&"'") {
            return false;
        }
        break 'lab0;
    }
    return true
}

fn r_O_(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        'lab1: loop {
            if !env.eq_s_b(&"ö") {
                break 'lab1;
            }
            break 'lab0;
        }
        'lab2: loop {
            if !env.eq_s_b(&"ø") {
                break 'lab2;
            }
            break 'lab0;
        }
        if !env.eq_s_b(&"'") {
            return false;
        }
        break 'lab0;
    }
    return true
}

fn r_case_ending(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_7, context);
    if among_var == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    match among_var {
        1 => {
            let v_2 = env.limit - env.cursor;
            'lab0: loop {
                let v_3 = env.limit - env.cursor;
                'lab1: loop {
                    let v_4 = env.limit - env.cursor;
                    'lab2: loop {
                        if !r_LV(env, context) {
                            break 'lab2;
                        }
                        break 'lab1;
                    }
                    env.cursor = env.limit - v_4;
                    if !env.eq_s_b(&"ie") {
                        env.cursor = env.limit - v_2;
                        break 'lab0;
                    }
                    break 'lab1;
                }
                env.cursor = env.limit - v_3;
                if env.cursor <= env.limit_backward {
                    env.cursor = env.limit - v_2;
                    break 'lab0;
                }
                env.previous_char();
                env.bra = env.cursor;
                break 'lab0;
            }
        }
        2 => {
            if !env.in_grouping_b(G_v, 97, 246) {
                return false;
            }
            if !env.in_grouping_b(G_C, 98, 122) {
                return false;
            }
        }
        3 => {
            if !env.eq_s_b(&"e") {
                return false;
            }
        }
        _ => ()
    }
    env.slice_del();
    context.b_ending_removed = true;
    return true
}

fn r_other_endings(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p2 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p2;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_8, context);
    if among_var == 0 {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_1;
    match among_var {
        1 => {
            'lab0: loop {
                if !env.eq_s_b(&"po") {
                    break 'lab0;
                }
                return false;
            }
        }
        _ => ()
    }
    env.slice_del();
    return true
}

fn r_i_plural(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if (env.cursor <= env.limit_backward || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 105 as u8 && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 106 as u8)) {
        env.limit_backward = v_1;
        return false;
    }

    env.cursor -= 1;
    env.bra = env.cursor;
    env.limit_backward = v_1;
    env.slice_del();
    return true
}

fn r_t_plural(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    env.ket = env.cursor;
    if !env.eq_s_b(&"t") {
        env.limit_backward = v_1;
        return false;
    }
    env.bra = env.cursor;
    let v_2 = env.limit - env.cursor;
    if !env.in_grouping_b(G_v, 97, 246) {
        env.limit_backward = v_1;
        return false;
    }
    env.cursor = env.limit - v_2;
    env.slice_del();
    env.limit_backward = v_1;
    if env.cursor < context.i_p2 {
        return false;
    }
    let v_3 = env.limit_backward;
    env.limit_backward = context.i_p2;
    env.ket = env.cursor;
    if (env.cursor - 2 <= env.limit_backward || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 97 as u8) {
        env.limit_backward = v_3;
        return false;
    }

    among_var = env.find_among_b(A_10, context);
    if among_var == 0 {
        env.limit_backward = v_3;
        return false;
    }
    env.bra = env.cursor;
    env.limit_backward = v_3;
    match among_var {
        1 => {
            'lab0: loop {
                if !env.eq_s_b(&"po") {
                    break 'lab0;
                }
                return false;
            }
        }
        _ => ()
    }
    env.slice_del();
    return true
}

fn r_tidy(env: &mut SnowballEnv, context: &mut Context) -> bool {
    if env.cursor < context.i_p1 {
        return false;
    }
    let v_1 = env.limit_backward;
    env.limit_backward = context.i_p1;
    let v_2 = env.limit - env.cursor;
    'lab0: loop {
        let v_3 = env.limit - env.cursor;
        if !r_LV(env, context) {
            break 'lab0;
        }
        env.cursor = env.limit - v_3;
        env.ket = env.cursor;
        if env.cursor <= env.limit_backward {
            break 'lab0;
        }
        env.previous_char();
        env.bra = env.cursor;
        env.slice_del();
        break 'lab0;
    }
    env.cursor = env.limit - v_2;
    let v_4 = env.limit - env.cursor;
    'lab1: loop {
        env.ket = env.cursor;
        if !env.in_grouping_b(G_AEI, 97, 228) {
            break 'lab1;
        }
        env.bra = env.cursor;
        if !env.in_grouping_b(G_C, 98, 122) {
            break 'lab1;
        }
        env.slice_del();
        break 'lab1;
    }
    env.cursor = env.limit - v_4;
    let v_5 = env.limit - env.cursor;
    'lab2: loop {
        env.ket = env.cursor;
        if !env.eq_s_b(&"j") {
            break 'lab2;
        }
        env.bra = env.cursor;
        'lab3: loop {
            'lab4: loop {
                if !env.eq_s_b(&"o") {
                    break 'lab4;
                }
                break 'lab3;
            }
            if !env.eq_s_b(&"u") {
                break 'lab2;
            }
            break 'lab3;
        }
        env.slice_del();
        break 'lab2;
    }
    env.cursor = env.limit - v_5;
    let v_6 = env.limit - env.cursor;
    'lab5: loop {
        env.ket = env.cursor;
        if !env.eq_s_b(&"o") {
            break 'lab5;
        }
        env.bra = env.cursor;
        if !env.eq_s_b(&"j") {
            break 'lab5;
        }
        env.slice_del();
        break 'lab5;
    }
    env.cursor = env.limit - v_6;
    env.limit_backward = v_1;
    let v_7 = env.limit - env.cursor;
    'lab6: loop {
        if !env.go_in_grouping_b(G_v, 97, 246) {
            break 'lab6;
        }
        env.ket = env.cursor;
        if !env.in_grouping_b(G_C, 98, 122) {
            break 'lab6;
        }
        env.bra = env.cursor;
        context.S_x = env.slice_to();
        if !env.eq_s_b(&context.S_x) {
            break 'lab6;
        }
        env.slice_del();
        break 'lab6;
    }
    env.cursor = env.limit - v_7;
    env.ket = env.cursor;
    if !env.eq_s_b(&"'") {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        b_ending_removed: false,
        S_x: String::new(),
        i_p2: 0,
        i_p1: 0,
    };
    let v_1 = env.cursor;
    r_mark_regions(env, context);
    env.cursor = v_1;
    context.b_ending_removed = false;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    let v_2 = env.limit - env.cursor;
    r_particle_etc(env, context);
    env.cursor = env.limit - v_2;
    let v_3 = env.limit - env.cursor;
    r_possessive(env, context);
    env.cursor = env.limit - v_3;
    let v_4 = env.limit - env.cursor;
    r_case_ending(env, context);
    env.cursor = env.limit - v_4;
    let v_5 = env.limit - env.cursor;
    r_other_endings(env, context);
    env.cursor = env.limit - v_5;
    'lab0: loop {
        'lab1: loop {
            if !context.b_ending_removed {
                break 'lab1;
            }
            let v_6 = env.limit - env.cursor;
            r_i_plural(env, context);
            env.cursor = env.limit - v_6;
            break 'lab0;
        }
        let v_7 = env.limit - env.cursor;
        r_t_plural(env, context);
        env.cursor = env.limit - v_7;
        break 'lab0;
    }
    let v_8 = env.limit - env.cursor;
    r_tidy(env, context);
    env.cursor = env.limit - v_8;
    env.cursor = env.limit_backward;
    return true
}
