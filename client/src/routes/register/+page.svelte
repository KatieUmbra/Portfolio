<script lang="ts">
    import "../../app.css";
    import type { ActionData } from "./$types";

    interface Props {
        form: ActionData;
    }

    let { form }: Props = $props();
</script>

<!--WRAPPER-->
<div class="border95 bg95-gray max-w-6xl p-1">
<div class="flex bg-gradient95"> <div class="mr-auto flex">
<img alt="logo of the website, it's a windows 95 styled cat coming out of a folder" class="img95 m-1" src="/assets/logo kathy dev2.png"/>
<p class=" mt-0.5 text-white">Register</p> </div>
<div class="ml-auto flex">
<div class="grid place-content-center btn95-gray">_</div>
<div class="grid place-content-center btn95-gray">▫</div>
<div class="grid place-content-center btn95-gray">⨉</div>
</div> </div> <div class="bg-gray-ccc border95-inv mt-1">
<!--WRAPPER END-->
    <form action="/register" method="POST">
    <div class="grid">
        <p class="m-3 text-xl font-bold">Register</p>
        <input
            type="text"
            class="txt-in95 m-3"
            name="username"
            placeholder="Username" />
        <input
            type="text"
            class="txt-in95 m-3"
            name="displayName"
            placeholder="Display Name" />
        <input
            type="email"
            class="txt-in95 m-3"
            name="email"
            placeholder="Email" />
        <input
            type="password"
            class="txt-in95 m-3"
            name="password"
            placeholder="Password" />
        <input
            type="password"
            class="txt-in95 m-3"
            name="verifyPassword"
            placeholder="Verify Password" />
        <button type="submit" class="btn95 m-3 max-w-min"
            ><div>Submit</div></button>
        {#if form?.failure}
            <p class="m-3">{form?.message}</p>
        {/if}
        </div>
    </form>
</div>
</div>
