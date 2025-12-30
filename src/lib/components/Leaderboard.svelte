<script>
    const ranks = [
        { rank: 1, name: "Trần Minh Đức", score: "2,450" },
        { rank: 2, name: "Lê Văn Hùng", score: "2,310" },
        { rank: 3, name: "Phạm Thị Mai", score: "2,100" },
    ];
</script>

<div class="bg-white rounded-xl p-5 shadow-sm">
    <div class="flex justify-between mb-4">
        <h3 class="font-semibold">Bảng Vàng</h3>
        🏆
    </div>

    <ol class="space-y-3">
        {#each ranks as r}
            <li class="flex items-center gap-3">
                <span class="font-bold text-red-600">{r.rank}</span>
                <div class="w-8 h-8 rounded-full bg-gray-300"></div>
                <div class="flex-1">
                    <p class="text-sm">{r.name}</p>
                    <p class="text-xs text-gray-500">{r.score} điểm</p>
                </div>
            </li>
        {/each}
    </ol>

    <button class="mt-4 w-full border rounded-lg py-2 text-sm">
        Xem toàn bộ bảng xếp hạng
    </button>
</div>
