<script lang="ts">
    import "../app.css";
</script>

<div class="taskbar95 flex min-w-full p-1">
    <div class="tb-btn95 flex">
        <img
            alt="website icon"
            class="mr-0.5 self-center"
            src="/assets/logo kathy dev2.png" />
        <p class="font-bold">Start</p>
    </div>
    <a class="tb-btn95 w-32" href="/"><div>Home</div></a>
    <a class="tb-btn95 w-32" href="/login"><div>Login</div></a>
    <a class="tb-btn95 w-32" href="/register"><div>Register</div></a>
    <a class="tb-btn95 w-32" href="/info"><div>Info</div></a>
    <a class="tb-btn95 w-32" href="/blog/new_post"><div>Make Post</div></a>
    <a class="tb-btn95 w-32" href="/blog"><div>Blog</div></a>
</div>

<slot />

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
