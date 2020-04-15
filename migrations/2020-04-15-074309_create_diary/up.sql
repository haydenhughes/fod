DROP TABLE items;

CREATE TABLE foods (
  foodid SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  description VARCHAR NOT NULL
);

CREATE TABLE mealtypes (
  mealtypeid SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE entries (
  entryid SERIAL PRIMARY KEY,
  timestamp TIMESTAMP NOT NULL,
  mealtype INTEGER REFERENCES mealtypes (mealtypeid),
  comments VARCHAR
);

CREATE TABLE meals (
  CONSTRAINT mealid PRIMARY KEY (foodid, entryid),
  foodid INTEGER REFERENCES foods (foodid) ON UPDATE CASCADE ON DELETE CASCADE,
  entryid INTEGER REFERENCES entries (entryid) ON UPDATE CASCADE ON DELETE CASCADE,
  qty INTEGER NOT NULL
);
