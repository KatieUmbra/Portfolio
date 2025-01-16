<script lang="ts">
    import type { PageData } from "./$types";
    import "../../../app.css";
    import { page } from "$app/state";

    let { data }: { data: PageData } = $props();
    const date = new Date(data.post.creation);

    function getId(): number {
        const postId: number = parseInt(page.url.searchParams.get("id") as string);
        return postId;
    }
</script>

<div class="max-h-95 bg-95 grid min-h-95 w-full items-center justify-center overflow-scroll">
    <div class="m-10 window95">
    <div class="grid">
        <div class="flex">
            <a href="/blog" class="m-3 btn95"><div>Go Back</div></a>
            <a href="/blog/posts?id={getId() - 1}" class="place-self-start m-3 btn95"><div>Prev</div></a>
            <a href="/blog/posts?id={getId() + 1}" class="ml-auto m-3 btn95"><div>Next</div></a>
        </div>
        <h1 class="txt95 m-3 text-6xl font-bold">{data.post.title}</h1>
        <p class="txt95 m-3 text-2xl">{data.post.description}</p>
        <div class="flex">
            <p class="txt95 text-2xl m-3">by: <b>{data.post.creator}</b></p>
            <p class="txt95 text-2xl m-3">{date.toLocaleDateString()}</p>
            <p class="txt95 text-2xl m-3">{date.toLocaleTimeString()}</p>
            <div class="m-3 flex">
                <button class="btn95 text-2xl"><div>Like</div></button>
                <p class="mt-1.5 ml-3 text-2xl">{data.post.likes}</p>
            </div>
        </div>
        <div class="grid place-self-center markdown txt95 m-3">
            {@html data.post.content}
        </div>
        </div>
    </div>
</div>

<style>
:global(.markdown) {
    max-width: 80lvw;
    min-width: 80lvw;
    display: grid;
    :global(*) {
        font-size: 18pt;
    }
    :global(h2) {
        font-size: 35pt;
    }
    :global(a) {
        font-style: italic;
        text-decoration: underline;
        color: blue;
    }
    :global(p) {
        margin-top: 0.75rem;
        margin-bottom: 0.75rem;
    }
    :global(pre) {
        overflow: scroll;
        margin-top: 0.75rem;
        margin-bottom: 0.75rem;
        background: white;
    }
    :global(img) {
        margin-top: 0.75rem;
        margin-bottom: 0.75rem;
    }
}
</style>
