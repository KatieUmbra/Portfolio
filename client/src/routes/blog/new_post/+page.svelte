<script lang="ts">
    import Markdown from "svelte-exmarkdown";
    import { gfmPlugin } from "svelte-exmarkdown/gfm";
    import { BlogPost } from "$lib/backend/schema/blog";
    import { windowTitle, shikiPromise } from "$lib/stores/global";
    import { get } from "svelte/store";

    windowTitle.set("New post");

    let title: string = $state("");
    let description: string = $state("");
    let md: string = $state("");

    let {
        form,
        data,
    }: {
        form: any;
        data: { status: number; post: BlogPost };
    } = $props();

    const plugins = [gfmPlugin(), { renderer: { h1: "b", h2: "b", h3: "b" } }];

    const shikiPluginPromise = get(shikiPromise);

    if (data.status == 200) {
        title = data.post.title;
        description = data.post.description;
        md = data.post.content;
    }
</script>

<svelte:head>
    {#if title == ""}
        <title>New post</title>
    {:else}
        <title>{title}</title>
    {/if}
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
        <button type="submit" class="btn95 m-3 max-w-min"> Submit </button>
        {#if form?.failure}
            <p class="m-3">{form?.message}</p>
        {/if}
    </div>
</form>
