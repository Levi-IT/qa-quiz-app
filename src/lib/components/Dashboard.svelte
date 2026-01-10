<script lang="ts">
  import { onMount } from "svelte";
  import { currentScreen, userProfile } from "$lib/store";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import { invoke } from "@tauri-apps/api/core";
    import TopicCard from "./TopicCard.svelte";
    import ProfileCard from "./ProfileCard.svelte";
    import Leaderboard from "./Leaderboard.svelte";
    import { open } from '@tauri-apps/plugin-dialog';

  const today = new Intl.DateTimeFormat('vi-VN', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  }).format(new Date());

  async function handleImport() {
    try {
      const path = await open({
        multiple: false,
        directory: false,
        filters: [{
          name: 'JSON',
          extensions: ['json']
        }]
      });

      if (!path) return;

      const result = await invoke("import_questions", { path });
      alert(result);
    } catch (e) {
      console.error(e);
      alert("Lỗi nhập file: " + e);
    }
  }

  onMount(async () => {
    try {
      if (navigator.onLine && !$userProfile.isOffline) {
        console.log("Syncing data from remote...");
        await invoke("sync_data");
        console.log("Data synced.");
      }
    } catch (e) {
      console.error("Sync failed:", e);
    }
  });
</script>

<div
  class="flex h-screen w-full bg-[#f4f6f4] font-sans text-[#1a2f1a] overflow-hidden"
>
  <Sidebar />

  <main
    class="flex-1 flex flex-col h-full bg-[#f4f6f4] overflow-hidden relative"
  >
    <header
      class="h-16 bg-white border-b border-[#c2cdc2] flex items-center justify-between px-8 shrink-0 z-10 shadow-sm"
    >
      <div class="flex items-center gap-4">
        <h2 class="text-xl font-heading font-bold text-[#1a2f1a]">
          Bảng Điều Khiển
        </h2>
      </div>
      <div class="flex items-center gap-4">
        <div class="flex items-center gap-2 text-sm font-medium text-[#1a2f1a]">
          <iconify-icon icon="solar:calendar-bold" class="text-[#356839]"
          ></iconify-icon>
          <span>{today}</span>
        </div>
      </div>
    </header>

    <div class="flex-1 overflow-y-auto p-8 z-10 relative">
      <div
        class="mb-8 bg-linear-to-r from-[#356839] to-[#2a4a2e] rounded-2xl p-8 text-white shadow-lg relative overflow-hidden"
      >
        <div class="relative z-10 max-w-2xl">
          <h1 class="text-3xl font-heading font-bold mb-3 leading-tight">
            Chào đồng chí {$userProfile.rank}
            {$userProfile.name}
          </h1>
          <p class="text-[#eef2ee] mb-6 max-w-lg">
            Đơn vị: {$userProfile.unit}
          </p>
          <div class="flex gap-4">
            <button
              onclick={() => currentScreen.set("CATEGORIES")}
              class="bg-[#ffcd00] text-[#4a422f] px-6 py-2.5 rounded-lg font-bold shadow hover:bg-[#ffe066] transition-colors flex items-center gap-2"
            >
              <iconify-icon icon="solar:play-bold" class="text-xl"
              ></iconify-icon>
              Vào Thi Ngay
            </button>

            <button
              onclick={handleImport}
              class="bg-white/10 backdrop-blur-sm text-white border border-white/20 px-6 py-2.5 rounded-lg font-medium hover:bg-white/20 transition-colors flex items-center gap-2"
            >
              <iconify-icon icon="solar:import-bold" class="text-xl"></iconify-icon>
              Import
            </button>

            {#if $userProfile.isAdmin}
              <button
                onclick={() => currentScreen.set("MANAGER")}
                class="bg-white/10 backdrop-blur-sm text-white border border-white/20 px-6 py-2.5 rounded-lg font-medium hover:bg-white/20 transition-colors"
              >
                Quản lý Câu hỏi
              </button>
            {/if}

            {#if !$userProfile.isAdmin}
              <button
                class="bg-white/10 backdrop-blur-sm text-white border border-white/20 px-6 py-2.5 rounded-lg font-medium hover:bg-white/20 transition-colors"
              >
                Xem kết quả gần nhất
              </button>
            {/if}
          </div>
        </div>
      </div>

      <div class="grid grid-cols-2 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <div
          class="bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex items-start justify-between"
        >
          <div>
            <p class="text-gray-500 text-sm font-medium mb-1">
              Tổng số bài thi
            </p>
            <h3 class="text-xl font-bold text-[#1a2f1a]">24</h3>
            <h6 class="text-sm text-[#356839]">#2 trong tuần này</h6>
          </div>
          <div
            class="size-12 rounded-lg bg-[#dbeedd] flex items-center justify-center text-[#356839]"
          >
            <iconify-icon icon="solar:medal-ribbons-star-bold" class="text-2xl"
            ></iconify-icon>
          </div>
        </div>
        <div
          class="bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex items-start justify-between"
        >
          <div>
            <p class="text-gray-500 text-sm font-medium mb-1">
              Điểm trung bình
            </p>
            <h3 class="text-xl font-bold text-[#1a2f1a]">8.5</h3>
            <h6 class="text-sm text-red-500">#2 so với tháng trước</h6>
          </div>
          <div
            class="size-12 rounded-lg bg-[#ffe8e9] flex items-center justify-center text-[#ce2029]"
          >
            <iconify-icon icon="solar:shield-star-bold" class="text-2xl"
            ></iconify-icon>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-12 gap-6 min-h-screen">
        <!-- Left -->
        <div class="col-span-8 space-y-6">
          <div>
            <div class="flex justify-between mb-4">
              <h2 class="font-bold text-lg border-l-8 border-red-500 pl-2">Chủ đề nổi bật</h2>
              <a class="text-sm text-green-600 cursor-pointer" href="#a"
                >Xem tất cả</a
              >
            </div>

            <div class="grid grid-cols-2 gap-4">
              <TopicCard
                tag="LỊCH SỬ"
                title="Chiến dịch Hồ Chí Minh 1975"
                desc="Kiểm tra kiến thức về chiến dịch lịch sử giải phóng hoàn toàn miền Nam."
                questions={40}
                time="45 phút"
                tagColor="bg-red-600"
              />

              <TopicCard
                tag="CHIẾN THUẬT"
                title="Điều lệnh đội ngũ QĐNDVN"
                desc="Nắm vững các quy định về điều lệnh đội ngũ."
                questions={25}
                time="30 phút"
                tagColor="bg-green-600"
              />
            </div>
          </div>
          <div
            class="bg-[#E8E3CC] rounded-xl p-5 flex justify-between items-center mt-6"
          >
            <div>
              <h3 class="font-semibold">Tạo đề thi mới?</h3>
              <p class="text-sm text-gray-600">
                Dành cho cán bộ quản lý huấn luyện
              </p>
            </div>

            <button class="bg-[#4A3F2A] text-white px-4 py-2 rounded-lg">
              Truy cập Quản lý
            </button>
          </div>
        </div>

        <!-- Right -->
        <div class="col-span-4 space-y-6">
          <ProfileCard />
          <Leaderboard />
        </div>
      </div>
    </div>
  </main>
</div>
