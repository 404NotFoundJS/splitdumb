#[cfg(test)]
mod tests {
    use crate::logic::{calculate_balances, calculate_settlements};
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

    #[test]
    fn test_calculate_balances_simple() {
        let (alice, bob, _) = create_test_users();

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![alice.clone(), bob.clone()],
            expenses: vec![create_expense(
                1,
                "Dinner",
                50.0,
                alice.clone(),
                vec![alice.clone(), bob.clone()],
            )],
        };

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 25.0);
        assert_eq!(balances["Bob"], -25.0);
    }

    #[test]
    fn test_calculate_balances_multiple_expenses() {
        let (alice, bob, _) = create_test_users();

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![alice.clone(), bob.clone()],
            expenses: vec![
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
        };

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 10.0);
        assert_eq!(balances["Bob"], -10.0);
    }

    #[test]
    fn test_calculate_balances_no_expenses() {
        let (alice, bob, _) = create_test_users();

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![alice.clone(), bob.clone()],
            expenses: vec![],
        };

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 0.0);
        assert_eq!(balances["Bob"], 0.0);
    }

    #[test]
    fn test_calculate_balances_three_people() {
        let (alice, bob, charlie) = create_test_users();

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![alice.clone(), bob.clone(), charlie.clone()],
            expenses: vec![create_expense(
                1,
                "Dinner",
                90.0,
                alice.clone(),
                vec![alice.clone(), bob.clone(), charlie.clone()],
            )],
        };

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 60.0);
        assert_eq!(balances["Bob"], -30.0);
        assert_eq!(balances["Charlie"], -30.0);
    }

    #[test]
    fn test_calculate_balances_partial_split() {
        let (alice, bob, charlie) = create_test_users();

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![alice.clone(), bob.clone(), charlie.clone()],
            expenses: vec![create_expense(
                1,
                "Movie",
                40.0,
                alice.clone(),
                vec![alice.clone(), bob.clone()],
            )],
        };

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 20.0);
        assert_eq!(balances["Bob"], -20.0);
        assert_eq!(balances["Charlie"], 0.0);
    }

    #[test]
    fn test_settlements_simple() {
        let (alice, bob, _) = create_test_users();

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![alice.clone(), bob.clone()],
            expenses: vec![create_expense(
                1,
                "Dinner",
                50.0,
                alice.clone(),
                vec![alice.clone(), bob.clone()],
            )],
        };

        let settlements = calculate_settlements(&group);
        assert_eq!(settlements.len(), 1);
        assert_eq!(settlements[0].from, "Bob");
        assert_eq!(settlements[0].to, "Alice");
        assert_eq!(settlements[0].amount, 25.0);
    }

    #[test]
    fn test_settlements_all_settled() {
        let (alice, bob, _) = create_test_users();

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![alice.clone(), bob.clone()],
            expenses: vec![],
        };

        let settlements = calculate_settlements(&group);
        assert!(settlements.is_empty());
    }

    #[test]
    fn test_settlements_three_people() {
        let (alice, bob, charlie) = create_test_users();

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![alice.clone(), bob.clone(), charlie.clone()],
            expenses: vec![create_expense(
                1,
                "Dinner",
                90.0,
                alice.clone(),
                vec![alice.clone(), bob.clone(), charlie.clone()],
            )],
        };

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

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![alice.clone(), bob.clone(), charlie.clone()],
            expenses: vec![
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
        };

        let settlements = calculate_settlements(&group);

        // Verify total settlements balance out
        let balances = calculate_balances(&group);
        let total_owed: f64 = balances.values().filter(|&&v| v < 0.0).map(|v| -v).sum();
        let total_settlements: f64 = settlements.iter().map(|s| s.amount).sum();

        assert!((total_owed - total_settlements).abs() < 0.01);
    }

    #[test]
    fn test_balances_with_decimal_amounts() {
        let (alice, bob, _) = create_test_users();

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![alice.clone(), bob.clone()],
            expenses: vec![create_expense(
                1,
                "Coffee",
                7.50,
                alice.clone(),
                vec![alice.clone(), bob.clone()],
            )],
        };

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 3.75);
        assert_eq!(balances["Bob"], -3.75);
    }
}
