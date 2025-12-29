<script lang="ts">
  import { userProfile, currentScreen } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";

  let email = $state("");
  let password = $state("");
  let name = $state("");
  let rank = $state("Binh nhì");
  let unit = $state("");
  let isRegisterMode = $state(true);
  let loading = $state(false);
  let errorMessage = $state("");

  // Validation States
  let errors = $state({
    email: "",
    password: "",
    name: "",
    unit: ""
  });

  function validate() {
    let isValid = true;
    errors = { email: "", password: "", name: "", unit: "" };
    errorMessage = "";

    if (!email) {
      errors.email = "Email không được để trống";
      isValid = false;
    }

    if (!password) {
      errors.password = "Mật khẩu không được để trống";
      isValid = false;
    }

    if (isRegisterMode) {
      if (!name) {
        errors.name = "Đồng chí chưa nhập họ tên";
        isValid = false;
      } else if (name.length < 2) {
        errors.name = "Họ tên phải có ít nhất 2 ký tự";
        isValid = false;
      } else if (!/^[\p{L}\s]+$/u.test(name)) {
        errors.name = "Họ tên không được chứa ký tự đặc biệt";
        isValid = false;
      }

      if (!unit) {
        errors.unit = "Vui lòng nhập phiên hiệu đơn vị";
        isValid = false;
      } else if (unit.length > 20) {
        errors.unit = "Tên đơn vị tối đa 20 ký tự";
        isValid = false;
      }
    }

    return isValid;
  }

  async function handleAuth() {
    if (!validate()) return;
    loading = true;
    errorMessage = "";

    // Check Network Status
    const isOnline = navigator.onLine;

    if (!isOnline && isRegisterMode) {
        handleOfflineRegister();
        return;
    }

    try {
      if (isRegisterMode) {
        // Online Register
        await invoke("register_user", { 
          email, 
          password, 
          name, 
          rank, 
          unit 
        });

        console.log("Đăng ký thành công, đang tự động đăng nhập...");
        const user: any = await invoke("login_user", { email, password });
        setUserAndNavigate(user, false);

      } else {
        // Online Login
        const user: any = await invoke("login_user", { email, password });
        setUserAndNavigate(user, false);
      }
    } catch (error: any) {
      console.error(error);
      if (isRegisterMode) {
          if (error.toString().includes("EMAIL_EXISTS")) {
            console.log("Email đã tồn tại, đang thử đăng nhập...");
            try {
                const user: any = await invoke("login_user", { email, password });
                console.log("HEREEEE")
                setUserAndNavigate(user, false);
                return;
            } catch (loginErr) {
              console.log("OKKKK", loginErr)
                errorMessage = "Email đã tồn tại nhưng mật khẩu không đúng. Vui lòng kiểm tra lại.";
                loading = false;
                return;
            }
          }
          
          errorMessage = "Đăng ký thất bại: " + error;
          loading = false;
      } else {
          errorMessage = "Đăng nhập thất bại: " + error;
          loading = false;
      }
    }
  }

  function handleOfflineRegister() {
    // Save to Local Storage (Simulate DB)
    const offlineUser = {
        uid: "offline_" + Date.now(),
        name,
        rank,
        unit,
        is_admin: false
    };

    localStorage.setItem("offline_user", JSON.stringify(offlineUser));
    setUserAndNavigate(offlineUser, true);
  }

  function setUserAndNavigate(user: any, isOffline: boolean) {
      userProfile.set({
          uid: user.uid,
          name: user.name,
          rank: user.rank,
          unit: user.unit,
          isAdmin: user.is_admin,
          isLoggedIn: true,
          isOffline: isOffline,
          email: user.email || email || ''
      });
      currentScreen.set("DASHBOARD");
      loading = false;
  }
</script>

<div class="h-screen w-full bg-[#1b331e] flex items-center justify-center relative overflow-hidden">
  <div class="absolute inset-0 bg-[url('https://images.unsplash.com/photo-1579912437766-79b8a34b2f15?w=1600&q=80')] opacity-20 bg-cover bg-center mix-blend-overlay"></div>

  <div class="bg-white/95 backdrop-blur-sm p-8 rounded-2xl shadow-2xl w-full max-w-md z-10 border-4 border-[#ffcd00] relative max-h-[90vh] overflow-y-auto bg-white">
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
      <!-- Email -->
      <div>
        <label for="email" class="block text-xs font-bold text-[#356839] mb-1 uppercase">Email</label>
        <input id="email" bind:value={email} type="email" class="w-full px-4 py-2 border-2 {errors.email ? 'border-red-500' : 'border-[#c2cdc2]'} rounded-lg focus:border-[#356839] outline-none font-medium" placeholder="email@viettel.vn" />
        {#if errors.email}
            <p class="text-red-500 text-xs mt-1 italic">{errors.email}</p>
        {/if}
      </div>

      <!-- Password -->
      <div>
        <label for="password" class="block text-xs font-bold text-[#356839] mb-1 uppercase">Mật khẩu</label>
        <input id="password" bind:value={password} type="password" class="w-full px-4 py-2 border-2 {errors.password ? 'border-red-500' : 'border-[#c2cdc2]'} rounded-lg focus:border-[#356839] outline-none font-medium" placeholder="********" />
        {#if errors.password}
            <p class="text-red-500 text-xs mt-1 italic">{errors.password}</p>
        {/if}
      </div>

      {#if isRegisterMode}
        <!-- Name -->
        <div>
          <label for="name" class="block text-xs font-bold text-[#356839] mb-1 uppercase">Họ và tên</label>
          <input id="name" bind:value={name} class="w-full px-4 py-2 border-2 {errors.name ? 'border-red-500' : 'border-[#c2cdc2]'} rounded-lg focus:border-[#356839] outline-none font-bold" placeholder="NHẬP HỌ TÊN..." />
          {#if errors.name}
            <p class="text-red-500 text-xs mt-1 italic">{errors.name}</p>
          {/if}
        </div>

        <div class="grid grid-cols-2 gap-4">
          <!-- Rank -->
          <div>
            <label for="rank" class="block text-xs font-bold text-[#356839] mb-1 uppercase">Cấp bậc</label>
            <select id="rank" bind:value={rank} class="w-full px-3 py-2 border-2 border-[#c2cdc2] rounded-lg focus:border-[#356839] outline-none text-sm">
              <option>Học viên</option>
              <option>Binh nhì</option>
              <option>Binh nhất</option>
              <option>Hạ sĩ</option>
              <option>Trung sĩ</option>
              <option>Thượng sĩ</option>
              <option>Thiếu úy</option>
              <option>Trung úy</option>
              <option>Thượng úy</option>
              <option>Đại úy</option>
            </select>
          </div>
          
          <!-- Unit -->
          <div>
            <label for="unit" class="block text-xs font-bold text-[#356839] mb-1 uppercase">Đơn vị</label>
            <input id="unit" bind:value={unit} class="w-full px-3 py-2 border-2 {errors.unit ? 'border-red-500' : 'border-[#c2cdc2]'} rounded-lg focus:border-[#356839] outline-none text-sm" placeholder="C1 - D1..." />
            {#if errors.unit}
                <p class="text-red-500 text-xs mt-1 italic">{errors.unit}</p>
            {/if}
          </div>
        </div>
      {/if}

      {#if errorMessage}
        <div class="p-3 bg-red-100 border border-red-400 text-red-700 rounded text-xs font-bold text-center">
            {errorMessage}
        </div>
      {/if}

      <button
        onclick={handleAuth}
        disabled={loading}
        class="w-full bg-[#ce2029] hover:bg-[#a01822] text-white font-bold py-3 rounded-lg shadow-lg uppercase tracking-wider transition-all flex items-center justify-center gap-2 mt-4"
      >
        {#if loading}
          <iconify-icon icon="line-md:loading-twotone-loop" class="text-2xl"></iconify-icon>
        {:else}
          <iconify-icon icon={isRegisterMode ? "solar:user-plus-bold" : "solar:login-2-bold"} class="text-2xl"></iconify-icon>
        {/if}
        {isRegisterMode ? "BÁO CÁO NHẬP TRẠM" : "Đăng nhập hệ thống"}
      </button>

      <div class="text-center mt-4">
        <button 
          onclick={() => { isRegisterMode = !isRegisterMode; errors = {email: "", password: "", name: "", unit: ""}; }}
          class="text-[#356839] text-sm font-bold hover:underline"
        >
          {isRegisterMode ? "Đã có tài khoản? Đăng nhập" : "Chưa có tài khoản? Báo cáo ngay"}
        </button>
      </div>
    </div>
  </div>
</div>
