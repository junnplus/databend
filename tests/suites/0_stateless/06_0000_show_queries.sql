DROP TABLE IF EXISTS t1;
DROP TABLE IF EXISTS t2;
DROP TABLE IF EXISTS t3;

CREATE TABLE t1(c1 int) ENGINE = Null;
CREATE TABLE t2(c1 int) ENGINE = Null;
CREATE TABLE t3(c1 int) ENGINE = Null;

SHOW TABLES;

SHOW TABLES LIKE 't%';
SHOW TABLES LIKE 't2';
SHOW TABLES LIKE 't';

SHOW TABLES WHERE name LIKE 't%';
SHOW TABLES WHERE name = 't%' AND 1 = 0;
SHOW TABLES WHERE name = 't2' OR 1 = 1;
SHOW TABLES WHERE name = 't2' AND 1 = 1;

DROP TABLE IF EXISTS t1;
DROP TABLE IF EXISTS t2;
DROP TABLE IF EXISTS t3;
