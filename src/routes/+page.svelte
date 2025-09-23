<script>
  import { invoke } from "@tauri-apps/api/core";
  import Sidebar from '$lib/components/Sidebar.svelte';

  let transcript = "";

  async function runTranscription() {
    transcript = await invoke("transcribe_audio", {
      filePath: "C:/Users/altho/OneDrive/文档/Sound Recordings/Recording.wav"
    });
  }
</script>

<div class="flex min-h-screen bg-gradient-to-br from-slate-50 to-slate-200">
  <Sidebar active="None" />
  <main class="flex-1 p-10 flex flex-col gap-10">
    <button class="px-6 py-2 rounded bg-primary text-white font-semibold shadow hover:bg-primary/90 transition w-40 mb-4" on:click={runTranscription}>Transcribe</button>
    <p class="text-lg text-gray-700 bg-white rounded p-4 shadow max-w-xl">{transcript}</p>
  </main>
</div>
<link rel="stylesheet" href="/src/styles/global.css">