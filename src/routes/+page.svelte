<script lang="ts">
  import { currentScreen } from '$lib/store';
  
  import LoginScreen from '$lib/components/LoginScreen.svelte';
  import Dashboard from '$lib/components/Dashboard.svelte';
  import QuizCategories from '$lib/components/QuizCategories.svelte';
  import QuizGame from '$lib/components/QuizGame.svelte';
  import ResultScreen from '$lib/components/ResultScreen.svelte';
</script>

{#if $currentScreen === 'LOGIN'}
    <LoginScreen />
{:else if $currentScreen === 'DASHBOARD'}
    <Dashboard />
{:else if $currentScreen === 'CATEGORIES'}
    <QuizCategories />
{:else if $currentScreen === 'QUIZ'}
    <QuizGame />
{:else if $currentScreen === 'RESULT'}
    <ResultScreen />
{/if}