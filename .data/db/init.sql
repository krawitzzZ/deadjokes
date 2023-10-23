-- Set the default time zone to UTC
SET TIME ZONE 'UTC';

-- Enable the uuid-ossp extension for generating UUIDs
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Enable the hstore extension for storing key-value pairs
CREATE EXTENSION IF NOT EXISTS hstore;

-- Enable the pgcrypto extension for generating secure random values
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Enable the unaccent extension for removing accents from text
CREATE EXTENSION IF NOT EXISTS unaccent;

-- Enable the citext extension for case-insensitive text comparison
CREATE EXTENSION IF NOT EXISTS citext;

-- Enable the fuzzystrmatch extension for fuzzy string matching
CREATE EXTENSION IF NOT EXISTS fuzzystrmatch;

-- Enable the pg_trgm extension for trigram matching
CREATE EXTENSION IF NOT EXISTS pg_trgm;
