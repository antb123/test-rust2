#![no_std]
use soroban_sdk::{contractimpl, vec, Env, Symbol, Vec};

pub struct Contract;

#[contractimpl(export_if = "export")]
impl Contract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        const GREETING: Symbol = Symbol::from_str("Hello");
        vec![&env, GREETING, to]
    }
}

#[cfg(test)]
mod test {
    use super::{Contract, hello};
    use soroban_sdk::{vec, Env, FixedBinary, Symbol};

    #[test]
    fn test() {
        let env = Env::default();
        let contract_id = FixedBinary::from_array(&env, [0; 32]);
        env.register_contract(&contract_id, Contract);

        let words = hello::invoke(&env, &contract_id, &Symbol::from_str("Dev"));
        assert_eq!(
            words,
            vec![&env, Symbol::from_str("Hello"), Symbol::from_str("Dev"),]
        );
    }
}