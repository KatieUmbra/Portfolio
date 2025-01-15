<script lang="ts">
    import type { PageData } from "./$types";
    import "../../app.css";
    import hljs from "highlight.js";
    import { tick } from "svelte";

    let { data }: { data: PageData } = $props();
    let posts = data.posts;

    $effect.pre(() => {
        tick().then(() => {
            hljs.highlightAll();
        });
    });
</script>

<div
    class="max-h-95 bg-95 min-h-95 grid w-full items-center justify-center overflow-scroll">
    <h1 class="m-3 mt-20 text-5xl">Blog Posts</h1>
    {#each posts as element}
        <div class="window95 m-3">
            <div class="grid p-2">
                <a
                    href={"/blog/posts?id=" + element.id}
                    class="min-w-40vw m-1 text-3xl font-bold"
                    >{element.title}</a>
                <p class="txt95 m-1">{element.description}</p>
                <div class="flex">
                    <p class="txt95 m-1">by: <b>{element.creator}</b></p>
                    <p class="txt95 m-1">Likes: {element.likes}</p>
                </div>
            </div>
        </div>
    {/each}
</div>
