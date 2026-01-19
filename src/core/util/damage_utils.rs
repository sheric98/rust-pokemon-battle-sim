pub fn rounded_damage_from_modifiers_with_default(
    modifiers: &Vec<f32>,
    default: Option<u32>,
) -> u32 {
    if modifiers.is_empty() {
        if default.is_none() {
            panic!("No modifiers provided")
        } else {
            return default.unwrap();
        }
    }

    if modifiers.len() == 1 {
        return modifiers[0].floor() as u32;
    }

    let mut modifier: f32 = modifiers[0] * modifiers[1];

    for mod_value in &modifiers[2..] {
        modifier = modifier.floor() * mod_value;
    }

    modifier.floor() as u32
}

pub fn rounded_damage_from_modifiers(modifiers: &Vec<f32>) -> u32 {
    rounded_damage_from_modifiers_with_default(modifiers, None)
}

pub fn get_damage_for_move(
    level: u8,
    bp: u32,
    atk: u32,
    def: u32,
    mod1: u32,
    mod2: u32,
    mod3: u32,
    crit_mult: f32,
    r: u32,
    stab_mult: f32,
    type1_mult: f32,
    type2_mult: f32,
) -> u32 {
    let inner1: f32 = (((level * 2) as f32) / 5.0).floor() + 2.0;
    let inner2: f32 =
        ((((inner1 * (bp as f32)).floor() * (atk as f32)) / 50.0).floor() / (def as f32)).floor()
            * (mod1 as f32)
            + 2.0;
    ((((((((inner2 * crit_mult).floor() * (mod2 as f32)).floor() * (r as f32)).floor() / 100.0)
        .floor()
        * stab_mult)
        .floor()
        * type1_mult)
        .floor()
        * type2_mult)
        .floor()
        * (mod3 as f32))
        .floor() as u32
}
