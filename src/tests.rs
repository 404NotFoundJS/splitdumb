#[cfg(test)]
mod tests {
    use crate::logic::{
        calculate_balances, calculate_settlements, calculate_simplified_settlements,
    };
    use crate::models::{Expense, Group, User};

    fn create_test_users() -> (User, User, User) {
        (
            User {
                id: 1,
                name: "Alice".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
            },
            User {
                id: 3,
                name: "Charlie".to_string(),
            },
        )
    }

    fn create_expense(
        id: usize,
        description: &str,
        amount: f64,
        payer: User,
        participants: Vec<User>,
    ) -> Expense {
        Expense {
            id,
            description: description.to_string(),
            amount,
            payer,
            participants,
            created_at: "2024-01-01T00:00:00Z".to_string(),
            category: None,
            notes: None,
        }
    }

    fn create_group(members: Vec<User>, expenses: Vec<Expense>) -> Group {
        Group {
            id: 1,
            name: "Test Group".to_string(),
            members,
            expenses,
            simplify_debts: false,
        }
    }

    #[test]
    fn test_calculate_balances_simple() {
        let (alice, bob, _) = create_test_users();
        let group = create_group(
            vec![alice.clone(), bob.clone()],
            vec![create_expense(
                1,
                "Dinner",
                50.0,
                alice.clone(),
                vec![alice.clone(), bob.clone()],
            )],
        );

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 25.0);
        assert_eq!(balances["Bob"], -25.0);
    }

    #[test]
    fn test_calculate_balances_multiple_expenses() {
        let (alice, bob, _) = create_test_users();
        let group = create_group(
            vec![alice.clone(), bob.clone()],
            vec![
                create_expense(
                    1,
                    "Dinner",
                    50.0,
                    alice.clone(),
                    vec![alice.clone(), bob.clone()],
                ),
                create_expense(
                    2,
                    "Museum",
                    30.0,
                    bob.clone(),
                    vec![alice.clone(), bob.clone()],
                ),
            ],
        );

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 10.0);
        assert_eq!(balances["Bob"], -10.0);
    }

    #[test]
    fn test_calculate_balances_no_expenses() {
        let (alice, bob, _) = create_test_users();
        let group = create_group(vec![alice.clone(), bob.clone()], vec![]);

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 0.0);
        assert_eq!(balances["Bob"], 0.0);
    }

    #[test]
    fn test_calculate_balances_three_people() {
        let (alice, bob, charlie) = create_test_users();
        let group = create_group(
            vec![alice.clone(), bob.clone(), charlie.clone()],
            vec![create_expense(
                1,
                "Dinner",
                90.0,
                alice.clone(),
                vec![alice.clone(), bob.clone(), charlie.clone()],
            )],
        );

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 60.0);
        assert_eq!(balances["Bob"], -30.0);
        assert_eq!(balances["Charlie"], -30.0);
    }

    #[test]
    fn test_calculate_balances_partial_split() {
        let (alice, bob, charlie) = create_test_users();
        let group = create_group(
            vec![alice.clone(), bob.clone(), charlie.clone()],
            vec![create_expense(
                1,
                "Movie",
                40.0,
                alice.clone(),
                vec![alice.clone(), bob.clone()],
            )],
        );

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 20.0);
        assert_eq!(balances["Bob"], -20.0);
        assert_eq!(balances["Charlie"], 0.0);
    }

    #[test]
    fn test_settlements_simple() {
        let (alice, bob, _) = create_test_users();
        let group = create_group(
            vec![alice.clone(), bob.clone()],
            vec![create_expense(
                1,
                "Dinner",
                50.0,
                alice.clone(),
                vec![alice.clone(), bob.clone()],
            )],
        );

        let settlements = calculate_settlements(&group);
        assert_eq!(settlements.len(), 1);
        assert_eq!(settlements[0].from, "Bob");
        assert_eq!(settlements[0].to, "Alice");
        assert_eq!(settlements[0].amount, 25.0);
    }

    #[test]
    fn test_settlements_all_settled() {
        let (alice, bob, _) = create_test_users();
        let group = create_group(vec![alice.clone(), bob.clone()], vec![]);

        let settlements = calculate_settlements(&group);
        assert!(settlements.is_empty());
    }

    #[test]
    fn test_settlements_three_people() {
        let (alice, bob, charlie) = create_test_users();
        let group = create_group(
            vec![alice.clone(), bob.clone(), charlie.clone()],
            vec![create_expense(
                1,
                "Dinner",
                90.0,
                alice.clone(),
                vec![alice.clone(), bob.clone(), charlie.clone()],
            )],
        );

        let settlements = calculate_settlements(&group);
        assert_eq!(settlements.len(), 2);

        let total_to_alice: f64 = settlements
            .iter()
            .filter(|s| s.to == "Alice")
            .map(|s| s.amount)
            .sum();
        assert!((total_to_alice - 60.0).abs() < 0.01);
    }

    #[test]
    fn test_settlements_complex_scenario() {
        let (alice, bob, charlie) = create_test_users();
        let group = create_group(
            vec![alice.clone(), bob.clone(), charlie.clone()],
            vec![
                create_expense(
                    1,
                    "Dinner",
                    90.0,
                    alice.clone(),
                    vec![alice.clone(), bob.clone(), charlie.clone()],
                ),
                create_expense(
                    2,
                    "Taxi",
                    30.0,
                    bob.clone(),
                    vec![alice.clone(), bob.clone(), charlie.clone()],
                ),
            ],
        );

        // Use simplified settlements for total balance check
        let settlements = calculate_simplified_settlements(&group);

        // Verify total settlements balance out (only true for simplified algorithm)
        let balances = calculate_balances(&group);
        let total_owed: f64 = balances.values().filter(|&&v| v < 0.0).map(|v| -v).sum();
        let total_settlements: f64 = settlements.iter().map(|s| s.amount).sum();

        assert!((total_owed - total_settlements).abs() < 0.01);
    }

    #[test]
    fn test_pairwise_settlements_stability() {
        let (alice, bob, charlie) = create_test_users();
        let group = create_group(
            vec![alice.clone(), bob.clone(), charlie.clone()],
            vec![
                create_expense(
                    1,
                    "Dinner",
                    90.0,
                    alice.clone(),
                    vec![alice.clone(), bob.clone(), charlie.clone()],
                ),
                create_expense(
                    2,
                    "Taxi",
                    30.0,
                    bob.clone(),
                    vec![alice.clone(), bob.clone(), charlie.clone()],
                ),
            ],
        );

        let settlements = calculate_settlements(&group);

        // With pairwise settlements:
        // Dinner: Bob owes Alice $30, Charlie owes Alice $30
        // Taxi: Alice owes Bob $10, Charlie owes Bob $10
        // Net: Bob owes Alice $20, Charlie owes Alice $30, Charlie owes Bob $10
        assert_eq!(settlements.len(), 3);

        let bob_to_alice = settlements
            .iter()
            .find(|s| s.from == "Bob" && s.to == "Alice");
        let charlie_to_alice = settlements
            .iter()
            .find(|s| s.from == "Charlie" && s.to == "Alice");
        let charlie_to_bob = settlements
            .iter()
            .find(|s| s.from == "Charlie" && s.to == "Bob");

        assert!(bob_to_alice.is_some());
        assert!((bob_to_alice.unwrap().amount - 20.0).abs() < 0.01);

        assert!(charlie_to_alice.is_some());
        assert!((charlie_to_alice.unwrap().amount - 30.0).abs() < 0.01);

        assert!(charlie_to_bob.is_some());
        assert!((charlie_to_bob.unwrap().amount - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_pairwise_settlements_after_settle() {
        let (alice, bob, charlie) = create_test_users();
        let group = create_group(
            vec![alice.clone(), bob.clone(), charlie.clone()],
            vec![
                create_expense(
                    1,
                    "Dinner",
                    90.0,
                    alice.clone(),
                    vec![alice.clone(), bob.clone(), charlie.clone()],
                ),
                create_expense(
                    2,
                    "Taxi",
                    30.0,
                    bob.clone(),
                    vec![alice.clone(), bob.clone(), charlie.clone()],
                ),
                // Bob settles his debt to Alice ($20)
                create_expense(3, "Bob paid Alice", 20.0, bob.clone(), vec![alice.clone()]),
            ],
        );

        let settlements = calculate_settlements(&group);

        // Verify Bob no longer owes Alice
        let bob_to_alice = settlements
            .iter()
            .find(|s| s.from == "Bob" && s.to == "Alice");
        assert!(
            bob_to_alice.is_none(),
            "Bob should not owe Alice after settlement"
        );

        // Verify Charlie's settlements remain unchanged
        let charlie_to_alice = settlements
            .iter()
            .find(|s| s.from == "Charlie" && s.to == "Alice");
        let charlie_to_bob = settlements
            .iter()
            .find(|s| s.from == "Charlie" && s.to == "Bob");

        assert!(charlie_to_alice.is_some());
        assert!(
            (charlie_to_alice.unwrap().amount - 30.0).abs() < 0.01,
            "Charlie→Alice should still be $30"
        );

        assert!(charlie_to_bob.is_some());
        assert!(
            (charlie_to_bob.unwrap().amount - 10.0).abs() < 0.01,
            "Charlie→Bob should still be $10"
        );

        assert_eq!(settlements.len(), 2, "Should have exactly 2 settlements");
    }

    #[test]
    fn test_balances_with_decimal_amounts() {
        let (alice, bob, _) = create_test_users();
        let group = create_group(
            vec![alice.clone(), bob.clone()],
            vec![create_expense(
                1,
                "Coffee",
                7.50,
                alice.clone(),
                vec![alice.clone(), bob.clone()],
            )],
        );

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 3.75);
        assert_eq!(balances["Bob"], -3.75);
    }
}
