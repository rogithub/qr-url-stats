-- Add migration script here
-- Eliminar la columna vieja
ALTER TABLE links DROP COLUMN created_at;

-- Agregar la nueva como TEXT para guardar con timezone
ALTER TABLE links ADD COLUMN created_at TEXT NOT NULL DEFAULT '1970-01-01T00:00:00-05:00';