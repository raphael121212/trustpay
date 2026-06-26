#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol, Vec,
};

#[contracttype]
#[derive(Clone)]
pub enum EscrowStatus {
    Pending,
    Completed,
    Cancelled,
}

#[contracttype]
#[derive(Clone)]
pub struct Escrow {
    pub id: u32,
    pub buyer: Address,
    pub seller: Address,
    pub amount: i128,
    pub status: EscrowStatus,
}

#[contracttype]
pub enum DataKey {
    Escrow(u32),
    Counter,
}

#[contract]
pub struct TrustPayEscrow;

#[contractimpl]
impl TrustPayEscrow {
    // Create a new escrow
    pub fn create_escrow(
        env: Env,
        buyer: Address,
        seller: Address,
        amount: i128,
    ) -> u32 {
        buyer.require_auth();

        let mut counter: u32 = env
            .storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        counter += 1;

        let escrow = Escrow {
            id: counter,
            buyer: buyer.clone(),
            seller,
            amount,
            status: EscrowStatus::Pending,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Escrow(counter), &escrow);

        env.storage()
            .instance()
            .set(&DataKey::Counter, &counter);

        env.events().publish(
            (symbol_short!("create"), counter),
            amount,
        );

        counter
    }

    // Confirm delivery and release payment
    pub fn confirm(env: Env, id: u32) {
        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&DataKey::Escrow(id))
            .expect("Escrow not found");

        escrow.buyer.require_auth();

        match escrow.status {
            EscrowStatus::Pending => {}
            _ => panic!("Already processed"),
        }

        // NOTE:
        // Real implementation should transfer USDC/XLM here
        // using Stellar Asset Client or Token Client.

        escrow.status = EscrowStatus::Completed;

        env.storage()
            .persistent()
            .set(&DataKey::Escrow(id), &escrow);

        env.events().publish(
            (symbol_short!("release"), id),
            escrow.amount,
        );
    }

    // Cancel escrow
    pub fn cancel(env: Env, id: u32) {
        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&DataKey::Escrow(id))
            .expect("Escrow not found");

        escrow.buyer.require_auth();

        match escrow.status {
            EscrowStatus::Pending => {}
            _ => panic!("Cannot cancel"),
        }

        escrow.status = EscrowStatus::Cancelled;

        env.storage()
            .persistent()
            .set(&DataKey::Escrow(id), &escrow);

        env.events().publish(
            (symbol_short!("cancel"), id),
            escrow.amount,
        );
    }

    // Read escrow information
    pub fn get_escrow(env: Env, id: u32) -> Escrow {
        env.storage()
            .persistent()
            .get(&DataKey::Escrow(id))
            .expect("Escrow not found")
    }

    // Return total escrows created
    pub fn total(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0)
    }

    // List escrow IDs (simple demo helper)
    pub fn list_ids(env: Env) -> Vec<u32> {
        let total = Self::total(env.clone());
        let mut ids = Vec::new(&env);

        let mut i = 1;
        while i <= total {
            ids.push_back(i);
            i += 1;
        }

        ids
    }
}