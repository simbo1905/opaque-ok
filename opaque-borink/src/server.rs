pub use crate::opaque_impl::{
    server_login_finish, server_login_start, server_register_finish, server_register_start,
};
pub use crate::opaque_impl::{
    PasswordFile, ServerLoginFinishResult, ServerLoginStartResult, ServerRegistrationStartResult,
    ServerSetup, ServerSetupView
};
pub use crate::opaque_impl::{
    LOGIN_SERVER_MESSAGE_LEN, LOGIN_SERVER_STATE_LEN, PASSWORD_FILE_LEN,
    REGISTER_SERVER_MESSAGE_LEN, SERVER_SETUP_LEN, SHARED_SECRET_LEN,
};

#[cfg(test)]
mod tests {
    use crate::client::{client_login_finish, client_login_start, ClientStateLogin};
    use crate::encoded::{decode_string, encode_bytes};
    use crate::opaque_impl::PASSWORD_FILE_SERIALIZED_LEN;
    use crate::test_util::gen_password_file_with_setup_and_pw;

    use super::*;

    #[test]
    fn server_setup_roundtrip() {
        let setup = ServerSetup::create();
        let encoded = encode_bytes(&setup.serialize());
        let deserialized = ServerSetup::deserialize(&decode_string(&encoded).unwrap()).unwrap();
        assert_eq!(setup.serialize(), deserialized.serialize());
        assert_eq!(SERVER_SETUP_LEN, setup.serialize().len());
    }

    #[test]
    fn server_register_roundtrip() {
        let setup = ServerSetup::create();
        let password_file =
            gen_password_file_with_setup_and_pw(&mut setup.view(), b"someperson", b"clientele");

        let encoded = encode_bytes(&password_file.serialize());
        let deserialized = PasswordFile::deserialize(&decode_string(&encoded).unwrap()).unwrap();
        assert_eq!(password_file.serialize(), deserialized.serialize());
        assert_eq!(PASSWORD_FILE_SERIALIZED_LEN, password_file.serialize().len());
    }

    #[test]
    fn server_login_flow() {
        let setup = ServerSetup::create();
        let password_file =
            gen_password_file_with_setup_and_pw(&mut setup.view(), b"someperson", b"clientele");

        let mut client_state = ClientStateLogin::setup();
        let client_start = client_login_start(&mut client_state, b"clientele").unwrap();

        let server_start = server_login_start(
            &mut setup.view(),
            &password_file,
            &client_start.response,
            "someperson",
        )
        .unwrap();
        assert_eq!(LOGIN_SERVER_MESSAGE_LEN, server_start.response.len());
        assert_eq!(LOGIN_SERVER_STATE_LEN, server_start.state.len());

        let client_finish = client_login_finish(&mut client_state, b"clientele", &server_start.response).unwrap();

        let server_finish = server_login_finish(&client_finish.response, &server_start.state).unwrap();
        assert_eq!(client_finish.shared_secret, server_finish.shared_secret);
        assert_eq!(SHARED_SECRET_LEN, server_finish.shared_secret.len());
    }

    #[test]
    fn server_login_wrong_password() {
        let setup = ServerSetup::create();
        let password_file =
            gen_password_file_with_setup_and_pw(&mut setup.view(), b"someperson", b"clientele");

        let mut client_state = ClientStateLogin::setup();
        let client_start = client_login_start(&mut client_state, b"wrong_password").unwrap();

        let server_start = server_login_start(
            &mut setup.view(),
            &password_file,
            &client_start.response,
            "someperson",
        )
        .unwrap();

        assert!(client_login_finish(&mut client_state, b"wrong_password", &server_start.response).is_err());
    }
}
