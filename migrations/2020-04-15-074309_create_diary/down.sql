CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  qty INTEGER NOT NULL,
  req_qty INTEGER,
  price REAL,
  notes TEXT
);

DROP TABLE meals;
DROP TABLE entries;
DROP TABLE mealtypes;
DROP TABLE foods;
