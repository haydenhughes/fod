mod user;

use diesel::dsl::Select;
use diesel::query_source::Table;
pub use user::{NewUser, User};

type All<T: Table> = Select<T, T::AllColumns>;
