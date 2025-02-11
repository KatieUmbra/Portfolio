<script lang="ts">
    import type { BlogPost } from "$lib/backend/schema/blog";
    import { windowTitle } from "$lib/stores/global";

    windowTitle.set("Kathy's Blog");

    const { data }: { data: { posts: BlogPost[] } } = $props();
    const posts = data.posts;
</script>

<div class="bg-gray-ccc border95-inv">
    <div class="minmax-w-60lvw flex">
        <h1 class="m-3 mr-auto text-5xl">Blog Posts</h1>
        <a class="btn95 m-5 ml-auto" href="/blog/new_post">New post</a>
    </div>
    {#if posts.length == 0}
        <p class="m-3 text-xl">There are no posts yet! :(</p>
    {/if}
    {#each posts as element}
        <div class="border95-inv m-1">
            <div class="grid p-2">
                <a
                    href={"/blog/post?id=" + element.id}
                    class="min-w-40vw m-1 text-2xl font-bold"
                    >{element.title}</a>
                <p class="txt95 m-1">{element.description}</p>
                <div class="flex">
                    <p class="txt95 m-1">by: <b>{element.creator}</b></p>
                    <p class="txt95 m-1">Likes: {element.likes}</p>
                    <p class="txt95 m-1">
                        Created: {(element.creation as Date).toLocaleDateString(
                            undefined,
                            {
                                weekday: "long",
                                year: "numeric",
                                month: "long",
                                day: "numeric",
                            },
                        )}
                    </p>
                </div>
            </div>
        </div>
    {/each}
</div>
