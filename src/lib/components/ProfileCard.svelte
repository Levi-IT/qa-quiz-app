<script>
    const stats = [
        { icon: "🏆", value: "1", label: "Hạng" },
        { icon: "🎒", value: "12", label: "Học liệu" },
        { icon: "🔥", value: "5 ngày", label: "Chuỗi" }
    ];
</script>

<div class="bg-white rounded-xl p-6 shadow-sm">
    <h3 class="font-semibold text-center mb-4">Hồ sơ Quân Nhân</h3>

    <div class="flex flex-col items-center">
        <div
            class="w-20 h-20 rounded-full bg-green-700 flex items-center justify-center text-yellow-400 text-2xl"
        >
            ★
        </div>

        <h4 class="font-semibold mt-2">Thượng úy Nguyễn Văn An</h4>
        <p class="text-sm text-gray-500">Tiểu đoàn 6 - Trung đoàn 88</p>
    </div>

    <div class="mt-4">
        <div class="flex justify-between text-sm">
            <span>Cấp bậc</span>
            <span class="text-red-600">Thượng úy</span>
        </div>

        <div class="w-full bg-gray-200 h-2 rounded mt-2">
            <div class="bg-red-600 h-2 rounded w-[72%]"></div>
        </div>

        <p class="text-xs text-gray-500 mt-1">Điểm tích lũy: 1,450 / 2,000</p>
    </div>

    <div class="grid grid-cols-3 text-center mt-4 gap-2 text-sm rounded">
        {#each stats as { icon, value, label }}
            <div class="bg-gray-200">
                <div class="text-lg">{icon}</div>
                <div class="font-semibold">{value}</div>
                <div class="text-xs text-gray-500">{label}</div>
            </div>
        {/each}
    </div>
</div>
