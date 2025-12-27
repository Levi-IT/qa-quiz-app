<script lang="ts">
  import { userProfile, currentScreen } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";

  let email = $state("");
  let password = $state("");
  let name = $state("");
  let rank = $state("Học viên");
  let unit = $state("");
  let isRegisterMode = $state(false);
  let loading = $state(false);

  async function handleAuth() {
    if (!email || !password) return alert("Vui lòng nhập đầy đủ email và mật khẩu!");
    loading = true;

    try {
      if (isRegisterMode) {
        if (!name) {
          alert("Vui lòng nhập họ tên!");
          loading = false;
          return;
        }

        // Call Backend Register
        await invoke("register_user", { 
          email, 
          password, 
          name, 
          rank, 
          unit 
        });

        alert("Đăng ký thành công! Vui lòng đăng nhập.");
        isRegisterMode = false;
        password = ""; // Clear password

      } else {
        // Call Backend Login
        const user: any = await invoke("login_user", { email, password });
        
        // Update Store
        userProfile.set({
          uid: user.uid,
          name: user.name,
          rank: user.rank,
          unit: user.unit,
          isAdmin: user.is_admin,
          isLoggedIn: true
        });

        currentScreen.set("DASHBOARD");
      }
    } catch (error: any) {
      console.error(error);
      alert("Lỗi: " + error);
    } finally {
      loading = false;
    }
  }
</script>

<div class="h-screen w-full bg-[#1b331e] flex items-center justify-center relative overflow-hidden">
  <div class="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1579912437766-79b8a34b2f15?w=1600&q=80')] opacity-20 bg-cover bg-center mix-blend-overlay"></div>

  <div class="bg-white/95 backdrop-blur-sm p-8 rounded-2xl shadow-2xl w-full max-w-md z-10 border-4 border-[#ffcd00] relative max-h-[90vh] overflow-y-auto">
    <div class="absolute -top-12 left-1/2 -translate-x-1/2">
      <div class="size-24 rounded-full bg-[#ce2029] flex items-center justify-center border-4 border-[#ffcd00] shadow-lg">
        <iconify-icon icon="solar:star-bold" class="text-5xl text-[#ffcd00]"></iconify-icon>
      </div>
    </div>

    <div class="mt-12 text-center space-y-2">
      <h1 class="text-2xl font-bold font-heading text-[#356839] uppercase tracking-wide">
        QĐND VIỆT NAM
      </h1>
      <p class="text-xs text-gray-600 font-bold uppercase tracking-widest">
        Hệ Thống Kiểm Tra Kiến Thức
      </p>
    </div>

    <div class="mt-6 space-y-4">
      <div>
        <label class="block text-xs font-bold text-[#356839] mb-1 uppercase">Email</label>
        <input bind:value={email} type="email" class="w-full px-4 py-2 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none font-medium" placeholder="email@viettel.vn" />
      </div>

      <div>
        <label class="block text-xs font-bold text-[#356839] mb-1 uppercase">Mật khẩu</label>
        <input bind:value={password} type="password" class="w-full px-4 py-2 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none font-medium" placeholder="********" />
      </div>

      {#if isRegisterMode}
        <div>
          <label class="block text-xs font-bold text-[#356839] mb-1 uppercase">Họ và tên</label>
          <input bind:value={name} class="w-full px-4 py-2 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none font-bold uppercase" placeholder="NHẬP HỌ TÊN..." />
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div>
            <label class="block text-xs font-bold text-[#356839] mb-1 uppercase">Cấp bậc</label>
            <select bind:value={rank} class="w-full px-3 py-2 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-sm">
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
            <label class="block text-xs font-bold text-[#356839] mb-1 uppercase">Đơn vị</label>
            <input bind:value={unit} class="w-full px-3 py-2 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-sm" placeholder="C1 - D1..." />
          </div>
        </div>
      {/if}

      <button
        on:click={handleAuth}
        disabled={loading}
        class="w-full bg-[#ce2029] hover:bg-[#a01822] text-white font-bold py-3 rounded-lg shadow-lg uppercase tracking-wider transition-all flex items-center justify-center gap-2 mt-4"
      >
        {#if loading}
          <iconify-icon icon="line-md:loading-twotone-loop" class="text-2xl"></iconify-icon>
        {:else}
          <iconify-icon icon={isRegisterMode ? "solar:user-plus-bold" : "solar:login-2-bold"} class="text-2xl"></iconify-icon>
        {/if}
        {isRegisterMode ? "Đăng ký tài khoản" : "Báo cáo nhập trạm"}
      </button>

      <div class="text-center mt-4">
        <button 
          on:click={() => isRegisterMode = !isRegisterMode}
          class="text-[#356839] text-sm font-bold hover:underline"
        >
          {isRegisterMode ? "Đã có tài khoản? Đăng nhập" : "Chưa có tài khoản? Đăng ký ngay"}
        </button>
      </div>
    </div>
  </div>
</div>
