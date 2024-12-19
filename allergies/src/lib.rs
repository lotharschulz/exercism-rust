pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    /// Constructs a new `Allergies` struct with the given score.
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    /// Determines if the patient is allergic to the given allergen.
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // Check if the allergen bit is set in the score
        self.score & (*allergen as u32) != 0
    }

    /// Returns a list of allergens the patient is allergic to.
    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result = Vec::new();
        // Iterate over all possible allergens
        for &allergen in [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ]
        .iter()
        {
            // Check if the patient is allergic to the current allergen
            if self.is_allergic_to(&allergen) {
                result.push(allergen);
            }
        }
        result
    }
}
