// @generated automatically by Diesel CLI.

diesel::table! {
    member (id) {
        id -> Integer,
        f_name -> Text,
        m_name -> Text,
        l_name -> Text,
        email -> Text,
        address -> Text,
        age -> Integer,
    }
}
