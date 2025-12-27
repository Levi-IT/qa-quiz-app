<script lang="ts">
  import { currentScreen, userProfile, quizResult } from '$lib/store';
</script>

<div class="h-screen w-full bg-[#f4f6f4] flex items-center justify-center p-4">
    <div class="bg-white rounded-2xl shadow-xl w-full max-w-2xl border border-[#c2cdc2] overflow-hidden">
        <div class="bg-[#1b331e] p-8 text-center relative overflow-hidden">
             <div class="absolute inset-0 bg-[url('https://www.transparenttextures.com/patterns/cubes.png')] opacity-10"></div>
             <div class="size-20 mx-auto bg-[#ce2029] rounded-full flex items-center justify-center border-4 border-[#ffcd00] mb-4 relative z-10 shadow-lg">
                 <iconify-icon icon="solar:cup-star-bold" class="text-4xl text-[#ffcd00]"></iconify-icon>
             </div>
             <h2 class="text-2xl font-bold text-white uppercase tracking-wider relative z-10 font-heading">Kết Quả Kiểm Tra</h2>
        </div>

        <div class="p-10 text-center">
            <p class="text-[#5c6b5c] font-medium text-sm uppercase tracking-wide mb-1">Đồng chí</p>
            <h3 class="text-3xl font-bold text-[#1a2f1a] uppercase mb-1 font-heading">{$userProfile.rank} {$userProfile.name}</h3>
            <p class="text-sm text-[#356839] font-bold uppercase tracking-wide mb-8">{$userProfile.unit}</p>

            <div class="bg-[#f4f6f4] rounded-xl p-8 mb-8 border border-[#c2cdc2] inline-block w-full">
                <p class="text-sm text-[#5c6b5c] uppercase tracking-widest font-bold mb-2">Tổng điểm đạt được</p>
                <div class="flex items-end justify-center gap-2 leading-none mb-3">
                    <span class="text-7xl font-black text-[#ce2029] font-heading">{$quizResult.score}</span>
                    <span class="text-2xl text-[#c2cdc2] font-bold mb-2">/100</span>
                </div>
                <div class="inline-flex items-center gap-2 px-4 py-1.5 bg-[#dbeedd] text-[#356839] rounded-full font-bold text-sm">
                    <iconify-icon icon="solar:check-circle-bold"></iconify-icon> 
                    Đúng {$quizResult.correct}/{$quizResult.total} câu
                </div>
            </div>

            <div class="flex gap-4 justify-center">
                <button on:click={() => currentScreen.set('DASHBOARD')} class="px-6 py-3 border-2 border-[#c2cdc2] rounded-lg font-bold text-[#5c6b5c] hover:bg-gray-100 uppercase text-sm">Về Trang Chủ</button>
                <button on:click={() => currentScreen.set('CATEGORIES')} class="px-6 py-3 bg-[#356839] text-white rounded-lg font-bold hover:bg-[#2a4a2e] shadow-lg flex items-center gap-2 uppercase text-sm">
                    <iconify-icon icon="solar:refresh-bold" class="text-lg"></iconify-icon> Thi Lại
                </button>
            </div>
        </div>
    </div>
</div>