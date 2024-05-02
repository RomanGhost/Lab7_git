// Импортируем модули из других файлов внутри текущей папки
mod mushrooms;
mod sour_cream;
mod shrimps;
mod cheese;
mod herbs_and_spices;
mod meat;

// Экспортируем модули из текущей папки
pub use mushrooms::Mushrooms;
pub use shrimps::Shrimps;
pub use sour_cream::SourCream;
pub use cheese::Cheese;
pub use herbs_and_spices::HerbsAndSpices;
pub use meat::Meat;
