<script lang="ts">
    import type { PageData } from "./$types";
    import "../../app.css";

    let { data }: { data: PageData } = $props();
    console.log(data);
</script>


<div class="max-h-95 bg-95 grid min-h-95 w-full items-center justify-center overflow-scroll">
    {#each data.posts as element}
    <div class="m-3 p-2 grid bg-white">
        <a href={"/blog/posts?id=" + element.id} class="min-w-40vw m-1 text-xl font-bold">{element.title}</a>
        <p class="txt95 m-1">{element.description}</p>
        <div class="flex">
            <p class="txt95 m-1">by: <b>{element.creator}</b></p>
            <p class="txt95 m-1">Likes: {element.likes}</p>
        </div>
    </div>
    {/each}
</div>
