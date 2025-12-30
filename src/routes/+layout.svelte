<script lang="ts">
  import "../app.css"; 
  import { onMount } from "svelte";
  import { isAppOnline } from "$lib/store";

  onMount(() => {
    function updateOnlineStatus() {
      isAppOnline.set(navigator.onLine);
    }

    window.addEventListener('online', updateOnlineStatus);
    window.addEventListener('offline', updateOnlineStatus);

    // Initial check
    updateOnlineStatus();

    return () => {
      window.removeEventListener('online', updateOnlineStatus);
      window.removeEventListener('offline', updateOnlineStatus);
    };
  });
</script>

<div class="relative min-h-screen">
    {#if !$isAppOnline}
        <div class="bg-red-600 text-white text-xs font-bold text-center py-1 absolute top-0 left-0 w-full z-50">
            MẤT KẾT NỐI MẠNG - Ứng dụng đang chạy chế độ Offline
        </div>
    {/if}
    <slot />
</div>