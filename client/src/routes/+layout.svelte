<script lang="ts">
    import WindowWrapper from "$lib/WindowWrapper.svelte";
    import "../app.css";
    import "$lib/styles/layout.css";
    import "$lib/styles/95components.css";
    import "$lib/styles/tailwind.css";
    import "$lib/styles/global.css";
</script>

<div class="taskbar95 flex min-w-full p-1">
    <a href="/" class="tb-start95">
        <img alt="website icon" class="mr-0.5" src="/assets/logo.png" />
        <p class="font-bold">Start</p>
    </a>
    <a class="tb-btn95" href="/portfolio"
        ><img alt="portfolio icon" class="mr-0.5" src="/assets/portfolio.png" />
        <p>Portfolio</p></a>
    <a class="tb-btn95" href="/blog"
        ><img alt="blog icon" class="mr-0.5" src="/assets/blog.png" />
        <p>Blog</p></a>
    <a class="tb-btn95" href="/contact"
        ><img alt="contact icon" class="mr-0.5" src="/assets/contact.png" />
        <p>Contact</p></a>
    <a class="tb-btn95" href="/faq"
        ><img alt="f.a.q. icon" class="mr-0.5" src="/assets/faq.png" />
        <p>Faq</p></a>
    <div class="ml-4 mr-4"></div>
</div>
<div
    class="max-h-95 bg-95 min-h-95 flex w-full items-center justify-center overflow-scroll">
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

    :global(hr) {
        color: black;
    }
</style>
