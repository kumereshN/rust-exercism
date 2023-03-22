#[derive(Debug)]
pub struct Allergies{
    score: u32
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies{score}
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        unimplemented!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        unimplemented!("Return the list of allergens contained within the score with which the Allergies struct was made.");
    }
}



fn main() {
    let a = Allergies::new(5);
    println!("{:?}", a);
}
