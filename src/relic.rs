enum Slot {
    Head,
    Hands,
    Body,
    Feet,
}

impl Slot {
    fn from(string: &str) -> Option<Slot> {
        match string {
            "head" => Some(Slot::Head),
            "hands" => Some(Slot::Hands),
            "body" => Some(Slot::Body),
            "feet" => Some(Slot::Feet),
            _ => None,
        }
    }
}

enum Stat {
    Hp(f64),
    Atk(f64),
    Def(f64),
    HpPercent(f64),
    AtkPercent(f64),
    DefPercent(f64),
    CritRate(f64),
    CritDamage(f64),
    Elemental(Element, f64),
    EffectHitRate(f64),
    EnergyRegenerationRate(f64),
    OutgoingHealingBoost(f64),
    BreakEffect(f64),
    Spd(f64),
}

impl Stat {
    fn from(string: &str, value: f64, element: Option<Element>) -> Option<Stat> {
        match string {
            "hp" => Some(Stat::Hp(value)),
            "atk" => Some(Stat::Atk(value)),
            "def" => Some(Stat::Def(value)),
            "hp_percent" => Some(Stat::HpPercent(value)),
            "atk_percent" => Some(Stat::AtkPercent(value)),
            "def_percent" => Some(Stat::DefPercent(value)),
            "crit_rate" => Some(Stat::CritRate(value)),
            "crit_damage" => Some(Stat::CritDamage(value)),
            "elemental" => Some(Stat::Elemental(element.unwrap(), value)),
            "effect" => Some(Stat::EffectHitRate(value)),
            "energy_regeneration" => Some(Stat::EnergyRegenerationRate(value)),
            _ => None,
        }
    }
}

enum Element {
    Fire,
    Lightning,
    Ice,
    Wind,
    Physical,
    Quantum,
    Imaginary,
}
impl Element {
    fn from(string: &str) -> Option<Element> {
        match string {
            "fire" => Some(Element::Fire),
            "lightning" => Some(Element::Lightning),
            "ice" => Some(Element::Ice),
            "wind" => Some(Element::Wind),
            "physical" => Some(Element::Physical),
            "quantum" => Some(Element::Quantum),
            "imaginary" => Some(Element::Imaginary),
            _ => None,
        }
    }
}

pub struct Relic {
    slot: Slot,
    level: i8,
    main_stat: Stat,
    sub_stats: [Stat; 4],
    set: String,
}
