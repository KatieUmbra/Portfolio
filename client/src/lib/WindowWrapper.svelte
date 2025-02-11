<script lang="ts">
    let { children }: { children?: any } = $props();
    import { windowTitle } from "./stores/global";

    let title = $state("");
    windowTitle.subscribe((it) => {
        title = it;
    });
</script>

<div class="border95 bg95-gray m-3 overflow-scroll p-1">
    <div class="bg-gradient95 flex">
        <div class="mr-auto flex">
            <img
                alt="logo of the website, it's a windows 95 styled cat coming out of a folder"
                class="img95 m-1"
                src="/assets/logo512.png" />
            <p class=" mr-3 mt-0.5 text-white">{title}</p>
        </div>
        <div class="ml-auto flex">
            <div class="btn95-gray grid place-content-center">_</div>
            <div class="btn95-gray grid place-content-center">▫</div>
            <div class="btn95-gray grid place-content-center">⨉</div>
        </div>
    </div>
    <div class="bg-gray-ccc border95-inv mt-1 overflow-scroll">
        {@render children()}
    </div>
</div>
