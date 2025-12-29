<script lang="ts">
  import { onMount } from 'svelte';
  import { currentScreen, quizResult, userProfile, selectedCategory } from '$lib/store';
  import { get } from 'svelte/store';
  import { invoke } from "@tauri-apps/api/core";

  interface Question {
    id: string;
    content: string;
    category: string;
    answer_a: string;
    answer_b: string;
    answer_c: string;
    answer_d: string;
    correct_answer: string;
  }

  let questions: Question[] = [];
  let currentIdx = 0;
  let userAnswers: (string | null)[] = [];
  let timeLeft = 20 * 60; // 20 minutes
  let timerInterval: any;
  let loading = true;

  async function loadQuestions() {
    try {
      const allQuestions: Question[] = await invoke("get_all_questions");
      const category = get(selectedCategory);
      
      if (category) {
        questions = allQuestions.filter(q => q.category === category.name);
      } else {
        questions = allQuestions;
      }

      if (questions.length === 0) {
        alert("Không tìm thấy câu hỏi nào cho chủ đề này! Vui lòng liên hệ Admin.");
        currentScreen.set("DASHBOARD");
        return;
      }
      userAnswers = new Array(questions.length).fill(null);
      loading = false;
      startTimer();
    } catch (e) {
      alert("Lỗi tải câu hỏi: " + e);
      currentScreen.set("DASHBOARD");
    }
  }

  function startTimer() {
    timerInterval = setInterval(() => {
      if (timeLeft > 0) {
        timeLeft--;
      } else {
        handleFinish();
      }
    }, 1000);
  }

  function formatTime(seconds: number) {
    const m = Math.floor(seconds / 60);
    const s = seconds % 60;
    return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  }

  function handleSelect(option: string) {
    userAnswers[currentIdx] = option;
  }

  function handleFinish() {
    clearInterval(timerInterval);
    let correctCount = 0;
    questions.forEach((q, i) => {
      if (userAnswers[i] === q.correct_answer) {
        correctCount++;
      }
    });

    const score = Math.round((correctCount / questions.length) * 100);
    const category = get(selectedCategory);
    quizResult.set({
      score,
      total: questions.length,
      correct: correctCount,
      categoryName: category ? category.name : "Kiểm tra tổng hợp"
    });
    currentScreen.set('RESULT');
  }

  onMount(loadQuestions);
</script>

<div class="flex h-screen w-full bg-[#f4f6f4] font-sans text-[#1a2f1a] overflow-hidden">
  {#if loading}
    <div class="flex-1 flex flex-col items-center justify-center">
       <iconify-icon icon="line-md:loading-twotone-loop" class="text-6xl text-[#356839] mb-4"></iconify-icon>
       <p class="font-bold text-lg">Đang chuẩn bị đề thi...</p>
    </div>
  {:else}
    <aside class="w-72 bg-[#1b331e] text-[#eef2ee] flex flex-col h-full border-r border-[#2a4a2e] shadow-xl z-20">
      <div class="p-6 border-b border-[#2a4a2e] flex items-center gap-3">
        <div class="size-10 rounded-full bg-[#ce2029] flex items-center justify-center shrink-0 border-2 border-[#ffcd00]">
          <iconify-icon icon="solar:star-bold" class="text-2xl text-[#ffcd00]"></iconify-icon>
        </div>
        <div>
          <h1 class="font-heading font-bold text-lg leading-tight text-white uppercase tracking-wider">QĐND Việt Nam</h1>
          <p class="text-[10px] text-[#a0bca3] uppercase tracking-widest">Học Tập & Rèn Luyện</p>
        </div>
      </div>
      
      <div class="flex-1 py-6 px-4 overflow-y-auto">
        <h3 class="px-3 mb-4 text-xs font-semibold text-[#a0bca3] uppercase tracking-wider">Danh sách câu hỏi</h3>
        <div class="grid grid-cols-4 gap-2">
          {#each questions as _, i}
            <button 
              on:click={() => currentIdx = i}
              class="aspect-square rounded-lg font-bold text-sm flex items-center justify-center transition-all
              {currentIdx === i ? 'bg-[#ffcd00] text-[#1a2f1a] ring-2 ring-white' : 
               userAnswers[i] !== null ? 'bg-[#356839] text-white' : 'bg-[#2a4a2e] text-[#a0bca3]'}"
            >
              {i + 1}
            </button>
          {/each}
        </div>
      </div>

      <div class="p-4 border-t border-[#2a4a2e] bg-[#152918]">
        <div class="flex items-center gap-3">
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-white truncate">{$userProfile.name}</p>
            <p class="text-xs text-[#ffcd00] truncate">{$userProfile.rank}</p>
          </div>
          <button on:click={() => { if(confirm("Bạn muốn bỏ thi?")) currentScreen.set('DASHBOARD') }} class="text-[#ce2029] font-bold text-xs uppercase hover:underline">Thoát</button>
        </div>
      </div>
    </aside>

    <main class="flex-1 flex flex-col h-full bg-[#f4f6f4] overflow-hidden">
      <header class="h-16 bg-white border-b border-[#c2cdc2] flex items-center justify-between px-8 shrink-0 z-10 shadow-sm">
        <div class="flex items-center gap-6">
          <div class="flex items-center gap-2">
            <iconify-icon icon="solar:clock-circle-bold" class="text-xl text-[#356839]"></iconify-icon>
            <span class="text-sm font-bold text-[#1a2f1a]">
              Thời gian: <span class="text-[#ce2029]">{formatTime(timeLeft)}</span>
            </span>
          </div>
        </div>
        <div class="absolute left-1/2 -translate-x-1/2 flex items-center gap-3">
          <h2 class="text-lg font-heading font-bold text-[#1a2f1a]">Câu {currentIdx + 1}/{questions.length}</h2>
        </div>
        <button on:click={handleFinish} class="px-6 py-2 bg-[#ce2029] text-white rounded-lg font-bold hover:bg-[#b51c24] transition-all shadow-md">
          Nộp bài
        </button>
      </header>

      <div class="h-2 bg-[#e2e8e2] shrink-0">
        <div style="width: {((currentIdx + 1) / questions.length) * 100}%" class="h-full bg-[#356839] transition-all duration-300"></div>
      </div>

      <div class="flex-1 overflow-y-auto p-8">
        <div class="max-w-4xl mx-auto space-y-8">
          <div class="bg-white rounded-xl p-8 border border-[#c2cdc2] shadow-sm">
            <h3 class="text-xl font-bold text-[#1a2f1a] leading-relaxed">
              {questions[currentIdx].content}
            </h3>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {#each [['A', questions[currentIdx].answer_a], ['B', questions[currentIdx].answer_b], ['C', questions[currentIdx].answer_c], ['D', questions[currentIdx].answer_d]] as [key, text]}
              <button 
                on:click={() => handleSelect(key)}
                class="w-full bg-white border-2 rounded-xl p-6 flex items-start gap-4 transition-all text-left
                {userAnswers[currentIdx] === key ? 'border-[#356839] bg-[#dbeedd]' : 'border-[#c2cdc2] hover:border-[#356839]'}"
              >
                <div class="size-10 rounded-lg flex items-center justify-center font-heading font-bold text-lg shrink-0
                  {userAnswers[currentIdx] === key ? 'bg-[#356839] text-white' : 'bg-[#e2e8e2] text-[#1a2f1a]'}">
                  {key}
                </div>
                <div class="flex-1 pt-1 font-medium text-[#1a2f1a]">{text}</div>
              </button>
            {/each}
          </div>

          <div class="flex items-center justify-between pt-6">
            <button 
              disabled={currentIdx === 0}
              on:click={() => currentIdx--}
              class="px-6 py-3 bg-[#e6dec6] text-[#4a422f] rounded-xl font-bold disabled:opacity-50 flex items-center gap-2"
            >
              <iconify-icon icon="solar:arrow-left-linear"></iconify-icon> Câu trước
            </button>
            <button 
              disabled={currentIdx === questions.length - 1}
              on:click={() => currentIdx++}
              class="px-6 py-3 bg-[#356839] text-white rounded-xl font-bold disabled:opacity-50 flex items-center gap-2"
            >
              Câu tiếp <iconify-icon icon="solar:arrow-right-linear"></iconify-icon>
            </button>
          </div>
        </div>
      </div>
    </main>
  {/if}
</div>
