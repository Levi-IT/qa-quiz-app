<script lang="ts">
  import { currentScreen } from "$lib/store";
</script>

<div
  class="flex h-screen w-full bg-[#f4f6f4] font-sans text-[#1a2f1a] overflow-hidden"
>
  <aside
    class="w-72 bg-[#1b331e] text-[#eef2ee] flex flex-col h-full border-r border-[#2a4a2e] shadow-xl z-20"
  >
    <div class="p-6 border-b border-[#2a4a2e] flex items-center gap-3">
      <div
        class="size-10 rounded-full bg-[#ce2029] flex items-center justify-center shrink-0 border-2 border-[#ffcd00]"
      >
        <iconify-icon icon="solar:star-bold" class="text-2xl text-[#ffcd00]"
        ></iconify-icon>
      </div>
      <div>
        <h1
          class="font-heading font-bold text-lg leading-tight text-white uppercase tracking-wider"
        >
          QĐND Việt Nam
        </h1>
        <p class="text-[10px] text-[#a0bca3] uppercase tracking-widest">
          Học Tập & Rèn Luyện
        </p>
      </div>
    </div>

    <div class="flex-1 py-6 px-4 space-y-1 overflow-y-auto">
      <div
        class="px-3 mb-2 text-xs font-semibold text-[#a0bca3] uppercase tracking-wider"
      >
        Menu Chính
      </div>
      <button
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg bg-[#ce2029] text-white shadow-md transition-all"
      >
        <iconify-icon icon="solar:home-2-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Trang Chủ</span>
      </button>
      <button
        on:click={() => currentScreen.set("CATEGORIES")}
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all"
      >
        <iconify-icon icon="solar:play-circle-bold" class="text-xl"
        ></iconify-icon>
        <span class="font-medium">Bắt Đầu Quiz</span>
      </button>
      <button
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all"
      >
        <iconify-icon icon="solar:history-bold" class="text-xl"></iconify-icon>
        <span class="font-medium">Lịch Sử Thi</span>
      </button>
      <button
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all"
      >
        <iconify-icon icon="solar:chart-square-bold" class="text-xl"
        ></iconify-icon>
        <span class="font-medium">Thống Kê</span>
      </button>

      <div class="my-4 border-t border-[#2a4a2e]"></div>

      <div
        class="px-3 mb-2 text-xs font-semibold text-[#a0bca3] uppercase tracking-wider"
      >
        Quản Trị
      </div>
      <button
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all"
      >
        <iconify-icon icon="solar:file-text-bold" class="text-xl"
        ></iconify-icon>
        <span class="font-medium">Ngân Hàng Câu Hỏi</span>
      </button>
      <button
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all"
      >
        <iconify-icon icon="solar:users-group-rounded-bold" class="text-xl"
        ></iconify-icon>
        <span class="font-medium">Quản Lý Chiến Sĩ</span>
      </button>
      <button
        class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all"
      >
        <iconify-icon icon="solar:medal-star-bold" class="text-xl"
        ></iconify-icon>
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
          <div
            class="absolute -bottom-1 -right-1 bg-[#ce2029] text-white text-[10px] font-bold px-1 rounded border border-[#1b331e]"
          >
            Đ/c
          </div>
        </div>
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-white truncate">Nguyễn Văn An</p>
          <p class="text-xs text-[#ffcd00] truncate">Thượng úy</p>
        </div>
        <button
          on:click={() => currentScreen.set("LOGIN")}
          class="text-[#a0bca3] hover:text-white"
          aria-label="Đăng xuất"
        >
          <iconify-icon icon="solar:logout-2-bold" class="text-xl"
          ></iconify-icon>
        </button>
      </div>
    </div>
  </aside>

  <main
    class="flex-1 flex flex-col h-full bg-[#f4f6f4] overflow-hidden relative"
  >
    <div
      style="background-image: url('https://images.unsplash.com/photo-1579912437766-79b8a34b2f15?w=1600&q=80')"
      class="absolute top-0 right-0 w-1/2 h-1/2 bg-[url('https://ggrhecslgdflloszjkwl.supabase.co/storage/v1/object/public/generation-assets/placeholder/landscape.png')] opacity-5 pointer-events-none mix-blend-multiply bg-cover bg-center"
    ></div>

    <header
      class="h-16 bg-white border-b border-[#c2cdc2] flex items-center justify-between px-8 shrink-0 z-10 shadow-sm"
    >
      <div class="flex items-center gap-4">
        <h2 class="text-xl font-heading font-bold text-[#1a2f1a]">
          Bảng Điều Khiển
        </h2>
        <span
          class="px-2 py-0.5 rounded text-xs font-medium bg-[#dbeedd] text-[#356839]"
        >
          Phiên bản 2.0
        </span>
      </div>
      <div class="flex items-center gap-4">
        <div class="relative">
          <iconify-icon
            icon="solar:bell-bold"
            class="text-2xl text-gray-500 hover:text-[#356839] cursor-pointer transition-colors"
          ></iconify-icon>
          <span
            class="absolute -top-1 -right-1 size-2.5 bg-[#ce2029] rounded-full border border-white"
          ></span>
        </div>
        <div class="h-8 w-px bg-[#c2cdc2] mx-2"></div>
        <div class="flex items-center gap-2 text-sm font-medium text-[#1a2f1a]">
          <iconify-icon icon="solar:calendar-bold" class="text-[#356839]"
          ></iconify-icon>
          <span>Thứ Hai, 20 Tháng 5, 2024</span>
        </div>
      </div>
    </header>

    <div class="flex-1 overflow-y-auto p-8 z-10">
      <div
        class="mb-8 bg-linear-to-r from-[#356839] to-[#2a4a2e] rounded-2xl p-8 text-white shadow-lg relative overflow-hidden"
      >
        <div
          class="absolute top-0 right-0 h-full w-1/3 opacity-20 bg-[url('https://www.transparenttextures.com/patterns/cubes.png')]"
        ></div>
        <div class="absolute -right-10 -bottom-20 opacity-10">
          <iconify-icon
            icon="solar:shield-star-bold"
            class="text-[250px] text-white"
          ></iconify-icon>
        </div>
        <div class="relative z-10 max-w-2xl">
          <div class="flex items-center gap-3 mb-2">
            <span
              class="bg-[#ce2029] text-white text-xs font-bold px-2 py-1 rounded uppercase tracking-wider"
            >
              Thông báo
            </span>
            <span class="text-sm opacity-90">
              Cuộc thi tìm hiểu lịch sử 70 năm Chiến thắng Điện Biên Phủ đang
              diễn ra
            </span>
          </div>
          <h1 class="text-3xl font-heading font-bold mb-3 leading-tight">
            Chào đồng chí Thượng úy Nguyễn Hoàng A
          </h1>
          <p class="text-[#eef2ee] mb-6 max-w-lg">
            Sẵn sàng cho nhiệm vụ học tập hôm nay? Hãy tiếp tục hoàn thành các
            bài kiểm tra kiến thức quốc phòng và lịch sử quân sự.
          </p>
          <div class="flex gap-4">
            <button
              on:click={() => currentScreen.set("CATEGORIES")}
              class="bg-[#ffcd00] text-[#4a422f] px-6 py-2.5 rounded-lg font-bold shadow hover:bg-[#ffe066] transition-colors flex items-center gap-2"
            >
              <iconify-icon icon="solar:play-bold" class="text-xl"
              ></iconify-icon>
              Vào Thi Ngay
            </button>
            <button
              class="bg-white/10 backdrop-blur-sm text-white border border-white/20 px-6 py-2.5 rounded-lg font-medium hover:bg-white/20 transition-colors"
            >
              Xem Kết Quả Gần Nhất
            </button>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <div
          class="bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex items-start justify-between group hover:border-[#356839] transition-colors"
        >
          <div>
            <p class="text-gray-500 text-sm font-medium mb-1">
              Tổng số bài thi
            </p>
            <h3 class="text-3xl font-bold text-[#1a2f1a]">24</h3>
            <p
              class="text-xs text-[#356839] font-medium mt-1 flex items-center"
            >
              <iconify-icon
                icon="solar:arrow-right-up-linear"
                class="text-xs mr-1"
              ></iconify-icon>
              +2 trong tuần này
            </p>
          </div>
          <div
            class="size-12 rounded-lg bg-[#dbeedd] flex items-center justify-center text-[#356839] group-hover:bg-[#356839] group-hover:text-white transition-colors"
          >
            <iconify-icon icon="solar:document-text-bold" class="text-2xl"
            ></iconify-icon>
          </div>
        </div>
        <div
          class="bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex items-start justify-between group hover:border-[#ce2029] transition-colors"
        >
          <div>
            <p class="text-gray-500 text-sm font-medium mb-1">
              Điểm trung bình
            </p>
            <h3 class="text-3xl font-bold text-[#1a2f1a]">8.5</h3>
            <p
              class="text-xs text-[#ce2029] font-medium mt-1 flex items-center"
            >
              <iconify-icon
                icon="solar:arrow-right-down-linear"
                class="text-xs mr-1"
              ></iconify-icon>
              -0.2 so với tháng trước
            </p>
          </div>
          <div
            class="size-12 rounded-lg bg-[#ffe8e9] flex items-center justify-center text-[#ce2029] group-hover:bg-[#ce2029] group-hover:text-white transition-colors"
          >
            <iconify-icon icon="solar:star-circle-bold" class="text-2xl"
            ></iconify-icon>
          </div>
        </div>
        <div
          class="bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex items-start justify-between group hover:border-[#ffcd00] transition-colors"
        >
          <div>
            <p class="text-gray-500 text-sm font-medium mb-1">
              Xếp hạng đơn vị
            </p>
            <h3 class="text-3xl font-bold text-[#1a2f1a]">#05</h3>
            <p
              class="text-xs text-[#ffcd00] font-medium mt-1 flex items-center shadow-sm"
            >
              Top 10% xuất sắc
            </p>
          </div>
          <div
            class="size-12 rounded-lg bg-[#fff9db] flex items-center justify-center text-[#b38f00] group-hover:bg-[#ffcd00] group-hover:text-[#4a422f] transition-colors"
          >
            <iconify-icon icon="solar:cup-bold" class="text-2xl"></iconify-icon>
          </div>
        </div>
        <div
          class="bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex items-start justify-between group hover:border-[#546e7a] transition-colors"
        >
          <div>
            <p class="text-gray-500 text-sm font-medium mb-1">Giờ học tập</p>
            <h3 class="text-3xl font-bold text-[#1a2f1a]">12h</h3>
            <p class="text-xs text-gray-500 font-medium mt-1">
              30 phút hôm nay
            </p>
          </div>
          <div
            class="size-12 rounded-lg bg-[#eceff1] flex items-center justify-center text-[#546e7a] group-hover:bg-[#546e7a] group-hover:text-white transition-colors"
          >
            <iconify-icon icon="solar:clock-circle-bold" class="text-2xl"
            ></iconify-icon>
          </div>
        </div>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <div class="lg:col-span-2 space-y-6">
          <div class="flex items-center justify-between">
            <h3 class="text-lg font-heading font-bold flex items-center gap-2">
              <span class="w-1.5 h-6 bg-[#ce2029] rounded-full"></span>
              Chủ Đề Nổi Bật
            </h3>
            <button class="text-sm font-medium text-[#356839] hover:underline">
              Xem tất cả
            </button>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div
              class="bg-white rounded-xl overflow-hidden border border-[#c2cdc2] shadow-sm hover:shadow-md transition-shadow group cursor-pointer"
              on:click={() => currentScreen.set("CATEGORIES")}
              on:keydown={(e) =>
                e.key === "Enter" && currentScreen.set("CATEGORIES")}
              role="button"
              tabindex="0"
            >
              <div class="h-40 bg-gray-200 relative overflow-hidden">
                <img
                  alt="Military History"
                  src="https://images.unsplash.com/photo-1555881400-74d7acaacd81?w=800&q=80"
                  class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                />
                <div
                  class="absolute top-3 left-3 bg-[#ce2029] text-white text-xs font-bold px-2 py-1 rounded shadow-sm uppercase"
                >
                  Lịch Sử
                </div>
              </div>
              <div class="p-5">
                <h4
                  class="font-heading font-bold text-lg mb-2 group-hover:text-[#ce2029] transition-colors"
                >
                  Chiến dịch Hồ Chí Minh 1975
                </h4>
                <p class="text-sm text-gray-500 mb-4 line-clamp-2">
                  Kiểm tra kiến thức về chiến dịch lịch sử giải phóng hoàn toàn
                  miền Nam, thống nhất đất nước.
                </p>
                <div class="flex items-center justify-between mt-auto">
                  <div class="flex items-center gap-3 text-xs text-gray-500">
                    <span class="flex items-center gap-1">
                      <iconify-icon
                        icon="solar:question-circle-bold"
                        class="text-xs"
                      ></iconify-icon> 40 Câu hỏi
                    </span>
                    <span class="flex items-center gap-1">
                      <iconify-icon
                        icon="solar:clock-circle-bold"
                        class="text-xs"
                      ></iconify-icon> 45 Phút
                    </span>
                  </div>
                  <button
                    class="size-8 rounded-full bg-[#f4f6f4] flex items-center justify-center text-[#1a2f1a] group-hover:bg-[#ce2029] group-hover:text-white transition-colors"
                    aria-label="Xem chi tiết"
                  >
                    <iconify-icon
                      icon="solar:arrow-right-linear"
                      class="text-lg"
                    ></iconify-icon>
                  </button>
                </div>
              </div>
            </div>

            <div
              class="bg-white rounded-xl overflow-hidden border border-[#c2cdc2] shadow-sm hover:shadow-md transition-shadow group cursor-pointer"
              on:click={() => currentScreen.set("CATEGORIES")}
              on:keydown={(e) =>
                e.key === "Enter" && currentScreen.set("CATEGORIES")}
              role="button"
              tabindex="0"
            >
              <div class="h-40 bg-gray-200 relative overflow-hidden">
                <img
                  alt="Military Tactics"
                  src="https://images.unsplash.com/photo-1595758694721-39656111005a?w=800&q=80"
                  class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500"
                />
                <div
                  class="absolute top-3 left-3 bg-[#356839] text-white text-xs font-bold px-2 py-1 rounded shadow-sm uppercase"
                >
                  Chiến Thuật
                </div>
              </div>
              <div class="p-5">
                <h4
                  class="font-heading font-bold text-lg mb-2 group-hover:text-[#356839] transition-colors"
                >
                  Điều lệnh đội ngũ QĐNDVN
                </h4>
                <p class="text-sm text-gray-500 mb-4 line-clamp-2">
                  Nắm vững các quy định về điều lệnh đội ngũ, nghi lễ trong quân
                  đội nhân dân Việt Nam.
                </p>
                <div class="flex items-center justify-between mt-auto">
                  <div class="flex items-center gap-3 text-xs text-gray-500">
                    <span class="flex items-center gap-1">
                      <iconify-icon
                        icon="solar:question-circle-bold"
                        class="text-xs"
                      ></iconify-icon> 25 Câu hỏi
                    </span>
                    <span class="flex items-center gap-1">
                      <iconify-icon
                        icon="solar:clock-circle-bold"
                        class="text-xs"
                      ></iconify-icon> 30 Phút
                    </span>
                  </div>
                  <button
                    class="size-8 rounded-full bg-[#f4f6f4] flex items-center justify-center text-[#1a2f1a] group-hover:bg-[#356839] group-hover:text-white transition-colors"
                    aria-label="Xem chi tiết"
                  >
                    <iconify-icon
                      icon="solar:arrow-right-linear"
                      class="text-lg"
                    ></iconify-icon>
                  </button>
                </div>
              </div>
            </div>
          </div>

          <div
            class="bg-[#e6dec6] rounded-xl p-4 flex flex-col sm:flex-row items-center justify-between gap-4 border border-[#d4cbb0]"
          >
            <div class="flex items-center gap-4">
              <div class="bg-[#4a422f] p-3 rounded-lg text-[#ffcd00]">
                <iconify-icon icon="solar:pen-new-square-bold" class="text-2xl"
                ></iconify-icon>
              </div>
              <div>
                <h4 class="font-bold text-[#4a422f]">Tạo đề thi mới?</h4>
                <p class="text-sm text-[#6d6145]">
                  Dành cho cán bộ quản lý huấn luyện
                </p>
              </div>
            </div>
            <button
              class="px-5 py-2 bg-[#4a422f] text-white rounded-lg font-medium hover:bg-[#363022] transition-colors whitespace-nowrap w-full sm:w-auto"
            >
              Truy cập Quản lý
            </button>
          </div>
        </div>

        <div class="space-y-6">
          <div
            class="bg-white rounded-xl p-6 border border-[#c2cdc2] shadow-sm"
          >
            <h3 class="font-heading font-bold text-lg mb-4 text-center">
              Hồ Sơ Quân Nhân
            </h3>
            <div class="flex flex-col items-center mb-6">
              <div class="relative mb-3">
                <div
                  class="size-24 rounded-full bg-[#356839] flex items-center justify-center border-4 border-[#ffcd00] shadow-md"
                >
                  <iconify-icon
                    icon="solar:star-bold"
                    class="text-5xl text-[#ffcd00]"
                  ></iconify-icon>
                </div>
                <div
                  class="absolute bottom-0 right-0 bg-[#ce2029] rounded-full p-1.5 border-2 border-white"
                >
                  <iconify-icon
                    icon="solar:medal-ribbons-star-bold"
                    class="text-base text-white"
                  ></iconify-icon>
                </div>
              </div>
              <h4 class="font-bold text-xl">Thượng úy Nguyễn Văn An</h4>
              <p class="text-sm text-gray-500">Tiểu đoàn 5 - Trung đoàn 88</p>
            </div>
            <div class="space-y-3">
              <div class="flex justify-between items-center text-sm">
                <span class="text-gray-500">Cấp bậc</span>
                <span class="font-bold text-[#ce2029]">Thượng úy</span>
              </div>
              <div class="w-full bg-[#f4f6f4] h-2 rounded-full overflow-hidden">
                <div class="bg-[#ce2029] h-full w-3/4 rounded-full"></div>
              </div>
              <div
                class="flex justify-between items-center text-xs text-gray-500 mt-1"
              >
                <span>Điểm tích lũy: 1,450</span>
                <span>Mục tiêu: 2,000</span>
              </div>
            </div>
            <div class="grid grid-cols-3 gap-2 mt-6">
              <div
                class="flex flex-col items-center p-2 bg-[#f4f6f4] rounded-lg"
              >
                <iconify-icon
                  icon="solar:cup-first-bold"
                  class="text-2xl text-[#ffcd00] mb-1"
                ></iconify-icon>
                <span class="text-xs font-bold">Hạng 1</span>
                <span class="text-[10px] text-gray-500">Tuần</span>
              </div>
              <div
                class="flex flex-col items-center p-2 bg-[#f4f6f4] rounded-lg"
              >
                <iconify-icon
                  icon="solar:medal-star-bold"
                  class="text-2xl text-[#356839] mb-1"
                ></iconify-icon>
                <span class="text-xs font-bold">12</span>
                <span class="text-[10px] text-gray-500">Huy hiệu</span>
              </div>
              <div
                class="flex flex-col items-center p-2 bg-[#f4f6f4] rounded-lg"
              >
                <iconify-icon
                  icon="solar:flame-bold"
                  class="text-2xl text-[#ce2029] mb-1"
                ></iconify-icon>
                <span class="text-xs font-bold">5 ngày</span>
                <span class="text-[10px] text-gray-500">Chuỗi</span>
              </div>
            </div>
          </div>

          <div
            class="bg-white rounded-xl p-6 border border-[#c2cdc2] shadow-sm"
          >
            <div class="flex items-center justify-between mb-4">
              <h3 class="font-heading font-bold text-lg">Bảng Vàng</h3>
              <iconify-icon
                icon="solar:cup-star-bold"
                class="text-xl text-[#ffcd00]"
              ></iconify-icon>
            </div>
            <div class="space-y-4">
              <div class="flex items-center gap-3">
                <div class="font-bold text-[#ce2029] w-4">1</div>
                <img
                  alt="User"
                  src="https://randomuser.me/api/portraits/men/44.jpg"
                  class="size-8 rounded-full border border-[#c2cdc2]"
                />
                <div class="flex-1 min-w-0">
                  <p class="text-sm font-medium truncate">Trần Minh Đức</p>
                  <p class="text-xs text-gray-500">2,450 điểm</p>
                </div>
                <iconify-icon
                  icon="solar:medal-ribbon-star-bold"
                  class="text-xl text-[#ffcd00]"
                ></iconify-icon>
              </div>
              <div class="flex items-center gap-3">
                <div class="font-bold text-[#356839] w-4">2</div>
                <img
                  alt="User"
                  src="https://randomuser.me/api/portraits/men/86.jpg"
                  class="size-8 rounded-full border border-[#c2cdc2]"
                />
                <div class="flex-1 min-w-0">
                  <p class="text-sm font-medium truncate">Lê Văn Hùng</p>
                  <p class="text-xs text-gray-500">2,310 điểm</p>
                </div>
              </div>
              <div class="flex items-center gap-3">
                <div class="font-bold text-[#8d6e63] w-4">3</div>
                <img
                  alt="User"
                  src="https://randomuser.me/api/portraits/women/65.jpg"
                  class="size-8 rounded-full border border-[#c2cdc2]"
                />
                <div class="flex-1 min-w-0">
                  <p class="text-sm font-medium truncate">Phạm Thị Mai</p>
                  <p class="text-xs text-gray-500">2,100 điểm</p>
                </div>
              </div>
            </div>
            <button
              class="w-full mt-4 py-2 text-xs font-medium text-gray-500 hover:text-[#1a2f1a] border border-[#c2cdc2] rounded-lg hover:bg-gray-100 transition-colors"
            >
              Xem toàn bộ bảng xếp hạng
            </button>
          </div>
        </div>
      </div>
    </div>
  </main>
</div>
