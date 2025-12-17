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
        let share = expense.amount / expense.participants.len() as f64;
        *balances.get_mut(&expense.payer.name).unwrap() += expense.amount;
        for participant in &expense.participants {
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
    #[serde(default)]
    pub settled: bool,
}

pub fn calculate_settlements(group: &Group) -> Vec<Settlement> {
    let mut debts: HashMap<String, HashMap<String, f64>> = HashMap::new();

    for member in &group.members {
        debts.insert(member.name.clone(), HashMap::new());
    }

    for expense in &group.expenses {
        let payer = &expense.payer.name;
        let share = expense.amount / expense.participants.len() as f64;

        for participant in &expense.participants {
            if participant.name != *payer {
                *debts
                    .get_mut(&participant.name)
                    .unwrap()
                    .entry(payer.clone())
                    .or_insert(0.0) += share;
            }
        }
    }

    let mut settlements = Vec::new();
    let mut processed: HashMap<(String, String), bool> = HashMap::new();

    for (person_a, owes) in &debts {
        for (person_b, amount_a_owes_b) in owes {
            let pair_key = if person_a < person_b {
                (person_a.clone(), person_b.clone())
            } else {
                (person_b.clone(), person_a.clone())
            };

            if processed.contains_key(&pair_key) {
                continue;
            }
            processed.insert(pair_key, true);

            let amount_b_owes_a = debts
                .get(person_b)
                .and_then(|d| d.get(person_a))
                .copied()
                .unwrap_or(0.0);

            let net = amount_a_owes_b - amount_b_owes_a;
            let rounded = (net.abs() * 100.0).round() / 100.0;

            if rounded > 0.01 {
                let (from, to) = if net > 0.0 {
                    (person_a.clone(), person_b.clone())
                } else {
                    (person_b.clone(), person_a.clone())
                };
                settlements.push(Settlement {
                    from,
                    to,
                    amount: rounded,
                    settled: false,
                });
            }
        }
    }

    settlements.sort_by(|a, b| a.from.cmp(&b.from).then_with(|| a.to.cmp(&b.to)));
    settlements
}

pub fn calculate_simplified_settlements(group: &Group) -> Vec<Settlement> {
    let balances = calculate_balances(group);
    let mut settlements = Vec::new();

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

    debtors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then_with(|| a.0.cmp(&b.0)));
    creditors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then_with(|| a.0.cmp(&b.0)));

    let mut i = 0;
    let mut j = 0;

    while i < debtors.len() && j < creditors.len() {
        let amount = debtors[i].1.min(creditors[j].1);

        if amount > 0.01 {
            settlements.push(Settlement {
                from: debtors[i].0.clone(),
                to: creditors[j].0.clone(),
                amount: (amount * 100.0).round() / 100.0,
                settled: false,
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
