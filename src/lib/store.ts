import { writable } from 'svelte/store';

// Các màn hình trong App
export type Screen = 'LOGIN' | 'DASHBOARD' | 'CATEGORIES' | 'QUIZ' | 'RESULT' | 'MANAGER' | 'HISTORY' | 'STATISTICS' | 'USER_MANAGER' | 'LEADERBOARD';

export const currentScreen = writable<Screen>('LOGIN');

// Thông tin chiến sĩ
export const userProfile = writable({
    uid: '',
    name: '',
    rank: 'Binh nhì',
    unit: '',
    isAdmin: false,
    isLoggedIn: false,
    isOffline: false,
    email: ''
});

export const isAppOnline = writable(true);

export const selectedCategory = writable<{id: string, name: string} | null>(null);

// Kết quả thi tạm thời
export const quizResult = writable({
    score: 0,
    total: 0,
    correct: 0,
    categoryName: ''
});