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

/// Calculate pairwise settlements - stable, each pair is independent
pub fn calculate_settlements(group: &Group) -> Vec<Settlement> {
    // Track pairwise debts: debts[A][B] = how much A owes B
    let mut debts: HashMap<String, HashMap<String, f64>> = HashMap::new();

    // Initialize for all members
    for member in &group.members {
        debts.insert(member.name.clone(), HashMap::new());
    }

    // Calculate pairwise debts from each expense
    for expense in &group.expenses {
        let payer = &expense.payer.name;
        let participants = &expense.participants;
        let share = expense.amount / participants.len() as f64;

        for participant in participants {
            if participant.name != *payer {
                // participant owes payer their share
                *debts
                    .get_mut(&participant.name)
                    .unwrap()
                    .entry(payer.clone())
                    .or_insert(0.0) += share;
            }
        }
    }

    // Net out mutual debts and create settlements
    let mut settlements = Vec::new();
    let mut processed: HashMap<(String, String), bool> = HashMap::new();

    for (person_a, owes) in &debts {
        for (person_b, amount_a_owes_b) in owes {
            // Skip if already processed this pair
            let pair_key = if person_a < person_b {
                (person_a.clone(), person_b.clone())
            } else {
                (person_b.clone(), person_a.clone())
            };

            if processed.contains_key(&pair_key) {
                continue;
            }
            processed.insert(pair_key, true);

            // Get reverse debt (B owes A)
            let amount_b_owes_a = debts
                .get(person_b)
                .and_then(|d| d.get(person_a))
                .copied()
                .unwrap_or(0.0);

            // Net out
            let net = amount_a_owes_b - amount_b_owes_a;
            let rounded = (net.abs() * 100.0).round() / 100.0;

            if rounded > 0.01 {
                if net > 0.0 {
                    // A owes B
                    settlements.push(Settlement {
                        from: person_a.clone(),
                        to: person_b.clone(),
                        amount: rounded,
                    });
                } else {
                    // B owes A
                    settlements.push(Settlement {
                        from: person_b.clone(),
                        to: person_a.clone(),
                        amount: rounded,
                    });
                }
            }
        }
    }

    // Sort for consistent display
    settlements.sort_by(|a, b| a.from.cmp(&b.from).then_with(|| a.to.cmp(&b.to)));

    settlements
}

/// Calculate simplified settlements - minimizes transactions but may be unstable
pub fn calculate_simplified_settlements(group: &Group) -> Vec<Settlement> {
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

    // Sort for consistent results (by amount desc, then name asc for ties)
    debtors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then_with(|| a.0.cmp(&b.0)));
    creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then_with(|| a.0.cmp(&b.0)));

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
