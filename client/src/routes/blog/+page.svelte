<script lang="ts">
    import type { PageData } from "./$types";
    import "../../app.css";

    let { data }: { data: PageData } = $props();
    let posts = data.posts;
</script>

<div class="bg-gray-ccc border95-inv">
    <h1 class="m-3 text-5xl">Blog Posts</h1>
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
                        Created: {element.localTime.toLocaleDateString(
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
