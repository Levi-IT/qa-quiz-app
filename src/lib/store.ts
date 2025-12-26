import { writable } from 'svelte/store';

// Các màn hình trong App
export type Screen = 'LOGIN' | 'DASHBOARD' | 'CATEGORIES' | 'QUIZ' | 'RESULT';

export const currentScreen = writable<Screen>('LOGIN');

// Thông tin chiến sĩ
export const userProfile = writable({
    name: '',
    rank: 'Binh nhì',
    unit: ''
});

// Kết quả thi tạm thời
export const quizResult = writable({
    score: 0,
    total: 0,
    correct: 0,
    categoryName: ''
});