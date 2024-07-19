// @generated automatically by Diesel CLI.

diesel::table! {
    fichas_tecnica (id) {
        id -> Int4,
        nome -> Varchar,
        sobrenome -> Varchar,
        numero -> Varchar,
        email -> Varchar,
        equipamento -> Varchar,
        defeito -> Varchar,
        concluido -> Bool,
    }
}
