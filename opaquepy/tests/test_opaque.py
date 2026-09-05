from typing import Generator
import pytest

from opaquepy import *


@pytest.fixture(scope="module")
def server_setup() -> Generator[None, None, str]:
    yield create_setup()


def test_create_setup(server_setup: str):
    assert server_setup


password = "pass"


@pytest.fixture(scope="module")
def client_registration() -> Generator[None, None, tuple[str]]:
    response, client_state = register_client(password)
    yield response, client_state


def test_register(server_setup: str, client_registration: tuple[str]):
    response = client_registration[0]
    client_state = client_registration[1]
    server_response = register(server_setup, response, "someperson")
    client_final_response = register_client_finish(
        client_state, password, server_response
    )
    assert client_final_response


def test_login_flow(server_setup: str):
    password = "clientele"
    cred_id = "someperson"

    # register
    register_response, register_state = register_client(password)
    server_register_response = register(server_setup, register_response, cred_id)
    register_finish_response = register_client_finish(
        register_state, password, server_register_response
    )
    password_file = register_finish(register_finish_response)

    # login
    login_response, login_state = login_client(password)
    server_login_response, server_login_state = login(
        server_setup, password_file, login_response, cred_id
    )
    client_finish_message, client_session_key = login_client_finish(
        login_state, password, server_login_response
    )
    server_session_key = login_finish(client_finish_message, server_login_state)

    assert client_session_key == server_session_key


def test_login_wrong_password(server_setup: str):
    password = "clientele"
    cred_id = "someperson"

    register_response, register_state = register_client(password)
    server_register_response = register(server_setup, register_response, cred_id)
    register_finish_response = register_client_finish(
        register_state, password, server_register_response
    )
    password_file = register_finish(register_finish_response)

    login_response, login_state = login_client("wrong_password")
    server_login_response, _ = login(
        server_setup, password_file, login_response, cred_id
    )

    with pytest.raises(ValueError):
        login_client_finish(login_state, "wrong_password", server_login_response)
