pub enum EventHandlerEffect {
    Damage(u32),
    DamageAndHeal(u32, bool, bool), // amount, damage target, heal target
}
