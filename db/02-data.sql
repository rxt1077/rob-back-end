\c rob

/* Demo users */
INSERT INTO users VALUES ('al2012',  FALSE, '', 'al2012@njit.edu',  TRUE, 'Alice Lenoy',    'Alice',  'Lenoy',   'http://www.example.com', 'en', 'njit.edu');
INSERT INTO users VALUES ('bp2',     FALSE, '', 'bp2@njit.edu',     TRUE, 'Burt Patel',     'Burt',   'Patel',   'http://www.example.com', 'en', 'njit.edu');
INSERT INTO users VALUES ('cxt1983', FALSE, '', 'cxt1983@njit.edu', TRUE, 'Curtis Tremlay', 'Curtis', 'Tremlay', 'http://www.example.com', 'en', 'njit.edu');
INSERT INTO users VALUES ('demi',    FALSE, '', 'demi@njit.edu',    TRUE, 'Demi Sysop',     'Demi',   'Sysop',   'http://www.example.com', 'en', 'njit.edu');
INSERT INTO users VALUES ('ej12',    FALSE, '', 'ej12@njit.edu',    TRUE, 'Emily Jhaz',     'Emily',  'Jhaz',    'http://www.example.com', 'en', 'njit.edu');
INSERT INTO users VALUES ('ftr2021', FALSE, '', 'ftr2021@njit.edu', TRUE, 'Francis Regan',  'Francis', 'Regan',  'http://www.example.com', 'en', 'njit.edu');
INSERT INTO users VALUES ('gg4',     FALSE, '', 'gg5@njit.edu',     TRUE, 'Greg Ginesh',    'Greg',    'Ginesh', 'http://www.example.com', 'en', 'njit.edu');
INSERT INTO users VALUES ('hh123',   FALSE, '', 'hh123@njit.edu',   TRUE, 'Herbert Holoy',  'Herbert', 'Holoy',  'http://www.example.com', 'en', 'njit.edu');
INSERT INTO users VALUES ('is9',     FALSE, '', 'is9@njit.edu',     TRUE, 'Israel Ses',     'Israel',  'Ses',    'http://www.example.com', 'en', 'njit.edu');
INSERT INTO users VALUES ('jj103',   FALSE, '', 'jj103@njit.edu',   TRUE, 'Jessica Jones',  'Jessica', 'Jones',  'http://www.example.com', 'en', 'njit.edu');
INSERT INTO users VALUES ('test_instructor', TRUE, '', 'test_instructor@njit.edu', TRUE, 'Test Instructor',  'Test', 'Instructor',  'http://www.example.com', 'en', 'njit.edu');
INSERT INTO users VALUES ('test_student', FALSE, '', 'test_student@njit.edu', TRUE, 'Test Student',  'Test', 'Student',  'http://www.example.com', 'en', 'njit.edu');

/* Demo groups */
INSERT INTO groups (name, git_url) VALUES ('Group 1', 'https://github.com/group1/project.git');
INSERT INTO groups (name, git_url) VALUES ('Marshal Stax', 'https://ms.tx/project.git');

/* Demo tests */
INSERT INTO tests (
    memsnapshot,
    name,
    description,
    tests_image,
    base_image,
    command,
    command_timeout,
    prompt)
  VALUES (
    NULL,
    'Boot Output',
    'Loads the image WITHOUT a memory snapshot and runs dmesg',
    'tests/tests.iso',
    'images/debian-10.qcow2',
    'dmesg',
    600,
    'root@rob-run:~#');
INSERT INTO tests (
    memsnapshot,
    name,
    description,
    tests_image,
    base_image,
    command,
    command_timeout,
    prompt)
  VALUES (
    'images/memsnapshot.gz',
    'Memsnapshot Test',
    'Loads the image with a memory snapshot and runs ls',
    'tests/tests.iso',
    'images/debian-10.qcow2',
    'ls /',
    600,
    'root@rob-run:~#');
INSERT INTO tests (
    memsnapshot,
    name,
    description,
    tests_image,
    base_image,
    command,
    command_timeout,
    prompt)
  VALUES (
    'images/memsnapshot.gz',
    'No scripts',
    'Tests running a VM without a scripts ISO',
    NULL,
    'images/debian-10.qcow2',
    'ps ax',
    600,
    'root@rob-run:~#');
INSERT INTO tests (
    memsnapshot,
    name,
    description,
    tests_image,
    base_image,
    command,
    command_timeout,
    prompt)
  VALUES (
    'images/memsnapshot.gz',
    'No jobs',
    'A test with no jobs using it',
    'tests/tests.iso',
    'images/debian-10.qcow2',
    'ps ax',
    600,
    'root@rob-run:~#');

/* Demo jobs */
INSERT INTO jobs (status, group_id, test_id) VALUES ('QUEUED', 1, 1);
INSERT INTO jobs (status, group_id, test_id) VALUES ('QUEUED', 2, 2);
INSERT INTO jobs (status, group_id, test_id) VALUES ('QUEUED', 1, 3);
