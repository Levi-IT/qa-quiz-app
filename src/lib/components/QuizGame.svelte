<script lang="ts">
  import { currentScreen, quizResult } from '$lib/store';

  // State giả lập cho UI giống 100%
  let currentIdx = 4; // Bắt đầu ở câu 5 (index 4) như thiết kế
  let selectedOption: number | null = null;
  
  // Danh sách 20 câu hỏi (Giả lập để render grid bên phải)
  const totalQuestions = 20; 
  
  function handleNext() {
      // Logic đơn giản để chuyển sang màn Result demo
      quizResult.set({ score: 85, total: 100, correct: 17, categoryName: 'Lịch Sử' });
      currentScreen.set('RESULT');
  }
</script>

<div class="flex h-screen w-full bg-[#f4f6f4] font-sans text-[#1a2f1a] overflow-hidden">
  <aside class="w-72 bg-[#1b331e] text-[#eef2ee] flex flex-col h-full border-r border-[#2a4a2e] shadow-xl z-20">
    <div class="p-6 border-b border-[#2a4a2e] flex items-center gap-3">
      <div class="size-10 rounded-full bg-[#ce2029] flex items-center justify-center shrink-0 border-2 border-[#ffcd00]">
        <iconify-icon icon="solar:star-bold" class="text-2xl text-[#ffcd00]"></iconify-icon>
      </div>
      <div>
        <h1 class="font-heading font-bold text-lg leading-tight text-white uppercase tracking-wider">
          QĐND Việt Nam
        </h1>
        <p class="text-[10px] text-[#a0bca3] uppercase tracking-widest">Học Tập & Rèn Luyện</p>
      </div>
    </div>
    
    <div class="flex-1 py-6 px-4 space-y-1 overflow-y-auto">
      <div class="px-3 mb-2 text-xs font-semibold text-[#a0bca3] uppercase tracking-wider">
        Menu Chính
      </div>
      <button on:click={() => currentScreen.set('DASHBOARD')} class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all">
        <iconify-icon icon="solar:home-2-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Trang Chủ</span>
      </button>
      <button class="w-full flex items-center gap-3 px-3 py-3 rounded-lg bg-[#ce2029] text-white shadow-md transition-all">
        <iconify-icon icon="solar:play-circle-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Bắt Đầu Quiz</span>
      </button>
      <button class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all">
        <iconify-icon icon="solar:history-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Lịch Sử Thi</span>
      </button>
      <button class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all">
        <iconify-icon icon="solar:chart-square-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Thống Kê</span>
      </button>
      
      <div class="my-4 border-t border-[#2a4a2e]"></div>
      
      <div class="px-3 mb-2 text-xs font-semibold text-[#a0bca3] uppercase tracking-wider">
        Quản Trị
      </div>
      <button class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all">
        <iconify-icon icon="solar:file-text-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Ngân Hàng Câu Hỏi</span>
      </button>
      <button class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all">
        <iconify-icon icon="solar:users-group-rounded-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Quản Lý Chiến Sĩ</span>
      </button>
      <button class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all">
        <iconify-icon icon="solar:medal-star-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Bảng Xếp Hạng</span>
      </button>
    </div>
    
    <div class="p-4 border-t border-[#2a4a2e] bg-[#152918]">
      <div class="flex items-center gap-3">
        <div class="relative">
          <img
            alt="User Avatar"
            src="https://randomuser.me/api/portraits/men/32.jpg"
            class="size-10 rounded-full border-2 border-[#ffcd00]"
          />
          <div class="absolute -bottom-1 -right-1 bg-[#ce2029] text-white text-[10px] font-bold px-1 rounded border border-[#1b331e]">
            Đ/c
          </div>
        </div>
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-white truncate">Nguyễn Văn An</p>
          <p class="text-xs text-[#ffcd00] truncate">Thượng úy</p>
        </div>
        <button on:click={() => currentScreen.set('LOGIN')} class="text-[#a0bca3] hover:text-white" aria-label="Đăng xuất">
          <iconify-icon icon="solar:logout-2-bold" class="text-xl"></iconify-icon>
        </button>
      </div>
    </div>
  </aside>

  <main class="flex-1 flex flex-col h-full bg-[#f4f6f4] overflow-hidden">
    <header class="h-16 bg-white border-b border-[#c2cdc2] flex items-center justify-between px-8 shrink-0 z-10 shadow-sm">
      <div class="flex items-center gap-6">
        <button on:click={() => currentScreen.set('DASHBOARD')} class="flex items-center gap-2 text-gray-500 hover:text-[#ce2029] transition-colors">
          <iconify-icon icon="solar:logout-2-bold" class="text-xl"></iconify-icon>
          <span class="text-sm font-medium">Thoát</span>
        </button>
        <div class="h-8 w-px bg-[#c2cdc2]"></div>
        <div class="flex items-center gap-2">
          <iconify-icon icon="solar:clock-circle-bold" class="text-xl text-[#356839]"></iconify-icon>
          <span class="text-sm font-bold text-[#1a2f1a]">
            Thời gian: <span class="text-[#ce2029]">23:45</span>
          </span>
        </div>
      </div>
      <div class="absolute left-1/2 -translate-x-1/2 flex items-center gap-3">
        <div class="size-10 rounded-full bg-[#356839] flex items-center justify-center border-2 border-[#ffcd00]">
          <iconify-icon icon="solar:star-bold" class="text-xl text-[#ffcd00]"></iconify-icon>
        </div>
        <div>
          <h2 class="text-lg font-heading font-bold text-[#1a2f1a]">Câu 5/20</h2>
          <p class="text-xs text-gray-500">Chiến dịch Hồ Chí Minh 1975</p>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <span class="text-xs text-gray-500">ID: QZ-2024-0521</span>
      </div>
    </header>

    <div class="h-2 bg-[#e2e8e2] shrink-0">
      <div style="width: 25%" class="h-full bg-[#356839]"></div>
    </div>

    <div class="flex-1 overflow-y-auto p-8">
      <div class="max-w-6xl mx-auto grid grid-cols-1 lg:grid-cols-4 gap-8">
        <div class="lg:col-span-3 space-y-6">
          <div class="bg-white rounded-xl p-8 border border-[#c2cdc2] shadow-sm">
            <div class="flex items-start gap-4 mb-6">
              <div class="size-12 rounded-lg bg-[#356839] text-white flex items-center justify-center font-heading font-bold text-xl shrink-0">
                05
              </div>
              <div class="flex-1">
                <div class="flex items-center gap-2 mb-3">
                  <span class="px-2 py-0.5 rounded text-xs font-bold bg-[#dbeedd] text-[#356839] uppercase">
                    Lịch Sử
                  </span>
                  <span class="px-2 py-0.5 rounded text-xs font-bold bg-[#ffe8e9] text-[#ce2029] uppercase">
                    Khó
                  </span>
                </div>
                <h3 class="text-xl font-bold text-[#1a2f1a] leading-relaxed">
                  Chiến dịch Hồ Chí Minh bắt đầu vào ngày nào và kéo dài trong bao lâu? Hãy chọn
                  câu trả lời chính xác nhất về mốc thời gian lịch sử quan trọng này của dân tộc
                  Việt Nam.
                </h3>
              </div>
            </div>
          </div>

          <div class="space-y-4">
            <button class="w-full bg-white border-2 border-[#c2cdc2] rounded-xl p-6 flex items-start gap-4 hover:border-[#356839] hover:bg-[#dbeedd] transition-all group text-left">
              <div class="size-12 rounded-lg bg-[#e2e8e2] text-[#1a2f1a] flex items-center justify-center font-heading font-bold text-xl shrink-0 group-hover:bg-[#356839] group-hover:text-white transition-colors">
                A
              </div>
              <div class="flex-1 pt-2">
                <p class="text-base text-[#1a2f1a] font-medium">
                  26 tháng 3 năm 1975, kéo dài 55 ngày đêm
                </p>
              </div>
            </button>
            
            <button class="w-full bg-[#356839] border-2 border-[#356839] rounded-xl p-6 flex items-start gap-4 shadow-md text-left">
              <div class="size-12 rounded-lg bg-white text-[#356839] flex items-center justify-center font-heading font-bold text-xl shrink-0">
                B
              </div>
              <div class="flex-1 pt-2 flex items-center justify-between">
                <p class="text-base text-white font-medium">
                  10 tháng 4 năm 1975, kéo dài 20 ngày đêm
                </p>
                <iconify-icon icon="solar:check-circle-bold" class="text-2xl text-white shrink-0 ml-4"></iconify-icon>
              </div>
            </button>

            <button class="w-full bg-white border-2 border-[#c2cdc2] rounded-xl p-6 flex items-start gap-4 hover:border-[#356839] hover:bg-[#dbeedd] transition-all group text-left">
              <div class="size-12 rounded-lg bg-[#e2e8e2] text-[#1a2f1a] flex items-center justify-center font-heading font-bold text-xl shrink-0 group-hover:bg-[#356839] group-hover:text-white transition-colors">
                C
              </div>
              <div class="flex-1 pt-2">
                <p class="text-base text-[#1a2f1a] font-medium">
                  15 tháng 3 năm 1975, kéo dài 45 ngày đêm
                </p>
              </div>
            </button>

            <button class="w-full bg-white border-2 border-[#c2cdc2] rounded-xl p-6 flex items-start gap-4 hover:border-[#356839] hover:bg-[#dbeedd] transition-all group text-left">
              <div class="size-12 rounded-lg bg-[#e2e8e2] text-[#1a2f1a] flex items-center justify-center font-heading font-bold text-xl shrink-0 group-hover:bg-[#356839] group-hover:text-white transition-colors">
                D
              </div>
              <div class="flex-1 pt-2">
                <p class="text-base text-[#1a2f1a] font-medium">
                  30 tháng 4 năm 1975, kéo dài 30 ngày đêm
                </p>
              </div>
            </button>
          </div>

          <div class="flex items-center justify-between pt-6 border-t border-[#c2cdc2]">
            <button class="px-6 py-3 bg-[#e6dec6] text-[#4a422f] rounded-xl font-bold hover:bg-[#d4cbb0] transition-colors flex items-center gap-2">
              <iconify-icon icon="solar:arrow-left-linear" class="text-xl"></iconify-icon>
              Câu trước
            </button>
            <button on:click={handleNext} class="px-8 py-3 bg-[#ce2029] text-white rounded-xl font-bold hover:bg-[#b51c24] transition-colors shadow-md flex items-center gap-2">
              <iconify-icon icon="solar:document-add-bold" class="text-xl"></iconify-icon>
              Nộp bài
            </button>
            <button class="px-6 py-3 bg-[#356839] text-white rounded-xl font-bold hover:bg-[#2a4a2e] transition-colors flex items-center gap-2">
              Câu tiếp
              <iconify-icon icon="solar:arrow-right-linear" class="text-xl"></iconify-icon>
            </button>
          </div>
        </div>

        <div class="lg:col-span-1">
          <div class="bg-white rounded-xl p-6 border border-[#c2cdc2] shadow-sm sticky top-8">
            <h3 class="font-heading font-bold text-base mb-4 flex items-center gap-2">
              <iconify-icon icon="solar:layers-minimalistic-bold" class="text-xl text-[#356839]"></iconify-icon>
              Danh sách câu hỏi
            </h3>
            
            <div class="grid grid-cols-5 gap-2 mb-6">
              {#each [1, 2, 3, 4] as item}
                <button class="aspect-square rounded-lg bg-[#dbeedd] text-[#356839] font-bold text-sm flex items-center justify-center hover:bg-[#356839] hover:text-white transition-colors">
                  {item}
                </button>
              {/each}

              <button class="aspect-square rounded-lg bg-[#356839] text-white font-bold text-sm flex items-center justify-center shadow-md border-2 border-[#ffcd00]">
                5
              </button>

              {#each Array(15) as _, i}
                <button class="aspect-square rounded-lg bg-[#e2e8e2] text-gray-500 font-bold text-sm flex items-center justify-center hover:bg-[#356839] hover:text-white transition-colors">
                  {i + 6}
                </button>
              {/each}
            </div>

            <div class="space-y-2 pt-4 border-t border-[#c2cdc2]">
              <div class="flex items-center gap-2 text-xs">
                <div class="size-4 rounded bg-[#dbeedd]"></div>
                <span class="text-gray-500">Đã trả lời</span>
              </div>
              <div class="flex items-center gap-2 text-xs">
                <div class="size-4 rounded bg-[#356839] border-2 border-[#ffcd00]"></div>
                <span class="text-gray-500">Câu hiện tại</span>
              </div>
              <div class="flex items-center gap-2 text-xs">
                <div class="size-4 rounded bg-[#e2e8e2]"></div>
                <span class="text-gray-500">Chưa trả lời</span>
              </div>
            </div>

            <div class="mt-6 p-4 bg-[#dbeedd] rounded-lg">
              <div class="grid grid-cols-2 gap-4 text-center">
                <div>
                  <p class="text-2xl font-bold text-[#356839]">4</p>
                  <p class="text-xs text-gray-500">Đã làm</p>
                </div>
                <div>
                  <p class="text-2xl font-bold text-gray-500">16</p>
                  <p class="text-xs text-gray-500">Còn lại</p>
                </div>
              </div>
            </div>
          </div>
        </div>

      </div>
    </div>
  </main>
</div>