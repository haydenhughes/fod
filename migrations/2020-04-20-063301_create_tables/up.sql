CREATE TABLE meal_types (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE exercise_types (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE foods (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  password VARCHAR NOT NULL
);

CREATE TABLE entries (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users (id) NOT NULL,
  timestamp TIMESTAMP NOT NULL,
  comments VARCHAR
);

CREATE TABLE meal_entries (
  id SERIAL PRIMARY KEY,
  entry_id INTEGER REFERENCES entries (id) NOT NULL,
  meal_type_id INTEGER REFERENCES meal_types (id) NOT NULL
);

CREATE TABLE meals (
  CONSTRAINT id PRIMARY KEY (food_id, meal_entry_id),
  food_id INTEGER REFERENCES foods (id) ON UPDATE CASCADE ON DELETE CASCADE,
  meal_entry_id INTEGER REFERENCES meal_entries (id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE exercise_entries (
  id SERIAL PRIMARY KEY,
  entry_id INTEGER REFERENCES entries (id) NOT NULL,
  exercise_type_id INTEGER REFERENCES exercise_types (id) NOT NULL,
  duration TIMESTAMP NOT NULL
);

CREATE TABLE sleep_entries (
  id SERIAL PRIMARY KEY,
  entry_id INTEGER REFERENCES entries (id) NOT NULL,
  duration TIMESTAMP NOT NULL
)
