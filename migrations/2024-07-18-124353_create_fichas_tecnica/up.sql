-- Your SQL goes here
CREATE TABLE fichas_tecnica (
    id SERIAL PRIMARY KEY,
    nome VARCHAR NOT NULL,
    sobrenome VARCHAR NOT NULL,
    numero VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    equipamento VARCHAR NOT NULL,
    defeito VARCHAR NOT NULL,
    concluido BOOLEAN NOT NULL DEFAULT false
)