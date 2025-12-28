<script lang="ts">
  import { onMount } from 'svelte';
  import { currentScreen, quizResult, userProfile } from '$lib/store';
  import { invoke } from "@tauri-apps/api/core";

  interface Question {
    id: string;
    content: string;
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
      questions = await invoke("get_all_questions");
      if (questions.length === 0) {
        alert("Ngân hàng câu hỏi trống! Vui lòng liên hệ Admin.");
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
    quizResult.set({
      score,
      total: questions.length,
      correct: correctCount,
      categoryName: "Kiểm tra tổng hợp"
    });
    currentScreen.set('RESULT');
  }

  onMount(loadQuestions);
</script>

<div class="flex h-screen w-full bg-[#f5f5f0] font-sans text-[#1a2f1a] overflow-hidden">
  {#if loading}
    <div class="flex-1 flex flex-col items-center justify-center">
       <iconify-icon icon="line-md:loading-twotone-loop" class="text-6xl text-[#356839] mb-4"></iconify-icon>
       <p class="font-bold text-lg">Đang chuẩn bị đề thi...</p>
    </div>
  {:else}
    <main class="flex-1 flex flex-col h-full bg-[#f5f5f0] overflow-hidden">
      <!-- Header -->
      <header class="bg-white border-b-2 border-[#356839] px-6 py-3 flex items-center justify-between shrink-0 shadow-sm">
        <div class="flex items-center gap-2">
          <div class="size-6 rounded-full bg-[#356839] flex items-center justify-center">
            <iconify-icon icon="solar:clock-circle-linear" class="text-sm text-white"></iconify-icon>
          </div>
          <span class="text-sm font-bold">
            Thời gian: <span class="text-[#ce2029]">{formatTime(timeLeft)}</span>
          </span>
        </div>
        <div class="flex items-center gap-3">
          <div class="size-12 rounded-full bg-[#356839] flex items-center justify-center relative border-4 border-[#ffcd00]">
            <iconify-icon icon="solar:star-bold" class="text-xl text-[#ffcd00]"></iconify-icon>
          </div>
          <div class="text-left">
            <h2 class="text-base font-bold">Câu {currentIdx + 1}/{questions.length}</h2>
            <p class="text-xs text-gray-600">Chiến dịch Hồ Chí Minh 1975</p>
          </div>
        </div>
        <div class="flex items-center gap-3">
          <span class="text-xs text-gray-600">Quản trị viên Cấp tướng 1979</span>
          <div class="size-10 rounded-full bg-[#356839] flex items-center justify-center text-white font-bold">
            {$userProfile.name?.charAt(0) || 'A'}
          </div>
        </div>
      </header>

      <div class="flex-1 overflow-y-auto p-8 relative pr-96">
        <div class="max-w-3xl mx-auto space-y-4">
          <!-- Question Card -->
          <div class="bg-white rounded-xl p-6 border border-gray-200 shadow-sm">
            <div class="flex items-start gap-4">
              <div class="size-12 rounded-lg bg-[#356839] text-white flex items-center justify-center font-bold text-lg shrink-0">
                {String(currentIdx + 1).padStart(2, '0')}
              </div>
              <div class="flex-1">
                <div class="flex items-center gap-2 mb-3">
                  <span class="px-3 py-1 bg-[#e8f5e9] text-[#356839] text-xs font-bold rounded">LỊCH SỬ</span>
                  <span class="px-3 py-1 bg-red-50 text-red-600 text-xs font-bold rounded">KHÓ</span>
                </div>
                <h3 class="text-base font-medium text-[#1a2f1a] leading-relaxed">
                  {questions[currentIdx].content}
                </h3>
              </div>
            </div>
          </div>

          <!-- Answer Options -->
          <div class="space-y-3">
            {#each [['A', questions[currentIdx].answer_a], ['B', questions[currentIdx].answer_b], ['C', questions[currentIdx].answer_c], ['D', questions[currentIdx].answer_d]] as [key, text]}
              <button
                on:click={() => handleSelect(key)}
                class="w-full rounded-xl p-4 flex items-center gap-4 transition-all text-left border-2
                {userAnswers[currentIdx] === key ? 'border-[#2d5730] bg-[#2d5730]' : 'bg-white border-gray-200 hover:border-[#356839]'}"
              >
                <div class="size-10 rounded-full flex items-center justify-center font-bold text-base shrink-0 border-2
                  {userAnswers[currentIdx] === key ? 'bg-white text-[#2d5730] border-white' : 'bg-white text-[#1a2f1a] border-gray-300'}">
                  {key}
                </div>
                <div class="flex-1 font-normal {userAnswers[currentIdx] === key ? 'text-white' : 'text-[#1a2f1a]'}">{text}</div>
                {#if userAnswers[currentIdx] === key}
                  <div class="shrink-0">
                    <iconify-icon icon="solar:check-circle-bold" class="text-2xl text-white"></iconify-icon>
                  </div>
                {/if}
              </button>
            {/each}
          </div>

          <!-- Navigation Buttons -->
          <div class="flex items-center justify-between pt-4">
            <button
              disabled={currentIdx === 0}
              on:click={() => currentIdx--}
              class="px-6 py-2.5 bg-[#e6dec6] text-[#4a422f] rounded-lg font-medium disabled:opacity-50 flex items-center gap-2 hover:bg-[#d9d1b8]"
            >
              <iconify-icon icon="solar:arrow-left-linear"></iconify-icon> Câu trước
            </button>
            <button
              on:click={handleFinish}
              class="px-6 py-2.5 bg-[#ce2029] text-white rounded-lg font-medium hover:bg-[#b51c24] transition-all flex items-center gap-2"
            >
              <iconify-icon icon="solar:check-circle-bold"></iconify-icon> Nộp bài
            </button>
            <button
              disabled={currentIdx === questions.length - 1}
              on:click={() => currentIdx++}
              class="px-6 py-2.5 bg-[#356839] text-white rounded-lg font-medium disabled:opacity-50 flex items-center gap-2 hover:bg-[#2d5730]"
            >
              Câu tiếp <iconify-icon icon="solar:arrow-right-linear"></iconify-icon>
            </button>
          </div>
        </div>

        <!-- Floating Right Card - Question Navigator -->
        <aside class="fixed top-24 right-8 w-80 bg-white rounded-2xl shadow-xl border border-gray-200 overflow-hidden">
          <div class="p-5 border-b border-gray-200 bg-white">
            <h3 class="font-bold text-base flex items-center gap-2">
              <iconify-icon icon="solar:list-bold" class="text-lg"></iconify-icon>
              Danh sách câu hỏi
            </h3>
          </div>

          <div class="p-5 space-y-4 max-h-[calc(100vh-200px)] overflow-y-auto">
            <!-- Question Grid -->
            <div class="grid grid-cols-5 gap-2.5">
              {#each questions as _, i}
                <button
                  on:click={() => currentIdx = i}
                  class="aspect-square rounded-lg font-bold text-sm flex items-center justify-center transition-all
                  {currentIdx === i ? 'bg-[#356839] text-white border-2 border-[#ffcd00] shadow-lg' :
                   userAnswers[i] !== null ? 'bg-[#c8e6c9] text-[#2d5730]' : 'bg-[#e8f0e8] text-gray-600 hover:bg-[#d4e7d7]'}"
                >
                  {i + 1}
                </button>
              {/each}
            </div>

            <hr class="border-gray-200">

            <!-- Legend -->
            <div class="space-y-2.5 text-xs">
              <div class="flex items-center gap-2.5">
                <div class="size-5 rounded-md bg-[#c8e6c9]"></div>
                <span class="text-gray-700 font-medium">Đã trả lời</span>
              </div>
              <div class="flex items-center gap-2.5">
                <div class="size-5 rounded-md bg-[#356839] border-2 border-[#ffcd00]"></div>
                <span class="text-gray-700 font-medium">Câu hiện tại</span>
              </div>
              <div class="flex items-center gap-2.5">
                <div class="size-5 rounded-md bg-[#e8f0e8]"></div>
                <span class="text-gray-700 font-medium">Chưa trả lời</span>
              </div>
            </div>

            <!-- Stats Cards -->
            <div class="grid grid-cols-2 gap-3 pt-2">
              <div class="bg-[#c8e6c9] rounded-xl p-4 text-center">
                <div class="text-4xl font-bold text-[#1a2f1a]">{userAnswers.filter(a => a !== null).length}</div>
                <div class="text-xs text-gray-700 mt-1.5 font-medium">Đã làm</div>
              </div>
              <div class="bg-[#c8e6c9] rounded-xl p-4 text-center">
                <div class="text-4xl font-bold text-[#1a2f1a]">{questions.length - userAnswers.filter(a => a !== null).length}</div>
                <div class="text-xs text-gray-700 mt-1.5 font-medium">Còn lại</div>
              </div>
            </div>
          </div>
        </aside>
      </div>
    </main>
  {/if}
</div>
