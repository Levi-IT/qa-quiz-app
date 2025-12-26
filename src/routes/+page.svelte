<script lang="ts">
  import { onMount } from "svelte";
  // Import từ core của Tauri v2
  import { invoke } from "@tauri-apps/api/core";

  // Định nghĩa kiểu dữ liệu cho rõ ràng (giống Class trong Flutter)
  type Question = {
    id: string;
    content: string;
    answer_a: string;
    answer_b: string;
    correct_answer: string;
  };

  let questions: Question[] = [];
  let form = { content: "", a: "", b: "", correct: "A" };

  // Hàm chạy khi trang vừa load xong (giống initState)
  onMount(() => {
    refreshList();
  });

  async function refreshList() {
    // Gọi hàm 'get_all_questions' từ Rust
    questions = await invoke("get_all_questions");
  }

  async function add() {
    if (!form.content) return;
    // Gọi hàm 'add_question' từ Rust
    questions = await invoke("add_question", {
      content: form.content,
      a: form.a,
      b: form.b,
      correct: form.correct
    });
    // Reset form nhập liệu
    form.content = ""; form.a = ""; form.b = "";
  }

  async function remove(id: string) {
    if(confirm("Bạn có chắc muốn xóa không?")) {
      questions = await invoke("delete_question", { id });
    }
  }
</script>

<main class="container">
  <h1>Quiz App (SvelteKit + Rust)</h1>

  <div class="input-box">
    <input placeholder="Nhập nội dung câu hỏi..." bind:value={form.content} />
    <div class="row">
      <input placeholder="Đáp án A" bind:value={form.a} />
      <input placeholder="Đáp án B" bind:value={form.b} />
    </div>
    <div class="row">
      <!-- svelte-ignore a11y_label_has_associated_control -->
      <label>Đáp án đúng:</label>
      <select bind:value={form.correct}>
        <option value="A">A</option>
        <option value="B">B</option>
      </select>
      <button on:click={add}>Lưu câu hỏi</button>
    </div>
  </div>

  <div class="list">
    {#each questions as q}
      <div class="card">
        <div class="info">
          <strong>{q.content}</strong>
          <p>A: {q.answer_a} | B: {q.answer_b}</p>
          <span class="badge">Đáp án đúng: {q.correct_answer}</span>
        </div>
        <button class="del" on:click={() => remove(q.id)}>Xóa</button>
      </div>
    {/each}
  </div>
</main>

<style>
  /* CSS đơn giản cho đẹp mắt */
  :global(body) { margin: 0; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background-color: #f4f4f9; color: #333; }
  .container { max-width: 700px; margin: 40px auto; padding: 0 20px; }
  h1 { text-align: center; color: #2c3e50; }
  
  .input-box { background: white; padding: 20px; border-radius: 12px; box-shadow: 0 4px 15px rgba(0,0,0,0.05); display: flex; flex-direction: column; gap: 15px; margin-bottom: 30px; }
  input, select, button { padding: 12px; border: 1px solid #ddd; border-radius: 8px; font-size: 16px; outline: none; transition: 0.2s; }
  input:focus, select:focus { border-color: #3498db; }
  
  .row { display: flex; gap: 10px; }
  .row > input { flex: 1; }
  
  button { background-color: #3498db; color: white; border: none; font-weight: 600; cursor: pointer; }
  button:hover { background-color: #2980b9; }

  .list { display: flex; flex-direction: column; gap: 15px; }
  .card { background: white; padding: 20px; border-radius: 10px; display: flex; justify-content: space-between; align-items: center; box-shadow: 0 2px 5px rgba(0,0,0,0.05); border-left: 5px solid #2ecc71; }
  .info p { margin: 5px 0; color: #666; font-size: 0.95em; }
  .badge { display: inline-block; background: #e8f8f5; color: #27ae60; padding: 4px 8px; border-radius: 4px; font-size: 0.85em; font-weight: bold; }
  
  .del { background-color: #e74c3c; width: auto; padding: 8px 16px; font-size: 14px; }
  .del:hover { background-color: #c0392b; }
</style>