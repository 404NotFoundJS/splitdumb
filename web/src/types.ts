export type User = {
    id: number;
    name: string;
};

export type Expense = {
    id: number;
    description: string;
    amount: number;
    payer: User;
    participants: User[];
    created_at: string;
    category?: string;
};

export type Group = {
    id: number;
    name: string;
    members: User[];
    expenses: Expense[];
};

export type Settlement = {
    from: string;
    to: string;
    amount: number;
};

export type BalanceResponse = {
    balances: Record<string, number>;
};

export type SettlementsResponse = {
    settlements: Settlement[];
};