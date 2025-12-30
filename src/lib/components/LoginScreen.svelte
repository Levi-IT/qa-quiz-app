<script lang="ts">
  import { userProfile, currentScreen } from "$lib/store";
  import { invoke } from "@tauri-apps/api/core";

  let email = $state("");
  let password = $state("");
  let name = $state("");
  let rank = $state("");
  let unit = $state("");
  let isRegisterMode = $state(true);
  let loading = $state(false);
  let errorMessage = $state("");
  let fireworks = $state<Array<{ id: number; x: number; y: number }>>([]);

  // Validation States
  let errors = $state({
    email: "",
    password: "",
    name: "",
    unit: "",
    rank: "",
  });

  // Fireworks effect
  function createFirework() {
    const x = Math.random() * 100;
    const y = Math.random() * 30 + 10;
    const id = Date.now() + Math.random();

    fireworks = [...fireworks, { id, x, y }];

    setTimeout(() => {
      fireworks = fireworks.filter(fw => fw.id !== id);
    }, 1500);
  }

  // Trigger fireworks every 6.67 seconds (20s / 3 texts)
  $effect(() => {
    const interval = setInterval(() => {
      createFirework();
      setTimeout(() => createFirework(), 200);
      setTimeout(() => createFirework(), 400);
    }, 6670);

    return () => clearInterval(interval);
  });

  function validate() {
    let isValid = true;
    errors = { email: "", password: "", name: "", unit: "", rank: "" };
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

      if (!rank) {
        errors.rank = "Vui lòng chọn cấp bậc";
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
          unit,
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
            console.log("HEREEEE");
            setUserAndNavigate(user, false);
            return;
          } catch (loginErr) {
            console.log("OKKKK", loginErr);
            errorMessage =
              "Email đã tồn tại nhưng mật khẩu không đúng. Vui lòng kiểm tra lại.";
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
      is_admin: false,
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
      email: user.email || email || "",
    });
    currentScreen.set("DASHBOARD");
    loading = false;
  }
</script>

<style>
  @keyframes marquee {
    0% {
      transform: translateX(0%);
    }
    100% {
      transform: translateX(-33.33%);
    }
  }

  .animate-marquee {
    display: inline-block;
    animation: marquee 20s linear infinite;
  }

  @keyframes firework-burst {
    0% {
      transform: translate(-50%, -50%) scale(0);
      opacity: 1;
    }
    50% {
      opacity: 1;
    }
    100% {
      transform: translate(-50%, -50%) scale(1.5);
      opacity: 0;
    }
  }

  .firework {
    position: absolute;
    width: 4px;
    height: 4px;
    border-radius: 50%;
    animation: firework-burst 1.5s ease-out forwards;
  }

  .firework::before,
  .firework::after {
    content: '';
    position: absolute;
    top: 50%;
    left: 50%;
    width: 100%;
    height: 100%;
    border-radius: 50%;
    box-shadow:
      0 -40px 0 #FFD700,
      0 40px 0 #FFD700,
      40px 0 0 #FFD700,
      -40px 0 0 #FFD700,
      28px 28px 0 #FF6B6B,
      -28px -28px 0 #FF6B6B,
      28px -28px 0 #4ECDC4,
      -28px 28px 0 #4ECDC4,
      0 -56px 0 #FFE66D,
      0 56px 0 #FFE66D,
      56px 0 0 #FFE66D,
      -56px 0 0 #FFE66D;
  }
</style>

<div
  class="h-screen w-full flex items-center justify-center relative overflow-hidden"
>
  <!-- Background Image -->
  <div
    class="absolute inset-0 bg-cover bg-center"
    style="background-image: url('/images/backgrounds/bg_qa_quiz.png');"
  ></div>

  <!-- Overlay -->
  <div class="absolute inset-0 bg-black/30"></div>

  <!-- Fireworks Container -->
  <div class="absolute inset-0 pointer-events-none z-50">
    {#each fireworks as fw (fw.id)}
      <div
        class="firework"
        style="left: {fw.x}%; top: {fw.y}%;"
      ></div>
    {/each}
  </div>

  <!-- Marquee Text at Top -->
  <div class="absolute top-2 left-0 w-full py-1 overflow-hidden z-40">
    <div class="animate-marquee whitespace-nowrap">
      <span class="text-[#FFD700] font-bold text-sm mx-8 drop-shadow-lg">
        🚁 Chúc mừng năm mới! Chúc các đồng chí sức khỏe tốt, hoàn thành nhiệm vụ tốt. 🇻🇳
      </span>
      <span class="text-[#FFD700] font-bold text-sm mx-8 drop-shadow-lg">
        🚁 Chúc mừng năm mới! Chúc các đồng chí sức khỏe tốt, hoàn thành nhiệm vụ tốt. 🇻🇳
      </span>
      <span class="text-[#FFD700] font-bold text-sm mx-8 drop-shadow-lg">
        🚁 Chúc mừng năm mới! Chúc các đồng chí sức khỏe tốt, hoàn thành nhiệm vụ tốt. 🇻🇳
      </span>
    </div>
  </div>

  <div class="relative w-full max-w-md">
    <!-- Star Badge at Top -->
    <div class="absolute -top-8 left-1/2 -translate-x-1/2 z-30">
      <div class="w-16 h-16 bg-[#C8102E] rounded-full flex items-center justify-center border-4 border-[#FFD700] shadow-lg">
        <svg class="w-8 h-8 text-[#FFD700]" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
        </svg>
      </div>
    </div>

  <div
    class="bg-white/5 backdrop-blur-2xl p-8 rounded-3xl w-full z-10 relative max-h-[90vh] overflow-y-auto border border-white/40 shadow-[0_8px_32px_0_rgba(0,0,0,0.37)]"
  >
    <!-- Logo -->
    <div class="flex justify-center -mb-4">
      <img
        src="/images/icons/logo_qa.png"
        alt="Logo"
        class="w-36 h-36 object-contain"
      />
    </div>

    <div class="text-center space-y-2 mb-2">
      <h1 class="text-2xl font-bold text-white uppercase tracking-wide">
        HỆ THỐNG ĐĂNG NHẬP
      </h1>
    </div>

    <div class="mt-2 space-y-4">
      <!-- Email -->
      <div class="relative">
        <div class="relative">
          <div
            class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none z-10"
          >
            <iconify-icon icon="mdi:lock-outline" class="text-white/60 text-lg"
            ></iconify-icon>
          </div>
          <input
            id="email"
            bind:value={email}
            type="email"
            class="peer w-full pl-9 pr-3 py-3 bg-white/10 border border-white/30 {errors.email
              ? 'border-red-500'
              : ''} rounded-lg focus:border-white focus:bg-white/20 outline-none text-sm text-black placeholder-transparent"
            placeholder="Email"
          />
          <label
            for="email"
            class="absolute left-9 top-3 text-white/60 text-sm font-semibold uppercase pointer-events-none transition-all duration-300 ease-in-out peer-placeholder-shown:top-3 peer-placeholder-shown:text-sm peer-placeholder-shown:bg-transparent peer-focus:-top-3.5 peer-focus:left-7 peer-focus:text-xs peer-focus:text-white peer-focus:bg-[#2c5f4a] peer-focus:px-3 peer-focus:py-1 peer-focus:rounded {email ? '-top-3.5 left-7 text-xs text-white bg-[#2c5f4a] px-3 py-1 rounded' : ''}"
          >
            Email
          </label>
        </div>
        {#if errors.email}
          <p class="text-red-300 text-xs mt-1 italic">{errors.email}</p>
        {/if}
      </div>

      <!-- Password -->
      <div class="relative">
        <div class="relative">
          <div
            class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none z-10"
          >
            <iconify-icon icon="mdi:lock-outline" class="text-white/60 text-lg"
            ></iconify-icon>
          </div>
          <input
            id="password"
            bind:value={password}
            type="password"
            class="peer w-full pl-9 pr-3 py-3 bg-white/10 border border-white/30 {errors.password
              ? 'border-red-500'
              : ''} rounded-lg focus:border-white focus:bg-white/20 outline-none text-sm text-black placeholder-transparent"
            placeholder="Mật khẩu"
          />
          <label
            for="password"
            class="absolute left-9 top-3 text-white/60 text-sm font-semibold uppercase pointer-events-none transition-all duration-300 ease-in-out peer-placeholder-shown:top-3 peer-placeholder-shown:text-sm peer-placeholder-shown:bg-transparent peer-focus:-top-3.5 peer-focus:left-7 peer-focus:text-xs peer-focus:text-white peer-focus:bg-[#2c5f4a] peer-focus:px-3 peer-focus:py-1 peer-focus:rounded {password ? '-top-3.5 left-7 text-xs text-white bg-[#2c5f4a] px-3 py-1 rounded' : ''}"
          >
            Mật khẩu
          </label>
        </div>
        {#if errors.password}
          <p class="text-red-300 text-xs mt-1 italic">{errors.password}</p>
        {/if}
      </div>

      {#if isRegisterMode}
        <!-- Name -->
        <div class="relative">
          <div class="relative">
            <div
              class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none z-10"
            >
              <iconify-icon
                icon="mdi:account-outline"
                class="text-white/60 text-lg"
              ></iconify-icon>
            </div>
            <input
              id="name"
              bind:value={name}
              class="peer w-full pl-9 pr-3 py-3 bg-white/10 border border-white/30 {errors.name
                ? 'border-red-500'
                : ''} rounded-lg focus:border-white focus:bg-white/20 outline-none text-sm text-black placeholder-transparent"
              placeholder="Họ và tên"
            />
            <label
              for="name"
              class="absolute left-9 top-3 text-white/60 text-sm font-semibold uppercase pointer-events-none transition-all duration-300 ease-in-out peer-placeholder-shown:top-3 peer-placeholder-shown:text-sm peer-placeholder-shown:bg-transparent peer-focus:-top-3.5 peer-focus:left-7 peer-focus:text-xs peer-focus:text-white peer-focus:bg-[#2c5f4a] peer-focus:px-3 peer-focus:py-1 peer-focus:rounded {name ? '-top-3.5 left-7 text-xs text-white bg-[#2c5f4a] px-3 py-1 rounded' : ''}"
            >
              Họ và tên
            </label>
          </div>
          {#if errors.name}
            <p class="text-red-300 text-xs mt-1 italic">{errors.name}</p>
          {/if}
        </div>

        <div class="grid grid-cols-2 gap-4">
          <!-- Rank -->
          <div class="relative">
            <div class="relative">
              <select
                id="rank"
                bind:value={rank}
                class="w-full h-10 px-3 pr-10 bg-transparent border-2 {errors.rank
                  ? 'border-red-500'
                  : 'border-[#c2cdc2]'} rounded-lg focus:border-[#356839] outline-none text-sm font-semibold appearance-none {!rank
                  ? 'text-white/60'
                  : 'text-black'}"
              >
                <option value="" disabled selected>Cấp bậc</option>
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
              <div
                class="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none"
              >
                <iconify-icon
                  icon="mdi:chevron-down"
                  class="text-[#356839] text-xl"
                ></iconify-icon>
              </div>
            </div>
            {#if errors.rank}
              <p class="text-red-300 text-xs mt-1 italic">{errors.rank}</p>
            {/if}
          </div>

          <!-- Unit -->
          <div class="relative">
            <input
              id="unit"
              bind:value={unit}
              class="peer w-full h-10 px-3 border-2 {errors.unit
                ? 'border-red-500'
                : 'border-[#c2cdc2]'} rounded-lg focus:border-[#356839] outline-none text-sm placeholder-transparent"
              placeholder="Đơn vị"
            />
            <label
              for="unit"
              class="absolute left-3 top-1/2 -translate-y-1/2 text-white/60 text-xs font-semibold uppercase pointer-events-none transition-all duration-300 ease-in-out peer-placeholder-shown:top-1/2 peer-placeholder-shown:-translate-y-1/2 peer-placeholder-shown:text-xs peer-placeholder-shown:bg-transparent peer-focus:-top-3.5 peer-focus:translate-y-0 peer-focus:left-2 peer-focus:text-[10px] peer-focus:text-white peer-focus:bg-[#2c5f4a] peer-focus:px-3 peer-focus:py-1 peer-focus:rounded {unit ? '-top-3.5 translate-y-0 left-2 text-[10px] text-white bg-[#2c5f4a] px-3 py-1 rounded' : ''}"
            >
              Đơn vị
            </label>
            {#if errors.unit}
              <p class="text-red-300 text-xs mt-1 italic">{errors.unit}</p>
            {/if}
          </div>
        </div>
      {/if}

      {#if errorMessage}
        <div
          class="p-3 bg-red-100 border border-red-400 text-red-700 rounded text-xs font-bold text-center"
        >
          {errorMessage}
        </div>
      {/if}

      <button
        onclick={handleAuth}
        disabled={loading}
        class="w-full bg-linear-to-r from-[#2c5f4a] to-[#1e4434] hover:from-[#1e4434] hover:to-[#153326] text-white font-bold py-3.5 rounded-lg shadow-lg uppercase tracking-wider transition-all flex items-center justify-center gap-2 mt-6"
      >
        {#if loading}
          <iconify-icon icon="line-md:loading-twotone-loop" class="text-2xl"
          ></iconify-icon>
          <span>{isRegisterMode ? "ĐANG ĐĂNG KÝ..." : "ĐANG ĐĂNG NHẬP..."}</span
          >
        {:else}
          <iconify-icon
            icon={isRegisterMode ? "solar:user-plus-bold" : "mdi:arrow-right"}
            class="text-xl"
          ></iconify-icon>
          <span>{isRegisterMode ? "ĐĂNG KÝ NGAY" : "ĐĂNG NHẬP"}</span>
        {/if}
      </button>

      <div class="text-center mt-6">
        <p class="text-white/80 text-sm">
          {isRegisterMode ? "Đã có tài khoản?" : "Chưa có tài khoản?"}
          <button
            onclick={() => {
              isRegisterMode = !isRegisterMode;
              errors = { email: "", password: "", name: "", unit: "", rank: "" };
            }}
            class="text-white font-bold hover:underline ml-1"
          >
            {isRegisterMode ? "ĐĂNG NHẬP NGAY" : "ĐĂNG KÝ NGAY"}
          </button>
        </p>
      </div>
    </div>
  </div>
  </div>
</div>
