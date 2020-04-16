ALTER TABLE users RENAME COLUMN id TO userid;

ALTER TABLE entries ADD userid INTEGER REFERENCES users (userid);
