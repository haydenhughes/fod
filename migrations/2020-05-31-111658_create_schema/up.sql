CREATE TABLE foods (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE
);


CREATE TABLE meal_types (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL
);

CREATE TABLE entries (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users (id) ON DELETE CASCADE NOT NULL,
  meal_type_id INTEGER REFERENCES meal_types (id) ON DELETE CASCADE NOT NULL,
  hunger_before INTEGER NOT NULL,
  hunger_after INTEGER NOT NULL,
  timestamp TIMESTAMP NOT NULL,
  comments VARCHAR
);

CREATE TABLE meals (
  CONSTRAINT id PRIMARY KEY (food_id, entry_id),
  food_id INTEGER REFERENCES foods (id) ON UPDATE CASCADE ON DELETE CASCADE,
  entry_id INTEGER REFERENCES entries (id) ON UPDATE CASCADE ON DELETE CASCADE
);
