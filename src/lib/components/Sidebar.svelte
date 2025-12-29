<script lang="ts">
  import { currentScreen, userProfile } from "$lib/store";

  async function handleLogout() {
    userProfile.set({
      uid: '',
      name: '',
      rank: 'Binh nhì',
      unit: '',
      isAdmin: false,
      isLoggedIn: false,
      isOffline: false,
      email: ''
    });
    currentScreen.set("LOGIN");
  }
</script>

<aside class="w-72 bg-[#1b331e] text-[#eef2ee] flex flex-col h-full border-r border-[#2a4a2e] shadow-xl z-20 shrink-0">
    <div class="p-6 border-b border-[#2a4a2e] flex items-center gap-3">
      <div class="size-10 rounded-full bg-[#ce2029] flex items-center justify-center shrink-0 border-2 border-[#ffcd00]">
        <iconify-icon icon="solar:star-bold" class="text-2xl text-[#ffcd00]"></iconify-icon>
      </div>
      <div>
        <h1 class="font-heading font-bold text-lg leading-tight text-white uppercase tracking-wider">
          QĐND Việt Nam
        </h1>
        <p class="text-[10px] text-[#a0bca3] uppercase tracking-widest">
          Học Tập & Rèn Luyện
        </p>
      </div>
    </div>

    <div class="flex-1 py-6 px-4 space-y-1 overflow-y-auto">
      <div class="px-3 mb-2 text-xs font-semibold text-[#a0bca3] uppercase tracking-wider">
        Menu Chính
      </div>
      <button 
        onclick={() => currentScreen.set("DASHBOARD")}
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg {$currentScreen === 'DASHBOARD' ? 'bg-[#ce2029] text-white shadow-md' : 'text-[#eef2ee] hover:bg-[#2a4a2e]'} transition-all"
      >
        <iconify-icon icon="solar:home-2-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Trang Chủ</span>
      </button>
      <button
        onclick={() => currentScreen.set("CATEGORIES")}
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg {$currentScreen === 'CATEGORIES' ? 'bg-[#ce2029] text-white shadow-md' : 'text-[#eef2ee] hover:bg-[#2a4a2e]'} transition-all"
      >
        <iconify-icon icon="solar:play-circle-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Bắt Đầu Quiz</span>
      </button>

      <button
        onclick={() => {
          currentScreen.set("HISTORY")
        }}
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg {$currentScreen === 'HISTORY' ? 'bg-[#ce2029] text-white shadow-md' : 'text-[#eef2ee] hover:bg-[#2a4a2e]'} transition-all"
      >
        <iconify-icon icon="solar:history-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Lịch Sử Thi</span>
      </button>

      <button
        onclick={() => {
          // currentScreen.set("STATISTICS")
        }}
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg {$currentScreen === 'STATISTICS' ? 'bg-[#ce2029] text-white shadow-md' : 'text-[#eef2ee] hover:bg-[#2a4a2e]'} transition-all"
      >
        <iconify-icon icon="solar:chart-2-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Thống Kê</span>
      </button>
      
      {#if $userProfile.isAdmin}
        <div class="my-4 border-t border-[#2a4a2e]"></div>
        <div class="px-3 mb-2 text-xs font-semibold text-[#a0bca3] uppercase tracking-wider">
          Quản Trị
        </div>
        <button
          onclick={() => currentScreen.set("MANAGER")}
          class="w-full flex items-center gap-3 px-3 py-3 rounded-lg {$currentScreen === 'MANAGER' ? 'bg-[#ffcd00] text-[#1a2f1a]' : 'text-[#eef2ee] hover:bg-[#2a4a2e]'} transition-all"
        >
          <iconify-icon icon="solar:file-text-bold" class="text-xl"></iconify-icon>
          <span class="font-medium">Ngân Hàng Câu Hỏi</span>
        </button>

        <button
          onclick={() => {
            // currentScreen.set("USER_MANAGER")
          }}
          class="w-full flex items-center gap-3 px-3 py-3 rounded-lg {$currentScreen === 'USER_MANAGER' ? 'bg-[#ffcd00] text-[#1a2f1a]' : 'text-[#eef2ee] hover:bg-[#2a4a2e]'} transition-all"
        >
          <iconify-icon icon="solar:users-group-rounded-bold" class="text-xl"></iconify-icon>
          <span class="font-medium">Quản Lý Chiến Sỹ</span>
        </button>

        <button
          onclick={() => {
            currentScreen.set("LEADERBOARD")
          }}
          class="w-full flex items-center gap-3 px-3 py-3 rounded-lg {$currentScreen === 'LEADERBOARD' ? 'bg-[#ffcd00] text-[#1a2f1a]' : 'text-[#eef2ee] hover:bg-[#2a4a2e]'} transition-all"
        >
          <iconify-icon icon="solar:cup-star-bold" class="text-xl"></iconify-icon>
          <span class="font-medium">Bảng Xếp Hạng</span>
        </button>
      {/if}
    </div>

    <div class="p-4 border-t border-[#2a4a2e] bg-[#152918]">
      <div class="flex items-center gap-3">
        <div class="relative">
          <div class="size-10 rounded-full bg-[#356839] flex items-center justify-center border-2 border-[#ffcd00]">
            <span class="text-white font-bold">{$userProfile.name.charAt(0)}</span>
          </div>
        </div>
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-white truncate">{$userProfile.name}</p>
          <p class="text-xs text-[#ffcd00] truncate">{$userProfile.rank}</p>
        </div>
        <button onclick={handleLogout} class="text-[#a0bca3] hover:text-white" aria-label="Đăng xuất">
          <iconify-icon icon="solar:logout-2-bold" class="text-xl"></iconify-icon>
        </button>
      </div>
    </div>
</aside>