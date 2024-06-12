/* settings table */
DROP TABLE IF EXISTS "settings";

CREATE TABLE "settings" (
    "key" VARCHAR(64) NOT NULL,
    "val" VARCHAR(256) DEFAULT NULL,
    "type" VARCHAR(16) NOT NULL DEFAULT 'string',
    "update" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY ("key")
);

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.update = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_update
BEFORE UPDATE ON settings
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

COMMENT ON COLUMN "settings"."key" IS '';
COMMENT ON COLUMN "settings"."val" IS '';
COMMENT ON COLUMN "settings"."type" IS '';
COMMENT ON COLUMN "settings"."update" IS '';
