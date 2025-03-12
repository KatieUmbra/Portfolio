<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/state";
    import type { BlogPost } from "$lib/backend/schema/blog";
    import type { BlogComment } from "$lib/backend/schema/blog";
    import type { Claims } from "$lib/backend/schema/user";
    import { windowTitle } from "$lib/stores/global";

    let comment: string = $state("");

    let {
        form,
        data,
    }: {
        form: any;
        data: {
            post: BlogPost;
            currentUser: Claims;
            comments: BlogComment[] | null;
            localTime: Date;
        };
    } = $props();

    let comments: BlogComment[] = data.comments as BlogComment[];

    console.log(data.post.id);

    windowTitle.set(`Post - ${data.post.title}`);

    let isCreator: boolean = $derived.by(() => {
        if (data.currentUser != null) {
            return (
                data.post.creator == data.currentUser?.username ||
                data.currentUser?.rank == 0
            );
        }
        return false;
    });

    function getId(): number {
        const postId: number = parseInt(
            page.url.searchParams.get("id") as string,
        );
        return postId;
    }

    let delText = $state("Delete");

    function deleteHandler() {
        if (delText == "Delete") {
            delText = "Are you sure?";
        } else {
            goto(`/blog/delete?id=${getId()}`);
        }
    }
</script>

<svelte:head>
    <title>{data.post.title}</title>
</svelte:head>

<div class="minmax-w-80lvw grid">
    <div class="flex">
        <a href="/blog" class="btn95 m-3"><div>Go Back</div></a>
        <a href="/blog/post?id={getId() - 1}" class="btn95 m-3 place-self-start"
            >&lt; Prev</a>
        {#if isCreator}
            <button onclick={deleteHandler} class="btn95 m-3 ml-auto">
                {delText}
            </button>
            <a href="/blog/new_post?edit={getId()}" class="btn95 m-3">Edit</a>
            <a href="/blog/post?id={getId() + 1}" class="btn95 m-3"
                >Next &gt;</a>
        {:else}
            <a href="/blog/post?id={getId() + 1}" class="btn95 m-3 ml-auto"
                >Next &gt;</a>
        {/if}
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
    <hr />
    {#if data.post.id != -1}
        <div class="grid">
            <form class="grid" method="POST">
                <p class="m-3 text-xl font-bold">
                    Comment something about this article!
                </p>
                <textarea
                    name="comment"
                    class="txt-in95 comment-box m-3 h-48 w-96 resize-none place-self-center"
                    bind:value={comment}
                    placeholder="comment"></textarea>

                <button type="submit" class="btn95 m-3 max-w-min">
                    Submit
                </button>
                {#if form?.failure}
                    <p class="m-3">{form?.message}</p>
                {/if}
            </form>
            <h2 class="m-3 place-self-start text-xl font-bold">Comments</h2>
            <div>
                {#if data.comments?.length == 0}
                    {#each comments as comment}
                        <p>{comment.content}</p>
                    {/each}
                {:else}
                    <p class="m-3">There are no comments! :(</p>
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    :global(.markdown) {
        display: grid;
        max-width: calc(100% - 1.5rem);
        min-width: calc(100% - 1.5rem);
        :global(*) {
            overflow-wrap: break-word;
            min-width: 100%;
            max-width: 100%;
            hyphens: auto;
        }
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
    .comment-box {
        max-width: calc(100% - 10rem);
        min-width: calc(100% - 1.5rem);
    }
</style>
