use serde::Serialize;
use std::collections::HashMap;

use crate::models::{Expense, Group};

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

#[derive(Debug, Clone, Serialize)]
pub struct Settlement {
    pub from: String,
    pub to: String,
    pub amount: f64,
}

pub fn calculate_settlements(group: &Group) -> Vec<Settlement> {
    let balances = calculate_balances(group);
    let mut settlements = Vec::new();

    // Separate debtors and creditors
    let mut debtors: Vec<(String, f64)> = balances
        .iter()
        .filter(|(_, balance)| **balance < -0.01)
        .map(|(name, balance)| (name.clone(), -balance))
        .collect();

    let mut creditors: Vec<(String, f64)> = balances
        .iter()
        .filter(|(_, balance)| **balance > 0.01)
        .map(|(name, balance)| (name.clone(), *balance))
        .collect();

    // Sort for consistent results
    debtors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Match debtors with creditors
    let mut i = 0;
    let mut j = 0;

    while i < debtors.len() && j < creditors.len() {
        let debt = debtors[i].1;
        let credit = creditors[j].1;
        let amount = debt.min(credit);

        if amount > 0.01 {
            settlements.push(Settlement {
                from: debtors[i].0.clone(),
                to: creditors[j].0.clone(),
                amount: (amount * 100.0).round() / 100.0,
            });
        }

        debtors[i].1 -= amount;
        creditors[j].1 -= amount;

        if debtors[i].1 < 0.01 {
            i += 1;
        }
        if creditors[j].1 < 0.01 {
            j += 1;
        }
    }

    settlements
}
