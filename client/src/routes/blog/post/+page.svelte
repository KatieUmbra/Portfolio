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

    let delText = $state("Delete");

    function getId(): number {
        const postId: number = parseInt(
            page.url.searchParams.get("id") as string,
        );
        return postId;
    }

    function deleteHandler() {
        if (delText == "Delete") {
            delText = "Are you sure?";
        } else {
            goto(`/blog/delete?id=${getId()}`);
        }
    }

    function parseDatetime(dt: string): string {
        const parsedDate = new Date(dt);
        const localTime = new Date(
            parsedDate.getTime() - parsedDate.getTimezoneOffset() * 60 * 1000,
        );
        return localTime.toLocaleString();
    }
</script>

<svelte:head>
    <title>{data.post.title}</title>
</svelte:head>

<div class="minmax-w-80lvw max-h-80lvh grid">
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
    <div class="txt95 m-3">
        <h1 class="m-3 text-6xl font-bold">{data.post.title}</h1>
        <hr />
        <p class="m-3 text-2xl">
            {data.post.description}
        </p>
        <div class="m-3 flex text-gray-400">
            <p class="mr-1">by: <b>{data.post.creator}</b></p>
            <p class="mr-1">
                at:
                {data.localTime.toLocaleTimeString()}
            </p>
            <p class="mr-1">
                {data.localTime.toLocaleDateString()}
            </p>
            <div class="mr-1 flex">
                <button class="mr-1"><u>Like</u></button>
                <p class="">{data.post.likes}</p>
            </div>
        </div>
    </div>
    <div class="markdown txt95 m-3 grid place-self-center">
        {@html data.post.content}
    </div>
    <hr />
    {#if data.post.id != -1}
        <div class="grid">
            {#if data.currentUser != null}
                <form class="grid" method="POST">
                    <p class="m-3 text-xl font-bold">
                        Comment something about this article!
                    </p>
                    <textarea
                        name="comment"
                        class="txt-in95 comment-box m-3 h-20 w-96 resize-none place-self-center"
                        bind:value={comment}
                        placeholder="comment"></textarea>

                    <button type="submit" class="btn95 m-3 max-w-min">
                        Submit
                    </button>
                    {#if form?.failure}
                        <p class="m-3">{form?.message}</p>
                    {/if}
                </form>
            {/if}
            <h2 class="m-3 place-self-start text-xl font-bold">Comments</h2>
            <div>
                {#if data.comments?.length == 0}
                    <p class="m-3">There are no comments! :(</p>
                {:else}
                    {#each data.comments!! as comment}
                        <div class="txt95 m-3 p-3">
                            <div class="flex">
                                <p class="mr-2">by: <b>{comment.creator}</b></p>
                                <p>
                                    at:
                                    {parseDatetime(comment.creation as string)}
                                </p>
                            </div>
                            <hr />
                            <p class="p-3">{comment.content}</p>
                        </div>
                    {/each}
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
