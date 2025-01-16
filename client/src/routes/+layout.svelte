<script lang="ts">
    import "../app.css";
</script>

<div class="bg-white min-w-full p-2 flex">
<a class="btn95 m-2" href="/"><div>Home</div></a>
<a class="btn95 m-2" href="/login"><div>Login</div></a>
<a class="btn95 m-2" href="/register"><div>Register</div></a>
<a class="btn95 m-2" href="/info"><div>Info</div></a>
<a class="btn95 m-2" href="/blog/post"><div>Make Post</div></a>
<a class="btn95 m-2" href="/blog"><div>Blog</div></a>
</div>

<slot />

<style>
@font-face {
    font-family: "W95FA";
    src: url("$lib/font/w95fa.woff") format("woff")
}

:global(*) {
    font-family: "W95FA", serif !important;
    font-family: "W95FA", sans-serif !important;
}
</style>
