<script lang="ts">
  import { currentScreen, userProfile } from "$lib/store";

  async function handleLogout() {
    userProfile.set({
      uid: '',
      name: '',
      rank: 'Binh nhì',
      unit: '',
      isAdmin: false,
      isLoggedIn: false
    });
    currentScreen.set("LOGIN");
  }

  const today = new Intl.DateTimeFormat('vi-VN', { 
    weekday: 'long', 
    year: 'numeric', 
    month: 'long', 
    day: 'numeric' 
  }).format(new Date());
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
        on:click={() => currentScreen.set("DASHBOARD")}
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg {$currentScreen === 'DASHBOARD' ? 'bg-[#ce2029] text-white shadow-md' : 'text-[#eef2ee] hover:bg-[#2a4a2e]'} transition-all"
      >
        <iconify-icon icon="solar:home-2-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Trang Chủ</span>
      </button>
      <button
        on:click={() => currentScreen.set("CATEGORIES")}
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all"
      >
        <iconify-icon icon="solar:play-circle-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Bắt Đầu Quiz</span>
      </button>
      
      {#if $userProfile.isAdmin}
        <div class="my-4 border-t border-[#2a4a2e]"></div>
        <div class="px-3 mb-2 text-xs font-semibold text-[#a0bca3] uppercase tracking-wider">
          Quản Trị
        </div>
        <button
          on:click={() => currentScreen.set("MANAGER")}
          class="w-full flex items-center gap-3 px-3 py-3 rounded-lg {$currentScreen === 'MANAGER' ? 'bg-[#ffcd00] text-[#1a2f1a]' : 'text-[#eef2ee] hover:bg-[#2a4a2e]'} transition-all"
        >
          <iconify-icon icon="solar:file-text-bold" class="text-xl"></iconify-icon>
          <span class="font-medium">Ngân Hàng Câu Hỏi</span>
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
        <button on:click={handleLogout} class="text-[#a0bca3] hover:text-white" aria-label="Đăng xuất">
          <iconify-icon icon="solar:logout-2-bold" class="text-xl"></iconify-icon>
        </button>
      </div>
    </div>
  </aside>

  <main class="flex-1 flex flex-col h-full bg-[#f4f6f4] overflow-hidden relative">
    <header class="h-16 bg-white border-b border-[#c2cdc2] flex items-center justify-between px-8 shrink-0 z-10 shadow-sm">
      <div class="flex items-center gap-4">
        <h2 class="text-xl font-heading font-bold text-[#1a2f1a]">
          Bảng Điều Khiển
        </h2>
      </div>
      <div class="flex items-center gap-4">
        <div class="flex items-center gap-2 text-sm font-medium text-[#1a2f1a]">
          <iconify-icon icon="solar:calendar-bold" class="text-[#356839]"></iconify-icon>
          <span>{today}</span>
        </div>
      </div>
    </header>

    <div class="flex-1 overflow-y-auto p-8 z-10">
      <div class="mb-8 bg-linear-to-r from-[#356839] to-[#2a4a2e] rounded-2xl p-8 text-white shadow-lg relative overflow-hidden">
        <div class="relative z-10 max-w-2xl">
          <h1 class="text-3xl font-heading font-bold mb-3 leading-tight">
            Chào đồng chí {$userProfile.rank} {$userProfile.name}
          </h1>
          <p class="text-[#eef2ee] mb-6 max-w-lg">
            Đơn vị: {$userProfile.unit}
          </p>
          <div class="flex gap-4">
            <button on:click={() => currentScreen.set("CATEGORIES")} class="bg-[#ffcd00] text-[#4a422f] px-6 py-2.5 rounded-lg font-bold shadow hover:bg-[#ffe066] transition-colors flex items-center gap-2">
              <iconify-icon icon="solar:play-bold" class="text-xl"></iconify-icon>
              Vào Thi Ngay
            </button>
            
            {#if $userProfile.isAdmin}
              <button on:click={() => currentScreen.set("MANAGER")} class="bg-white/10 backdrop-blur-sm text-white border border-white/20 px-6 py-2.5 rounded-lg font-medium hover:bg-white/20 transition-colors">
                Quản lý Câu hỏi
              </button>
            {/if}
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <div class="bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex items-start justify-between">
          <div>
            <p class="text-gray-500 text-sm font-medium mb-1">Cấp bậc</p>
            <h3 class="text-xl font-bold text-[#1a2f1a]">{$userProfile.rank}</h3>
          </div>
          <div class="size-12 rounded-lg bg-[#dbeedd] flex items-center justify-center text-[#356839]">
            <iconify-icon icon="solar:medal-ribbons-star-bold" class="text-2xl"></iconify-icon>
          </div>
        </div>
        <div class="bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex items-start justify-between">
          <div>
            <p class="text-gray-500 text-sm font-medium mb-1">Đơn vị</p>
            <h3 class="text-xl font-bold text-[#1a2f1a]">{$userProfile.unit}</h3>
          </div>
          <div class="size-12 rounded-lg bg-[#ffe8e9] flex items-center justify-center text-[#ce2029]">
            <iconify-icon icon="solar:shield-star-bold" class="text-2xl"></iconify-icon>
          </div>
        </div>
      </div>
    </div>
  </main>
</div>