CREATE TABLE IF NOT EXISTS links (
    id VARCHAR(10) primary key not null unique,
    url TEXT not null,
    stopcode VARCHAR(20)
)