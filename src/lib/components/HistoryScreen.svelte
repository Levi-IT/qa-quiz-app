<script lang="ts">
  import Sidebar from "$lib/components/Sidebar.svelte";

  const today = new Intl.DateTimeFormat('vi-VN', { 
    weekday: 'long', 
    year: 'numeric', 
    month: 'long', 
    day: 'numeric' 
  }).format(new Date());
</script>

<div class="flex h-screen w-full bg-[#f4f6f4] font-sans text-[#1a2f1a] overflow-hidden">
  <Sidebar />

  <main class="flex-1 flex flex-col h-full bg-[#f4f6f4] overflow-hidden relative">
    <header class="h-16 bg-white border-b border-[#c2cdc2] flex items-center justify-between px-8 shrink-0 z-10 shadow-sm">
      <div class="flex items-center gap-4">
        <h2 class="text-xl font-heading font-bold text-[#1a2f1a]">
          Lịch Sử Thi & Thống Kê
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
      <h3 class="text-lg font-bold text-[#1a2f1a] mb-4 uppercase tracking-wider">Thống kê học tập</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <!-- Điểm trung bình -->
        <div class="bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex items-start justify-between">
            <div>
              <p class="text-gray-500 text-sm font-medium mb-1">Điểm trung bình</p>
              <h3 class="text-xl font-bold text-[#1a2f1a]">---:---</h3>
            </div>
            <div class="size-12 rounded-lg bg-blue-100 flex items-center justify-center text-blue-600">
              <iconify-icon icon="solar:chart-2-bold" class="text-2xl"></iconify-icon>
            </div>
        </div>

        <!-- Xếp hạng -->
        <div class="bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex items-start justify-between">
            <div>
              <p class="text-gray-500 text-sm font-medium mb-1">Xếp hạng</p>
              <h3 class="text-xl font-bold text-[#1a2f1a]">---:---</h3>
            </div>
            <div class="size-12 rounded-lg bg-purple-100 flex items-center justify-center text-purple-600">
              <iconify-icon icon="solar:cup-star-bold" class="text-2xl"></iconify-icon>
            </div>
        </div>

        <!-- Giờ học tập -->
        <div class="bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex items-start justify-between">
            <div>
              <p class="text-gray-500 text-sm font-medium mb-1">Giờ học tập</p>
              <h3 class="text-xl font-bold text-[#1a2f1a]">---:---</h3>
            </div>
            <div class="size-12 rounded-lg bg-orange-100 flex items-center justify-center text-orange-600">
              <iconify-icon icon="solar:clock-circle-bold" class="text-2xl"></iconify-icon>
            </div>
        </div>

        <!-- Lịch sử thi -->
        <div class="bg-white p-5 rounded-xl border border-[#c2cdc2] shadow-sm flex items-start justify-between">
            <div>
              <p class="text-gray-500 text-sm font-medium mb-1">Tổng số bài thi</p>
              <h3 class="text-xl font-bold text-[#1a2f1a]">---:---</h3>
            </div>
            <div class="size-12 rounded-lg bg-teal-100 flex items-center justify-center text-teal-600">
              <iconify-icon icon="solar:history-bold" class="text-2xl"></iconify-icon>
            </div>
        </div>
      </div>

      <h3 class="text-lg font-bold text-[#1a2f1a] mb-4 uppercase tracking-wider">Danh sách bài thi gần đây</h3>
      <div class="bg-white rounded-xl border border-[#c2cdc2] shadow-sm overflow-hidden">
          <table class="w-full text-left border-collapse">
              <thead class="bg-gray-50 border-b border-[#c2cdc2]">
                  <tr>
                      <th class="px-6 py-4 text-xs font-bold text-gray-500 uppercase">Ngày thi</th>
                      <th class="px-6 py-4 text-xs font-bold text-gray-500 uppercase">Chủ đề</th>
                      <th class="px-6 py-4 text-xs font-bold text-gray-500 uppercase text-center">Số câu đúng</th>
                      <th class="px-6 py-4 text-xs font-bold text-gray-500 uppercase text-center">Điểm số</th>
                      <th class="px-6 py-4 text-xs font-bold text-gray-500 uppercase text-right">Thao tác</th>
                  </tr>
              </thead>
              <tbody>
                  <tr>
                      <td colspan="5" class="px-6 py-12 text-center text-gray-400 italic">
                          Chưa có lịch sử thi nào được ghi lại.
                      </td>
                  </tr>
              </tbody>
          </table>
      </div>
    </div>
  </main>
</div>