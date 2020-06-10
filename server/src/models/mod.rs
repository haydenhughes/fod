mod entry;
mod food;
mod meal;
mod meal_type;
mod user;

use diesel::dsl::Select;
use diesel::query_source::Table;
pub use entry::{Entry, NewEntry};
pub use food::Food;
pub use meal::Meal;
pub use meal_type::{MealType, NewMealType};
pub use user::{NewUser, User};

#[allow(type_alias_bounds)]  // Should never be a problem
type All<T: Table> = Select<T, T::AllColumns>;
