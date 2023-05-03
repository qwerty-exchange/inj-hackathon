#[cfg(test)]
mod tests {
    use crate::helpers::CwTemplateContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const ALICE: &str = "inj1hn2qgt0yn8g5u4gqrn9myjj4ruae2sykv2haqu";
    const BOB: &str = "inj1wxlx8uecfa0mnzv43x9gdvp0kfpxc4ugusqsjw";
    const ADMIN: &str = "inj1mwdzyanq9fpvp5dt7qrv460c98w528kwfn63y4";

    const INJ_DENOM: &str = "inj";
    const ATOM_DENOM: &str = "atom";
    const USDT_DENOM: &str = "usdt";
    const SOL_DENOM: &str = "sol";

    fn mock_app() -> App {
        let init_coins_balance = vec![
            Coin {
                denom: INJ_DENOM.to_string(),
                amount: Uint128::new(1000),
            },
            Coin {
                denom: ATOM_DENOM.to_string(),
                amount: Uint128::new(1000),
            },
            Coin {
                denom: USDT_DENOM.to_string(),
                amount: Uint128::new(1000),
            },
            Coin {
                denom: SOL_DENOM.to_string(),
                amount: Uint128::new(1000),
            },
        ];

        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked(ALICE), init_coins_balance.clone())
                .unwrap();

            router
                .bank
                .init_balance(storage, &Addr::unchecked(BOB), init_coins_balance.clone())
                .unwrap();

            router
                .bank
                .init_balance(storage, &Addr::unchecked(ADMIN), init_coins_balance.clone())
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());

        let msg = InstantiateMsg {};
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    mod proposition_ask_test {

        use cosmwasm_std::{coin, Timestamp};

        use super::*;
        use crate::{
            state::Proposition,
            msg::{ExecuteMsg, QueryMsg},
        };

        fn init_test_case() -> (App, CwTemplateContract) {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::CreateProposition {
                proposition_type: crate::msg::PropositionType::Ask,
                deposit: Coin {
                    denom: INJ_DENOM.to_string(),
                    amount: Uint128::new(100),
                },
                assets: Coin {
                    denom: ATOM_DENOM.to_string(),
                    amount: Uint128::new(100),
                },
                premium: Coin {
                    denom: USDT_DENOM.to_string(),
                    amount: Uint128::new(100),
                },
                period: 100,
                expiry: app.block_info().time.seconds() + 100,
                contractor: None,
            };

            let cosmos_msg = cw_template_contract
                .call_with_funds(
                    msg,
                    [
                        Coin {
                            denom: INJ_DENOM.to_string(),
                            amount: Uint128::new(100),
                        },
                        Coin {
                            denom: USDT_DENOM.to_string(),
                            amount: Uint128::new(100),
                        },
                    ]
                    .to_vec(),
                )
                .unwrap();


            let _result = app.execute(Addr::unchecked(ALICE), cosmos_msg).unwrap();
            let msg = QueryMsg::GetPropositions {
                start_before: None,
                limit: None,
            };
            let _result = app.wrap().query_wasm_smart::<Vec<(u64, Proposition)>>(
                cw_template_contract.addr().clone(),
                &msg,
            );

            (app, cw_template_contract)
        }

        fn bob_accept_state() -> (App, CwTemplateContract) {
            let (mut app, cw_template_contract) = init_test_case();

            let msg = ExecuteMsg::AcceptProposition { proposition_id: 1 };

            let cosmos_msg = cw_template_contract
                .call_with_funds(
                    msg,
                    [Coin {
                        denom: ATOM_DENOM.to_string(),
                        amount: Uint128::new(100),
                    }]
                    .to_vec(),
                )
                .unwrap();

            let _result = app.execute(Addr::unchecked(BOB), cosmos_msg).unwrap();

            (app, cw_template_contract)
        }

        #[test]
        fn test_init() {
            let (app, cw_template_contract) = init_test_case();

            is_account_balance(
                &app,
                Addr::unchecked(ALICE),
                &[
                    coin(Uint128::new(900).into(), INJ_DENOM),
                    coin(Uint128::new(1000).into(), ATOM_DENOM),
                    coin(Uint128::new(900).into(), USDT_DENOM),
                ],
            );

           
            is_account_balance(
                &app,
                Addr::unchecked(cw_template_contract.addr()),
                &[
                    coin(Uint128::new(100).into(), INJ_DENOM),
                    coin(Uint128::new(0).into(), ATOM_DENOM),
                    coin(Uint128::new(100).into(), USDT_DENOM),
                ],
            );
        }

        #[test]
        fn test_alice_reject() {
            let (mut app, cw_template_contract) = init_test_case();

            let msg = ExecuteMsg::RejectProposition { proposition_id: 1 };

            let cosmos_msg = cw_template_contract
                .call_with_funds(msg, [].to_vec())
                .unwrap();

            let _result = app.execute(Addr::unchecked(ALICE), cosmos_msg).unwrap();

            is_account_balance(
                &app,
                Addr::unchecked(ALICE),
                &[
                    coin(Uint128::new(1000).into(), INJ_DENOM),
                    coin(Uint128::new(1000).into(), ATOM_DENOM),
                    coin(Uint128::new(1000).into(), USDT_DENOM),
                ],
            );

           
            is_account_balance(
                &app,
                Addr::unchecked(cw_template_contract.addr()),
                &[
                    coin(Uint128::new(0).into(), INJ_DENOM),
                    coin(Uint128::new(0).into(), ATOM_DENOM),
                    coin(Uint128::new(0).into(), USDT_DENOM),
                ],
            );
        }

        fn is_account_balance(app: &App, addr: Addr, coins: &[Coin]) {
            let balances = app.wrap().query_all_balances(addr).unwrap();

            for coin in coins {
                if !has_coins2(&balances, coin) && !coin.amount.is_zero() {
                    panic!("Incorrect {}{}", coin.amount.to_string(),coin.denom);
                }
            }
        }

        pub fn has_coins2(coins: &[Coin], required: &Coin) -> bool {
            coins
                .iter()
                .find(|c| c.denom == required.denom)
                .map(|m| m.amount == required.amount)
                .unwrap_or(false)
        }

        #[test]
        fn test_bob_accept() {
            let (app, cw_template_contract) = bob_accept_state();

            is_account_balance(
                &app,
                Addr::unchecked(ALICE),
                &[
                    coin(Uint128::new(900).into(), INJ_DENOM),
                    coin(Uint128::new(1100).into(), ATOM_DENOM),
                    coin(Uint128::new(900).into(), USDT_DENOM),
                ],
            );    

            is_account_balance(
                &app,
                Addr::unchecked(BOB),
                &[
                    coin(Uint128::new(1000).into(), INJ_DENOM),
                    coin(Uint128::new(900).into(), ATOM_DENOM),
                    coin(Uint128::new(1100).into(), USDT_DENOM),
                ],
            );

            is_account_balance(
                &app,
                Addr::unchecked(cw_template_contract.addr()),
                &[
                    coin(Uint128::new(100).into(), INJ_DENOM),
                    coin(Uint128::new(0).into(), ATOM_DENOM),
                    coin(Uint128::new(0).into(), USDT_DENOM),
                ],
            );
        }

        #[test]
        fn test_alice_close() {
            let (mut app, cw_template_contract) = bob_accept_state();

            let msg = ExecuteMsg::CloseProposition { proposition_id: 1 };

            let cosmos_msg = cw_template_contract
                .call_with_funds(
                    msg,
                    [Coin {
                        denom: ATOM_DENOM.to_string(),
                        amount: Uint128::new(100),
                    }]
                    .to_vec(),
                )
                .unwrap();

            let _result = app.execute(Addr::unchecked(ALICE), cosmos_msg).unwrap();

            is_account_balance(
                &app,
                Addr::unchecked(ALICE),
                &[
                    coin(Uint128::new(1000).into(), INJ_DENOM),
                    coin(Uint128::new(1000).into(), ATOM_DENOM),
                    coin(Uint128::new(900).into(), USDT_DENOM),
                ],
            );    

            is_account_balance(
                &app,
                Addr::unchecked(BOB),
                &[
                    coin(Uint128::new(1000).into(), INJ_DENOM),
                    coin(Uint128::new(1000).into(), ATOM_DENOM),
                    coin(Uint128::new(1100).into(), USDT_DENOM),
                ],
            );

            is_account_balance(
                &app,
                Addr::unchecked(cw_template_contract.addr()),
                &[
                    coin(Uint128::new(0).into(), INJ_DENOM),
                    coin(Uint128::new(0).into(), ATOM_DENOM),
                    coin(Uint128::new(0).into(), USDT_DENOM),
                ],
            );
        }

        #[test]
        fn test_bob_try_close() {
            let (mut app, cw_template_contract) = bob_accept_state();

            let msg = ExecuteMsg::CloseProposition { proposition_id: 1 };

            let cosmos_msg = cw_template_contract
                .call_with_funds(msg, [].to_vec())
                .unwrap();

            let result = app.execute(Addr::unchecked(BOB), cosmos_msg.clone());

            assert!(result.is_err());

            app.update_block(|f| f.time = Timestamp::from_seconds(9999999999));

            let result = app.execute(Addr::unchecked(BOB), cosmos_msg.clone());
            assert!(result.is_ok());

            is_account_balance(
                &app,
                Addr::unchecked(ALICE),
                &[
                    coin(Uint128::new(900).into(), INJ_DENOM),
                    coin(Uint128::new(1100).into(), ATOM_DENOM),
                    coin(Uint128::new(900).into(), USDT_DENOM),
                ],
            );    

            is_account_balance(
                &app,
                Addr::unchecked(BOB),
                &[
                    coin(Uint128::new(1100).into(), INJ_DENOM),
                    coin(Uint128::new(900).into(), ATOM_DENOM),
                    coin(Uint128::new(1100).into(), USDT_DENOM),
                ],
            );

            is_account_balance(
                &app,
                Addr::unchecked(cw_template_contract.addr()),
                &[
                    coin(Uint128::new(0).into(), INJ_DENOM),
                    coin(Uint128::new(0).into(), ATOM_DENOM),
                    coin(Uint128::new(0).into(), USDT_DENOM),
                ],
            );
        }
    }

    mod proposition_bid_test {

        use cosmwasm_std::Timestamp;

        use super::*;
        use crate::{
            state::Proposition,
            msg::{ExecuteMsg, QueryMsg},
        };

        fn init_test_case() -> (App, CwTemplateContract) {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::CreateProposition {
                proposition_type: crate::msg::PropositionType::Bid,
                deposit: Coin {
                    denom: INJ_DENOM.to_string(),
                    amount: Uint128::new(100),
                },
                assets: Coin {
                    denom: ATOM_DENOM.to_string(),
                    amount: Uint128::new(100),
                },
                premium: Coin {
                    denom: USDT_DENOM.to_string(),
                    amount: Uint128::new(100),
                },
                period: 100,
                expiry: app.block_info().time.seconds() + 100,
                contractor: None,
            };

            let cosmos_msg = cw_template_contract
                .call_with_funds(
                    msg,
                    [Coin {
                        denom: ATOM_DENOM.to_string(),
                        amount: Uint128::new(100),
                    }]
                    .to_vec(),
                )
                .unwrap();

            let _result = app.execute(Addr::unchecked(ALICE), cosmos_msg).unwrap();

            let msg = QueryMsg::GetPropositions {
                start_before: None,
                limit: None,
            };
            let _result = app.wrap().query_wasm_smart::<Vec<(u64, Proposition)>>(
                cw_template_contract.addr().clone(),
                &msg,
            );

            (app, cw_template_contract)
        }

        fn bob_accept_state() -> (App, CwTemplateContract) {
            let (mut app, cw_template_contract) = init_test_case();

            let msg = ExecuteMsg::AcceptProposition { proposition_id: 1 };

            let cosmos_msg = cw_template_contract
                .call_with_funds(
                    msg,
                    [
                        Coin {
                            denom: INJ_DENOM.to_string(),
                            amount: Uint128::new(100),
                        },
                        Coin {
                            denom: USDT_DENOM.to_string(),
                            amount: Uint128::new(100),
                        },
                    ]
                    .to_vec(),
                )
                .unwrap();

            let _result = app.execute(Addr::unchecked(BOB), cosmos_msg).unwrap();

            (app, cw_template_contract)
        }

        #[test]
        fn test_init() {
            let (app, cw_template_contract) = init_test_case();

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), INJ_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1000)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), USDT_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1000)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(900)
            );

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(100)
            );
        }

        #[test]
        fn test_alice_reject() {
            let (mut app, cw_template_contract) = init_test_case();

            let msg = ExecuteMsg::RejectProposition { proposition_id: 1 };

            let cosmos_msg = cw_template_contract
                .call_with_funds(msg, [].to_vec())
                .unwrap();

            let _result = app.execute(Addr::unchecked(ALICE), cosmos_msg).unwrap();

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), INJ_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1000)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1000)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), USDT_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1000)
            );

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), INJ_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(0)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(0)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), USDT_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(0)
            );
        }

        #[test]
        fn test_bob_accept() {
            let (app, cw_template_contract) = bob_accept_state();

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), INJ_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1000)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(900)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), USDT_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1100)
            );

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(BOB), INJ_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(900)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(BOB), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1100)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(BOB), USDT_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(900)
            );

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), INJ_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(100)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), USDT_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(0)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(0)
            );
        }

        #[test]
        fn test_bob_close() {
            let (mut app, cw_template_contract) = bob_accept_state();

            let msg = ExecuteMsg::CloseProposition { proposition_id: 1 };

            let cosmos_msg = cw_template_contract
                .call_with_funds(
                    msg,
                    [Coin {
                        denom: ATOM_DENOM.to_string(),
                        amount: Uint128::new(100),
                    }]
                    .to_vec(),
                )
                .unwrap();

            let _result = app.execute(Addr::unchecked(BOB), cosmos_msg).unwrap();

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), INJ_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1000)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1000)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), USDT_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1100)
            );

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(BOB), INJ_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1000)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(BOB), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1000)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(BOB), USDT_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(900)
            );

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), INJ_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(0)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), USDT_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(0)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(0)
            );
        }

        #[test]
        fn test_alice_try_close() {
            let (mut app, cw_template_contract) = bob_accept_state();

            let msg = ExecuteMsg::CloseProposition { proposition_id: 1 };

            let cosmos_msg = cw_template_contract
                .call_with_funds(msg, [].to_vec())
                .unwrap();

            let result = app.execute(Addr::unchecked(ALICE), cosmos_msg.clone());

            assert!(result.is_err());

            app.update_block(|f| f.time = Timestamp::from_seconds(9999999999));

            let result = app.execute(Addr::unchecked(ALICE), cosmos_msg.clone());
            assert!(result.is_ok());

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), INJ_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1100)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(900)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(ALICE), USDT_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1100)
            );

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(BOB), INJ_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(900)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(BOB), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(1100)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(BOB), USDT_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(900)
            );

            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), INJ_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(0)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), USDT_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(0)
            );
            assert_eq!(
                app.wrap()
                    .query_balance(Addr::unchecked(cw_template_contract.addr()), ATOM_DENOM)
                    .unwrap()
                    .amount,
                Uint128::new(0)
            );
        }
    }
}
