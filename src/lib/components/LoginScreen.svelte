<script lang="ts">
  import { userProfile, currentScreen } from "$lib/store";

  let name = "";
  let rank = "Học viên";
  let unit = "";

  function handleStart() {
    if (!name) return alert("Đồng chí chưa nhập họ tên!");
    userProfile.set({ name, rank, unit });
    currentScreen.set("DASHBOARD");
  }
</script>

<div
  class="h-screen w-full bg-[#1b331e] flex items-center justify-center relative overflow-hidden"
>
  <div
    class="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1579912437766-79b8a34b2f15?w=1600&q=80')] opacity-20 bg-cover bg-center mix-blend-overlay"
  ></div>

  <div
    class="bg-white/95 backdrop-blur-sm p-10 rounded-2xl shadow-2xl w-full max-w-md z-10 border-4 border-[#ffcd00] relative"
  >
    <div class="absolute -top-12 left-1/2 -translate-x-1/2">
      <div
        class="size-24 rounded-full bg-[#ce2029] flex items-center justify-center border-4 border-[#ffcd00] shadow-lg"
      >
        <iconify-icon icon="solar:star-bold" class="text-5xl text-[#ffcd00]"
        ></iconify-icon>
      </div>
    </div>

    <div class="mt-12 text-center space-y-2">
      <h1
        class="text-3xl font-bold font-heading text-[#356839] uppercase tracking-wide"
      >
        QĐND VIỆT NAM
      </h1>
      <p class="text-sm text-gray-600 font-bold uppercase tracking-widest">
        Hệ Thống Kiểm Tra Kiến Thức
      </p>
    </div>

    <div class="mt-8 space-y-5">
      <div>
        <!-- svelte-ignore a11y_label_has_associated_control -->
        <label class="block text-sm font-bold text-[#356839] mb-1 uppercase"
          >Họ và tên</label
        >
        <div class="relative">
          <iconify-icon
            icon="solar:user-bold"
            class="absolute left-3 top-3.5 text-[#356839] text-lg"
          ></iconify-icon>
          <input
            bind:value={name}
            class="w-full pl-10 pr-4 py-3 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none font-bold uppercase text-[#1a2f1a]"
            placeholder="NHẬP HỌ TÊN..."
          />
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4">
        <div>
          <!-- svelte-ignore a11y_label_has_associated_control -->
          <label class="block text-sm font-bold text-[#356839] mb-1 uppercase"
            >Cấp bậc</label
          >
          <select
            bind:value={rank}
            class="w-full px-3 py-3 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none font-medium"
          >
            <option>Học viên</option>
            <option>Binh nhì</option>
            <option>Binh nhất</option>
            <option>Hạ sĩ</option>
            <option>Trung sĩ</option>
            <option>Thiếu úy</option>
            <option>Trung úy</option>
            <option>Thượng úy</option>
          </select>
        </div>
        <div>
          <!-- svelte-ignore a11y_label_has_associated_control -->
          <label class="block text-sm font-bold text-[#356839] mb-1 uppercase"
            >Đơn vị</label
          >
          <input
            bind:value={unit}
            class="w-full px-3 py-3 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none font-medium"
            placeholder="C1 - D1..."
          />
        </div>
      </div>

      <button
        on:click={handleStart}
        class="w-full bg-[#ce2029] hover:bg-[#a01822] text-white font-bold py-4 rounded-lg shadow-lg uppercase tracking-wider transition-all flex items-center justify-center gap-2 mt-6 text-lg"
      >
        <iconify-icon icon="solar:login-2-bold" class="text-2xl"></iconify-icon>
        Báo cáo nhập trạm
      </button>
    </div>
  </div>
</div>
