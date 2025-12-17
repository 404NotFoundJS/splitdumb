import React, { useState } from 'react';
import { createUser } from '../services/api';

interface UserFormProps {
    onUserAdded: () => void;
}

const UserForm: React.FC<UserFormProps> = ({ onUserAdded }) => {
    const [name, setName] = useState('');
    const [error, setError] = useState<string | null>(null);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        if (!name) return;

        setError(null);
        try {
            await createUser(name);
            setName('');
            onUserAdded();
        } catch (err: any) {
            setError(err.response?.data?.error || 'Failed to add user');
        }
    };

    return (
        <div className="card form-card">
            <div className="card-body">
                <h5 className="card-title">👤 Add New User</h5>
                {error && (
                    <div className="alert alert-danger" style={{ marginBottom: '1rem' }}>
                        {error}
                    </div>
                )}
                <form onSubmit={handleSubmit}>
                    <div className="form-group">
                        <label htmlFor="userName" className="form-label">Name</label>
                        <input
                            id="userName"
                            type="text"
                            className="form-control"
                            placeholder="Enter name"
                            value={name}
                            onChange={(e) => setName(e.target.value)}
                            required
                        />
                    </div>
                    <button type="submit" className="btn btn-primary w-100">Add User</button>
                </form>
            </div>
        </div>
    );
};

export default UserForm;
