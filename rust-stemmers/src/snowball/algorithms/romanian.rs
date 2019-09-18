//! This file was generated automatically by the Snowball to Rust compiler
//! https://snowballstem.org/

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use snowball::SnowballEnv;
use snowball::Among;

static A_0: &'static [Among<Context>; 3] = &[
    Among("", -1, 3, None),
    Among("I", 0, 1, None),
    Among("U", 0, 2, None),
];

static A_1: &'static [Among<Context>; 16] = &[
    Among("ea", -1, 3, None),
    Among("a\u{0163}ia", -1, 7, None),
    Among("aua", -1, 2, None),
    Among("iua", -1, 4, None),
    Among("a\u{0163}ie", -1, 7, None),
    Among("ele", -1, 3, None),
    Among("ile", -1, 5, None),
    Among("iile", 6, 4, None),
    Among("iei", -1, 4, None),
    Among("atei", -1, 6, None),
    Among("ii", -1, 4, None),
    Among("ului", -1, 1, None),
    Among("ul", -1, 1, None),
    Among("elor", -1, 3, None),
    Among("ilor", -1, 4, None),
    Among("iilor", 14, 4, None),
];

static A_2: &'static [Among<Context>; 46] = &[
    Among("icala", -1, 4, None),
    Among("iciva", -1, 4, None),
    Among("ativa", -1, 5, None),
    Among("itiva", -1, 6, None),
    Among("icale", -1, 4, None),
    Among("a\u{0163}iune", -1, 5, None),
    Among("i\u{0163}iune", -1, 6, None),
    Among("atoare", -1, 5, None),
    Among("itoare", -1, 6, None),
    Among("\u{0103}toare", -1, 5, None),
    Among("icitate", -1, 4, None),
    Among("abilitate", -1, 1, None),
    Among("ibilitate", -1, 2, None),
    Among("ivitate", -1, 3, None),
    Among("icive", -1, 4, None),
    Among("ative", -1, 5, None),
    Among("itive", -1, 6, None),
    Among("icali", -1, 4, None),
    Among("atori", -1, 5, None),
    Among("icatori", 18, 4, None),
    Among("itori", -1, 6, None),
    Among("\u{0103}tori", -1, 5, None),
    Among("icitati", -1, 4, None),
    Among("abilitati", -1, 1, None),
    Among("ivitati", -1, 3, None),
    Among("icivi", -1, 4, None),
    Among("ativi", -1, 5, None),
    Among("itivi", -1, 6, None),
    Among("icit\u{0103}i", -1, 4, None),
    Among("abilit\u{0103}i", -1, 1, None),
    Among("ivit\u{0103}i", -1, 3, None),
    Among("icit\u{0103}\u{0163}i", -1, 4, None),
    Among("abilit\u{0103}\u{0163}i", -1, 1, None),
    Among("ivit\u{0103}\u{0163}i", -1, 3, None),
    Among("ical", -1, 4, None),
    Among("ator", -1, 5, None),
    Among("icator", 35, 4, None),
    Among("itor", -1, 6, None),
    Among("\u{0103}tor", -1, 5, None),
    Among("iciv", -1, 4, None),
    Among("ativ", -1, 5, None),
    Among("itiv", -1, 6, None),
    Among("ical\u{0103}", -1, 4, None),
    Among("iciv\u{0103}", -1, 4, None),
    Among("ativ\u{0103}", -1, 5, None),
    Among("itiv\u{0103}", -1, 6, None),
];

static A_3: &'static [Among<Context>; 62] = &[
    Among("ica", -1, 1, None),
    Among("abila", -1, 1, None),
    Among("ibila", -1, 1, None),
    Among("oasa", -1, 1, None),
    Among("ata", -1, 1, None),
    Among("ita", -1, 1, None),
    Among("anta", -1, 1, None),
    Among("ista", -1, 3, None),
    Among("uta", -1, 1, None),
    Among("iva", -1, 1, None),
    Among("ic", -1, 1, None),
    Among("ice", -1, 1, None),
    Among("abile", -1, 1, None),
    Among("ibile", -1, 1, None),
    Among("isme", -1, 3, None),
    Among("iune", -1, 2, None),
    Among("oase", -1, 1, None),
    Among("ate", -1, 1, None),
    Among("itate", 17, 1, None),
    Among("ite", -1, 1, None),
    Among("ante", -1, 1, None),
    Among("iste", -1, 3, None),
    Among("ute", -1, 1, None),
    Among("ive", -1, 1, None),
    Among("ici", -1, 1, None),
    Among("abili", -1, 1, None),
    Among("ibili", -1, 1, None),
    Among("iuni", -1, 2, None),
    Among("atori", -1, 1, None),
    Among("osi", -1, 1, None),
    Among("ati", -1, 1, None),
    Among("itati", 30, 1, None),
    Among("iti", -1, 1, None),
    Among("anti", -1, 1, None),
    Among("isti", -1, 3, None),
    Among("uti", -1, 1, None),
    Among("i\u{015F}ti", -1, 3, None),
    Among("ivi", -1, 1, None),
    Among("it\u{0103}i", -1, 1, None),
    Among("o\u{015F}i", -1, 1, None),
    Among("it\u{0103}\u{0163}i", -1, 1, None),
    Among("abil", -1, 1, None),
    Among("ibil", -1, 1, None),
    Among("ism", -1, 3, None),
    Among("ator", -1, 1, None),
    Among("os", -1, 1, None),
    Among("at", -1, 1, None),
    Among("it", -1, 1, None),
    Among("ant", -1, 1, None),
    Among("ist", -1, 3, None),
    Among("ut", -1, 1, None),
    Among("iv", -1, 1, None),
    Among("ic\u{0103}", -1, 1, None),
    Among("abil\u{0103}", -1, 1, None),
    Among("ibil\u{0103}", -1, 1, None),
    Among("oas\u{0103}", -1, 1, None),
    Among("at\u{0103}", -1, 1, None),
    Among("it\u{0103}", -1, 1, None),
    Among("ant\u{0103}", -1, 1, None),
    Among("ist\u{0103}", -1, 3, None),
    Among("ut\u{0103}", -1, 1, None),
    Among("iv\u{0103}", -1, 1, None),
];

static A_4: &'static [Among<Context>; 94] = &[
    Among("ea", -1, 1, None),
    Among("ia", -1, 1, None),
    Among("esc", -1, 1, None),
    Among("\u{0103}sc", -1, 1, None),
    Among("ind", -1, 1, None),
    Among("\u{00E2}nd", -1, 1, None),
    Among("are", -1, 1, None),
    Among("ere", -1, 1, None),
    Among("ire", -1, 1, None),
    Among("\u{00E2}re", -1, 1, None),
    Among("se", -1, 2, None),
    Among("ase", 10, 1, None),
    Among("sese", 10, 2, None),
    Among("ise", 10, 1, None),
    Among("use", 10, 1, None),
    Among("\u{00E2}se", 10, 1, None),
    Among("e\u{015F}te", -1, 1, None),
    Among("\u{0103}\u{015F}te", -1, 1, None),
    Among("eze", -1, 1, None),
    Among("ai", -1, 1, None),
    Among("eai", 19, 1, None),
    Among("iai", 19, 1, None),
    Among("sei", -1, 2, None),
    Among("e\u{015F}ti", -1, 1, None),
    Among("\u{0103}\u{015F}ti", -1, 1, None),
    Among("ui", -1, 1, None),
    Among("ezi", -1, 1, None),
    Among("a\u{015F}i", -1, 1, None),
    Among("se\u{015F}i", -1, 2, None),
    Among("ase\u{015F}i", 28, 1, None),
    Among("sese\u{015F}i", 28, 2, None),
    Among("ise\u{015F}i", 28, 1, None),
    Among("use\u{015F}i", 28, 1, None),
    Among("\u{00E2}se\u{015F}i", 28, 1, None),
    Among("i\u{015F}i", -1, 1, None),
    Among("u\u{015F}i", -1, 1, None),
    Among("\u{00E2}\u{015F}i", -1, 1, None),
    Among("\u{00E2}i", -1, 1, None),
    Among("a\u{0163}i", -1, 2, None),
    Among("ea\u{0163}i", 38, 1, None),
    Among("ia\u{0163}i", 38, 1, None),
    Among("e\u{0163}i", -1, 2, None),
    Among("i\u{0163}i", -1, 2, None),
    Among("ar\u{0103}\u{0163}i", -1, 1, None),
    Among("ser\u{0103}\u{0163}i", -1, 2, None),
    Among("aser\u{0103}\u{0163}i", 44, 1, None),
    Among("seser\u{0103}\u{0163}i", 44, 2, None),
    Among("iser\u{0103}\u{0163}i", 44, 1, None),
    Among("user\u{0103}\u{0163}i", 44, 1, None),
    Among("\u{00E2}ser\u{0103}\u{0163}i", 44, 1, None),
    Among("ir\u{0103}\u{0163}i", -1, 1, None),
    Among("ur\u{0103}\u{0163}i", -1, 1, None),
    Among("\u{00E2}r\u{0103}\u{0163}i", -1, 1, None),
    Among("\u{00E2}\u{0163}i", -1, 2, None),
    Among("am", -1, 1, None),
    Among("eam", 54, 1, None),
    Among("iam", 54, 1, None),
    Among("em", -1, 2, None),
    Among("asem", 57, 1, None),
    Among("sesem", 57, 2, None),
    Among("isem", 57, 1, None),
    Among("usem", 57, 1, None),
    Among("\u{00E2}sem", 57, 1, None),
    Among("im", -1, 2, None),
    Among("\u{0103}m", -1, 2, None),
    Among("ar\u{0103}m", 64, 1, None),
    Among("ser\u{0103}m", 64, 2, None),
    Among("aser\u{0103}m", 66, 1, None),
    Among("seser\u{0103}m", 66, 2, None),
    Among("iser\u{0103}m", 66, 1, None),
    Among("user\u{0103}m", 66, 1, None),
    Among("\u{00E2}ser\u{0103}m", 66, 1, None),
    Among("ir\u{0103}m", 64, 1, None),
    Among("ur\u{0103}m", 64, 1, None),
    Among("\u{00E2}r\u{0103}m", 64, 1, None),
    Among("\u{00E2}m", -1, 2, None),
    Among("au", -1, 1, None),
    Among("eau", 76, 1, None),
    Among("iau", 76, 1, None),
    Among("indu", -1, 1, None),
    Among("\u{00E2}ndu", -1, 1, None),
    Among("ez", -1, 1, None),
    Among("easc\u{0103}", -1, 1, None),
    Among("ar\u{0103}", -1, 1, None),
    Among("ser\u{0103}", -1, 2, None),
    Among("aser\u{0103}", 84, 1, None),
    Among("seser\u{0103}", 84, 2, None),
    Among("iser\u{0103}", 84, 1, None),
    Among("user\u{0103}", 84, 1, None),
    Among("\u{00E2}ser\u{0103}", 84, 1, None),
    Among("ir\u{0103}", -1, 1, None),
    Among("ur\u{0103}", -1, 1, None),
    Among("\u{00E2}r\u{0103}", -1, 1, None),
    Among("eaz\u{0103}", -1, 1, None),
];

static A_5: &'static [Among<Context>; 5] = &[
    Among("a", -1, 1, None),
    Among("e", -1, 1, None),
    Among("ie", 1, 1, None),
    Among("i", -1, 1, None),
    Among("\u{0103}", -1, 1, None),
];

static G_v: &'static [u8; 21] = &[17, 65, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 32, 0, 0, 4];

#[derive(Clone)]
struct Context {
    b_standard_suffix_removed: bool,
    i_p2: usize,
    i_p1: usize,
    i_pV: usize,
}

fn r_prelude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 31
    // repeat, line 32
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            // goto, line 32
            'golab2: loop {
                let v_2 = env.cursor;
                'lab3: loop {
                    // (, line 32
                    if !env.in_grouping(G_v, 97, 259) {
                        break 'lab3;
                    }
                    // [, line 33
                    env.bra = env.cursor;
                    // or, line 33
                    'lab4: loop {
                        let v_3 = env.cursor;
                        'lab5: loop {
                            // (, line 33
                            // literal, line 33
                            if !env.eq_s(&"u") {
                                break 'lab5;
                            }
                            // ], line 33
                            env.ket = env.cursor;
                            if !env.in_grouping(G_v, 97, 259) {
                                break 'lab5;
                            }
                            // <-, line 33
                            if !env.slice_from("U") {
                                return false;
                            }
                            break 'lab4;
                        }
                        env.cursor = v_3;
                        // (, line 34
                        // literal, line 34
                        if !env.eq_s(&"i") {
                            break 'lab3;
                        }
                        // ], line 34
                        env.ket = env.cursor;
                        if !env.in_grouping(G_v, 97, 259) {
                            break 'lab3;
                        }
                        // <-, line 34
                        if !env.slice_from("I") {
                            return false;
                        }
                        break 'lab4;
                    }
                    env.cursor = v_2;
                    break 'golab2;
                }
                env.cursor = v_2;
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

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 38
    context.i_pV = env.limit;
    context.i_p1 = env.limit;
    context.i_p2 = env.limit;
    // do, line 44
    let v_1 = env.cursor;
    'lab0: loop {
        // (, line 44
        // or, line 46
        'lab1: loop {
            let v_2 = env.cursor;
            'lab2: loop {
                // (, line 45
                if !env.in_grouping(G_v, 97, 259) {
                    break 'lab2;
                }
                // or, line 45
                'lab3: loop {
                    let v_3 = env.cursor;
                    'lab4: loop {
                        // (, line 45
                        if !env.out_grouping(G_v, 97, 259) {
                            break 'lab4;
                        }
                        // gopast, line 45
                        'golab5: loop {
                            'lab6: loop {
                                if !env.in_grouping(G_v, 97, 259) {
                                    break 'lab6;
                                }
                                break 'golab5;
                            }
                            if env.cursor >= env.limit {
                                break 'lab4;
                            }
                            env.next_char();
                        }
                        break 'lab3;
                    }
                    env.cursor = v_3;
                    // (, line 45
                    if !env.in_grouping(G_v, 97, 259) {
                        break 'lab2;
                    }
                    // gopast, line 45
                    'golab7: loop {
                        'lab8: loop {
                            if !env.out_grouping(G_v, 97, 259) {
                                break 'lab8;
                            }
                            break 'golab7;
                        }
                        if env.cursor >= env.limit {
                            break 'lab2;
                        }
                        env.next_char();
                    }
                    break 'lab3;
                }
                break 'lab1;
            }
            env.cursor = v_2;
            // (, line 47
            if !env.out_grouping(G_v, 97, 259) {
                break 'lab0;
            }
            // or, line 47
            'lab9: loop {
                let v_6 = env.cursor;
                'lab10: loop {
                    // (, line 47
                    if !env.out_grouping(G_v, 97, 259) {
                        break 'lab10;
                    }
                    // gopast, line 47
                    'golab11: loop {
                        'lab12: loop {
                            if !env.in_grouping(G_v, 97, 259) {
                                break 'lab12;
                            }
                            break 'golab11;
                        }
                        if env.cursor >= env.limit {
                            break 'lab10;
                        }
                        env.next_char();
                    }
                    break 'lab9;
                }
                env.cursor = v_6;
                // (, line 47
                if !env.in_grouping(G_v, 97, 259) {
                    break 'lab0;
                }
                // next, line 47
                if env.cursor >= env.limit {
                    break 'lab0;
                }
                env.next_char();
                break 'lab9;
            }
            break 'lab1;
        }
        // setmark pV, line 48
        context.i_pV = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    // do, line 50
    let v_8 = env.cursor;
    'lab13: loop {
        // (, line 50
        // gopast, line 51
        'golab14: loop {
            'lab15: loop {
                if !env.in_grouping(G_v, 97, 259) {
                    break 'lab15;
                }
                break 'golab14;
            }
            if env.cursor >= env.limit {
                break 'lab13;
            }
            env.next_char();
        }
        // gopast, line 51
        'golab16: loop {
            'lab17: loop {
                if !env.out_grouping(G_v, 97, 259) {
                    break 'lab17;
                }
                break 'golab16;
            }
            if env.cursor >= env.limit {
                break 'lab13;
            }
            env.next_char();
        }
        // setmark p1, line 51
        context.i_p1 = env.cursor;
        // gopast, line 52
        'golab18: loop {
            'lab19: loop {
                if !env.in_grouping(G_v, 97, 259) {
                    break 'lab19;
                }
                break 'golab18;
            }
            if env.cursor >= env.limit {
                break 'lab13;
            }
            env.next_char();
        }
        // gopast, line 52
        'golab20: loop {
            'lab21: loop {
                if !env.out_grouping(G_v, 97, 259) {
                    break 'lab21;
                }
                break 'golab20;
            }
            if env.cursor >= env.limit {
                break 'lab13;
            }
            env.next_char();
        }
        // setmark p2, line 52
        context.i_p2 = env.cursor;
        break 'lab13;
    }
    env.cursor = v_8;
    return true;
}

fn r_postlude(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // repeat, line 56
    'replab0: loop{
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            // (, line 56
            // [, line 58
            env.bra = env.cursor;
            // substring, line 58
            among_var = env.find_among(A_0, context);
            if among_var == 0 {
                break 'lab1;
            }
            // ], line 58
            env.ket = env.cursor;
            if among_var == 1 {
                // (, line 59
                // <-, line 59
                if !env.slice_from("i") {
                    return false;
                }
            } else if among_var == 2 {
                // (, line 60
                // <-, line 60
                if !env.slice_from("u") {
                    return false;
                }
            } else if among_var == 3 {
                // (, line 61
                // next, line 61
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

fn r_step_0(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // (, line 72
    // [, line 73
    env.ket = env.cursor;
    // substring, line 73
    among_var = env.find_among_b(A_1, context);
    if among_var == 0 {
        return false;
    }
    // ], line 73
    env.bra = env.cursor;
    // call R1, line 73
    if !r_R1(env, context) {
        return false;
    }
    if among_var == 1 {
        // (, line 75
        // delete, line 75
        if !env.slice_del() {
            return false;
        }
    } else if among_var == 2 {
        // (, line 77
        // <-, line 77
        if !env.slice_from("a") {
            return false;
        }
    } else if among_var == 3 {
        // (, line 79
        // <-, line 79
        if !env.slice_from("e") {
            return false;
        }
    } else if among_var == 4 {
        // (, line 81
        // <-, line 81
        if !env.slice_from("i") {
            return false;
        }
    } else if among_var == 5 {
        // (, line 83
        // not, line 83
        let v_1 = env.limit - env.cursor;
        'lab0: loop {
            // literal, line 83
            if !env.eq_s_b(&"ab") {
                break 'lab0;
            }
            return false;
        }
        env.cursor = env.limit - v_1;
        // <-, line 83
        if !env.slice_from("i") {
            return false;
        }
    } else if among_var == 6 {
        // (, line 85
        // <-, line 85
        if !env.slice_from("at") {
            return false;
        }
    } else if among_var == 7 {
        // (, line 87
        // <-, line 87
        if !env.slice_from("a\u{0163}i") {
            return false;
        }
    }
    return true;
}

fn r_combo_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // test, line 91
    let v_1 = env.limit - env.cursor;
    // (, line 91
    // [, line 92
    env.ket = env.cursor;
    // substring, line 92
    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    // ], line 92
    env.bra = env.cursor;
    // call R1, line 92
    if !r_R1(env, context) {
        return false;
    }
    // (, line 92
    if among_var == 1 {
        // (, line 100
        // <-, line 101
        if !env.slice_from("abil") {
            return false;
        }
    } else if among_var == 2 {
        // (, line 103
        // <-, line 104
        if !env.slice_from("ibil") {
            return false;
        }
    } else if among_var == 3 {
        // (, line 106
        // <-, line 107
        if !env.slice_from("iv") {
            return false;
        }
    } else if among_var == 4 {
        // (, line 112
        // <-, line 113
        if !env.slice_from("ic") {
            return false;
        }
    } else if among_var == 5 {
        // (, line 117
        // <-, line 118
        if !env.slice_from("at") {
            return false;
        }
    } else if among_var == 6 {
        // (, line 121
        // <-, line 122
        if !env.slice_from("it") {
            return false;
        }
    }
    // set standard_suffix_removed, line 125
    context.b_standard_suffix_removed = true;
    env.cursor = env.limit - v_1;
    return true;
}

fn r_standard_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // (, line 129
    // unset standard_suffix_removed, line 130
    context.b_standard_suffix_removed = false;
    // repeat, line 131
    'replab0: loop{
        let v_1 = env.limit - env.cursor;
        'lab1: for _ in 0..1 {
            // call combo_suffix, line 131
            if !r_combo_suffix(env, context) {
                break 'lab1;
            }
            continue 'replab0;
        }
        env.cursor = env.limit - v_1;
        break 'replab0;
    }
    // [, line 132
    env.ket = env.cursor;
    // substring, line 132
    among_var = env.find_among_b(A_3, context);
    if among_var == 0 {
        return false;
    }
    // ], line 132
    env.bra = env.cursor;
    // call R2, line 132
    if !r_R2(env, context) {
        return false;
    }
    // (, line 132
    if among_var == 1 {
        // (, line 148
        // delete, line 149
        if !env.slice_del() {
            return false;
        }
    } else if among_var == 2 {
        // (, line 151
        // literal, line 152
        if !env.eq_s_b(&"\u{0163}") {
            return false;
        }
        // ], line 152
        env.bra = env.cursor;
        // <-, line 152
        if !env.slice_from("t") {
            return false;
        }
    } else if among_var == 3 {
        // (, line 155
        // <-, line 156
        if !env.slice_from("ist") {
            return false;
        }
    }
    // set standard_suffix_removed, line 160
    context.b_standard_suffix_removed = true;
    return true;
}

fn r_verb_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    // setlimit, line 164
    if env.cursor < context.i_pV {
        return false;
    }
    let v_2 = env.limit_backward;
    env.limit_backward = context.i_pV;
    // (, line 164
    // [, line 165
    env.ket = env.cursor;
    // substring, line 165
    among_var = env.find_among_b(A_4, context);
    if among_var == 0 {
        env.limit_backward = v_2;
        return false;
    }
    // ], line 165
    env.bra = env.cursor;
    if among_var == 1 {
        // (, line 200
        // or, line 200
        'lab0: loop {
            let v_3 = env.limit - env.cursor;
            'lab1: loop {
                if !env.out_grouping_b(G_v, 97, 259) {
                    break 'lab1;
                }
                break 'lab0;
            }
            env.cursor = env.limit - v_3;
            // literal, line 200
            if !env.eq_s_b(&"u") {
                env.limit_backward = v_2;
                return false;
            }
            break 'lab0;
        }
        // delete, line 200
        if !env.slice_del() {
            return false;
        }
    } else if among_var == 2 {
        // (, line 214
        // delete, line 214
        if !env.slice_del() {
            return false;
        }
    }
    env.limit_backward = v_2;
    return true;
}

fn r_vowel_suffix(env: &mut SnowballEnv, context: &mut Context) -> bool {
    // (, line 218
    // [, line 219
    env.ket = env.cursor;
    // substring, line 219
    if env.find_among_b(A_5, context) == 0 {
        return false;
    }
    // ], line 219
    env.bra = env.cursor;
    // call RV, line 219
    if !r_RV(env, context) {
        return false;
    }
    // (, line 220
    // delete, line 220
    if !env.slice_del() {
        return false;
    }
    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        b_standard_suffix_removed: false,
        i_p2: 0,
        i_p1: 0,
        i_pV: 0,
    };
    // (, line 225
    // do, line 226
    let v_1 = env.cursor;
    // call prelude, line 226
    r_prelude(env, context);
    env.cursor = v_1;
    // do, line 227
    // call mark_regions, line 227
    r_mark_regions(env, context);
    // backwards, line 228
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    // (, line 228
    // do, line 229
    let v_3 = env.limit - env.cursor;
    // call step_0, line 229
    r_step_0(env, context);
    env.cursor = env.limit - v_3;
    // do, line 230
    let v_4 = env.limit - env.cursor;
    // call standard_suffix, line 230
    r_standard_suffix(env, context);
    env.cursor = env.limit - v_4;
    // do, line 231
    let v_5 = env.limit - env.cursor;
    'lab0: loop {
        // (, line 231
        // or, line 231
        'lab1: loop {
            let v_6 = env.limit - env.cursor;
            'lab2: loop {
                // Boolean test standard_suffix_removed, line 231
                if !context.b_standard_suffix_removed {
                    break 'lab2;
                }
                break 'lab1;
            }
            env.cursor = env.limit - v_6;
            // call verb_suffix, line 231
            if !r_verb_suffix(env, context) {
                break 'lab0;
            }
            break 'lab1;
        }
        break 'lab0;
    }
    env.cursor = env.limit - v_5;
    // do, line 232
    let v_7 = env.limit - env.cursor;
    // call vowel_suffix, line 232
    r_vowel_suffix(env, context);
    env.cursor = env.limit - v_7;
    env.cursor = env.limit_backward;
    // do, line 234
    let v_8 = env.cursor;
    // call postlude, line 234
    r_postlude(env, context);
    env.cursor = v_8;
    return true;
}
