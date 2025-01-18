<script lang="ts">
    import WindowWrapper from "$lib/WindowWrapper.svelte";
    import "../app.css";
    import { fade } from "svelte/transition";
</script>

<div class="taskbar95 flex min-w-full p-1">
    <a href="/" class="tb-btn95 flex w-16 items-center justify-items-center">
        <img
            alt="website icon"
            class="mr-0.5"
            src="/assets/logo kathy dev2.png" />
        <p class="font-bold">Start</p>
    </a>
    <a class="tb-btn95 w-32" href="/login"><div>Login</div></a>
    <a class="tb-btn95 w-32" href="/register"><div>Register</div></a>
    <a class="tb-btn95 w-32" href="/info"><div>Info</div></a>
    <a class="tb-btn95 w-32" href="/blog/new_post"><div>Make Post</div></a>
    <a class="tb-btn95 w-32" href="/blog"><div>Blog</div></a>
</div>
<div
    transition:fade
    class="max-h-95 bg-95 min-h-95 grid w-full items-center justify-center overflow-scroll">
    <WindowWrapper>
        <slot />
    </WindowWrapper>
</div>

<style>
    @font-face {
        font-family: "W95FA";
        src: url("$lib/font/w95fa.woff") format("woff");
    }

    :global(*) {
        font-family: "W95FA", serif !important;
        font-family: "W95FA", sans-serif !important;
    }
</style>
