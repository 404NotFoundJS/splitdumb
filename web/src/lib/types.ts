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
  notes?: string;
};

export type Group = {
  id: number;
  name: string;
  members: User[];
  expenses: Expense[];
  simplify_debts: boolean;
};

export type Settlement = {
  from: string;
  to: string;
  amount: number;
  settled: boolean;
};

export type BalanceResponse = {
  balances: Record<string, number>;
};

export type SettlementsResponse = {
  settlements: Settlement[];
};
