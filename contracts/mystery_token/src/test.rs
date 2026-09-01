#![cfg(test)]

use super::{MysteryToken, MysteryTokenClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup() -> (Env, MysteryTokenClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(MysteryToken, ());
    let client = MysteryTokenClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    client.initialize(&owner);

    (env, client, owner)
}

#[test]
fn test_initialize_acuna_supply_inicial() {
    let (_env, client, owner) = setup();

    assert_eq!(client.balance(&owner), 2_000_000);
    assert_eq!(client.total_supply(), 2_000_000);
}

#[test]
fn test_transfer_normal_mueve_balances() {
    let (env, client, owner) = setup();
    let bob = Address::generate(&env);

    client.transfer(&owner, &bob, &200);

    assert_eq!(client.balance(&owner), 2_000_000 - 200);
    assert_eq!(client.balance(&bob), 200);
}

// ─────────────────────────────────────────────────────────────
//  Tests del Reto 2 — la "luz verde" de transfer_with_fee.
//  Fallan mientras la funcion tenga el panic! de plantilla.
// ─────────────────────────────────────────────────────────────

#[test]
fn test_transfer_with_fee_cobra_uno_por_ciento() {
    let (env, client, owner) = setup();
    let bob = Address::generate(&env);

    client.transfer_with_fee(&owner, &bob, &1_000);

    // 1% de 1000 = 10 de comision -> a bob le llegan 990
    assert_eq!(client.balance(&bob), 990);
}

#[test]
fn test_transfer_with_fee_descuenta_el_monto_completo_al_emisor() {
    let (env, client, owner) = setup();
    let bob = Address::generate(&env);

    client.transfer_with_fee(&owner, &bob, &1_000);

    assert_eq!(client.balance(&owner), 2_000_000 - 1_000);
}

#[test]
fn test_transfer_with_fee_quema_la_comision_del_supply() {
    let (env, client, owner) = setup();
    let bob = Address::generate(&env);

    client.transfer_with_fee(&owner, &bob, &1_000);

    // el supply total baja en la comision quemada (10)
    assert_eq!(client.total_supply(), 2_000_000 - 10);
}

#[test]
#[should_panic(expected = "saldo insuficiente")]
fn test_transfer_with_fee_falla_sin_saldo() {
    let (env, client, owner) = setup();
    let bob = Address::generate(&env);

    client.transfer_with_fee(&owner, &bob, &10_000_000);
}
