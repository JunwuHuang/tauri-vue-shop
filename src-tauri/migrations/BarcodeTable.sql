CREATE TABLE IF NOT EXISTS Barcode (
    id varchar(64) PRIMARY KEY,
    barcode text NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
);