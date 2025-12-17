import axios from 'axios';

const API_URL = 'http://localhost:3000/api';

export const getGroup = async () => {
    const response = await axios.get(`${API_URL}/group`);
    return response.data;
};

export const getBalances = async () => {
    const response = await axios.get(`${API_URL}/balances`);
    return response.data;
};

export const getSettlements = async () => {
    const response = await axios.get(`${API_URL}/settlements`);
    return response.data;
};

export const createUser = async (name: string) => {
    const response = await axios.post(`${API_URL}/users`, { name });
    return response.data;
};

export const deleteUser = async (id: number) => {
    const response = await axios.delete(`${API_URL}/users/${id}`);
    return response.data;
};

export const createExpense = async (description: string, amount: number, payer: string, participants: string[], category?: string, notes?: string) => {
    const response = await axios.post(`${API_URL}/expenses`, { description, amount, payer, participants, category, notes });
    return response.data;
};

export const deleteExpense = async (id: number) => {
    const response = await axios.delete(`${API_URL}/expenses/${id}`);
    return response.data;
};

export const listGroups = async () => {
    const response = await axios.get(`${API_URL}/groups`);
    return response.data;
};

export const createGroup = async (name: string) => {
    const response = await axios.post(`${API_URL}/groups`, { name });
    return response.data;
};

export const switchGroup = async (groupId: number) => {
    const response = await axios.put(`${API_URL}/groups/current`, { group_id: groupId });
    return response.data;
};

export const updateGroup = async (groupId: number, name: string) => {
    const response = await axios.put(`${API_URL}/groups/${groupId}`, { name });
    return response.data;
};

export const deleteGroup = async (groupId: number) => {
    const response = await axios.delete(`${API_URL}/groups/${groupId}`);
    return response.data;
};
