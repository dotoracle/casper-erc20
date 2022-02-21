#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    use casper_types::{Key, U256};

    use crate::test_fixture::{Sender, TestFixture};

    #[test]
    fn should_install() {
        let fixture = TestFixture::install_contract();
        assert_eq!(fixture.token_name(), TestFixture::TOKEN_NAME);
        assert_eq!(fixture.token_symbol(), TestFixture::TOKEN_SYMBOL);
        assert_eq!(fixture.token_decimals(), TestFixture::TOKEN_DECIMALS);
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
    }

    #[test]
    fn should_transfer() {
        let mut fixture = TestFixture::install_contract();
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
        let transfer_amount_1 = U256::from(42);
        fixture.transfer(
            Key::from(fixture.bob),
            transfer_amount_1,
            Sender(fixture.ali),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(transfer_amount_1)
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply() - transfer_amount_1)
        );

        let transfer_amount_2 = U256::from(20);
        fixture.transfer(
            Key::from(fixture.ali),
            transfer_amount_2,
            Sender(fixture.bob),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply() - transfer_amount_1 + transfer_amount_2),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(transfer_amount_1 - transfer_amount_2)
        );
    }

    #[test]
    fn should_transfer_full_amount() {
        let mut fixture = TestFixture::install_contract();

        let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        fixture.transfer(
            Key::from(fixture.bob),
            initial_ali_balance,
            Sender(fixture.ali),
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(initial_ali_balance)
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(U256::zero())
        );

        fixture.transfer(
            Key::from(fixture.ali),
            initial_ali_balance,
            Sender(fixture.bob),
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(U256::zero())
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(initial_ali_balance)
        );
        let mut bal = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        println!("bal {}, {}", bal, initial_ali_balance);

        //burn
        fixture.burn(initial_ali_balance, Sender(fixture.ali));
        let supply = fixture.total_supply();
        bal = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        println!("bal {}, {}, {}", bal, initial_ali_balance, supply);
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(U256::zero())
        );

        let dev = fixture.dev();
        println!("dev {}", dev);

        //mint
        fixture.mint(Key::from(fixture.bob), U256::from(20), U256::zero(), "mintid1".to_string(), Sender(fixture.minter));
        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(U256::from(20))
        );
    }
    #[test]
    fn should_mint_with_fee() {
        let mut fixture = TestFixture::install_contract();

        let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        let mut dev_balance = fixture.balance_of(Key::from(fixture.dev)).unwrap_or_default();
        //assert_eq!(dev_balance, U256::zero());

        let mut bob_balance = fixture.balance_of(Key::from(fixture.bob)).unwrap_or_default();
        assert_eq!(bob_balance, U256::zero());

        //change dev fee
        fixture.change_swap_fee(U256::from(10), Sender(fixture.dev));
        //mint 1000
        fixture.mint(Key::from(fixture.bob), U256::from(1000), U256::from(10), "mintid1".to_string(), Sender(fixture.minter));

        let new_dev_balance = fixture.balance_of(Key::from(fixture.dev)).unwrap();
        assert_eq!(new_dev_balance, U256::from(10) + dev_balance);

        bob_balance = fixture.balance_of(Key::from(fixture.bob)).unwrap();
        assert_eq!(bob_balance, U256::from(1000 - 10));
    }

    #[test]
    fn should_change_dev() {
        let mut fixture = TestFixture::install_contract();

        //change dev fee
        fixture.change_dev(Key::from(fixture.bob), Sender(fixture.dev));
        
        let dev = fixture.dev();
        assert_eq!(dev, fixture.bob.to_formatted_string());
    }

    #[should_panic(expected = "ApiError::User(65528) [131064]")]
    #[test]
    fn should_not_mint_without_correct_fee() {
        let mut fixture = TestFixture::install_contract();

        //change dev fee
        fixture.change_swap_fee(U256::from(10), Sender(fixture.dev));
        //mint 1000
        fixture.mint(Key::from(fixture.bob), U256::from(1000), U256::from(5), "mintid1".to_string(), Sender(fixture.minter));
    }

    #[should_panic(expected = "ApiError::EarlyEndOfStream [17]")]
    #[test]
    fn should_not_mint_with_dup_key() {
        let mut fixture = TestFixture::install_contract();

        //mint 1000
        fixture.mint(Key::from(fixture.bob), U256::from(1000), U256::from(0), "mintid1".to_string(), Sender(fixture.minter));
        fixture.mint(Key::from(fixture.bob), U256::from(1000), U256::from(0), "mintid1".to_string(), Sender(fixture.minter));
    }

    #[should_panic(expected = "ApiError::User(65527) [131063]")]
    #[test]
    fn should_not_request_bridge_back_with_same_id() {
        let mut fixture = TestFixture::install_contract();
        
        fixture.request_bridge_back(U256::from(1000), U256::zero(), U256::from(1), "receiver_address".to_string(), U256::from(1), Sender(fixture.ali));
        fixture.request_bridge_back(U256::from(1000), U256::zero(), U256::from(1), "receiver_address".to_string(), U256::from(1), Sender(fixture.ali));
    }

    #[test]
    fn should_request_bridge_back_with_good_fee() {
        let mut fixture = TestFixture::install_contract();
        fixture.mint(Key::from(fixture.ali), U256::from(2000), U256::zero(), "mint1".to_string(), Sender(fixture.minter));
        fixture.change_swap_fee(U256::from(10), Sender(fixture.dev));
        fixture.transfer(Key::from(fixture.bob), U256::from(2000), Sender(fixture.ali));

        let dev_balance = fixture.balance_of(Key::from(fixture.dev)).unwrap_or_default();

        //bridge 1000
        fixture.request_bridge_back(U256::from(1000), U256::from(10), U256::from(10), "receiver_address".to_string(), U256::from(1), Sender(fixture.bob));

        //bob should have less than 1k compared
        let bob_balance = fixture.balance_of(Key::from(fixture.bob)).unwrap_or_default();
        assert_eq!(bob_balance, U256::from(1000));

        let new_dev_balance = fixture.balance_of(Key::from(fixture.dev)).unwrap_or_default();

        assert_eq!(new_dev_balance, dev_balance + 10);
    }

    #[test]
    fn should_change_minter_success() {
        let mut fixture = TestFixture::install_contract();

        let mut current_minter = fixture.get_minter();
        assert_eq!(current_minter, fixture.minter.to_formatted_string());

        fixture.change_minter(Key::from(fixture.bob), Sender(fixture.minter));

        current_minter = fixture.get_minter();
        assert_eq!(current_minter, fixture.bob.to_formatted_string());
    }

    #[should_panic(expected = "ApiError::User(65531) [131067]")]
    #[test]
    fn should_not_mint_without_authorization() {
        let mut fixture = TestFixture::install_contract();

        fixture.mint(Key::from(fixture.bob), U256::from(20), U256::zero(), "mintid1".to_string(), Sender(fixture.bob));
    }

    #[should_panic(expected = "ApiError::User(65534) [131070]")]
    #[test]
    fn should_not_transfer_with_insufficient_balance() {
        let mut fixture = TestFixture::install_contract();

        let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        fixture.transfer(
            Key::from(fixture.bob),
            initial_ali_balance + U256::one(),
            Sender(fixture.ali),
        );
    }

    #[test]
    fn should_transfer_from() {
        let approve_amount = U256::from(100);
        let transfer_amount = U256::from(42);
        assert!(approve_amount > transfer_amount);

        let mut fixture = TestFixture::install_contract();

        let owner = fixture.ali;
        let spender = fixture.bob;
        let recipient = fixture.joe;

        let owner_balance_before = fixture
            .balance_of(Key::from(owner))
            .expect("owner should have balance");
        fixture.approve(Key::from(spender), approve_amount, Sender(owner));
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount)
        );

        fixture.transfer_from(
            Key::from(owner),
            Key::from(recipient),
            transfer_amount,
            Sender(spender),
        );

        assert_eq!(
            fixture.balance_of(Key::from(owner)),
            Some(owner_balance_before - transfer_amount),
            "should decrease balance of the owner"
        );
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount - transfer_amount),
            "should decrease allowance of the spender"
        );
        assert_eq!(
            fixture.balance_of(Key::from(recipient)),
            Some(transfer_amount),
            "recipient should receive tokens"
        );
    }

    #[should_panic(expected = "ApiError::User(65533) [131069]")]
    #[test]
    fn should_not_transfer_from_more_than_approved() {
        let approve_amount = U256::from(100);
        let transfer_amount = U256::from(42);
        assert!(approve_amount > transfer_amount);

        let mut fixture = TestFixture::install_contract();

        let owner = fixture.ali;
        let spender = fixture.bob;
        let recipient = fixture.joe;

        fixture.approve(Key::from(spender), approve_amount, Sender(owner));
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount)
        );

        fixture.transfer_from(
            Key::from(owner),
            Key::from(recipient),
            approve_amount + U256::one(),
            Sender(spender),
        );
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
