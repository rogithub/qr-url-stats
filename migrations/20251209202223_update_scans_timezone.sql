-- Add migration script here
-- Eliminar la columna vieja
ALTER TABLE scans DROP COLUMN scanned_at;

-- Agregar la nueva como TEXT para guardar con timezone
ALTER TABLE scans ADD COLUMN scanned_at TEXT NOT NULL DEFAULT '1970-01-01T00:00:00-06:00';