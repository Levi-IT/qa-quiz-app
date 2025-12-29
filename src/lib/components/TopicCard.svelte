<script>
    export let tag;
    export let title;
    export let desc;
    export let questions;
    export let time;
    export let tagColor = "bg-red-600";
</script>

<div class="bg-white rounded-xl shadow-sm overflow-hidden">
    <div class="h-32 bg-gray-200 relative">
        <span
            class={`absolute top-3 left-3 text-xs text-white px-2 py-1 rounded ${tagColor}`}
        >
            {tag}
        </span>
    </div>

    <div class="p-4 space-y-2">
        <h3 class="font-semibold">{title}</h3>
        <p class="text-sm text-gray-500">{desc}</p>

        <div class="flex text-xs text-gray-500 gap-4">
            <span>● {questions} câu hỏi</span>
            <span>● {time}</span>
        </div>
    </div>
</div>
