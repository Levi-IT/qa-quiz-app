<script lang="ts">
    import { currentScreen, selectedCategory } from '$lib/store';
    import Sidebar from "$lib/components/Sidebar.svelte";

    function startQuiz(name: string) {
        selectedCategory.set({ id: name, name });
        currentScreen.set('QUIZ');
    }
</script>

<div class="flex h-screen w-full bg-[#f4f6f4] font-sans text-[#1a2f1a] overflow-hidden">
    <Sidebar />

    <main class="flex-1 flex flex-col h-full bg-[#f4f6f4] overflow-hidden relative">
        <div class="absolute top-0 right-0 w-1/2 h-1/2 bg-[url('https://images.unsplash.com/photo-1579912437766-79b8a34b2f15?w=1600&q=80')] opacity-5 pointer-events-none mix-blend-multiply bg-cover bg-center"></div>
        
        <header class="h-16 bg-white border-b border-[#c2cdc2] flex items-center justify-between px-8 shrink-0 z-10 shadow-sm">
            <div class="flex items-center gap-4">
                <button onclick={() => currentScreen.set('DASHBOARD')} class="size-10 flex items-center justify-center rounded-lg hover:bg-gray-100 transition-colors" aria-label="Quay lại">
                    <iconify-icon icon="solar:arrow-left-linear" class="text-2xl text-[#1a2f1a]"></iconify-icon>
                </button>
                <h2 class="text-xl font-heading font-bold text-[#1a2f1a]">Chọn Chủ Đề Quiz</h2>
            </div>
            <div class="flex items-center gap-4">
                <div class="flex items-center gap-2 text-sm font-medium text-[#1a2f1a]">
                    <iconify-icon icon="solar:calendar-bold" class="text-[#356839]"></iconify-icon>
                    <span>{new Date().toLocaleDateString('vi-VN')}</span>
                </div>
            </div>
        </header>

        <div class="flex-1 overflow-y-auto p-8 z-10">
            <div class="mb-8 bg-linear-to-r from-[#356839] to-[#2a4a2e] rounded-2xl p-8 text-white shadow-lg relative overflow-hidden">
                <div class="absolute top-0 right-0 h-full w-1/3 opacity-20 bg-[url('https://www.transparenttextures.com/patterns/cubes.png')]"></div>
                <div class="relative z-10 max-w-3xl">
                    <div class="flex items-center gap-3 mb-3">
                        <span class="bg-[#ffcd00] text-[#4a422f] text-xs font-bold px-2 py-1 rounded uppercase tracking-wider">
                            Chọn Chủ Đề
                        </span>
                        <span class="text-sm opacity-90">Các chủ đề kiến thức quốc phòng - an ninh</span>
                    </div>
                    <h1 class="text-3xl font-heading font-bold mb-3 leading-tight">
                        Lĩnh Vực Kiểm Tra Kiến Thức
                    </h1>
                    <p class="text-[#eef2ee] max-w-2xl">
                        Lựa chọn chủ đề phù hợp với kế hoạch học tập của bạn. Hệ thống sẽ tự động lọc câu hỏi theo chủ đề đã chọn.
                    </p>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 pb-8">
                <!-- Lý Thuyết Quân Sự -->
                <button class="w-full text-left bg-white rounded-xl overflow-hidden border border-[#c2cdc2] shadow-sm hover:shadow-lg hover:border-[#356839] transition-all group cursor-pointer" 
                     onclick={() => startQuiz('Lý Thuyết Quân Sự')}>
                    <div class="h-40 bg-linear-to-br from-[#356839] to-[#2a4a2e] relative flex items-center justify-center">
                        <iconify-icon icon="solar:book-bookmark-bold" class="text-7xl text-white opacity-90 group-hover:scale-110 transition-transform duration-300"></iconify-icon>
                    </div>
                    <div class="p-5">
                        <h3 class="font-heading font-bold text-lg mb-1 group-hover:text-[#356839] transition-colors">Lý Thuyết Quân Sự</h3>
                        <p class="text-xs text-gray-500">Kiến thức nền tảng về quân sự.</p>
                    </div>
                </button>

                <!-- Vũ Khí Trang Bị -->
                <button class="w-full text-left bg-white rounded-xl overflow-hidden border border-[#c2cdc2] shadow-sm hover:shadow-lg hover:border-[#ce2029] transition-all group cursor-pointer"
                     onclick={() => startQuiz('Vũ Khí Trang Bị')}>
                    <div class="h-40 bg-linear-to-br from-[#ce2029] to-[#a01822] relative flex items-center justify-center">
                        <iconify-icon icon="solar:shield-bold" class="text-7xl text-white opacity-90 group-hover:scale-110 transition-transform duration-300"></iconify-icon>
                    </div>
                    <div class="p-5">
                        <h3 class="font-heading font-bold text-lg mb-1 group-hover:text-[#ce2029] transition-colors">Vũ Khí Trang Bị</h3>
                        <p class="text-xs text-gray-500">Các loại vũ khí, khí tài quân dụng.</p>
                    </div>
                </button>

                <!-- Chính Trị -->
                <button class="w-full text-left bg-white rounded-xl overflow-hidden border border-[#c2cdc2] shadow-sm hover:shadow-lg hover:border-blue-600 transition-all group cursor-pointer"
                     onclick={() => startQuiz('Chính Trị')}>
                    <div class="h-40 bg-linear-to-br from-blue-600 to-blue-800 relative flex items-center justify-center">
                        <iconify-icon icon="solar:flag-bold" class="text-7xl text-white opacity-90 group-hover:scale-110 transition-transform duration-300"></iconify-icon>
                    </div>
                    <div class="p-5">
                        <h3 class="font-heading font-bold text-lg mb-1 group-hover:text-blue-600 transition-colors">Chính Trị</h3>
                        <p class="text-xs text-gray-500">Kiến thức về công tác chính trị.</p>
                    </div>
                </button>

                <!-- Pháp Luật -->
                <button class="w-full text-left bg-white rounded-xl overflow-hidden border border-[#c2cdc2] shadow-sm hover:shadow-lg hover:border-purple-600 transition-all group cursor-pointer"
                     onclick={() => startQuiz('Pháp Luật')}>
                    <div class="h-40 bg-linear-to-br from-purple-600 to-purple-800 relative flex items-center justify-center">
                        <iconify-icon icon="solar:document-text-bold" class="text-7xl text-white opacity-90 group-hover:scale-110 transition-transform duration-300"></iconify-icon>
                    </div>
                    <div class="p-5">
                        <h3 class="font-heading font-bold text-lg mb-1 group-hover:text-purple-600 transition-colors">Pháp Luật</h3>
                        <p class="text-xs text-gray-500">Kiến thức pháp luật quân sự.</p>
                    </div>
                </button>

                <!-- Điều Lệnh -->
                <button class="w-full text-left bg-white rounded-xl overflow-hidden border border-[#c2cdc2] shadow-sm hover:shadow-lg hover:border-teal-600 transition-all group cursor-pointer"
                     onclick={() => startQuiz('Điều Lệnh')}>
                    <div class="h-40 bg-linear-to-br from-teal-600 to-teal-800 relative flex items-center justify-center">
                        <iconify-icon icon="solar:ruler-bold" class="text-7xl text-white opacity-90 group-hover:scale-110 transition-transform duration-300"></iconify-icon>
                    </div>
                    <div class="p-5">
                        <h3 class="font-heading font-bold text-lg mb-1 group-hover:text-teal-600 transition-colors">Điều Lệnh</h3>
                        <p class="text-xs text-gray-500">Điều lệnh đội ngũ, quản lý bộ đội.</p>
                    </div>
                </button>
            </div>
        </div>
    </main>
</div>