<script lang="ts">
  import { onMount } from "svelte";
  import { currentScreen, userProfile } from "$lib/store";
  import { get } from "svelte/store";
  import { invoke } from "@tauri-apps/api/core";

  interface Question {
    id: string;
    content: string;
    category: string;
    created_by: string;
    answer_a: string;
    answer_b: string;
    answer_c: string;
    answer_d: string;
    correct_answer: string;
    synced?: boolean;
  }

  let questions = $state<Question[]>([]);
  let loading = $state(false);
  let syncing = $state(false);
  let editingId = $state<string | null>(null);
  
  // Delete Modal State
  let showDeleteModal = $state(false);
  let deleteTargetId = $state<string | null>(null);

  // Form state
  let content = $state("");
  let category = $state("Lý Thuyết Quân Sự");
  let answer_a = $state("");
  let answer_b = $state("");
  let answer_c = $state("");
  let answer_d = $state("");
  let correct_answer = $state("A");
  let currentCreatedBy = $state("");

  const categories = [
    "Lý Thuyết Quân Sự",
    "Vũ Khí Trang Bị",
    "Chính Trị",
    "Pháp Luật",
    "Điều Lệnh",
    "Chuyên Đề Nâng Cao"
  ];

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

  async function handleSync() {
    if (syncing) return;
    syncing = true;
    try {
      await invoke("sync_data");
      await loadQuestions();
      alert("Đồng bộ dữ liệu thành công!");
    } catch (e) {
      console.error(e);
      alert("Lỗi đồng bộ: " + e);
    } finally {
      syncing = false;
    }
  }

  async function handleSubmit() {
    console.log("handleSubmit called");
    if (!content || !answer_a || !answer_b || !answer_c || !answer_d) {
        console.log("Validation failed");
        return alert("Vui lòng nhập đầy đủ nội dung!");
    }
    
    const user = get(userProfile);
    const creatorEmail = user.email || "admin@system.com";

    try {
      if (editingId) {
        console.log("Invoking update_question");
        questions = await invoke("update_question", { 
          id: editingId,
          content, 
          category,
          createdBy: currentCreatedBy || creatorEmail,
          a: answer_a, 
          b: answer_b, 
          c: answer_c, 
          d: answer_d,
          correct: correct_answer 
        });
      } else {
        console.log("Invoking add_question", { 
          content, 
          category,
          createdBy: creatorEmail,
          a: answer_a, 
          b: answer_b, 
          c: answer_c, 
          d: answer_d,
          correct: correct_answer 
        });
        
        // Add new
        questions = await invoke("add_question", { 
          content, 
          category,
          createdBy: creatorEmail,
          a: answer_a, 
          b: answer_b, 
          c: answer_c, 
          d: answer_d,
          correct: correct_answer 
        });
        console.log("add_question returned", questions);
      }
      resetForm();
    } catch (e) {
      console.error("Invoke error:", e);
      alert("Lỗi lưu câu hỏi: " + e);
    }
  }

  function handleEdit(q: Question) {
    editingId = q.id;
    content = q.content;
    category = q.category || "Lý Thuyết Quân Sự";
    currentCreatedBy = q.created_by;
    answer_a = q.answer_a;
    answer_b = q.answer_b;
    answer_c = q.answer_c;
    answer_d = q.answer_d;
    correct_answer = q.correct_answer;
  }

  function requestDelete(id: string) {
    deleteTargetId = id;
    showDeleteModal = true;
  }

  async function confirmDelete() {
    if (!deleteTargetId) return;
    try {
      questions = await invoke("delete_question", { id: deleteTargetId });
      if (editingId === deleteTargetId) resetForm();
    } catch (e) {
      alert("Lỗi xoá câu hỏi: " + e);
    } finally {
        showDeleteModal = false;
        deleteTargetId = null;
    }
  }

  function resetForm() {
    editingId = null;
    content = "";
    category = "Lý Thuyết Quân Sự";
    currentCreatedBy = "";
    answer_a = "";
    answer_b = "";
    answer_c = "";
    answer_d = "";
    correct_answer = "A";
  }

  onMount(loadQuestions);
</script>

<div class="flex h-screen w-full bg-[#f4f6f4] ">
  <!-- Sidebar Mini -->
  <aside class="w-14 bg-[#1b331e] flex flex-col items-center py-4 gap-4 shrink-0">
    <button onclick={() => currentScreen.set("DASHBOARD")} class="text-[#a0bca3] hover:text-white transition-colors" aria-label="Quay lại Dashboard">
      <iconify-icon icon="solar:alt-arrow-left-bold" class="text-2xl"></iconify-icon>
    </button>
  </aside>

  <main class="flex-1 flex flex-col h-full  p-4 sm:p-6">
    <header class="flex justify-between items-center mb-4 shrink-0">
      <div>
        <h1 class="text-2xl font-heading font-bold text-[#1a2f1a]">Quản Lý Ngân Hàng Câu Hỏi</h1>
      </div>
      <button onclick={handleSync} disabled={syncing} class="flex items-center gap-2 bg-[#356839] text-white px-4 py-2 rounded-lg font-bold hover:bg-[#2a522d] transition-colors disabled:opacity-50">
        <iconify-icon icon={syncing ? "line-md:loading-twotone-loop" : "solar:cloud-upload-bold"} class="text-xl"></iconify-icon>
        {syncing ? "Đang đồng bộ..." : "Đồng Bộ Dữ Liệu"}
      </button>
    </header>

    <div class="flex flex-col lg:flex-row gap-6 flex-1">
      <!-- Form thêm mới / Sửa -->
      <div class="w-full lg:w-80 xl:w-96 bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex flex-col shrink-0 overflow-y-auto lg:max-h-full">
        <h3 class="text-lg font-bold mb-4 flex items-center gap-2 {editingId ? 'text-[#ffcd00]' : 'text-[#ce2029]'}">
          <iconify-icon icon={editingId ? "solar:pen-new-square-bold" : "solar:add-circle-bold"}></iconify-icon>
          {editingId ? "Cập Nhật Câu Hỏi" : "Thêm Câu Hỏi Mới"}
        </h3>
        <div class="space-y-3">
          <div>
            <label for="category" class="block text-xs font-bold text-gray-700 mb-1 uppercase">Chủ đề</label>
            <select id="category" bind:value={category} class="w-full px-3 py-2 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-sm">
              {#each categories as cat}
                <option value={cat}>{cat}</option>
              {/each}
            </select>
          </div>

          <div>
            <label for="content" class="block text-xs font-bold text-gray-700 mb-1 uppercase">Nội dung</label>
            <textarea id="content" bind:value={content} class="w-full px-3 py-2 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none min-h-[70px] text-sm" placeholder="Nhập câu hỏi..."></textarea>
          </div>
          
          <div class="grid grid-cols-2 gap-2">
            <div>
              <label for="answer_a" class="block text-[10px] font-bold text-gray-500 mb-0.5 uppercase">Đáp án A</label>
              <input id="answer_a" bind:value={answer_a} class="w-full px-2 py-1.5 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-xs" placeholder="A..." />
            </div>
            <div>
              <label for="answer_b" class="block text-[10px] font-bold text-gray-500 mb-0.5 uppercase">Đáp án B</label>
              <input id="answer_b" bind:value={answer_b} class="w-full px-2 py-1.5 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-xs" placeholder="B..." />
            </div>
          </div>

          <div class="grid grid-cols-2 gap-2">
            <div>
              <label for="answer_c" class="block text-[10px] font-bold text-gray-500 mb-0.5 uppercase">Đáp án C</label>
              <input id="answer_c" bind:value={answer_c} class="w-full px-2 py-1.5 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-xs" placeholder="C..." />
            </div>
            <div>
              <label for="answer_d" class="block text-[10px] font-bold text-gray-500 mb-0.5 uppercase">Đáp án D</label>
              <input id="answer_d" bind:value={answer_d} class="w-full px-2 py-1.5 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-xs" placeholder="D..." />
            </div>
          </div>

          <div class="flex items-center gap-3">
            <div class="flex-1">
              <label for="correct_answer" class="block text-xs font-bold text-gray-700 mb-1 uppercase">Đáp án đúng</label>
              <select id="correct_answer" bind:value={correct_answer} class="w-full px-2 py-1.5 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-sm">
                <option value="A">A</option>
                <option value="B">B</option>
                <option value="C">C</option>
                <option value="D">D</option>
              </select>
            </div>
          </div>

          <button onclick={handleSubmit} class="w-full {editingId ? 'bg-[#ffcd00] text-[#1a2f1a]' : 'bg-[#ce2029] text-white'} py-2.5 rounded-lg font-bold uppercase text-sm hover:opacity-90 transition-all shadow mt-2">
            {editingId ? "Cập Nhật" : "Lưu Câu Hỏi"}
          </button>
          
          {#if editingId}
            <button onclick={resetForm} class="w-full bg-gray-200 text-gray-600 py-2 rounded-lg font-bold uppercase text-xs hover:bg-gray-300 transition-all">
              Hủy bỏ
            </button>
          {/if}
        </div>
      </div>

      <!-- Danh sách câu hỏi -->
      <div class="flex-1 bg-white rounded-xl border border-[#c2cdc2] shadow-sm overflow-scroll">
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
                    <div class="flex flex-wrap gap-2 items-center">
                      <span class="text-[9px] font-bold text-[#356839] bg-[#dbeedd] px-1.5 py-0.5 rounded uppercase">ID: {q.id.slice(-6)}</span>
                      {#if q.synced}
                        <span class="text-[9px] font-bold text-green-700 bg-green-100 px-1.5 py-0.5 rounded uppercase flex items-center gap-1" title="Đã đồng bộ">
                          <iconify-icon icon="solar:check-circle-bold"></iconify-icon> SYNC
                        </span>
                      {:else}
                         <span class="text-[9px] font-bold text-orange-700 bg-orange-100 px-1.5 py-0.5 rounded uppercase flex items-center gap-1" title="Chưa đồng bộ">
                          <iconify-icon icon="solar:cloud-upload-linear"></iconify-icon> NOT SYNC
                        </span>
                      {/if}
                      <span class="text-[9px] font-bold text-blue-700 bg-blue-100 px-1.5 py-0.5 rounded uppercase">{q.category || 'Chưa phân loại'}</span>
                      <span class="text-[9px] font-bold text-gray-600 bg-gray-100 px-1.5 py-0.5 rounded lowercase italic">{q.created_by || 'admin@system.com'}</span>
                    </div>
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
                  <button onclick={() => handleEdit(q)} class="text-gray-400 hover:text-[#ffcd00] transition-colors p-1" title="Sửa" aria-label="Sửa câu hỏi">
                    <iconify-icon icon="solar:pen-bold" class="text-xl"></iconify-icon>
                  </button>
                  <button onclick={() => requestDelete(q.id)} class="text-gray-400 hover:text-[#ce2029] transition-colors p-1" title="Xoá" aria-label="Xoá câu hỏi">
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

  {#if showDeleteModal}
    <div class="fixed inset-0 bg-black/50 z-50 flex items-center justify-center p-4">
      <div class="bg-white rounded-xl shadow-xl p-6 w-full max-w-sm animate-scale-in">
        <div class="text-center mb-6">
          <div class="w-12 h-12 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-3">
            <iconify-icon icon="solar:trash-bin-trash-bold" class="text-2xl text-[#ce2029]"></iconify-icon>
          </div>
          <h3 class="text-lg font-bold text-[#1a2f1a] mb-2">Xác nhận xoá?</h3>
          <p class="text-sm text-gray-500">Bạn có chắc chắn muốn xoá câu hỏi này? Hành động này sẽ được đồng bộ hoá.</p>
        </div>
        <div class="flex gap-3">
          <button onclick={() => showDeleteModal = false} class="flex-1 px-4 py-2 bg-gray-100 text-gray-700 font-bold rounded-lg hover:bg-gray-200 transition-colors">
            Huỷ bỏ
          </button>
          <button onclick={confirmDelete} class="flex-1 px-4 py-2 bg-[#ce2029] text-white font-bold rounded-lg hover:bg-[#b01c23] transition-colors">
            Xoá ngay
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>