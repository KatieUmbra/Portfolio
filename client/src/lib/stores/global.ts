import rehypeShikiFromHighlighter from "@shikijs/rehype/core";
import type { Plugin } from "svelte-exmarkdown";
import { createHighlighterCore } from "shiki";
import { writable, readable } from "svelte/store";

export let windowTitle = writable("");

export let shikiPromise = readable(createHighlighterCore({
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
    }));

