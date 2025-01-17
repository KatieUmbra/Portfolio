<script lang="ts">
    import "../../../app.css";
    import { page } from "$app/state";

    let { data } = $props();

    function getId(): number {
        const postId: number = parseInt(page.url.searchParams.get("id") as string);
        return postId;
    }
</script>

<svelte:head>
    <title>{data.post.title}</title>
</svelte:head>

<!--WRAPPER-->
<div class="border95 bg95-gray max-w-8xl p-1 m-5">
<div class="flex bg-gradient95"> <div class="mr-auto flex">
<img alt="logo of the website, it's a windows 95 styled cat coming out of a folder" class="img95 m-1" src="/assets/logo kathy dev2.png"/>
<p class=" mt-0.5 text-white">{data.post.title}</p> </div>
<div class="ml-auto flex">
<div class="grid place-content-center btn95-gray">_</div>
<div class="grid place-content-center btn95-gray">▫</div>
<div class="grid place-content-center btn95-gray">⨉</div>
</div> </div> <div class="bg-gray-ccc border95-inv mt-1">
<!--WRAPPER END-->
    <div class="grid">
        <div class="flex">
            <a href="/blog" class="m-3 btn95"><div>Go Back</div></a>
            <a href="/blog/post?id={getId() - 1}" class="place-self-start m-3 btn95"><div>&lt; Prev</div></a>
            <a href="/blog/post?id={getId() + 1}" class="ml-auto m-3 btn95"><div>Next &gt;</div></a>
        </div>
        <h1 class="txt95 m-3 text-6xl font-bold">{data.post.title}</h1>
        <p class="txt95 m-3 text-2xl">{data.post.description}</p>
        <div class="flex">
            <p class="txt95 text-2xl m-3">by: <b>{data.post.creator}</b></p>
            <p class="txt95 text-2xl m-3">{data.localTime.toLocaleDateString()}</p>
            <p class="txt95 text-2xl m-3">{data.localTime.toLocaleTimeString()}</p>
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

