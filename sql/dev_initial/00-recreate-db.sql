-- app_user, app_db
-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
 usename = 'app_user' OR datname = 'app_db';

DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

-- DEV ONLY - Dev only password (for local dev and unit test).
CREATE USER app_user PASSWORD 'dev_only_pwd';
CREATE DATABASE app_db owner app_user ENCODING = 'UTF-8';



-- cderma_user, cderma_db
-- DEV ONLY - Brute Force DROP DB (for local dev and unit test)
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
 usename = 'cderma_user' OR datname = 'cderma_db';

DROP DATABASE IF EXISTS cderma_db;
DROP USER IF EXISTS cderma_user;

-- DEV ONLY - Dev only password (for local dev and unit test).
CREATE USER cderma_user PASSWORD 'dev_only_pwd';
CREATE DATABASE cderma_db owner cderma_user ENCODING = 'UTF-8';
