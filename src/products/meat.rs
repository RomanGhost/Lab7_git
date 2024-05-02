use crate::product::Product;

pub struct Meat {
    weight: f64,
    calories: f64,
    proteins: f64,
    fats: f64,
    carbohydrates: f64,
    price: f64
}

impl Meat {
    pub fn new(weight: f64) -> Self {
        Meat {
            weight,
            calories: 143.0,
            proteins: 26.0,
            fats: 3.5,
            carbohydrates: 0.00,
            price: 78.9
        }
    }
}

impl Product for Meat {
    fn get_calories(&self) -> f64 {
        self.calories
    }

    fn get_proteins(&self) -> f64 {
        self.proteins
    }

    fn get_fats(&self) -> f64 {
        self.fats
    }

    fn get_carbohydrates(&self) -> f64 {
        self.carbohydrates
    }

    fn get_price(&self) -> f64 {
        self.price
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }
}
