<script lang="ts">
    import "../../../app.css";
    import Markdown from "svelte-exmarkdown";
    import { gfmPlugin } from "svelte-exmarkdown/gfm";
    import type { Plugin } from "svelte-exmarkdown";
    import rehypeShikiFromHighlighter from "@shikijs/rehype/core";
    import { createHighlighterCore } from "shiki";
    import type { ActionData } from "../$types";

    let title = $state("");

    interface Props {
        form: ActionData;
    }
    let { form }: Props = $props();
    const plugins = [gfmPlugin() /* { renderer: { h1: "b" } }*/];

    const shikiPluginPromise = createHighlighterCore({
        themes: [import("shiki/themes/light-plus.mjs")],
        langs: [
        import("shiki/langs/rust.mjs"), 
        import("shiki/langs/cpp.mjs"),
        import("shiki/langs/cmake.mjs"),
        import("shiki/langs/javascript.mjs"),
        import("shiki/langs/sql.mjs"),
        import("shiki/langs/typescript.mjs"),
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

    let md: string = $state("");
</script>

<svelte:head>
    <title>New post {title}</title>
</svelte:head>

<div class="max-h-95 bg-95 grid min-h-95 w-full items-center justify-center overflow-scroll">
<!--WRAPPER-->
<div class="border95 bg95-gray max-w-6xl p-1">
<div class="flex bg-purple-800"> <div class="mr-auto flex">
<img alt="logo of the website, it's a windows 95 styled cat coming out of a folder" class="img95 m-1" src="/assets/logo kathy dev2.png"/>
<p class=" mt-0.5 text-white">home page</p> </div>
<div class="ml-auto flex">
<div class="grid place-content-center btn95-gray">_</div>
<div class="grid place-content-center btn95-gray">▫</div>
<div class="grid place-content-center btn95-gray">⨉</div>
</div> </div> <div class="bg-white border95-inv mt-1">
<!--WRAPPER END-->
    <form class="w-max bg-gray-ccc" method="POST">
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
</div>
</div>
</div>
