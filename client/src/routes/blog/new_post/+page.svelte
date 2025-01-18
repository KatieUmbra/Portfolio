<script lang="ts">
    import "../../../app.css";
    import Markdown from "svelte-exmarkdown";
    import { gfmPlugin } from "svelte-exmarkdown/gfm";
    import type { Plugin } from "svelte-exmarkdown";
    import rehypeShikiFromHighlighter from "@shikijs/rehype/core";
    import { createHighlighterCore } from "shiki";
    import type { ActionData, PageData } from "../$types";

    let title: string = $state("");
    let description: string = $state("");
    let md: string = $state("");

    let { form, data }: { form: ActionData; data: PageData } = $props();

    const plugins = [gfmPlugin(), { renderer: { h1: "b", h2: "b", h3: "b" } }];

    const shikiPluginPromise = createHighlighterCore({
        themes: [import("shiki/themes/light-plus.mjs")],
        langs: [
            import("shiki/langs/rust.mjs"),
            import("shiki/langs/cpp.mjs"),
            import("shiki/langs/cmake.mjs"),
            import("shiki/langs/javascript.mjs"),
            import("shiki/langs/sql.mjs"),
            import("shiki/langs/typescript.mjs"),
            import("shiki/langs/kotlin.mjs"),
            import("shiki/langs/c.mjs"),
            import("shiki/langs/css.mjs"),
            import("shiki/langs/html.mjs"),
            import("shiki/langs/csharp.mjs"),
            import("shiki/langs/xml.mjs"),
            import("shiki/langs/toml.mjs"),
            import("shiki/langs/jsonl.mjs"),
            import("shiki/langs/yaml.mjs"),
            import("shiki/langs/python.mjs"),
        ],
        loadWasm: import("shiki/wasm"),
    }).then((highlighter): Plugin => {
        return {
            rehypePlugin: [
                rehypeShikiFromHighlighter,
                highlighter,
                { theme: "light-plus" },
            ],
        };
    });

    if (data.status == 200) {
        title = data.post.title;
        description = data.post.description;
        md = data.post.content;
    }
</script>

<svelte:head>
    <title>New post {title}</title>
</svelte:head>

<form class="bg-gray-ccc w-max" method="POST">
    <div class="grid">
        <p class="m-3 text-xl font-bold">Create a new post</p>
        <input
            type="text"
            class="txt-in95 m-3"
            name="title"
            bind:value={title}
            placeholder="title" />
        <input
            type="text"
            class="txt-in95 m-3"
            name="description"
            bind:value={description}
            placeholder="description" />
        <div class="grid grid-cols-2 gap-2 p-3">
            <textarea
                name="content"
                placeholder="Post Content"
                class="txt-in95 h-96 w-96 min-w-full resize-none"
                bind:value={md}></textarea>
            <div class="txt95 max-h-96 w-0 min-w-full overflow-scroll">
                {#await shikiPluginPromise}
                    <Markdown {md} {plugins} />
                {:then shikiPlugin}
                    <Markdown {md} plugins={[shikiPlugin, ...plugins]} />
                {:catch}
                    <Markdown {md} {plugins} />
                {/await}
            </div>
        </div>
        <button type="submit" class="btn95 m-3 max-w-min">
            <div>Submit</div>
        </button>
        {#if form?.failure}
            <p class="m-3">{form?.message}</p>
        {/if}
    </div>
</form>
