pub enum EventHandlerEffect {
    Damage(u32, bool),              // damage target
    DamageAndHeal(u32, bool, bool), // amount, damage target, heal target
}
