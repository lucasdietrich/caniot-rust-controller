/* settings table */
DROP TABLE IF EXISTS "settings";

CREATE TABLE "settings" (
    "key" VARCHAR(64) NOT NULL,
    "val" VARCHAR(256) DEFAULT NULL,
    "type" VARCHAR(16) NOT NULL DEFAULT 'string',
    "last_updated" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("key")
);

-- -- Postgres specific
-- CREATE OR REPLACE FUNCTION update_timestamp()
-- RETURNS TRIGGER AS $$
-- BEGIN
--     NEW.last_updated = CURRENT_TIMESTAMP;
--     RETURN NEW;
-- END;
-- $$ LANGUAGE plpgsql;

-- CREATE TRIGGER update_last_updated
-- BEFORE UPDATE ON settings
-- FOR EACH ROW
-- EXECUTE FUNCTION update_timestamp();

-- COMMENT ON COLUMN "settings"."key" IS '';
-- COMMENT ON COLUMN "settings"."val" IS '';
-- COMMENT ON COLUMN "settings"."type" IS '';
-- COMMENT ON COLUMN "settings"."last_updated" IS '';
