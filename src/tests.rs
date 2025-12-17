#[cfg(test)]
mod tests {
    use crate::logic::calculate_balances;
    use crate::models::{Expense, Group, User};

    #[test]
    fn test_calculate_balances_simple() {
        let user1 = User {
            id: 1,
            name: "Alice".to_string(),
        };
        let user2 = User {
            id: 2,
            name: "Bob".to_string(),
        };

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![user1.clone(), user2.clone()],
            expenses: vec![Expense {
                id: 1,
                description: "Dinner".to_string(),
                amount: 50.0,
                payer: user1.clone(),
                participants: vec![user1.clone(), user2.clone()],
                created_at: "2024-01-01T00:00:00Z".to_string(),
                category: None,
                notes: None,
            }],
        };

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 25.0);
        assert_eq!(balances["Bob"], -25.0);
    }

    #[test]
    fn test_calculate_balances_multiple_expenses() {
        let user1 = User {
            id: 1,
            name: "Alice".to_string(),
        };
        let user2 = User {
            id: 2,
            name: "Bob".to_string(),
        };

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![user1.clone(), user2.clone()],
            expenses: vec![
                Expense {
                    id: 1,
                    description: "Dinner".to_string(),
                    amount: 50.0,
                    payer: user1.clone(),
                    participants: vec![user1.clone(), user2.clone()],
                    created_at: "2024-01-01T00:00:00Z".to_string(),
                    category: None,
                    notes: None,
                },
                Expense {
                    id: 2,
                    description: "Museum".to_string(),
                    amount: 30.0,
                    payer: user2.clone(),
                    participants: vec![user1.clone(), user2.clone()],
                    created_at: "2024-01-01T00:00:00Z".to_string(),
                    category: None,
                    notes: None,
                },
            ],
        };

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 10.0);
        assert_eq!(balances["Bob"], -10.0);
    }

    #[test]
    fn test_calculate_balances_no_expenses() {
        let user1 = User {
            id: 1,
            name: "Alice".to_string(),
        };
        let user2 = User {
            id: 2,
            name: "Bob".to_string(),
        };

        let group = Group {
            id: 1,
            name: "Test Group".to_string(),
            members: vec![user1.clone(), user2.clone()],
            expenses: vec![],
        };

        let balances = calculate_balances(&group);
        assert_eq!(balances["Alice"], 0.0);
        assert_eq!(balances["Bob"], 0.0);
    }
}
