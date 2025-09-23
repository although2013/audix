<script>
  import Button from '$lib/components/ui/button/button.svelte';
  import Card from '$lib/components/ui/card/card.svelte';
  import CardContent from '$lib/components/ui/card/card-content.svelte';
  import {
    Table,
    TableHeader,
    TableHead,
    TableRow,
    TableCell,
    TableBody,
  } from '$lib/components/ui/table';
  import Input from "$lib/components/ui/input/input.svelte"
  import { CheckCircle, Loader2 } from '@lucide/svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';

  
  /**
     * @type {any[]}
     */
  let files = [];

  /**
     * @param {{ target: { files: Iterable<any> | ArrayLike<any>; }; }} event
     */

  async function handleFileChange(event) {
    const selectedFiles = Array.from(event.target.files);
    files = selectedFiles.map(file => ({
      name: file.name,
      size: `${Math.round(file.size / 1024)}kb`,
      duration: "", // 可替换为实际音频时长
      status: "uploading"
    }));

    // 上传每个文件（示例：上传到 /api/upload，需后端支持）
    for (const file of selectedFiles) {
      const formData = new FormData();
      formData.append('file', file);
      try {
        // 这里假设有 /api/upload 接口，实际可根据你的API调整
        await fetch('/api/upload', {
          method: 'POST',
          body: formData
        });
        files = files.map(f => f.name === file.name ? { ...f, status: 'finished' } : f);
      } catch (e) {
        files = files.map(f => f.name === file.name ? { ...f, status: 'error' } : f);
      }
    }
  }

</script>


<div class="flex min-h-screen bg-gradient-to-br from-slate-50 to-slate-200">
  <Sidebar active="Jobs" />
  <!-- Main Content -->
  <main class="flex-1 p-10 flex flex-col gap-10">
    <div class="flex gap-8 flex-wrap">
      <Card class="flex-1 min-w-[320px] flex items-center justify-center h-48 bg-gradient-to-br from-slate-100 to-slate-200 border-0 shadow-md">
        <CardContent class="flex items-center justify-center w-full h-full">
          <span class="text-2xl font-bold text-gray-700 tracking-tight">realtime recording</span>
        </CardContent>
      </Card>
      <Card class="flex-1 min-w-[320px] flex flex-col items-center justify-center h-48 bg-gradient-to-br from-slate-100 to-slate-200 border-0 shadow-md">
        <CardContent class="flex flex-col items-center justify-center w-full h-full gap-4">
          <span class="text-2xl font-bold text-gray-700 mb-2 tracking-tight">From Files</span>
          <label class="relative inline-block cursor-pointer">
            <Input type="file" multiple accept=".wav" class="cursor-pointer w-full h-full hidden" onchange={handleFileChange} />
            <Loader2 class="w-4 h-4 animate-spin hidden" />
            选择文件
          </label>
        </CardContent>
      </Card>
    </div>

    <Card class="shadow-lg border-0">
      <CardContent class="p-0">
        <Table>
          <TableHeader class="bg-slate-100">
            <TableRow>
              <TableHead class="font-bold text-base py-3">file name</TableHead>
              <TableHead class="font-bold text-base py-3">size</TableHead>
              <TableHead class="font-bold text-base py-3">duration</TableHead>
              <TableHead class="font-bold text-base py-3">status</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each files as file}
              <TableRow class="hover:bg-slate-50 border-t transition">
                <TableCell class="py-2">{file.name}</TableCell>
                <TableCell class="py-2">{file.size}</TableCell>
                <TableCell class="py-2">{file.duration}</TableCell>
                <TableCell class="py-2">
                  {#if file.status === 'finished'}
                    <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-green-100 text-green-700 text-xs font-semibold">
                      <CheckCircle class="w-3 h-3" /> finished
                    </span>
                  {:else}
                    <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full bg-gray-200 text-gray-500 text-xs font-semibold">
                      <Loader2 class="w-3 h-3 animate-spin" /> pending
                    </span>
                  {/if}
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  </main>
</div>

<style>
  /* 可根据需要自定义样式 */
</style>

<link rel="stylesheet" href="/src/styles/global.css">


