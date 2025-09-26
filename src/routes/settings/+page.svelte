<script lang="ts">
import { Input } from '$lib/components/ui/input';
import { Label } from '$lib/components/ui/label';
import { Store } from '@tauri-apps/plugin-store';

let globalSetting = {
    whisper: {
        model_path: ""
    }
};

(async () => {
    // Store 会在 JavaScript 绑定时自动加载。
    const store = await Store.load('store.json');

    let model_path_raw = await store.get('whisper.model_path');
    let model_path = typeof model_path_raw === "string" ? model_path_raw : "";
    globalSetting.whisper.model_path = model_path;

    let pre_converted_models = [
        "tiny",
        "tiny.en",
        "tiny-q5_1",
        "tiny.en-q5_1",
        "tiny-q8_0",
        "base",
        "base.en",
        "base-q5_1",
        "base.en-q5_1",
        "base-q8_0",
        "small",
        "small.en",
        "small.en-tdrz",
        "small-q5_1",
        "small.en-q5_1",
        "small-q8_0",
        "medium",
        "medium.en",
        "medium-q5_0",
        "medium.en-q5_0",
        "medium-q8_0",
        "large-v1",
        "large-v2",
        "large-v2-q5_0",
        "large-v2-q8_0",
        "large-v3",
        "large-v3-q5_0",
        "large-v3-turbo",
        "large-v3-turbo-q5_0",
        "large-v3-turbo-q8_0",
    ]
    let model_key = "";
    for (let key in pre_converted_models) {
        if (model_path.endsWith("ggml-" + pre_converted_models[key] + ".bin") ) {
            model_key = pre_converted_models[key];
            break;
        }
    }

    await store.save();
})();


function handleModelChange(event: Event & { currentTarget: EventTarget & HTMLInputElement; }) {
    let input = event.currentTarget;
    if (!input.value) return;
    globalSetting.whisper.model_path = input.value;

    (async () => {
        const store = await Store.load('store.json');
        await store.set('whisper.model_path', globalSetting.whisper.model_path);
        await store.save();
    })();
}

</script>


<div class="flex w-full max-w-sm flex-col gap-1.5">
  <Label for="specify-model">Model File</Label>
  <Input type="value" id="specify-model" placeholder="" onchange={handleModelChange} value={globalSetting.whisper.model_path} />
  <p class="text-muted-foreground text-sm">Enter a local model path. e.g. C:\path\to\ggml-small.bin</p>
</div>