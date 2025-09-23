<script>
  import Button from '$lib/components/ui/button/button.svelte';
  import { CheckCircle } from '@lucide/svelte';
  export let active = 'None'; // 可选：'Jobs' | 'Transcriptions' | 'Settings'
</script>

<aside class="w-60 border-r bg-white flex flex-col items-center py-8 gap-8 shadow-lg relative">
  <div class="mb-10 flex flex-col items-center w-full">
    <div class="mb-4 text-2xl font-extrabold tracking-tight text-primary flex items-center gap-2 select-none">
      <img src="/favicon.png" alt="logo" class="w-8 h-8 rounded shadow" />
      Audix
    </div>
  </div>
  <nav class="flex flex-col gap-2 text-base w-full px-6">
    <a href="/jobs" class="rounded px-3 py-2 transition font-medium text-gray-700 hover:bg-slate-100 hover:text-primary {active === 'Jobs' ? 'bg-slate-200 text-primary' : ''}">Jobs</a>
    <a href="/transcriptions" class="rounded px-3 py-2 transition font-medium text-gray-700 hover:bg-slate-100 hover:text-primary {active === 'Transcriptions' ? 'bg-slate-200 text-primary' : ''}">Transcriptions</a>
    <a href="/settings" class="rounded px-3 py-2 transition font-medium text-gray-700 hover:bg-slate-100 hover:text-primary {active === 'Settings' ? 'bg-slate-200 text-primary' : ''}">Settings</a>
  </nav>
  <div class="absolute bottom-4 left-0 w-full text-center text-xs text-gray-400 select-none">© 2025 Audix</div>
</aside>
