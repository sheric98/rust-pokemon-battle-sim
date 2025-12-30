pub fn calculate_non_hp_stat(
    level: u8,
    base_stat: u16,
    iv: u8,
    ev: u8,
    nature_mult: f32,
) -> u16 {
    let ev_inc: u8 = ev / 4;

    ((((((2 * base_stat) + iv as u16 + ev_inc as u16) * level as u16) / 100) + 5) as f32 * nature_mult).floor() as u16
}

pub fn calculate_hp_stat(
    level: u8,
    base_stat: u16,
    iv: u8,
    ev: u8
) -> u16 {
    let ev_inc: u8 = ev / 4;

    ((((2 * base_stat) + iv as u16 + ev_inc as u16) * level as u16) / 100) + level as u16 + 10
}