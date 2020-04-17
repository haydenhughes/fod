ALTER TABLE entries RENAME TO mealentries;
ALTER TABLE mealentries RENAME COLUMN entryid TO mealentryid;

CREATE TABLE sleepentries (
  sleepentryid SERIAL PRIMARY KEY,
  userid INTEGER REFERENCES users (userid),
  starttime TIMESTAMP NOT NULL,
  endtime TIMESTAMP NOT NULL,
  comments VARCHAR
);

CREATE TABLE exercisetypes (
  exercisetypeid SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);


CREATE TABLE exerciseentries (
  exerciseentryid SERIAL PRIMARY KEY,
  userid INTEGER REFERENCES users (userid),
  exercisetype INTEGER REFERENCES exercisetypes (exercisetypeid),
  starttime TIMESTAMP NOT NULL,
  endtime TIMESTAMP NOT NULL,
  comments VARCHAR
);
