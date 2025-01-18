<script lang="ts">
    import "../../../app.css";
    import { page } from "$app/state";

    let { data } = $props();

    function getId(): number {
        const postId: number = parseInt(
            page.url.searchParams.get("id") as string,
        );
        return postId;
    }
</script>

<svelte:head>
    <title>{data.post.title}</title>
</svelte:head>

<div class="grid">
    <div class="flex">
        <a href="/blog" class="btn95 m-3"><div>Go Back</div></a>
        <a href="/blog/post?id={getId() - 1}" class="btn95 m-3 place-self-start"
            ><div>&lt; Prev</div></a>
        <a href="/blog/new_post?edit={getId()}" class="btn95 m-3 ml-auto"
            ><div>Edit</div></a>
        <a href="/blog/post?id={getId() + 1}" class="btn95 m-3"
            ><div>Next &gt;</div></a>
    </div>
    <h1 class="txt95 m-3 text-6xl font-bold">{data.post.title}</h1>
    <p class="txt95 m-3 text-2xl">{data.post.description}</p>
    <div class="flex">
        <p class="txt95 m-3 text-2xl">by: <b>{data.post.creator}</b></p>
        <p class="txt95 m-3 text-2xl">
            {data.localTime.toLocaleDateString()}
        </p>
        <p class="txt95 m-3 text-2xl">
            {data.localTime.toLocaleTimeString()}
        </p>
        <div class="m-3 flex">
            <button class="btn95 text-2xl"><div>Like</div></button>
            <p class="ml-3 mt-1.5 text-2xl">{data.post.likes}</p>
        </div>
    </div>
    <div class="markdown txt95 m-3 grid place-self-center">
        {@html data.post.content}
    </div>
</div>

<style>
    :global(.markdown) {
        max-width: 80lvw;
        min-width: 80lvw;
        display: grid;
        :global(h2) {
            font-size: 24pt;
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
            padding: 1rem;
        }
        :global(img) {
            border-width: 2px;
            border-color: #eee;
            border-top-color: black;
            border-left-color: black;
            margin-top: 0.75rem;
            margin-bottom: 0.75rem;
            margin: auto;
        }
    }
</style>
