DROP DATABASE IF EXISTS rob;

CREATE DATABASE rob;

\c rob

CREATE TABLE users (
    ucid TEXT PRIMARY KEY,
    instructor BOOLEAN NOT NULL,
    google_id TEXT NOT NULL,
    google_email TEXT NOT NULL,
    google_verified_email BOOLEAN NOT NULL,
    google_name TEXT NOT NULL,
    google_given_name TEXT NOT NULL,
    google_family_name TEXT NOT NULL,
    google_picture TEXT NOT NULL,
    google_locale TEXT NOT NULL,
    google_hd TEXT NOT NULL
);

CREATE TABLE groups (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  git_url TEXT NOT NULL
);

CREATE TABLE group_membership (
  ucid TEXT NOT NULL REFERENCES users (ucid),
  id INTEGER NOT NULL REFERENCES groups (id),
  PRIMARY KEY(ucid, id)
);

CREATE TABLE tests (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  memsnapshot TEXT DEFAULT NULL,
  tests_image TEXT DEFAULT NULL,
  base_image TEXT NOT NULL,
  command TEXT NOT NULL,
  command_timeout INTEGER NOT NULL,
  prompt TEXT NOT NULL
);

CREATE TABLE jobs (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  started_at TIMESTAMPTZ DEFAULT NULL,
  completed_at TIMESTAMPTZ DEFAULT NULL,
  status TEXT NOT NULL,
  group_id INTEGER NOT NULL REFERENCES groups (id),
  test_id INTEGER NOT NULL REFERENCES tests (id),
  output TEXT NOT NULL DEFAULT ''
);

INSERT INTO users VALUES (
  'rxt1077',
  TRUE,
  '104011277083293490929',
  'rxt1077@njit.edu',
  TRUE,
  'Ryan Tolboom',
  'Ryan',
  'Tolboom',
  'https://lh5.googleusercontent.com/-zXrHWkmWoSo/AAAAAAAAAAI/AAAAAAAAAAA/AMZuucmcjzXsaPGt-M0znDxq9nlr9AEC5Q/s96-c/photo.jpg',
  'en',
  'njit.edu'
);
