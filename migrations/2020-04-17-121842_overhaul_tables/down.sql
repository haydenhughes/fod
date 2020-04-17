ALTER TABLE mealentries RENAME TO entries;
ALTER TABLE mealentries RENAME COLUMN mealentryid TO entryid;


DROP TABLE sleepentries;
DROP TABLE exerciseentries;
DROP TABLE exercisetypes;
