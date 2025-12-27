<script lang="ts">
  import { onMount } from "svelte";
  import { currentScreen } from "$lib/store";
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
  let loading = false;
  let editingId: string | null = null; // Track which question is being edited

  // Form state
  let content = "";
  let answer_a = "";
  let answer_b = "";
  let answer_c = "";
  let answer_d = "";
  let correct_answer = "A";

  async function loadQuestions() {
    loading = true;
    try {
      questions = await invoke("get_all_questions");
    } catch (e) {
      console.error(e);
      alert("Lỗi tải câu hỏi: " + e);
    } finally {
      loading = false;
    }
  }

  async function handleSubmit() {
    if (!content || !answer_a || !answer_b || !answer_c || !answer_d) return alert("Vui lòng nhập đầy đủ nội dung!");
    
    try {
      if (editingId) {
        // Update existing
        questions = await invoke("update_question", { 
          id: editingId,
          content, 
          a: answer_a, 
          b: answer_b, 
          c: answer_c,
          d: answer_d,
          correct: correct_answer 
        });
      } else {
        // Add new
        questions = await invoke("add_question", { 
          content, 
          a: answer_a, 
          b: answer_b, 
          c: answer_c,
          d: answer_d,
          correct: correct_answer 
        });
      }
      resetForm();
    } catch (e) {
      alert("Lỗi lưu câu hỏi: " + e);
    }
  }

  function handleEdit(q: Question) {
    editingId = q.id;
    content = q.content;
    answer_a = q.answer_a;
    answer_b = q.answer_b;
    answer_c = q.answer_c;
    answer_d = q.answer_d;
    correct_answer = q.correct_answer;
  }

  async function handleDelete(id: string) {
    if (!confirm("Bạn có chắc chắn muốn xoá câu hỏi này?")) return;
    try {
      questions = await invoke("delete_question", { id });
      if (editingId === id) resetForm();
    } catch (e) {
      alert("Lỗi xoá câu hỏi: " + e);
    }
  }

  function resetForm() {
    editingId = null;
    content = "";
    answer_a = "";
    answer_b = "";
    answer_c = "";
    answer_d = "";
    correct_answer = "A";
  }

  onMount(loadQuestions);
</script>

<div class="flex h-screen w-full bg-[#f4f6f4] overflow-hidden">
  <!-- Sidebar Mini -->
  <aside class="w-14 bg-[#1b331e] flex flex-col items-center py-4 gap-4 shrink-0">
    <button on:click={() => currentScreen.set("DASHBOARD")} class="text-[#a0bca3] hover:text-white transition-colors">
      <iconify-icon icon="solar:alt-arrow-left-bold" class="text-2xl"></iconify-icon>
    </button>
  </aside>

  <main class="flex-1 flex flex-col h-full overflow-hidden p-4 sm:p-6">
    <header class="flex justify-between items-center mb-4 shrink-0">
      <div>
        <h1 class="text-2xl font-heading font-bold text-[#1a2f1a]">Quản Lý Ngân Hàng Câu Hỏi</h1>
      </div>
    </header>

    <div class="flex flex-col lg:flex-row gap-6 flex-1 overflow-hidden">
      <!-- Form thêm mới / Sửa -->
      <div class="w-full lg:w-80 xl:w-96 bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex flex-col shrink-0 overflow-y-auto max-h-[40vh] lg:max-h-full">
        <h3 class="text-lg font-bold mb-4 flex items-center gap-2 {editingId ? 'text-[#ffcd00]' : 'text-[#ce2029]'}">
          <iconify-icon icon={editingId ? "solar:pen-new-square-bold" : "solar:add-circle-bold"}></iconify-icon>
          {editingId ? "Cập Nhật Câu Hỏi" : "Thêm Câu Hỏi Mới"}
        </h3>
        <div class="space-y-3">
          <div>
            <label class="block text-xs font-bold text-gray-700 mb-1 uppercase">Nội dung</label>
            <textarea bind:value={content} class="w-full px-3 py-2 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none min-h-[70px] text-sm" placeholder="Nhập câu hỏi..."></textarea>
          </div>
          
          <div class="grid grid-cols-2 gap-2">
            <div>
              <label class="block text-[10px] font-bold text-gray-500 mb-0.5 uppercase">Đáp án A</label>
              <input bind:value={answer_a} class="w-full px-2 py-1.5 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-xs" placeholder="A..." />
            </div>
            <div>
              <label class="block text-[10px] font-bold text-gray-500 mb-0.5 uppercase">Đáp án B</label>
              <input bind:value={answer_b} class="w-full px-2 py-1.5 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-xs" placeholder="B..." />
            </div>
          </div>

          <div class="grid grid-cols-2 gap-2">
            <div>
              <label class="block text-[10px] font-bold text-gray-500 mb-0.5 uppercase">Đáp án C</label>
              <input bind:value={answer_c} class="w-full px-2 py-1.5 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-xs" placeholder="C..." />
            </div>
            <div>
              <label class="block text-[10px] font-bold text-gray-500 mb-0.5 uppercase">Đáp án D</label>
              <input bind:value={answer_d} class="w-full px-2 py-1.5 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-xs" placeholder="D..." />
            </div>
          </div>

          <div class="flex items-center gap-3">
            <div class="flex-1">
              <label class="block text-xs font-bold text-gray-700 mb-1 uppercase">Đáp án đúng</label>
              <select bind:value={correct_answer} class="w-full px-2 py-1.5 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-sm">
                <option value="A">A</option>
                <option value="B">B</option>
                <option value="C">C</option>
                <option value="D">D</option>
              </select>
            </div>
          </div>

          <button on:click={handleSubmit} class="w-full {editingId ? 'bg-[#ffcd00] text-[#1a2f1a]' : 'bg-[#ce2029] text-white'} py-2.5 rounded-lg font-bold uppercase text-sm hover:opacity-90 transition-all shadow mt-2">
            {editingId ? "Cập Nhật" : "Lưu Câu Hỏi"}
          </button>
          
          {#if editingId}
            <button on:click={resetForm} class="w-full bg-gray-200 text-gray-600 py-2 rounded-lg font-bold uppercase text-xs hover:bg-gray-300 transition-all">
              Hủy bỏ
            </button>
          {/if}
        </div>
      </div>

      <!-- Danh sách câu hỏi -->
      <div class="flex-1 bg-white rounded-xl border border-[#c2cdc2] shadow-sm flex flex-col overflow-hidden">
        <div class="px-5 py-3 border-b border-[#f0f0f0] flex justify-between items-center bg-gray-50/50">
          <h3 class="font-bold text-[#1a2f1a]">Danh Sách ({questions.length})</h3>
        </div>
        
        <div class="flex-1 overflow-y-auto p-4 space-y-3">
          {#if loading}
            <div class="flex flex-col items-center justify-center h-full text-gray-400">
               <iconify-icon icon="line-md:loading-twotone-loop" class="text-4xl mb-2"></iconify-icon>
            </div>
          {:else}
            {#each questions as q}
              <div class="p-3 border border-[#eef2ee] rounded-lg hover:border-[#356839] transition-all group relative bg-white flex gap-4">
                <div class="flex-1 min-w-0">
                  <div class="flex justify-between items-start mb-1">
                    <span class="text-[9px] font-bold text-[#356839] bg-[#dbeedd] px-1.5 py-0.5 rounded uppercase">ID: {q.id.slice(-6)}</span>
                  </div>
                  <p class="font-bold text-[#1a2f1a] mb-2 text-sm leading-tight">{q.content}</p>
                  <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 text-[11px]">
                    <div class="px-2 py-1 rounded {q.correct_answer === 'A' ? 'bg-[#dbeedd] border border-[#356839]' : 'bg-gray-50'} truncate">A: {q.answer_a}</div>
                    <div class="px-2 py-1 rounded {q.correct_answer === 'B' ? 'bg-[#dbeedd] border border-[#356839]' : 'bg-gray-50'} truncate">B: {q.answer_b}</div>
                    <div class="px-2 py-1 rounded {q.correct_answer === 'C' ? 'bg-[#dbeedd] border border-[#356839]' : 'bg-gray-50'} truncate">C: {q.answer_c}</div>
                    <div class="px-2 py-1 rounded {q.correct_answer === 'D' ? 'bg-[#dbeedd] border border-[#356839]' : 'bg-gray-50'} truncate">D: {q.answer_d}</div>
                  </div>
                </div>
                
                <div class="flex flex-col gap-2 justify-start pt-1">
                  <button on:click={() => handleEdit(q)} class="text-gray-400 hover:text-[#ffcd00] transition-colors p-1" title="Sửa">
                    <iconify-icon icon="solar:pen-bold" class="text-xl"></iconify-icon>
                  </button>
                  <button on:click={() => handleDelete(q.id)} class="text-gray-400 hover:text-[#ce2029] transition-colors p-1" title="Xoá">
                    <iconify-icon icon="solar:trash-bin-trash-bold" class="text-xl"></iconify-icon>
                  </button>
                </div>
              </div>
            {/each}
          {/if}
        </div>
      </div>
    </div>
  </main>
</div>