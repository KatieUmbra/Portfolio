<script lang="ts">
    import type { PageData } from "./$types";
    import "../../app.css";

    let { data }: { data: PageData } = $props();
    let posts = data.posts;
</script>

<div class="max-h-95 bg-95 min-h-95 grid w-full items-center justify-center overflow-scroll">
<!--WRAPPER-->
<div class="border95 bg95-gray max-w-6xl p-1">
<div class="flex bg-purple-800"> <div class="mr-auto flex">
<img alt="logo of the website, it's a windows 95 styled cat coming out of a folder" class="img95 m-1" src="/assets/logo kathy dev2.png"/>
<p class=" mt-0.5 text-white">Blog Posts</p> </div>
<div class="ml-auto flex">
<div class="grid place-content-center btn95-gray">_</div>
<div class="grid place-content-center btn95-gray">▫</div>
<div class="grid place-content-center btn95-gray">⨉</div>
</div> </div> <div class="bg-white border95-inv mt-1">
<!--WRAPPER END-->
<div class="bg-gray-ccc border95-inv">

    <h1 class="m-3 text-5xl">Blog Posts</h1>
    {#each posts as element}
        <div class="border95-inv m-1">
            <div class="grid p-2">
                <a
                    href={"/blog/post?id=" + element.id}
                    class="min-w-40vw m-1 text-5xl font-bold"
                    >{element.title}</a>
                <p class="txt95 m-1">{element.description}</p>
                <div class="flex">
                    <p class="txt95 m-1">by: <b>{element.creator}</b></p>
                    <p class="txt95 m-1">Likes: {element.likes}</p>
                    <p class="txt95 m-1">Created: {element.localTime.toLocaleDateString(undefined, {
                        weekday: "long",
                        year: "numeric",
                        month: "long",
                        day: "numeric"
                    })}</p>
                </div>
            </div>
        </div>
    {/each}

</div></div>
</div>
</div>
