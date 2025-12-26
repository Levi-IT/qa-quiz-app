<script lang="ts">
    import { currentScreen } from '$lib/store';
</script>

<div class="flex h-screen w-full bg-[#f4f6f4] font-sans text-[#1a2f1a] overflow-hidden">
    <aside class="w-72 bg-[#1b331e] text-[#eef2ee] flex flex-col h-full border-r border-[#2a4a2e] shadow-xl z-20">
        <div class="p-6 border-b border-[#2a4a2e] flex items-center gap-3">
            <div class="size-10 rounded-full bg-[#ce2029] flex items-center justify-center shrink-0 border-2 border-[#ffcd00]">
                <iconify-icon icon="solar:star-bold" class="text-xl text-[#ffcd00]"></iconify-icon>
            </div>
            <div>
                <h1 class="font-heading font-bold text-lg leading-tight text-white uppercase tracking-wider">
                    QĐND Việt Nam
                </h1>
                <p class="text-[10px] text-[#a0bca3] uppercase tracking-widest">Học Tập & Rèn Luyện</p>
            </div>
        </div>

        <div class="flex-1 py-6 px-4 space-y-1 overflow-y-auto">
            <div class="px-3 mb-2 text-xs font-semibold text-[#a0bca3] uppercase tracking-wider">
                Menu Chính
            </div>
            <button on:click={() => currentScreen.set('DASHBOARD')} class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all">
                <iconify-icon icon="solar:home-2-bold" class="text-xl"></iconify-icon>
                <span class="font-medium">Trang Chủ</span>
            </button>
            <button class="w-full flex items-center gap-3 px-3 py-3 rounded-lg bg-[#ce2029] text-white shadow-md transition-all">
                <iconify-icon icon="solar:play-circle-bold" class="text-xl"></iconify-icon>
                <span class="font-medium">Bắt Đầu Quiz</span>
            </button>
            <button class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all">
                <iconify-icon icon="solar:history-bold" class="text-xl"></iconify-icon>
                <span class="font-medium">Lịch Sử Thi</span>
            </button>
            
            <div class="my-4 border-t border-[#2a4a2e]"></div>
            
            <div class="px-3 mb-2 text-xs font-semibold text-[#a0bca3] uppercase tracking-wider">
                Quản Trị
            </div>
            <button class="w-full flex items-center gap-3 px-3 py-3 rounded-lg text-[#eef2ee] hover:bg-[#2a4a2e] transition-all">
                <iconify-icon icon="solar:file-text-bold" class="text-xl"></iconify-icon>
                <span class="font-medium">Ngân Hàng Câu Hỏi</span>
            </button>
        </div>

        <div class="p-4 border-t border-[#2a4a2e] bg-[#152918]">
            <div class="flex items-center gap-3">
                <div class="relative">
                    <img alt="User Avatar" src="https://randomuser.me/api/portraits/men/32.jpg" class="size-10 rounded-full border-2 border-[#ffcd00]" />
                    <div class="absolute -bottom-1 -right-1 bg-[#ce2029] text-white text-[10px] font-bold px-1 rounded border border-[#1b331e]">
                        Đ/c
                    </div>
                </div>
                <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium text-white truncate">Nguyễn Văn An</p>
                    <p class="text-xs text-[#ffcd00] truncate">Thượng úy</p>
                </div>
                <!-- svelte-ignore a11y_consider_explicit_label -->
                <button on:click={() => currentScreen.set('LOGIN')} class="text-[#a0bca3] hover:text-white">
                    <iconify-icon icon="solar:logout-2-bold" class="text-xl"></iconify-icon>
                </button>
            </div>
        </div>
    </aside>

    <main class="flex-1 flex flex-col h-full bg-[#f4f6f4] overflow-hidden relative">
        <div class="absolute top-0 right-0 w-1/2 h-1/2 bg-[url('https://images.unsplash.com/photo-1579912437766-79b8a34b2f15?w=1600&q=80')] opacity-5 pointer-events-none mix-blend-multiply bg-cover bg-center"></div>
        
        <header class="h-16 bg-white border-b border-[#c2cdc2] flex items-center justify-between px-8 shrink-0 z-10 shadow-sm">
            <div class="flex items-center gap-4">
                <!-- svelte-ignore a11y_consider_explicit_label -->
                <button on:click={() => currentScreen.set('DASHBOARD')} class="size-10 flex items-center justify-center rounded-lg hover:bg-gray-100 transition-colors">
                    <iconify-icon icon="solar:arrow-left-linear" class="text-2xl text-[#1a2f1a]"></iconify-icon>
                </button>
                <h2 class="text-xl font-heading font-bold text-[#1a2f1a]">Chọn Chủ Đề Quiz</h2>
            </div>
            <div class="flex items-center gap-4">
                <div class="relative">
                    <iconify-icon icon="solar:bell-bold" class="text-2xl text-gray-500 hover:text-[#356839] cursor-pointer transition-colors"></iconify-icon>
                    <span class="absolute -top-1 -right-1 size-2.5 bg-[#ce2029] rounded-full border border-white"></span>
                </div>
                <div class="h-8 w-px bg-[#c2cdc2] mx-2"></div>
                <div class="flex items-center gap-2 text-sm font-medium text-[#1a2f1a]">
                    <iconify-icon icon="solar:calendar-bold" class="text-[#356839]"></iconify-icon>
                    <span>{new Date().toLocaleDateString('vi-VN')}</span>
                </div>
            </div>
        </header>

        <div class="flex-1 overflow-y-auto p-8 z-10">
            <div class="mb-8 bg-gradient-to-r from-[#356839] to-[#2a4a2e] rounded-2xl p-8 text-white shadow-lg relative overflow-hidden">
                <div class="absolute top-0 right-0 h-full w-1/3 opacity-20 bg-[url('https://www.transparenttextures.com/patterns/cubes.png')]"></div>
                <div class="absolute -right-10 -bottom-20 opacity-10">
                    <iconify-icon icon="solar:documents-bold" class="text-[250px] text-white"></iconify-icon>
                </div>
                <div class="relative z-10 max-w-3xl">
                    <div class="flex items-center gap-3 mb-3">
                        <span class="bg-[#ffcd00] text-[#4a422f] text-xs font-bold px-2 py-1 rounded uppercase tracking-wider">
                            Chọn Chủ Đề
                        </span>
                        <span class="text-sm opacity-90">5 chủ đề kiến thức quốc phòng - an ninh</span>
                    </div>
                    <h1 class="text-3xl font-heading font-bold mb-3 leading-tight">
                        Chọn Lĩnh Vực Kiểm Tra Kiến Thức
                    </h1>
                    <p class="text-[#eef2ee] max-w-2xl">
                        Lựa chọn chủ đề phù hợp với năng lực và kế hoạch học tập của bạn. Mỗi chủ đề được thiết kế để kiểm tra và nâng cao kiến thức chuyên môn quân sự.
                    </p>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-8">
                <div class="bg-white p-4 rounded-xl border border-[#c2cdc2] shadow-sm flex items-center gap-3">
                    <div class="size-10 rounded-lg bg-[#dbeedd] flex items-center justify-center text-[#356839]">
                        <iconify-icon icon="solar:folder-bold" class="text-xl"></iconify-icon>
                    </div>
                    <div>
                        <p class="text-xs text-gray-500">Tổng chủ đề</p>
                        <p class="text-xl font-bold text-[#1a2f1a]">5</p>
                    </div>
                </div>
                <div class="bg-white p-4 rounded-xl border border-[#c2cdc2] shadow-sm flex items-center gap-3">
                    <div class="size-10 rounded-lg bg-[#ffe8e9] flex items-center justify-center text-[#ce2029]">
                        <iconify-icon icon="solar:question-circle-bold" class="text-xl"></iconify-icon>
                    </div>
                    <div>
                        <p class="text-xs text-gray-500">Tổng câu hỏi</p>
                        <p class="text-xl font-bold text-[#1a2f1a]">385</p>
                    </div>
                </div>
                 <div class="bg-white p-4 rounded-xl border border-[#c2cdc2] shadow-sm flex items-center gap-3">
                    <div class="size-10 rounded-lg bg-[#fff9db] flex items-center justify-center text-[#b38f00]">
                        <iconify-icon icon="solar:check-circle-bold" class="text-xl"></iconify-icon>
                    </div>
                    <div>
                        <p class="text-xs text-gray-500">Đã hoàn thành</p>
                        <p class="text-xl font-bold text-[#1a2f1a]">3/5</p>
                    </div>
                </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <div class="bg-white rounded-xl overflow-hidden border border-[#c2cdc2] shadow-sm hover:shadow-lg hover:border-[#356839] transition-all group cursor-pointer" 
                     on:click={() => currentScreen.set('QUIZ')} role="button" tabindex="0">
                    <div class="h-48 bg-gradient-to-br from-[#356839] to-[#2a4a2e] relative overflow-hidden flex items-center justify-center">
                        <iconify-icon icon="solar:book-bookmark-bold" class="text-8xl text-white opacity-90 group-hover:scale-110 transition-transform duration-300"></iconify-icon>
                        <div class="absolute top-3 right-3 bg-[#ffcd00] text-[#4a422f] text-xs font-bold px-2 py-1 rounded shadow flex items-center">
                            <iconify-icon icon="solar:check-circle-bold" class="text-xs mr-1"></iconify-icon> Hoàn Thành
                        </div>
                    </div>
                    <div class="p-5">
                        <h3 class="font-heading font-bold text-xl mb-2 group-hover:text-[#356839] transition-colors">Lý Thuyết Quân Sự</h3>
                        <p class="text-sm text-gray-500 mb-4 line-clamp-2">Kiến thức nền tảng về chủ nghĩa Mác-Lênin, tư tưởng Hồ Chí Minh.</p>
                        <div class="flex items-center justify-between">
                             <div class="flex items-center gap-1 text-[#ffcd00]">
                                <iconify-icon icon="solar:star-bold"></iconify-icon>
                                <iconify-icon icon="solar:star-bold"></iconify-icon>
                                <iconify-icon icon="solar:star-bold"></iconify-icon>
                                <iconify-icon icon="solar:star-bold"></iconify-icon>
                                <span class="text-xs font-bold text-black ml-1">8.5</span>
                             </div>
                             <button class="px-4 py-2 bg-[#356839] text-white rounded-lg text-sm font-medium hover:bg-[#2a4a2e]">Thi Lại</button>
                        </div>
                    </div>
                </div>

                <!-- svelte-ignore a11y_click_events_have_key_events -->
                <div class="bg-white rounded-xl overflow-hidden border border-[#c2cdc2] shadow-sm hover:shadow-lg hover:border-[#ce2029] transition-all group cursor-pointer"
                     on:click={() => currentScreen.set('QUIZ')} role="button" tabindex="0">
                    <div class="h-48 bg-gradient-to-br from-[#ce2029] to-[#a01822] relative overflow-hidden flex items-center justify-center">
                        <iconify-icon icon="solar:shield-bold" class="text-8xl text-white opacity-90 group-hover:scale-110 transition-transform duration-300"></iconify-icon>
                    </div>
                    <div class="p-5">
                        <h3 class="font-heading font-bold text-xl mb-2 group-hover:text-[#ce2029] transition-colors">Vũ Khí Trang Bị</h3>
                        <p class="text-sm text-gray-500 mb-4 line-clamp-2">Hiểu biết về các loại vũ khí, trang bị kỹ thuật quân sự.</p>
                        <div class="flex items-center justify-between">
                             <div class="flex items-center gap-1 text-[#ffcd00]">
                                <iconify-icon icon="solar:star-bold"></iconify-icon>
                                <iconify-icon icon="solar:star-bold"></iconify-icon>
                                <iconify-icon icon="solar:star-bold"></iconify-icon>
                                <iconify-icon icon="solar:star-bold"></iconify-icon>
                                <iconify-icon icon="solar:star-bold"></iconify-icon>
                                <span class="text-xs font-bold text-black ml-1">9.0</span>
                             </div>
                             <button class="px-4 py-2 bg-[#ce2029] text-white rounded-lg text-sm font-medium hover:bg-[#a01822]">Thi Lại</button>
                        </div>
                    </div>
                </div>

                <div class="bg-white rounded-xl overflow-hidden border border-[#c2cdc2] shadow-sm opacity-60 cursor-not-allowed relative">
                    <div class="h-48 bg-gradient-to-br from-[#c2cdc2] to-[#a8b3a8] relative overflow-hidden flex items-center justify-center">
                        <iconify-icon icon="solar:lock-password-bold" class="text-8xl text-white opacity-70"></iconify-icon>
                        <div class="absolute inset-0 bg-black/5 backdrop-blur-sm flex items-center justify-center">
                            <div class="bg-white rounded-lg px-4 py-2 shadow-lg">
                                <p class="text-sm font-bold text-[#1a2f1a]">Mở khóa ở cấp độ tiếp theo</p>
                            </div>
                        </div>
                    </div>
                    <div class="p-5">
                        <h3 class="font-heading font-bold text-xl mb-2 text-gray-500">Chuyên Đề Nâng Cao</h3>
                        <p class="text-sm text-gray-500 mb-4">Hoàn thành 4/5 chủ đề cơ bản để mở khóa.</p>
                        <div class="flex items-center justify-between">
                             <div class="flex items-center gap-2 text-gray-500">
                                <iconify-icon icon="solar:lock-password-bold"></iconify-icon>
                                <span class="text-xs font-medium">Chưa khả dụng</span>
                             </div>
                             <button class="px-4 py-2 bg-gray-200 text-gray-500 rounded-lg text-sm font-medium cursor-not-allowed">Đã Khóa</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </main>
</div>