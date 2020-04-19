CREATE TABLE entries (
  entryid SERIAL PRIMARY KEY,
  userid INTEGER REFERENCES users (userid),
  mealentryid INTEGER REFERENCES mealentries (mealentryid),
  exerciseentryid INTEGER REFERENCES exerciseentries (exerciseentryid),
  sleepentryid INTEGER REFERENCES sleepentries (sleepentryid),
  timestamp TIMESTAMP NOT NULL
);

ALTER TABLE exerciseentries
DROP COLUMN userid,
DROP COLUMN starttime;

ALTER TABLE sleepentries
DROP COLUMN userid,
DROP COLUMN starttime;

ALTER TABLE mealentries
DROP COLUMN userid,
DROP COLUMN timestamp;
