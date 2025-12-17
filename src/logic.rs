use crate::models::{Expense, Group};
use std::collections::HashMap;

pub fn add_expense(expense: Expense, group: &mut Group) {
    group.expenses.push(expense);
}

pub fn calculate_balances(group: &Group) -> HashMap<String, f64> {
    let mut balances = HashMap::new();

    for member in &group.members {
        balances.insert(member.name.clone(), 0.0);
    }

    for expense in &group.expenses {
        let payer = &expense.payer;
        let amount = expense.amount;
        let participants = &expense.participants;
        let share = amount / participants.len() as f64;

        // Payer gets credited
        *balances.get_mut(&payer.name).unwrap() += amount;

        // Participants get debited
        for participant in participants {
            *balances.get_mut(&participant.name).unwrap() -= share;
        }
    }

    balances
}
