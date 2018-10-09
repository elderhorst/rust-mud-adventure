use std::collections::HashMap;

pub struct Race {
    pub name: String,
    pub plural: String,
    pub description: String,
    pub bonuses: HashMap<String, i32>,
}

pub struct RaceFactory {
    races: HashMap<String, Race>
}

impl RaceFactory {
    pub fn new() -> RaceFactory {
        let mut race_factory = RaceFactory {
            races: HashMap::new()
        };
        
        race_factory.create_races();

        race_factory
    }

    fn create_races(&mut self) {
        self.races.insert("Aasimar".to_string(),
            Race {
                name: "Aasimar".to_string(),
                plural: "Aasimar".to_string(),
                description:    "Aasimar are placed in the world to serve \
                                as guardians of law and good. Their patrons expect \
                                them to strike at evil, lead by example, and further \
                                the cause of justice.".to_string(),
                bonuses: [("cha".to_string(), 2)].iter().cloned().collect(),
            }
        );

        self.races.insert("Dragonborn".to_string(),
            Race {
                name: "Dragonborn".to_string(),
                plural: "Dragonborns".to_string(),
                description:    "Dragonborn look very much like dragons standing \
                                erect in humanoid form, though they lack wings or a \
                                tail.".to_string(),
                bonuses: [("str".to_string(), 2), ("cha".to_string(), 1)].iter().cloned().collect(),
            }
        );

        self.races.insert("Dwarf".to_string(),
            Race {
                name: "Dwarf".to_string(),
                plural: "Dwarves".to_string(),
                description:    "Dwarves are short, stocky demi-humans with long, \
                                respectable beards and heavy stout bodies. Their skin \
                                is earthen toned and their hair black, gray or dark \
                                brown. Stubborn but practical; dwarves love grand \
                                feasts and strong ale. They can be dangerous opponents, \
                                able to fight with any weapon, melee or ranged. They \
                                admire craftsmanship and are fond of gold and stonework. \
                                Dwarves are dependable fighters and sturdy against \
                                magical influences.".to_string(),
                bonuses: [("con".to_string(), 2)].iter().cloned().collect(),
            }
        );

        self.races.insert("Elf".to_string(),
            Race {
                name: "Elf".to_string(),
                plural: "Elves".to_string(),
                description:    "Elves are graceful, slender demi-humans with delicate \
                                features and pointy ears. Elves are known to use magic \
                                spells, but prefer to spend their time feasting and \
                                frolicking in wooded glades. They rarely visit cities of \
                                men. Elves are fascinated by magic and never grow weary \
                                of collecting spells or magic items. Elves love \
                                beautifully crafted items and choose to live an agrarian \
                                life in accord with nature.".to_string(),
                bonuses: [("dex".to_string(), 2)].iter().cloned().collect(),
            }
        );

        self.races.insert("Firbolg".to_string(),
            Race {
                name: "Firbolg".to_string(),
                plural: "Firbolgs".to_string(),
                description:    "Firbolg tribes cloister in remote forest \
                                strongholds, preferring to spend their days in \
                                quiet harmony with the woods.".to_string(),
                bonuses: [("wis".to_string(), 2), ("str".to_string(), 1)].iter().cloned().collect(),
            }
        );

        self.races.insert("Gnome".to_string(),
            Race {
                name: "Gnome".to_string(),
                plural: "Gnomes".to_string(),
                description:    "A Gnomes's energy and enthusiasm for living \
                                shines through every inch of his or her tiny \
                                body.".to_string(),
                bonuses: [("int".to_string(), 2)].iter().cloned().collect(),
            }
        );

        self.races.insert("Half-Elf".to_string(),
            Race {
                name: "Half-Elf".to_string(),
                plural: "Half-Elves".to_string(),
                description:    "Half-elves|n combine what some say are the best \
                                qualities of their elf and human parents.".to_string(),
                bonuses: [("cha".to_string(), 2)].iter().cloned().collect(),
            }
        );

        self.races.insert("Halfling".to_string(),
            Race {
                name: "Halfling".to_string(),
                plural: "Halflings".to_string(),
                description:    "The diminutive halflings survive in a world full \
                                of larger creatures by avoiding notice or, barring \
                                that, avoiding offense.".to_string(),
                bonuses: [("dex".to_string(), 1)].iter().cloned().collect(),
            }
        );

        self.races.insert("Human".to_string(),
            Race {
                name: "Human".to_string(),
                plural: "Humans".to_string(),
                description:    "Humans are the most widespread of all the races. \
                                The human traits of curiosity, resourcefulness and \
                                unyielding courage have helped them to adapt, survive \
                                and prosper in every world they have explored.".to_string(),
                bonuses: [("wis".to_string(), 1)].iter().cloned().collect(),
            }
        );

        self.races.insert("Kenku".to_string(),
            Race {
                name: "Kenku".to_string(),
                plural: "Kenku".to_string(),
                description:    "Kenku are haunted by an ancient crime that robbed \
                                them of their wings, they  wander the world as vagabonds \
                                and burglars who live at the edge of human society.".to_string(),
                bonuses: [("wis".to_string(), 2)].iter().cloned().collect(),
            }
        );

        self.races.insert("Tiefling".to_string(),
            Race {
                name: "Tiefling".to_string(),
                plural: "Tieflings".to_string(),
                description:    "Tieflings are greeted with stares and whispers, \
                                to suffer violence and insult on the street, to see \
                                mistrust and fear in every eye: this is the lot of \
                                the tiefling.".to_string(),
                bonuses: [("cha".to_string(), 2), ("int".to_string(), 1)].iter().cloned().collect(),
            }
        );

        self.races.insert("Triton".to_string(),
            Race {
                name: "Triton".to_string(),
                plural: "Tritons".to_string(),
                description:    "Tritons are long-established guardians of the deep \
                                ocean floor, in recent years the noble tritons have \
                                become increasingly active in the world above.".to_string(),
                bonuses: [("wis".to_string(), 1)].iter().cloned().collect(),
            }
        );
    }
}